# Rust Embedded PWM Example with Varying Duty Cycles on STM32

This example demonstrates an embedded Rust program on an STM32 microcontroller using Embassy. The program initializes a PWM channel and varies the duty cycle incrementally, allowing gradual changes in output. This technique can be used for controlling devices like LEDs, motors, or other PWM-compatible components.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Excludes the standard library (`std`), as it is generally unavailable in embedded systems.
- **`#![no_main]`**: Disables the default `main` entry point, allowing the program to define a custom asynchronous entry function (`#[embassy_executor::main]`) suitable for embedded contexts.

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

- **`defmt` and `defmt_rtt`**: Provide efficient logging optimized for embedded systems, enabling debug output during runtime.
- **`embassy_executor::Spawner`**: Used to handle asynchronous task execution.
- **`embassy_stm32::gpio::OutputType`**: Sets the GPIO output type, in this case, `PushPull` for stable output.
- **`embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm}`**: Manages PWM signal generation, allowing control over the duty cycle and frequency.
- **`embassy_time::Timer`**: Provides asynchronous timing functionality, enabling delays between duty cycle adjustments.

### Main Function

The `main` function initializes PWM on channel 2 (PA9) and varies the duty cycle incrementally in an infinite loop.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize STM32 peripherals with default configuration
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!"); // Log message to indicate program start
```

- **`#[embassy_executor::main]`**: Marks this function as the asynchronous entry point of the embedded application.
- **`info!("Hello World!")`**: Logs a message indicating that the program has started.

### PWM Initialization

```rust
// Set up PA9 as a PWM pin on channel 2 (ch2) with push-pull output type
let sg90_pin = PwmPin::new_ch2(p.PA9, OutputType::PushPull);

// Initialize PWM on TIM1 with a frequency of 50Hz (20 ms period)
let mut pwm = SimplePwm::new(p.TIM1, None, Some(sg90_pin), None, None, hz(50), Default::default());
    
// Create a channel instance for PWM on channel 2
let mut ch2 = pwm.ch2();

// Enable the PWM channel to start generating signals
ch2.enable();

info!("PWM initialized"); // Log that PWM has been initialized
info!("PWM max duty {}", ch2.max_duty_cycle()); // Log the maximum duty cycle for channel 2
```

- **`PwmPin::new_ch2`**: Configures `PA9` as a PWM output pin for channel 2 with `PushPull` output mode.
- **`SimplePwm::new`**: Initializes PWM on timer `TIM1` with a frequency of 50 Hz, corresponding to a 20 ms period. This frequency is commonly used for controlling servo motors and dimming LEDs.
- **`ch2.enable()`**: Enables channel 2 to start generating the PWM signal.

### Duty Cycle Control Loop

The main control loop gradually changes the duty cycle in incremental steps, creating a smooth transition between different power levels.

```rust
loop {
    // Turn off PWM on channel 2 (0% duty cycle)
    ch2.set_duty_cycle_fully_off();
    Timer::after_millis(500).await; // Wait for 500 ms with PWM off

    // Set duty cycle to 1/20 of max (5%) and wait
    ch2.set_duty_cycle_fraction(1, 20);
    Timer::after_millis(500).await;

    // Set duty cycle to 1/16 of max (~6.25%) and wait
    ch2.set_duty_cycle_fraction(1, 16);
    Timer::after_millis(500).await;

    // Set duty cycle to 1/13 of max (~7.7%) and wait
    ch2.set_duty_cycle_fraction(1, 13);
    Timer::after_millis(500).await;

    // Set duty cycle to 1/12 of max (~8.33%) and wait
    ch2.set_duty_cycle_fraction(1, 12);
    Timer::after_millis(500).await;

    // Set duty cycle to 1/10 of max (10%) and wait
    ch2.set_duty_cycle_fraction(1, 10);
    Timer::after_millis(500).await;

    // Set duty cycle to 1/8 of max (12.5%) and wait
    ch2.set_duty_cycle_fraction(1, 8);
    Timer::after_millis(500).await;

    // Set duty cycle to 1/7 of max (~14.3%) and wait
    ch2.set_duty_cycle_fraction(1, 7);
    Timer::after_millis(500).await;
}
```

- **Duty Cycle Adjustments**:
  - The loop adjusts the duty cycle in fractions of the maximum value, starting from fully off to gradually increasing values.
  - The fractional values (e.g., `1/20`, `1/16`, etc.) allow for fine-grained control over the duty cycle, providing smooth transitions.

- **Timing Control**:
  - Each duty cycle adjustment is followed by a 500 ms delay using `Timer::after_millis`, giving time for the change to be observed before moving to the next duty cycle.

### Summary

This code sets up PWM on an STM32 microcontroller with Embassy, and uses it to control a device by incrementally adjusting the duty cycle in a loop. The gradual increase in duty cycle creates a smooth ramp-up effect, which is useful for applications such as LED dimming or motor control.

- **Libraries**: `defmt`, `embassy_stm32`, `embassy_time`
- **Concepts**: PWM signal generation, Duty cycle control, Asynchronous timing