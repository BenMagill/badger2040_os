use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point}, pixelcolor::BinaryColor, Drawable, text::{Text, TextStyle}, mono_font::{MonoTextStyle, ascii::{FONT_10X20, FONT_8X13_BOLD, FONT_8X13_ITALIC, FONT_8X13}, }};

use crate::os::os::{App, Pins, UcDisplay};

pub struct TextApp {}

impl App for TextApp {
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
            "Normal",
            bounds.top_left + Point::new(5, 20),
            MonoTextStyle::new(&FONT_8X13, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        Text::new(
            "Bold",
            bounds.top_left + Point::new(5, 40),
            MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        Text::new(
            "Italic",
            bounds.top_left + Point::new(5, 60),
            MonoTextStyle::new(&FONT_8X13_ITALIC, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
    }

    fn render(&mut self, _buttons: &Pins, _display: &mut UcDisplay, _bounds: Rectangle) {}
}
