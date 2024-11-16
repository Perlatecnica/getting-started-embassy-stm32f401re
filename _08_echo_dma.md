# Rust Embedded USART Echo Example on STM32 using DMA

This example demonstrates an embedded Rust program for an STM32 microcontroller using Embassy. The program initializes USART2 for serial communication and creates an echo function, reading incoming data and sending it back. This setup is useful for testing and debugging serial communication.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Excludes the standard library (`std`), as embedded systems typically do not support it.
- **`#![no_main]`**: Disables the default `main` entry point, allowing the use of a custom entry function (`#[embassy_executor::main]`) suitable for asynchronous execution in embedded environments.

### Imports

```rust
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, usart, peripherals};
use embassy_stm32::usart::{Config, Uart};
use {defmt_rtt as _, panic_probe as _};
```

- **`embassy_executor::Spawner`**: Manages asynchronous tasks.
- **`embassy_stm32::{bind_interrupts, usart, peripherals}`**: Provides STM32-specific bindings for peripherals, USART, and interrupt handling.
- **`embassy_stm32::usart::{Config, Uart}`**: Sets up USART configuration and functions.
- **`panic_probe`**: Provides debugging support by capturing panics and sending debug messages.

### Main Function

The `main` function initializes USART2 on the STM32, sets up the DMA channels and interrupt, and enters an infinite loop where it reads and echoes messages.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize STM32 peripherals with default configuration
    let p = embassy_stm32::init(Default::default());
```

- **`#[embassy_executor::main]`**: Marks this function as the asynchronous entry point.
- **`embassy_stm32::init(Default::default())`**: Initializes STM32 peripherals with default configurations.

### USART2 Interrupt Binding

```rust
// Bind the interrupt for USART2 to its handler
bind_interrupts!(struct Irq {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});
```

- **`bind_interrupts!`**: Binds the `USART2` interrupt to its handler, allowing it to manage incoming and outgoing data asynchronously.

### USART2 Configuration

```rust
// Configure USART2 with TX and RX pins and DMA channels
let mut usart = Uart::new(
    p.USART2,        // Use USART2 peripheral
    p.PA3,           // RX pin for USART2
    p.PA2,           // TX pin for USART2
    Irq,             // Assign the interrupt handler struct to USART2
    p.DMA1_CH6,      // DMA channel 6 for USART2 TX
    p.DMA1_CH5,      // DMA channel 5 for USART2 RX
    Config::default(), // Default USART configuration
).unwrap();
```

- **`Uart::new`**: Initializes USART2 with:
  - **Peripheral**: `p.USART2`
  - **RX Pin**: `PA3`
  - **TX Pin**: `PA2`
  - **DMA Channels**: `DMA1_CH6` for TX and `DMA1_CH5` for RX
  - **Configuration**: Default settings, such as baud rate and word length

### Initial Message

```rust
// Write initial message to USART indicating that echo is starting
usart.write(b"Starting Echo\r\n").await.unwrap();
```

- **`usart.write(...)`**: Sends a startup message, `"Starting Echo"`, to the connected serial device, indicating that the USART echo function is active.

### Echo Loop

The main loop continuously reads data into a buffer and then writes it back, creating an echo.

```rust
// Define a buffer to hold incoming messages (8 bytes)
let mut msg: [u8; 8] = [0; 8];

// Enter an infinite loop to read and echo messages
loop {
    // Wait to read a full buffer of data into `msg`
    usart.read(&mut msg).await.unwrap();
    
    // Write the received message back to USART (echo)
    usart.write(&msg).await.unwrap();
}
```

- **Buffer Definition**: `msg` is an 8-byte buffer to store incoming data.
- **Echo Function**:
  - **`usart.read(&mut msg).await`**: Reads incoming data into `msg` and waits until the buffer is full.
  - **`usart.write(&msg).await`**: Writes the contents of `msg` back to USART, echoing the received message.

### Summary

This code sets up a USART echo on an STM32 microcontroller using Embassy. It initializes USART2, configures DMA channels and interrupt handling, and continuously reads and echoes messages. This setup is commonly used to test serial communication.

- **Libraries**: `embassy_stm32`, `defmt`, `embassy_executor`
- **Concepts**: USART communication, DMA channel configuration, Asynchronous tasks, Echo functionality