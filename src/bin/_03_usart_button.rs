/* Copyright (c) 2024 Perlatecnica APS ETS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/****************************************************
*            RAPID PROTOTYPING WITH NUCLEO          *
* Example Code 4: USART Button                      *
* Author: Salvatore Bramante                        *
* Organization: Perlatecnica APS ETS                *
*****************************************************/
#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};

use heapless::String;

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{Config, Uart,};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

static BLINK_MS: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn led_task(led: AnyPin) {
    // Configure the LED pin as a low -speed output and obtain a handler
    // Initialize LED output to Low
    // On the Nucleo FR401 theres an on-board LED connected to pin PA5
    let mut led = Output::new(led, Level::Low, Speed::Low);

    loop {
        let del = BLINK_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(del.into())).await;
        led.toggle();
    }
}

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
});
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());

    // Configure the button pin (if needed) and obtain handler.
    // On the Nucleo FR401 there is a button connected to pin PC13.
    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);

    //Configure UART
    let mut usart: Uart<'_, embassy_stm32::mode::Blocking> = Uart::new_blocking(p.USART2, p.PA3, p.PA2, Config::default()).unwrap();
    //let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default()).unwrap();

    // Create and initialize a delay variable to manage delay loop
    let mut del_var = 2000;

    // Publish blink duration value to global context
    BLINK_MS.store(del_var, Ordering::Relaxed);

    // Spawn LED blinking task
    spawner.spawn(led_task(p.PA5.degrade())).unwrap();

    // Variable to keep track of how many button presses occured
    let mut value: u8 = 0;

    // Create empty String for message
    let mut msg: String<8> = String::new();

    loop {
        // Check if button got pressed
        button.wait_for_rising_edge().await;

        // If button pressed decrease the delay value
        del_var = del_var - 300;
        // If updated delay value drops below 300 then reset it back to starting value
        if del_var < 500 {
            del_var = 2000;
        }
        // Publish updated delay value to global context
        BLINK_MS.store(del_var, Ordering::Relaxed);

        // Format Message
        core::writeln!(&mut msg, "{:02}", value).unwrap();

        // Transmit Message
        let _ = usart.blocking_write(msg.as_bytes()); //usart.blocking_write(msg.as_bytes());

        // Update Value Parameter
        value = value.wrapping_add(1);

        // Clear String for next message
        msg.clear();
    }
}
