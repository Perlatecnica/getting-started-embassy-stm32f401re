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
* Example Code 7: ADC                               *
* Author: Salvatore Bramante                        *
* Organization: Perlatecnica APS ETS                *
*****************************************************/
#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, Temperature, VrefInt};
use embassy_time::{Delay, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut delay = Delay;
    let mut adc = Adc::new(p.ADC1);
    let mut pin = p.PA0;

    let mut vrefint = adc.enable_vrefint();

    // Startup delay can be combined to the maximum of either
    delay.delay_us(Temperature::start_time_us().max(VrefInt::start_time_us()));

    let vrefint_sample = adc.blocking_read(&mut vrefint);

    let convert_to_millivolts = |sample| {
        // From http://www.st.com/resource/en/datasheet/DM00071990.pdf
        // 6.3.24 Reference voltage
        const VREFINT_MV: u32 = 1210; // mV

        (u32::from(sample) * VREFINT_MV / u32::from(vrefint_sample)) as u16
    };

    loop {
        // Read pin
        let v = adc.blocking_read(&mut pin);
        info!("PA0: {} ({} mV)", v, convert_to_millivolts(v));

        Timer::after_millis(100).await;
    }
}
