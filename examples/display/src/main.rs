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

    display.scroll("Hello, World!").await;
    defmt::info!("Application started, press buttons!");
    loop {
        match select(btn_a.wait_for_low(), btn_b.wait_for_low()).await {
            Either::First(_) => {
                display
                    .display(fonts::ARROW_RIGHT, Duration::from_secs(1))
                    .await;
            }
            Either::Second(_) => {
                display
                    .display(fonts::ARROW_LEFT, Duration::from_secs(1))
                    .await;
            }
        }
    }
}
