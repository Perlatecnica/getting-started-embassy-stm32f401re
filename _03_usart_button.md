# Rust Embedded Async Example: Button-Controlled LED Blink Rate and UART Communication on STM32

This example demonstrates an embedded Rust program for the STM32 microcontroller using Embassy. It sets up a button-controlled LED with a dynamic blink rate, which can be adjusted on each button press. Additionally, it sends data over UART, tracking the number of button presses.

## Code Breakdown

### Attributes

- **`#![no_std]`**: Disables the standard library (`std`), as embedded systems typically lack standard library support.
- **`#![no_main]`**: Disables the default `main` entry point, allowing for a custom asynchronous entry function (`#[embassy_executor::main]`) that’s suited for embedded applications.

### Imports

```rust
use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};
use heapless::String;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
```

- **`core::fmt::Write`**: Enables formatted string operations in embedded Rust, useful for UART message formatting.
- **`core::sync::atomic::{AtomicU32, Ordering}`**: Manages atomic operations, specifically `AtomicU32`, for safely updating shared data (`BLINK_MS`).
- **`heapless::String`**: Provides a fixed-capacity, heapless string suitable for embedded systems.
- **`embassy_executor::Spawner`**: Spawns asynchronous tasks.
- **`embassy_stm32::{bind_interrupts, usart}`**: Handles interrupt bindings for UART.
- **`embassy_stm32::exti::ExtiInput` and `embassy_stm32::gpio`**: Configures GPIO pins and enables external interrupt handling.
- **`embassy_time::{Duration, Timer}`**: Provides non-blocking delays.

### Global Variables

```rust
static BLINK_MS: AtomicU32 = AtomicU32::new(0);
```

- **`BLINK_MS`**: A globally accessible atomic variable to manage the LED blink interval in milliseconds, allowing safe access across tasks.

### LED Task

```rust
#[embassy_executor::task]
async fn led_task(led: AnyPin) {
    let mut led = Output::new(led, Level::Low, Speed::Low);

    loop {
        let del = BLINK_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(del.into())).await;
        led.toggle();
    }
}
```

- **`#[embassy_executor::task]`**: Marks `led_task` as an asynchronous task.
- **`BLINK_MS.load(Ordering::Relaxed)`**: Loads the current blink interval without enforcing memory ordering.
- **`Timer::after(Duration::from_millis(del.into())).await`**: Pauses the task asynchronously for the duration defined in `BLINK_MS`.
- **`led.toggle()`**: Toggles the LED state, blinking it at the interval set by `BLINK_MS`.

### UART Interrupt Binding

```rust
bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
});
```

- **`bind_interrupts!`**: Binds the interrupt for `USART2` to the Embassy USART handler, enabling UART communication.

### Main Function

The main function initializes peripherals and spawns tasks.

```rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
```

- **`#[embassy_executor::main]`**: Marks `main` as the asynchronous entry point.
- **`embassy_stm32::init(Default::default())`**: Initializes the STM32 peripherals with default configurations, returning a handle `p`.

### GPIO and UART Setup

```rust
let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);
let mut usart: Uart<'_, embassy_stm32::mode::Blocking> = Uart::new_blocking(p.USART2, p.PA3, p.PA2, Config::default()).unwrap();
```

- **`ExtiInput::new`**: Configures pin `PC13` with a pull-down resistor and external interrupt (`EXTI13`) for button input.
- **`Uart::new_blocking`**: Configures a blocking UART interface with pins `PA3` (RX) and `PA2` (TX), allowing data transmission over `USART2`.

### Button Press Handling

```rust
let mut del_var = 2000;
BLINK_MS.store(del_var, Ordering::Relaxed);
spawner.spawn(led_task(p.PA5.degrade())).unwrap();

let mut value: u8 = 0;
let mut msg: String<8> = String::new();
```

- **`BLINK_MS.store(del_var, Ordering::Relaxed)`**: Initializes the blink interval to 2000 ms.
- **`spawner.spawn(led_task(p.PA5.degrade()))`**: Spawns the `led_task` on pin `PA5`.
- **`value`**: Tracks the number of button presses.
- **`msg`**: A heapless string buffer for constructing messages to send over UART.

### Main Loop

The main loop waits for button presses and adjusts the LED blink rate.

```rust
loop {
    button.wait_for_rising_edge().await;

    del_var -= 300;
    if del_var < 500 {
        del_var = 2000;
    }
    BLINK_MS.store(del_var, Ordering::Relaxed);

    core::writeln!(&mut msg, "{:02}", value).unwrap();
    let _ = usart.blocking_write(msg.as_bytes());

    value = value.wrapping_add(1);
    msg.clear();
}
```

- **`button.wait_for_rising_edge().await`**: Awaits a rising edge (button release).
- **`del_var -= 300`**: Decreases the blink interval by 300 ms.
- **`if del_var < 500`**: Resets `del_var` to 2000 ms if it falls below 500 ms.
- **`BLINK_MS.store(del_var, Ordering::Relaxed)`**: Updates the global blink interval.
- **`writeln!(&mut msg, "{:02}", value).unwrap();`**: Formats the `value` into `msg` as a two-digit number.
- **`usart.blocking_write(msg.as_bytes())`**: Sends the `msg` string over UART.
- **`value.wrapping_add(1)`**: Increments `value`, wrapping around on overflow.
- **`msg.clear()`**: Clears `msg` for the next message.

## Summary

This code demonstrates an embedded Rust application on STM32, which:
- Controls an LED with an adjustable blink rate based on button presses.
- Sends the number of button presses over UART.
- Uses `no_std` and asynchronous execution with Embassy.

- **Libraries**: `defmt`, `embassy_executor`, `embassy_stm32`, `heapless`
- **Concepts**: Embedded Rust, Asynchronous tasks, GPIO interrupts, UART communication, Atomic operations