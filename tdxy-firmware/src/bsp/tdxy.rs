use esp_hal::{gpio::{Input, InputConfig, Level, Output, OutputConfig}, peripherals::Peripherals, spi::master::{Config, Spi}, Blocking, DriverMode};

use crate::bsp::{display::Display8x8, input::{Button, Encoder}};

type DisplaySpi = Spi<'static, Blocking>;


pub struct TdXY {
    pub display: Display8x8<DisplaySpi>,
    pub encoder_x: Encoder<Input<'static>, Input<'static>>,
    pub button_x: Button<Input<'static>>,
    pub encoder_y: Encoder<Input<'static>, Input<'static>>,
    pub button_y: Button<Input<'static>>,
    pub button: Button<Input<'static>>,
    pub led: Output<'static>,
}

impl TdXY {

    pub fn init(peripherals: Peripherals) -> Self {

        let spi = Spi::new(peripherals.SPI2, Config::default()).unwrap();
        let display = Display8x8::new(spi, false);

        let encoder_x = Encoder::new(
            Input::new(peripherals.GPIO0, InputConfig::default()),
            Input::new(peripherals.GPIO1, InputConfig::default())
        );

        let button_x = Button::new(Input::new(peripherals.GPIO2, InputConfig::default()));

        let encoder_y = Encoder::new(
            Input::new(peripherals.GPIO3, InputConfig::default()),
            Input::new(peripherals.GPIO4, InputConfig::default())
        );

        let button_y = Button::new(Input::new(peripherals.GPIO5, InputConfig::default()));
        let button = Button::new(Input::new(peripherals.GPIO6, InputConfig::default()));
        let led = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());

        TdXY {
            display,
            encoder_x,
            button_x,
            encoder_y,
            button_y,
            button,
            led
        }
    }

}
