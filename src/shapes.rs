use alloc::boxed::Box;
use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point, Size}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_6X13, FONT_10X20}}};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use pimoroni_badger2040::{hal::{gpio::{bank0::*, Output, PushPull, Pin, PullDown, Input, PullUp}, spi::Enabled}, pac::SPI0};
use uc8151::{Uc8151, WIDTH, HEIGHT};
use pimoroni_badger2040::hal::Spi;

use crate::os::os::{App, Pins, UcDisplay, APP_X};

pub struct Shapes {}

impl App for Shapes {
    fn init(&mut self, buttons: &Pins, display: &mut UcDisplay) {
        let bounds = Rectangle::new(Point::new(APP_X as i32, 0), Size::new(WIDTH-APP_X, HEIGHT));

        bounds
            .into_styled(
                PrimitiveStyleBuilder::default()
                .stroke_color(BinaryColor::Off)
                .fill_color(BinaryColor::On)
                .stroke_width(1)
                .build(),
                )
            .draw(display)
            .unwrap();

        Text::new(
            "TODO",
            bounds.top_left + Point::new(10, 20),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();

        display.partial_update(bounds.try_into().unwrap()).unwrap();

    }

    fn render(&mut self, buttons: &Pins, display: &mut UcDisplay) {}
}

