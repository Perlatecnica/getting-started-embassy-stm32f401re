# Rust Embedded Async Example: Button-Controlled LED on STM32 with Async Events

This example demonstrates an asynchronous embedded Rust program to control an LED based on the state of a button using the STM32 microcontroller. This program uses the `no_std` and `no_main` attributes, suitable for embedded environments, and leverages asynchronous features with the Embassy framework.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Informs the compiler that the program does not use the standard library (`std`), as embedded systems often lack standard library support.
- **`#![no_main]`**: Disables the default `main` entry point, allowing for a custom entry function (`#[embassy_executor::main]`) which is suitable for asynchronous execution in embedded contexts.

### Imports

```rust
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};
```

- **`defmt` and `defmt_rtt`**: Provide logging utilities optimized for embedded systems.
- **`embassy_executor::Spawner`**: Handles the asynchronous task spawning in an embedded environment.
- **`embassy_stm32::exti::ExtiInput`**: Provides an abstraction for external interrupts on GPIO pins, enabling asynchronous edge detection.
- **`embassy_stm32::gpio::{Level, Output, Pull, Speed}`**: Manages general-purpose input/output (GPIO) functionality, including pin configurations, levels, and speeds.
- **`panic_probe`**: Logs panic information, useful for debugging.

### Main Function

The main function serves as the entry point of this asynchronous application.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());
```

- **`#[embassy_executor::main]`**: Marks this function as the main entry point for an asynchronous application. The `async` keyword allows the use of `await` for non-blocking operations.
- **`info!("Hello World!")`**: Logs a message to indicate that the program has started.
- **`embassy_stm32::init(Default::default())`**: Initializes the STM32 peripherals using default configurations, storing the initialized peripherals in `p`.

### GPIO Setup with External Interrupts

```rust
let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);
let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);
```

- **`ExtiInput::new`**: Configures `p.PC13` as an input pin with a pull-down resistor and sets it up with an external interrupt `p.EXTI13`, allowing asynchronous edge detection.
- **`Output::new`**: Configures `p.PA5` as an output pin for the LED, with the initial state set to high and low-speed operation.
  - **Pull::Down**: Configures a pull-down resistor for the button, ensuring it reads low when unpressed.
  - **Level::High**: Sets the initial state of the LED to high (off, depending on hardware setup).
  - **Speed::Low**: Sets the GPIO speed to low, reducing power consumption.

### Main Loop with Async Button Monitoring

```rust
loop {
    button.wait_for_rising_edge().await;
    info!("Released!");
    led1.set_low();

    button.wait_for_falling_edge().await;
    info!("Pressed!");
    led1.set_high();
}
```

- The **loop** continuously monitors the button state asynchronously.
  - **`button.wait_for_rising_edge().await`**: Asynchronously waits for the button to transition from low to high (released).
    - When detected, logs `"Released!"` and turns the LED off by setting it to low.
  - **`button.wait_for_falling_edge().await`**: Asynchronously waits for the button to transition from high to low (pressed).
    - When detected, logs `"Pressed!"` and turns the LED on by setting it to high.

The use of `await` in this loop allows the program to perform other tasks if needed without blocking, making it efficient for embedded asynchronous applications.

## Summary

This example demonstrates a basic async program for embedded Rust, utilizing Embassy’s support for STM32 peripherals to toggle an LED based on button presses. It uses asynchronous edge detection with external interrupts to react to button events without constantly polling the pin, improving responsiveness and efficiency.

- **Libraries**: `defmt`, `embassy_executor`, `embassy_stm32`
- **Concepts**: Embedded Rust, Asynchronous programming, GPIO control with external interrupts
