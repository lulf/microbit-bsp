#![no_std]
#![no_main]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _};

use embassy_microbit::*;

use embassy_executor::executor::Spawner;
use embassy_nrf::Peripherals;

#[embassy_executor::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let mut board = Microbit::new(p);

    defmt::info!("HELLO");
    loop {
        board.display.scroll("Hello, World!").await;
    }
}
