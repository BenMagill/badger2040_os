use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive, Circle, PrimitiveStyle, Triangle}, prelude::{Point, Size}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_10X20}}};

use crate::os::os::{App, Pins, UcDisplay};

pub struct Shapes {}

impl App for Shapes {
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

       Circle::new(bounds.top_left + Point::new(60, 60), 50) 
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(display)
            .unwrap();
       
        Triangle::new(bounds.top_left + Point::new(40, 20), bounds.top_left + Point::new(80, 20), bounds.top_left + Point::new(60, 60))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(display).unwrap();

        Rectangle::new(bounds.top_left + Point::new(120, 50), Size::new(50, 50))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(display).unwrap();
    }

    fn render(&mut self, _buttons: &Pins, _display: &mut UcDisplay, _bounds: Rectangle) {}
}

