# embassy-microbit

[![CI](https://github.com/embassy-rs/embassy-microbit/actions/workflows/ci.yaml/badge.svg)](https://github.com/embassy-rs/embassy-microbit/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/v/embassy-microbit.svg)](https://crates.io/crates/embassy-microbit)
[![docs.rs](https://docs.rs/embassy-microbit/badge.svg)](https://docs.rs/embassy-microbit)
[![Matrix](https://img.shields.io/matrix/embasssy-rs:matrix.org)](https://matrix.to/#/#embassy-rs:matrix.org)

embassy-microbit is a board support package (BSP) library for the BBC micro:bit v2 and newer.

## Features

* LED display driver with fonts
* Uses embassy-nrf HAL for peripherals


## Example application

```
#![no_std]
#![no_main]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use embassy_microbit::*;

use embassy_executor::{executor::Spawner, time::Duration};
use embassy_nrf::Peripherals;
use embassy_util::{select, Either};

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let board = Microbit::new(p);

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

```
cd examples/display
cargo run --release
```
