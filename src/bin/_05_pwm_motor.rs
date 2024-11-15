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
* Example Code 8: PWM DC MOTOR                      *
* Author: Salvatore Bramante                        *
* Organization: Perlatecnica APS ETS                *
*****************************************************/

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let ch1_pin = PwmPin::new_ch1(p.PB6, OutputType::PushPull);
    let ch2_pin = PwmPin::new_ch2(p.PA7, OutputType::PushPull);

    let mut pwm = SimplePwm::new(p.TIM4, Some(ch1_pin), None, None, None, hz(100), Default::default());
    let mut pwm_2 = SimplePwm::new(p.TIM3, None, Some(ch2_pin), None, None, hz(100), Default::default());
    let mut ch1 = pwm.ch1();
    let mut ch2 = pwm_2.ch2();
    ch2.enable();
    ch1.enable();

    info!("PWM initialized");
    info!("PWM max duty {}", ch2.max_duty_cycle());

    loop {
        ch2.set_duty_cycle_percent(50);
        Timer::after_secs(3).await;
        ch2.set_duty_cycle_fully_off();
        ch1.set_duty_cycle_percent(100);
        Timer::after_secs(3).await;
        ch1.set_duty_cycle_fully_off();

    }
}
