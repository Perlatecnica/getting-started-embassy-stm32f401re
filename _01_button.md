# Rust Embedded Example: Button-Controlled LED on STM32

This example demonstrates a simple embedded Rust program to control an LED based on the state of a button using the STM32 microcontroller. The program is built using the `no_std` and `no_main` attributes, which make it suitable for embedded environments without a standard library.

## Code Breakdown

### Attributes

- **`#![no_std]`**: This attribute tells the Rust compiler that the program does not use the standard library (`std`), as it’s designed for embedded systems where the standard library is often unavailable.
- **`#![no_main]`**: This attribute disables the usual `main` function entry point, allowing the use of custom entry points (such as `#[entry]` here) that suit embedded systems.

### Imports

```rust
use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};
```

- **`cortex_m_rt::entry`**: Imports the `entry` attribute, used to define the entry point of the program.
- **`defmt` and `defmt_rtt`**: Provides logging capabilities for debugging in embedded contexts.
- **`embassy_stm32::gpio::{Input, Level, Output, Pull, Speed}`**: Imports GPIO functionality from the Embassy framework for the STM32 microcontroller family, allowing control of pins as input or output.
- **`panic_probe`**: A panic handler that logs panic information for debugging purposes.

### Main Function

The main function is the entry point for this embedded application.

```rust
#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());
```

- **`#[entry]`**: Marks this function as the entry point of the program.
- **`info!("Hello World!")`**: A log statement to indicate the program has started.
- **`embassy_stm32::init(Default::default())`**: Initializes the STM32 peripherals with default settings, storing the result in `p`.

### GPIO Setup

```rust
let button = Input::new(p.PC13, Pull::Down);
let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);
```

- **`Input::new`**: Configures `p.PC13` (a specific pin) as an input with a pull-down resistor.
- **`Output::new`**: Configures `p.PA5` as an output pin, setting its initial level to `High` and its speed to `Low`.
  - **Pull::Down**: Sets a pull-down resistor on the button pin to ensure the pin reads as low when unpressed.
  - **Level::High**: Sets the LED’s initial state to high (off, depending on hardware configuration).
  - **Speed::Low**: Sets the GPIO pin’s speed to low, which reduces power consumption.

### Main Loop

```rust
loop {
    if button.is_high() {
        info!("unpressed");
        led1.set_low();
    } else {
        info!("pressed");
        led1.set_high();
    }
}
```

- The **loop** continuously checks the button’s state.
  - **`button.is_high()`**: Checks if the button pin is high (indicating it is unpressed, assuming pull-down logic).
    - If high, it logs `"unpressed"` and sets `led1` to low (turning off the LED).
    - If low, it logs `"pressed"` and sets `led1` to high (turning on the LED).

## Summary

This code is a simple embedded Rust program that toggles an LED based on the state of a button, logging the button state via RTT. It uses `no_std`, `no_main`, and Embassy’s GPIO abstractions for an STM32 microcontroller.

- **Libraries**: `defmt`, `embassy_stm32`, `cortex_m_rt`
- **Concepts**: Embedded Rust, GPIO control, Logging in embedded contexts
