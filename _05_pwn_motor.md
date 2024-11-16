# Rust Embedded PWM Example: Alternating Channels on STM32

This example demonstrates an embedded Rust program for an STM32 microcontroller using Embassy. The program initializes two PWM channels on different timers, enabling them alternately to control devices connected to the pins (e.g., motors or LEDs). It includes delay intervals between each activation phase.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Disables the standard library (`std`), which is unavailable in embedded environments.
- **`#![no_main]`**: Disables the default `main` entry point, allowing a custom entry function (`#[embassy_executor::main]`) suited for asynchronous embedded applications.

### Imports

```rust
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
```

- **`defmt` and `defmt_rtt`**: Provide efficient logging optimized for embedded contexts.
- **`embassy_executor::Spawner`**: Used to handle asynchronous task execution.
- **`embassy_stm32::gpio::OutputType`**: Configures the GPIO output type.
- **`embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm}`**: Manages PWM signal generation on STM32, enabling control of duty cycles and frequency.
- **`embassy_time::Timer`**: Provides asynchronous timing functionality, allowing delays in the PWM signal control.

### Main Function

The `main` function sets up two PWM channels on separate timers and alternates their duty cycles in an infinite loop.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize STM32 peripherals with default configuration
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!"); // Log message to indicate the program has started
```

- **`#[embassy_executor::main]`**: Marks this function as the asynchronous entry point.
- **`info!("Hello World!")`**: Logs a message indicating the program has started.

### PWM Pin and Timer Setup

```rust
// Set up PWM pins for two channels:
// - `pwm_a` on channel 1 (PB6)
// - `pwm_b` on channel 2 (PA7)
let pwm_a = PwmPin::new_ch1(p.PB6, OutputType::PushPull);
let pwm_b = PwmPin::new_ch2(p.PA7, OutputType::PushPull);
```

- **`PwmPin::new_ch1`** and **`PwmPin::new_ch2`**: Configure `PB6` and `PA7` as PWM output pins, set in push-pull mode for stable output signals.

```rust
// Initialize PWM on different timers for each channel:
// - `TIM4` for `pwm_a` with a frequency of 100Hz
// - `TIM3` for `pwm_b` also at 100Hz
let mut pwm1 = SimplePwm::new(p.TIM4, Some(pwm_a), None, None, None, hz(100), Default::default());
let mut pwm2 = SimplePwm::new(p.TIM3, None, Some(pwm_b), None, None, hz(100), Default::default());
```

- **`SimplePwm::new`**: Sets up PWM for `TIM4` and `TIM3`, each configured with a frequency of 100 Hz.
  - `pwm1` uses `TIM4` for `pwm_a`.
  - `pwm2` uses `TIM3` for `pwm_b`.

### Channel Enabling and Main Loop

```rust
// Create channel instances for each PWM channel
let mut ch1 = pwm1.ch1();
let mut ch2 = pwm2.ch2();

// Enable PWM channels to start generating PWM signals
ch1.enable();
ch2.enable();
    
info!("PWM initialized"); // Log that PWM has been initialized
```

- **Channel Instances (`ch1` and `ch2`)**: Create instances to control each PWM channel’s duty cycle.
- **`ch1.enable()` and `ch2.enable()`**: Enable the channels, allowing PWM signal generation.

### Main Control Loop

The main loop alternates between activating `ch1` and `ch2`, each at a 100% duty cycle for 3 seconds, with 1-second pauses in between when both channels are off.

```rust
loop {
    // Turn off channel 1 and set channel 2 to 100% duty cycle (full power)
    ch1.set_duty_cycle_fully_off();
    ch2.set_duty_cycle_percent(100);
    Timer::after_secs(3).await; // Wait for 3 seconds with channel 2 active at 100%

    // Turn off channel 2 after 3 seconds
    ch2.set_duty_cycle_fully_off();
    Timer::after_secs(1).await; // Pause for 1 second with both channels off

    // Set channel 1 to 100% duty cycle and turn off channel 2
    ch1.set_duty_cycle_percent(100);
    ch2.set_duty_cycle_fully_off();
    Timer::after_secs(3).await; // Wait for 3 seconds with channel 1 active at 100%

    // Turn off channel 1 after 3 seconds
    ch1.set_duty_cycle_fully_off();
    Timer::after_secs(1).await; // Pause for 1 second with both channels off
}
```

- **Duty Cycle Settings**:
  - **`ch2` at 100%**: Activates `ch2` at full duty cycle for 3 seconds, then turns it off.
  - **`ch1` at 100%**: Activates `ch1` at full duty cycle for 3 seconds, then turns it off.
- **Timing Control**: The `Timer::after_secs` calls create pauses, allowing the motor or LED to rest between cycles.

### Summary

This code demonstrates PWM control on an STM32 microcontroller, toggling two channels with alternate duty cycles. This setup is useful for controlling devices like DC motors or LEDs with precise timing and power modulation.

- **Libraries**: `defmt`, `embassy_stm32`, `embassy_time`
- **Concepts**: PWM signal generation, Duty cycle control, Asynchronous delay