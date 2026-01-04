//! Microphone example
//!
//! This example demonstrates how to use the microphone on the micro:bit BSP

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use microbit_bsp::{
    display::{Brightness, Frame},
    embassy_nrf::{bind_interrupts, saadc::InterruptHandler},
    embassy_time::Duration,
    mic::Microphone,
    LedMatrix, Microbit,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_s: Spawner) {
    let board = Microbit::default();
    defmt::info!("Microphone example started!");

    let mut display = board.display;
    display.set_brightness(Brightness::MAX);

    const DISPLAY_DURATION_MS: u64 = 50;

    // Bind interrupt to the SAADC peripheral for microphone
    bind_interrupts!(
        struct InterruptRequests {
            SAADC => InterruptHandler;
        }
    );

    let irqs = InterruptRequests {};
    let mut microphone = Microphone::new(board.saadc, irqs, board.microphone, board.micen);

    let mut last_sound_level = 0;

    loop {
        let sound_level = microphone.sound_level().await;
        // Only update display if sound level has changed
        if sound_level != last_sound_level {
            last_sound_level = sound_level;
            info!("Sound level: {}", sound_level);
            display_sound_indicator(&mut display, Duration::from_millis(DISPLAY_DURATION_MS), sound_level).await;
        }
    }
}

/// Display the sound level as an expanding indicator on the LED matrix
/// Starts with the center column and expands outward as volume increases
async fn display_sound_indicator(display: &mut LedMatrix, length: Duration, sound_level: u8) {
    let mut frame = Frame::<5, 5>::empty();

    const CENTER_COLUMN: usize = 2;
    const MAX_ROWS: usize = 5;

    // Convert sound level (0-255) to number of rows to light up (0-5)
    let num_rows = match sound_level {
        0 => 0,
        1..=51 => 1,
        52..=102 => 2,
        103..=153 => 3,
        154..=204 => 4,
        205..=255 => MAX_ROWS,
    };

    if num_rows > 0 {
        // Light up rows from bottom to top, starting with center column
        for row in (MAX_ROWS - num_rows)..MAX_ROWS {
            frame.set(CENTER_COLUMN, row);
        }

        // For medium levels, expand to adjacent columns
        if num_rows >= 3 {
            for row in (MAX_ROWS - num_rows)..MAX_ROWS {
                frame.set(1, row);
                frame.set(3, row);
            }
        }

        // For high levels, expand to all columns
        if num_rows >= 4 {
            for row in (MAX_ROWS - num_rows)..MAX_ROWS {
                frame.set(0, row);
                frame.set(4, row);
            }
        }
    }

    display.display(frame, length).await;
}
