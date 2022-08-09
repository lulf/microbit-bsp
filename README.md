# embassy-microbit

[![CI](https://github.com/lulf/embassy-microbit/actions/workflows/ci.yaml/badge.svg)](https://github.com/lulf/embassy-microbit/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/v/embassy-microbit.svg)](https://crates.io/crates/embassy-microbit)
[![docs.rs](https://docs.rs/embassy-microbit/badge.svg)](https://docs.rs/embassy-microbit)
[![Matrix](https://img.shields.io/matrix/drogue-iot:matrix.org)](https://matrix.to/#/#drogue-iot:matrix.org)

embassy-microbit is a boards support package (BSP) library for the BBC micro:bit v2 and newer. 

## features

* LED display driver with fonts (requires embassy-executor for time-keeping)
* Uses embassy-nrf HAL for peripherals

## usage

To run an example:

```
cd examples/display
cargo run --release
```
