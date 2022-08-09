use crate::display::LedMatrix as LedMatrixDriver;
use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    peripherals::{
        P0_00, P0_01, P0_03, P0_04, P0_06, P0_08, P0_09, P0_10, P0_12, P0_13, P0_14, P0_16, P0_17,
        P0_23, P0_26, P1_00, P1_02, P1_08, PPI_CH0, PPI_CH1, PWM0, RNG, TIMER0, TWISPI0, UARTE0,
    },
};

type Peripherals = embassy_nrf::Peripherals;
pub type LedMatrix = LedMatrixDriver<Output<'static, AnyPin>, 5, 5>;

pub type ButtonA = Input<'static, P0_14>;
pub type ButtonB = Input<'static, P0_23>;

pub struct Microbit {
    pub display: LedMatrix,
    pub btn_a: ButtonA,
    pub btn_b: ButtonB,
    pub uarte0: UARTE0,
    pub timer0: TIMER0,
    pub speaker: P0_00,

    pub p1: P0_03,
    pub p2: P0_04,
    pub p8: P0_10,
    pub p9: P0_09,
    pub p12: P0_12,
    pub p13: P0_17,
    pub p14: P0_01,
    pub p15: P0_13,
    pub p16: P1_02,
    pub p17: P0_06,
    pub p19: P0_26,
    pub p20: P1_00,
    pub p22: P0_08,
    pub p23: P0_16,
    pub p25: P1_08,

    pub twispi0: TWISPI0,
    pub pwm0: PWM0,
    pub ppi_ch0: PPI_CH0,
    pub ppi_ch1: PPI_CH1,
    pub rng: RNG,
}

impl Microbit {
    pub fn new(p: Peripherals) -> Self {
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
            btn_a: Input::new(p.P0_14, Pull::Up),
            btn_b: Input::new(p.P0_23, Pull::Up),
            uarte0: p.UARTE0,
            timer0: p.TIMER0,
            speaker: p.P0_00,
            p1: p.P0_03,
            p2: p.P0_04,
            p8: p.P0_10,
            p9: p.P0_09,
            p12: p.P0_12,
            p13: p.P0_17,
            p14: p.P0_01,
            p15: p.P0_13,
            p16: p.P1_02,
            p17: p.P0_06,
            p19: p.P0_26,
            p20: p.P1_00,
            p22: p.P0_08,
            p23: p.P0_16,
            p25: p.P1_08,
            ppi_ch0: p.PPI_CH0,
            ppi_ch1: p.PPI_CH1,
            twispi0: p.TWISPI0,
            pwm0: p.PWM0,
            rng: p.RNG,
        }
    }
}

fn output_pin(pin: AnyPin) -> Output<'static, AnyPin> {
    Output::new(pin, Level::Low, OutputDrive::Standard)
}
