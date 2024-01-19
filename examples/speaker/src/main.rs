#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::pwm::SimplePwm,
    speaker::{NamedPitch, Note, PwmSpeaker},
    Microbit,
};
use {defmt_rtt as _, panic_probe as _};

const TUNE: [(NamedPitch, u32); 18] = {
    #[allow(clippy::enum_glob_use)]
    use NamedPitch::*;
    [
        (D4, 1),
        (DS4, 1),
        (E4, 1),
        (C5, 2),
        (E4, 1),
        (C5, 2),
        (E4, 1),
        (C5, 3),
        (C4, 1),
        (D4, 1),
        (DS4, 1),
        (E4, 1),
        (C4, 1),
        (D4, 1),
        (E4, 2),
        (B4, 1),
        (D5, 2),
        (C4, 4),
    ]
};

#[embassy_executor::main]
async fn main(_s: Spawner) {
    let board = Microbit::default();
    defmt::info!("Application started!");
    let mut speaker = PwmSpeaker::new(SimplePwm::new_1ch(board.pwm0, board.speaker));
    loop {
        defmt::info!("Playing tune!");
        for (pitch, ticks) in TUNE {
            speaker.play(&Note(pitch.into(), 200 * ticks)).await;
        }
        Timer::after_secs(5).await;
    }
}
