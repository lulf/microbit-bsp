pub use embassy_nrf::config::Config;
use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
pub use embassy_nrf::interrupt::Priority;
use embassy_nrf::peripherals::{
    P0_00, P0_01, P0_02, P0_03, P0_04, P0_05, P0_06, P0_08, P0_09, P0_10, P0_12, P0_13, P0_16, P0_17, P0_20, P0_26,
    P1_00, P1_02, P1_08, PPI_CH0, PPI_CH1, PWM0, PWM1, PWM2, PWM3, RNG, SAADC, TIMER0, TWISPI0, TWISPI1, UARTE0, UARTE1,
};
pub use embassy_nrf::wdt;

#[cfg(feature = "trouble")]
use crate::ble;
use crate::display::LedMatrix as LedMatrixDriver;

/// LED matrix peripheral for the micro:bit
pub type LedMatrix = LedMatrixDriver<Output<'static>, 5, 5>;

/// Button 'A'
pub type Button = Input<'static>;

/// Represents all the peripherals and pins available for the BBC micro:bit.
pub struct Microbit {
    /// LED matrix display
    pub display: LedMatrix,
    /// Button 'A'
    pub btn_a: Button,
    /// Button 'B'
    pub btn_b: Button,
    /// UART0 peripheral
    pub uarte0: UARTE0,
    /// UART1 peripheral
    pub uarte1: UARTE1,
    /// TIMER0 peripheral
    pub timer0: TIMER0,
    /// Speaker pin
    pub speaker: P0_00,
    /// Microphone pin
    pub microphone: P0_05,
    /// Microphone pin enable
    pub micen: P0_20,

    /// P0 connector pin
    pub p0: P0_02,
    /// P1 connector pin
    pub p1: P0_03,
    /// P2 connector pin
    pub p2: P0_04,
    /// P8 connector pin
    pub p8: P0_10,
    /// P9 connector pin
    pub p9: P0_09,
    /// P12 connector pin
    pub p12: P0_12,
    /// P13 connector pin
    pub p13: P0_17,
    /// P14 connector pin
    pub p14: P0_01,
    /// P15 connector pin
    pub p15: P0_13,
    /// P16 connector pin
    pub p16: P1_02,
    /// P19 connector pin
    pub p19: P0_26,
    /// P20 connector pin
    pub p20: P1_00,

    /// Internal I2C/TWI SCL to accelerometer & debug MCU
    pub i2c_int_scl: P0_08,
    /// Internal I2C/TWI SDA to accelerometer & debug MCU
    pub i2c_int_sda: P0_16,

    /// UART TX to debug MCU
    pub uart_int_tx: P1_08,
    /// UART RX to debug MCU
    pub uart_int_rx: P0_06,

    /// SPI0/I2C0 peripheral
    pub twispi0: TWISPI0,
    /// SPI1/I2C1 peripheral
    pub twispi1: TWISPI1,
    /// PWM0 peripheral
    pub pwm0: PWM0,
    /// PWM1 peripheral
    pub pwm1: PWM1,
    /// PWM2 peripheral
    pub pwm2: PWM2,
    /// PWM3 peripheral
    pub pwm3: PWM3,
    /// PPI channel 0
    pub ppi_ch0: PPI_CH0,
    /// PPI channel 1
    pub ppi_ch1: PPI_CH1,
    /// Random number generator
    pub rng: RNG,
    /// Analog digital converter
    pub saadc: SAADC,
    #[cfg(feature = "trouble")]
    /// Bluetooth Low Energy peripheral
    pub ble: ble::BleControllerBuilder<'static>,
}

impl Default for Microbit {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Microbit {
    /// Create a new instance based on HAL configuration
    pub fn new(config: embassy_nrf::config::Config) -> Self {
        let p = embassy_nrf::init(config);
        // LED Matrix
        let rows = [
            output_pin(p.P0_21.degrade()),
            output_pin(p.P0_22.degrade()),
            output_pin(p.P0_15.degrade()),
            output_pin(p.P0_24.degrade()),
            output_pin(p.P0_19.degrade()),
        ];

        let cols = [
            output_pin(p.P0_28.degrade()),
            output_pin(p.P0_11.degrade()),
            output_pin(p.P0_31.degrade()),
            output_pin(p.P1_05.degrade()),
            output_pin(p.P0_30.degrade()),
        ];

        Self {
            display: LedMatrixDriver::new(rows, cols),
            btn_a: Input::new(p.P0_14.degrade(), Pull::None),
            btn_b: Input::new(p.P0_23.degrade(), Pull::None),
            uarte0: p.UARTE0,
            uarte1: p.UARTE1,
            timer0: p.TIMER0,
            speaker: p.P0_00,
            microphone: p.P0_05,
            micen: p.P0_20,
            p0: p.P0_02,
            p1: p.P0_03,
            p2: p.P0_04,
            p8: p.P0_10,
            p9: p.P0_09,
            p12: p.P0_12,
            p13: p.P0_17,
            p14: p.P0_01,
            p15: p.P0_13,
            p16: p.P1_02,
            p19: p.P0_26,
            p20: p.P1_00,
            i2c_int_scl: p.P0_08,
            i2c_int_sda: p.P0_16,
            uart_int_tx: p.P1_08,
            uart_int_rx: p.P0_06,
            ppi_ch0: p.PPI_CH0,
            ppi_ch1: p.PPI_CH1,
            twispi0: p.TWISPI0,
            twispi1: p.TWISPI1,
            pwm0: p.PWM0,
            pwm1: p.PWM1,
            pwm2: p.PWM2,
            pwm3: p.PWM3,
            rng: p.RNG,
            saadc: p.SAADC,
            #[cfg(feature = "trouble")]
            ble: ble::BleControllerBuilder::new(
                p.RTC0, p.TEMP, p.PPI_CH17, p.PPI_CH18, p.PPI_CH19, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23,
                p.PPI_CH24, p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29, p.PPI_CH30, p.PPI_CH31,
            ),
        }
    }
}

fn output_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::Low, OutputDrive::Standard)
}
