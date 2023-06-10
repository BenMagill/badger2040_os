use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_10X20}}};
use embedded_hal::digital::v2::InputPin;

use crate::os::os::{App, Pins, UcDisplay};

pub struct Buttons {
    a: bool,
}

impl Buttons {
    pub fn new () -> Buttons {
        return Buttons {
            a: false, 
        }
    }
}

impl App for Buttons {

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
            "OFF",
            bounds.top_left + Point::new(10, 20),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
    }

    fn render(&mut self, buttons: &Pins, display: &mut UcDisplay, bounds: Rectangle) {
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
        
        if !self.a && buttons.a.is_high().unwrap() {
            self.a = true;
            Text::new(
                "ON",
                bounds.top_left + Point::new(10, 20),
                MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
                )
                .draw(display)
                .unwrap();

            display.partial_update(bounds.try_into().unwrap()).unwrap();
        }
    }
}

