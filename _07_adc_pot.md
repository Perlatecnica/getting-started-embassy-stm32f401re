# Rust Embedded ADC Example: Analog Readings with Calibration on STM32

This example demonstrates an embedded Rust program on an STM32 microcontroller using Embassy. The program initializes an ADC to read analog values from `PA0`, calibrates these values with an internal reference voltage, and logs both the raw ADC values and the converted millivolt readings.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Disables the standard library (`std`), as embedded systems typically lack support for it.
- **`#![no_main]`**: Disables the default `main` entry point, allowing for a custom entry function (`#[embassy_executor::main]`) suited for asynchronous embedded applications.

### Imports

```rust
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, Temperature, VrefInt};
use embassy_time::{Delay, Timer};
use {defmt_rtt as _, panic_probe as _};
```

- **`defmt` and `defmt_rtt`**: Provide efficient logging optimized for embedded environments, enabling formatted output.
- **`embassy_executor::Spawner`**: Manages asynchronous tasks in embedded applications.
- **`embassy_stm32::adc::{Adc, Temperature, VrefInt}`**: Provides ADC functionality and calibration using the internal reference voltage (`VrefInt`) and temperature sensor.
- **`embassy_time::{Delay, Timer}`**: Provides timing utilities for delay and scheduling purposes.

### Main Function

The `main` function initializes the STM32 ADC to read from pin `PA0`, calibrates the ADC values with the internal reference voltage, and logs both the raw values and millivolt readings.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize STM32 peripherals with default configuration
    let p = embassy_stm32::init(Default::default());

    // Initialize a delay object for timing purposes
    let mut delay = Delay;

    // Initialize the ADC peripheral (ADC1)
    let mut adc = Adc::new(p.ADC1);

    // Define pin PA0 as the analog input
    let mut pin = p.PA0;
```

- **`#[embassy_executor::main]`**: Marks this function as the asynchronous entry point.
- **`embassy_stm32::init`**: Initializes the STM32 peripherals.
- **`Adc::new`**: Configures `ADC1` to read analog signals.
- **`p.PA0`**: Defines `PA0` as the analog input pin for ADC readings.

### Enabling the Internal Reference Voltage (VrefInt)

```rust
// Enable the internal reference voltage (Vrefint) for ADC calibration
let mut vrefint = adc.enable_vrefint();

// Wait for the startup time required by both Temperature and VrefInt sensors
delay.delay_us(Temperature::start_time_us().max(VrefInt::start_time_us()));
```

- **`adc.enable_vrefint()`**: Enables `VrefInt`, an internal reference voltage used for accurate ADC calibration.
- **`delay.delay_us(...)`**: Adds a startup delay, allowing `VrefInt` and `Temperature` to stabilize before readings begin.

### ADC Calibration and Conversion

```rust
// Perform a blocking read of the internal reference voltage
let vrefint_sample = adc.blocking_read(&mut vrefint);

// Closure to convert ADC samples to millivolts (mV) using Vrefint for calibration
let convert_to_millivolts = |sample| {
    // From STM32 datasheet, section 6.3.24 (Reference Voltage)
    const VREFINT_MV: u32 = 1210; // Vrefint in mV

    // Calculate the voltage in mV based on the sample and the reference sample
    (u32::from(sample) * VREFINT_MV / u32::from(vrefint_sample)) as u16
};
```

- **`vrefint_sample`**: Stores the calibration reference voltage from `VrefInt`.
- **`convert_to_millivolts`**: A closure that uses the calibration value from `VrefInt` to convert raw ADC values to millivolts. The conversion uses the `VREFINT_MV` constant from the STM32 datasheet, which is typically set at 1210 mV.

### Main Control Loop

The main loop reads the ADC values from `PA0`, converts them to millivolts, and logs the results.

```rust
loop {
    // Perform a blocking ADC read on PA0
    let v = adc.blocking_read(&mut pin);
    
    // Log the ADC value and its equivalent in millivolts
    info!("PA0: {} ({} mV)", v, convert_to_millivolts(v));

    // Wait for 100 milliseconds before the next reading
    Timer::after_millis(100).await;
}
```

- **`adc.blocking_read(&mut pin)`**: Reads an analog value from `PA0`.
- **`info!(...)`**: Logs both the raw ADC reading and the converted millivolt value.
- **`Timer::after_millis(100).await`**: Adds a 100 ms delay before the next reading.

### Summary

This code reads analog values from `PA0` on an STM32 microcontroller, calibrates them using the internal reference voltage, and logs both the raw and calibrated values in millivolts. The program repeats the readings every 100 milliseconds, making it suitable for applications that require periodic voltage monitoring.

- **Libraries**: `defmt`, `embassy_stm32`, `embassy_time`
- **Concepts**: ADC calibration, Voltage conversion, Periodic reading