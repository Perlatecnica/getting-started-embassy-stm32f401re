# Rust Embedded UART Example with Error Handling: Hello World and Echo on STM32

This example demonstrates an embedded Rust program for STM32 using UART communication. The program initializes UART, sends a "Hello World" message, and enters an echo loop, where it reads incoming data and sends it back to the sender. This version includes error handling to manage UART framing errors gracefully.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Excludes the standard library (`std`), which is unavailable in embedded environments.
- **`#![no_main]`**: Disables the default `main` entry point, allowing the use of a custom entry function (`#[entry]`) suitable for embedded applications.

### Imports

```rust
use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};
```

- **`cortex_m_rt::entry`**: Provides the `#[entry]` attribute, which defines the main entry point for the program.
- **`defmt` and `defmt_rtt`**: Provide efficient logging optimized for embedded contexts, enabling formatted output during runtime.
- **`embassy_stm32::usart::{Config, Uart}`**: Configures and manages UART for serial communication.
- **`embassy_stm32::{bind_interrupts, peripherals, usart}`**: Sets up UART interrupts and provides access to STM32 peripheral mappings.
- **`panic_probe`**: Provides a panic handler for logging critical errors, aiding in debugging.

### UART Interrupt Binding

```rust
bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});
```

- **`bind_interrupts!`**: Links the interrupt for `USART2` to Embassy’s UART interrupt handler, allowing the handler to manage UART data transmission and reception.

### Main Function

The `main` function initializes UART communication, sends a welcome message, and enters an echo loop where it handles UART read errors gracefully.

```rust
#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());
```

- **`#[entry]`**: Marks this function as the main entry point of the program.
- **`info!("Hello World!")`**: Logs a message to indicate that the program has started.
- **`embassy_stm32::init(Default::default())`**: Initializes STM32 peripherals with default configurations, returning a handle `p`.

### UART Configuration

```rust
let config = Config::default();
let mut usart: Uart<'_, embassy_stm32::mode::Blocking> = 
    Uart::new_blocking(p.USART2, p.PA3, p.PA2, config).unwrap();
```

- **`Config::default()`**: Creates a default configuration for UART.
- **`Uart::new_blocking`**: Initializes UART in blocking mode, assigning `USART2` as the UART peripheral, with `PA3` (RX) and `PA2` (TX) as the receive and transmit pins.

### Hello World Message and Echo Loop with Error Handling

```rust
unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
info!("wrote Hello, starting echo");

let mut buf = [0u8; 1]; // Buffer to hold one byte of data

loop {
    // Attempt to read a byte from UART
    match usart.blocking_read(&mut buf) {
        Ok(_) => {
            // If read is successful, write the byte back to echo it
            let _ = usart.blocking_write(&buf);
        }
        Err(e) => {
            // Handle read error (e.g., framing error) by logging and continuing
            info!("UART read error: {:?}", e);
        }
    }
}
```

- **`usart.blocking_write(b"Hello Embassy World!\r\n")`**: Sends a welcome message over UART, signaling that the program has started. The message includes `\r\n` for a new line on many serial consoles.
- **`info!("wrote Hello, starting echo")`**: Logs that the hello message was successfully sent and that the program is beginning to echo data.
- **`let mut buf = [0u8; 1];`**: Creates a buffer to store one byte of received data.
- **`loop { ... }`**: Enters an infinite loop to read and echo data over UART:
  - **`match usart.blocking_read(&mut buf)`**: Attempts to read a byte from UART.
    - If successful (`Ok(_)`), it sends the received byte back over UART to echo it.
    - If an error occurs (`Err(e)`), it logs the error (e.g., a framing error) without interrupting the program flow.

### Summary

This program initializes UART communication on STM32, sends an initial "Hello World" message, and then enters an echo loop where it reads data and sends it back. Error handling is included to manage UART read issues, such as framing errors, by logging errors without stopping the program.

- **Libraries**: `defmt`, `embassy_stm32`, `panic_probe`
- **Concepts**: Embedded Rust, UART communication, Error handling, Logging