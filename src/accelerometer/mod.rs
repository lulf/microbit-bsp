//! Accelerometer for the micro:bit
use embassy_nrf::interrupt::typelevel::{self, Binding};
use embassy_nrf::peripherals::{P0_08, P0_16, TWISPI0};
use embassy_nrf::twim;
use embassy_nrf::twim::InterruptHandler;
use embassy_sync::channel::DynamicSender;
use embassy_time::{Duration, Ticker};
use lsm303agr::interface::I2cInterface;
use lsm303agr::mode::MagOneShot;
pub use lsm303agr::AccelOutputDataRate;
use lsm303agr::{AccelMode, Acceleration, Error as LsmError, Lsm303agr, Status};

type I2C<'d> = twim::Twim<'d, TWISPI0>;

/// Accelerometer error
pub type Error = LsmError<twim::Error>;

/// Accelerometer peripheral present on the microbit
pub struct Accelerometer<'d> {
    sensor: Lsm303agr<I2cInterface<I2C<'d>>, MagOneShot>,
}

impl<'d> Accelerometer<'d> {
    /// Create and initialize the accelerometer
    pub fn new(
        twispi0: TWISPI0,
        irq: impl Binding<typelevel::TWISPI0, InterruptHandler<TWISPI0>> + 'd,
        sda: P0_16,
        scl: P0_08,
    ) -> Result<Self, Error> {
        let config = twim::Config::default();
        let twi = twim::Twim::new(twispi0, irq, sda, scl, config);

        let mut sensor = Lsm303agr::new_with_i2c(twi);
        sensor.init()?;
        sensor.set_accel_mode_and_odr(&mut embassy_time::Delay, AccelMode::Normal, AccelOutputDataRate::Hz10)?;

        Ok(Self { sensor })
    }

    /// Return status of accelerometer
    pub fn accel_status(&mut self) -> Result<Status, Error> {
        self.sensor.accel_status()
    }

    /// Return accelerometer data
    ///
    /// Returned in mg (milli-g) where 1g is 9.8m/s².
    pub fn accel_data(&mut self) -> Result<Acceleration, Error> {
        self.sensor.acceleration()
    }

    /// Run a continuous task outputing accelerometer data at the configured data rate
    pub async fn run(
        &mut self,
        rate: AccelOutputDataRate,
        sender: DynamicSender<'_, Acceleration>,
    ) -> Result<(), Error> {
        let delay = match rate {
            AccelOutputDataRate::Hz1 => Duration::from_millis(1000),
            AccelOutputDataRate::Hz10 => Duration::from_millis(100),
            AccelOutputDataRate::Hz25 => Duration::from_millis(40),
            AccelOutputDataRate::Hz50 => Duration::from_millis(20),
            AccelOutputDataRate::Hz100 => Duration::from_millis(10),
            AccelOutputDataRate::Hz200 => Duration::from_millis(5),
            AccelOutputDataRate::Hz400 => Duration::from_micros(2500),
            AccelOutputDataRate::Khz1_344 => Duration::from_micros(744),
            AccelOutputDataRate::Khz1_620LowPower => Duration::from_micros(617),
            AccelOutputDataRate::Khz5_376LowPower => Duration::from_micros(186),
        };
        let mut ticker = Ticker::every(delay);
        loop {
            ticker.next().await;
            let data = self.accel_data()?;
            let _ = sender.try_send(data);
        }
    }
}
