use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_10X20}}};
use embedded_hal::digital::v2::InputPin;

use crate::os::os::{App, Pins, UcDisplay};

pub struct Buttons {
    a: bool,
    b: bool,
    c: bool,
}

impl Buttons {
    pub fn new () -> Buttons {
        return Buttons {
            a: false, 
            b: false, 
            c: false, 
        }
    }
    
    fn button_status(value: bool) -> &'static str {
        match value {
            true => "WORKING",
            false => "NOT PRESSED",
        }
    }

    fn draw(&mut self, display: &mut UcDisplay, bounds: Rectangle) {
        Text::new(
            "A: ",
            bounds.top_left + Point::new(10, 20),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        Text::new(
            "B: ",
            bounds.top_left + Point::new(10, 40),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        Text::new(
            "C: ",
            bounds.top_left + Point::new(10, 60),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        
        let a_text = Buttons::button_status(self.a);
        let b_text = Buttons::button_status(self.b);
        let c_text = Buttons::button_status(self.c);

        Text::new(
            a_text,
            bounds.top_left + Point::new(40, 20),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        Text::new(
            b_text,
            bounds.top_left + Point::new(40, 40),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
        Text::new(
            c_text,
            bounds.top_left + Point::new(40, 60),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(display)
            .unwrap();
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
        
        self.draw(display, bounds);
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

        let mut change = false;        
        if !self.a && buttons.a.is_high().unwrap() {
            self.a = true;
            change = true;
        }
        if !self.b && buttons.b.is_high().unwrap() {
            self.b = true;
            change = true;
        }
        if !self.c && buttons.c.is_high().unwrap() {
            self.c = true;
            change = true
        }

        if change {
            self.draw(display, bounds);
            display.partial_update(bounds.try_into().unwrap()).unwrap();
        }
    }
}

