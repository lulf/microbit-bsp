//! Simple speaker utilities for PWM-based synth
use embassy_nrf::pwm;

/// Pitch for standard scale
#[allow(dead_code, missing_docs)]
#[derive(Copy, Clone, PartialEq)]
pub enum Pitch {
    C0 = 16,
    CS0 = 17,
    D0 = 18,
    DS0 = 19,
    E0 = 20,
    F0 = 21,
    FS0 = 23,
    G0 = 24,
    GS0 = 25,
    A0 = 27,
    AS0 = 29,
    B0 = 30,
    C1 = 32,
    CS1 = 34,
    D1 = 36,
    DS1 = 38,
    E1 = 41,
    F1 = 43,
    FS1 = 46,
    G1 = 49,
    GS1 = 51,
    A1 = 55,
    AS1 = 58,
    B1 = 61,
    C2 = 65,
    CS2 = 69,
    D2 = 73,
    DS2 = 77,
    E2 = 82,
    F2 = 87,
    FS2 = 92,
    G2 = 98,
    GS2 = 103,
    A2 = 110,
    AS2 = 116,
    B2 = 123,
    C3 = 130,
    CS3 = 138,
    D3 = 146,
    DS3 = 155,
    E3 = 164,
    F3 = 174,
    FS3 = 185,
    G3 = 196,
    GS3 = 207,
    A3 = 220,
    AS3 = 233,
    B3 = 246,
    C4 = 261,
    CS4 = 277,
    D4 = 293,
    DS4 = 311,
    E4 = 329,
    F4 = 349,
    FS4 = 369,
    G4 = 392,
    GS4 = 415,
    A4 = 440,
    AS4 = 466,
    B4 = 493,
    C5 = 523,
    CS5 = 554,
    D5 = 587,
    DS5 = 622,
    E5 = 659,
    F5 = 698,
    FS5 = 739,
    G5 = 783,
    GS5 = 830,
    A5 = 880,
    AS5 = 932,
    B5 = 987,
    C6 = 1046,
    CS6 = 1108,
    D6 = 1174,
    DS6 = 1244,
    E6 = 1318,
    F6 = 1396,
    FS6 = 1479,
    G6 = 1567,
    GS6 = 1661,
    A6 = 1760,
    AS6 = 1864,
    B6 = 1975,
    C7 = 2093,
    CS7 = 2217,
    D7 = 2349,
    DS7 = 2489,
    E7 = 2637,
    F7 = 2793,
    FS7 = 2959,
    G7 = 3135,
    GS7 = 3322,
    A7 = 3520,
    AS7 = 3729,
    B7 = 3951,
    C8 = 4186,
    CS8 = 4434,
    D8 = 4698,
    DS8 = 4978,
    E8 = 5274,
    F8 = 5587,
    FS8 = 5919,
    G8 = 6271,
    GS8 = 6644,
    A8 = 7040,
    AS8 = 7458,
    B8 = 7902,
    Silent = 0,
}

/// A note is a pitch + a duration
#[derive(Clone, Copy)]
pub struct Note(pub Pitch, pub u32);

/// PWM based speaker capable of playing notes with a given pitch
pub struct PwmSpeaker<'a, T: pwm::Instance> {
    pwm: pwm::SimplePwm<'a, T>,
}

impl<'a, T: pwm::Instance> PwmSpeaker<'a, T> {
    /// Create a new speaker instance
    pub fn new(pwm: pwm::SimplePwm<'a, T>) -> Self {
        Self { pwm }
    }

    /// Play a note
    pub async fn play(&mut self, note: &Note) {
        use embassy_time::{Duration, Timer};
        if note.0 != Pitch::Silent {
            self.pwm.set_prescaler(pwm::Prescaler::Div4);
            self.pwm.set_period(note.0 as u32);
            self.pwm.enable();

            self.pwm.set_duty(0, self.pwm.max_duty() / 2);
            Timer::after(Duration::from_millis(u64::from(note.1))).await;
            self.pwm.disable();
        } else {
            Timer::after(Duration::from_millis(u64::from(note.1))).await;
        }
    }
}
