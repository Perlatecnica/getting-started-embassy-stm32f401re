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
* Example Code 6: USART echo DMA                    *
* Author: Salvatore Bramante                        *
* Organization: Perlatecnica APS ETS                *
*****************************************************/


#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts,usart,peripherals};
use embassy_stm32::usart::{Config, Uart};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    bind_interrupts!(struct Irq {
        USART2 => usart::InterruptHandler<peripherals::USART2>;
    });
    let mut usart = Uart::new(
        p.USART2,
        p.PA3,
        p.PA2,
        Irq,
        p.DMA1_CH6, // TX DMA channel 6 for USART2
        p.DMA1_CH5, // %X DMA channel 5 for USART2
        Config::default(),
    ).unwrap();

    usart.write(b"Starting Echo\r\n").await.unwrap();

    let mut msg: [u8; 8] = [0; 8];

    loop {
        // await means in this case that will read until the buffer is full
        usart.read(&mut msg).await.unwrap();
        usart.write(&msg).await.unwrap();
    }
}
