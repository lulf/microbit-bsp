# microbit-bsp

[![CI](https://github.com/lulf/microbit-bsp/actions/workflows/ci.yaml/badge.svg)](https://github.com/lulf/microbit-bsp/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/v/microbit-bsp.svg)](https://crates.io/crates/microbit-bsp)
[![docs.rs](https://docs.rs/microbit-bsp/badge.svg)](https://docs.rs/microbit-bsp)

microbit-bsp is a board support package (BSP) library for the BBC micro:bit v2 and newer.

## Features

**Based on:**

* `embassy-nrf` HAL for peripherals
* Rust Async/Await

**Hardware Support:**

Should support the complete hardware features of the BBC micro:bit v2:

* Front Buttons A and B
* 5x5 LED matrix display with fonts and brightness control
* Microphone
* Speaker
* Accelerometer
* Magnetometer
* Bluetooth LE support via `trouble-host` or `nrf-softdevice`
* GPIO pins for external connections

## Example application

```rust
#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use microbit_bsp::*;

use {
    embassy_executor::Spawner,
    embassy_futures::select::{select, Either},
    embassy_time::Duration,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Microbit::default();

    let mut display = board.display;
    let mut btn_a = board.btn_a;
    let mut btn_b = board.btn_b;

    display.set_brightness(display::Brightness::MAX);
    display.scroll("Hello, World!").await;
    defmt::info!("Application started, press buttons!");
    loop {
        match select(btn_a.wait_for_low(), btn_b.wait_for_low()).await {
            Either::First(_) => {
                display
                    .display(display::fonts::ARROW_LEFT, Duration::from_secs(1))
                    .await;
            }
            Either::Second(_) => {
                display
                    .display(display::fonts::ARROW_RIGHT, Duration::from_secs(1))
                    .await;
            }
        }
    }
}
```

## Examples

To run an example:

```bash
cd examples/display
cargo run --release
```

## Cargo Features

* `defmt` - enabled by default, and allows some crates to print things
* `trouble` - enables BLE support via the `trouble-host` crate
