# microbit-bsp-ble-nrf-softdevice-example

Demonstrating the use of Bluetooth Low Energy (BLE) on the BBC micro:bit using the nRF Softdevice.

## Prerequisites

Software:

* [`rustup`](https://rustup.rs/)
* [`probe-rs`](https://github.com/probe-rs/probe-rs)

Hardware:

* [BBC micro:bit v2](https://microbit.org/)

## Running

Download the [softdevice](https://www.nordicsemi.com/Products/Development-software/S113/Download) and unpack.

Flash the softdevice onto the micro:bit (only needed the first time you run it):

```
probe-rs download s113_nrf52_7.3.0_softdevice.hex --binary-format Hex --chip nRF52833_xxAA
```

Run the application:

```
cargo run --release
```
