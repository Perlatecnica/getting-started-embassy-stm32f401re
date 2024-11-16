
# Rust Embedded Example: Blinky LED on STM32


This Rust code is designed to run an embedded application on an STM32 microcontroller without an operating system, using the `embassy_stm32` library for GPIO (General Purpose Input/Output) handling. Here’s a detailed explanation of each part.

## Full Code
```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let button = Input::new(p.PC13, Pull::Down);
    let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);

    loop {
        if button.is_high() {
            info!("unpressed");
            led1.set_low();
        } else {
            info!("pressed");
            led1.set_high();
        }
    }
}
```

## Code Breakdown

### 1. Configuration Attributes
```rust
#![no_std]
#![no_main]
```
- `#![no_std]`: This attribute tells the compiler not to use Rust's standard library, as it's typically unavailable in embedded environments. The code instead relies on the `core` library, which is optimized for embedded systems.
- `#![no_main]`: This disables the default main function. Instead, we define a custom entry point with `#[entry]`, provided by the `cortex_m_rt` crate.

### 2. Imports
```rust
use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};
```
- `cortex_m_rt::entry`: Provides the `#[entry]` macro to set a custom entry point for embedded applications.
- `defmt::*`: Used for efficient logging, useful for debugging purposes (e.g., `info!`).
- `embassy_stm32::gpio::*`: Manages GPIO pin configurations for input/output, voltage levels (`Level`), pull-up/pull-down resistors (`Pull`), and speed settings (`Speed`).
- `defmt_rtt` and `panic_probe`: These provide debugging tools and manage `panic` situations specifically for embedded environments.

### 3. Main Function
The `main` function is the entry point of the program.

```rust
#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());
```
- `info!("Hello World!");`: Sends a log message using `defmt`, which helps confirm that the program is running.
- `let p = embassy_stm32::init(Default::default());`: Initializes the STM32 peripherals with default settings, providing access to the GPIO pins.

### 4. GPIO Configuration
```rust
    let button = Input::new(p.PC13, Pull::Down);
    let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);
```
- `let button = Input::new(p.PC13, Pull::Down);`: Configures pin `PC13` as an input with a pull-down resistor, typically connected to a button.
- `let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);`: Configures pin `PA5` as an output, initially set to `High` (LED on), with low speed.

### 5. Infinite Loop
The program enters an infinite loop, where it continuously reads the state of the button.

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
}
```
- `loop { ... }`: An infinite loop that constantly checks the button’s state.
  - `if button.is_high()`: Checks if the button is not pressed (`is_high()` returns true), which causes the LED to turn off (`led1.set_low()`).
  - `else`: If the button is pressed, the LED turns on (`led1.set_high()`), signaling that the button is pressed.

This loop creates a simple interaction where the LED reflects the state of the button: on when pressed, off when unpressed.
