use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point, Size}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_10X20}}};
use uc8151::{WIDTH, HEIGHT};
use crate::os::os::{App, Pins, UcDisplay, APP_X};

pub struct Home {}

impl App for Home {
    fn init(&mut self, _buttons: &Pins, display: &mut UcDisplay, bounds: Rectangle) {
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
            "hello world",
            bounds.top_left + Point::new(10, 20),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();

        display.partial_update(bounds.try_into().unwrap()).unwrap();

    }

    fn render(&mut self, _buttons: &Pins, _display: &mut UcDisplay, bounds: Rectangle) {}
}
