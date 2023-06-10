use alloc::boxed::Box;
use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point, Size}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_6X13, FONT_10X20}}};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use pimoroni_badger2040::{hal::{gpio::{bank0::*, Output, PushPull, Pin, PullDown, Input, PullUp}, spi::Enabled}, pac::SPI0};
use uc8151::{Uc8151, WIDTH, HEIGHT};
use pimoroni_badger2040::hal::Spi;

use crate::{home::Home, shapes::Shapes};

static TOTAL_OPTIONS: u32 = 5;
pub static APP_X: u32 = WIDTH/3;

pub type UcDisplay = Uc8151<Spi<Enabled, SPI0, 8>, Pin<Gpio17, Output<PushPull>>, Pin<Gpio20, Output<PushPull>>, Pin<Gpio26, Input<PullUp>>, Pin<Gpio21, Output<PushPull>>>;
type LED = Pin<Gpio25, Output<PushPull>>;

pub trait App {
    // This should not call the update funciton, instead let the os do it
    fn init(&mut self, pins: &Pins, display: &mut UcDisplay, bounds: Rectangle) -> ();

    // This should call the update function when it is necessary
    fn render(&mut self, pins: &Pins, display: &mut UcDisplay, bounds: Rectangle) -> ();
}

pub struct Pins {
    pub led: LED,
    pub a: Pin<Gpio12, Input<PullDown>>,
    pub b: Pin<Gpio13, Input<PullDown>>,
    pub c: Pin<Gpio14, Input<PullDown>>,
    pub up: Pin<Gpio15, Input<PullDown>>,
    pub down: Pin<Gpio11, Input<PullDown>>,
}

pub struct Os {
    pins: Pins,
    options: &'static[&'static str; 5],
    selected_option: u32,
    display: UcDisplay,
    app: Box<dyn App>,
    app_bounds: Rectangle,
}

impl Os {
    pub fn new(buttons: Pins, display: UcDisplay) -> Os {
        let a = Home {};
        return Os {
            pins: buttons,
            options: &[
                "Home",
                "Shapes",
                "Text",
                "Buttons",
                "Images",
            ],
            selected_option: 0,
            display,
            app: Box::new(a),
            app_bounds: Rectangle::new(Point::new(APP_X as i32, 0), Size::new(WIDTH-APP_X, HEIGHT)),
        }
    }

    pub fn run(&mut self) -> ! {
        self.pins.led.set_high().unwrap();

        self.handle_option_change();

        loop {
            let mut option_changed = false;
            if self.pins.down.is_high().unwrap() {
                if self.selected_option != TOTAL_OPTIONS -1 {
                    self.selected_option += 1;
                    option_changed = true;
                } 
            } else if self.pins.up.is_high().unwrap() {
                if self.selected_option != 0 {
                    self.selected_option -= 1;
                    option_changed = true;
                } 
            }

            if option_changed {
                self.handle_option_change();
            }

            self.app.render(&self.pins, &mut self.display, self.app_bounds);
            //self.display.partial_update(self.app_bounds.try_into().unwrap()).unwrap();
        }
    }

    fn handle_option_change(&mut self) {
        self.load_app();
        self.draw_sidebar();
        
        self.app.init(&self.pins, &mut self.display, self.app_bounds);

        self.display.partial_update(Rectangle::new(Point::new(0, 0), Size::new(WIDTH, HEIGHT)).try_into().unwrap()).unwrap();
    }

    fn draw_sidebar(&mut self) {
        let bounds = Rectangle::new(Point::new(0, 0), Size::new(WIDTH/3, HEIGHT));
        bounds.into_styled(
            PrimitiveStyleBuilder::default()
                .stroke_color(BinaryColor::Off)
                .fill_color(BinaryColor::On)
                .stroke_width(2)
                .build(),
            )
            .draw(&mut self.display)
            .unwrap();

        for i in 0..=4 {
            self.draw_option_box(i, self.options[i as usize], i == self.selected_option as i32);
        }
    }

    fn draw_option_box(&mut self, n: i32, text: &str, selected: bool) {
        let size = HEIGHT/5;
        let bounds = Rectangle::new(Point::new(0, n*(size as i32)), Size::new(WIDTH/3, size));
        let stroke = BinaryColor::Off;
        let fill = match selected {
           true => BinaryColor::Off,
           false => BinaryColor::On
        };

        bounds.into_styled(
            PrimitiveStyleBuilder::default()
                .stroke_color(stroke)
                .fill_color(fill)
                .stroke_width(2)
                .build(),
            )
            .draw(&mut self.display)
            .unwrap();
        Text::new(
            text,
            bounds.center() + Point::new(0, 2),
            MonoTextStyle::new(&FONT_6X13, fill.invert()),
            )
            .draw(&mut self.display)
            .unwrap();
    }

    fn load_app(&mut self) {
        // TODO: This should set the current app loaded in the "os"
        // It should be allocated on a heap so that each app can have its own data and setup
        match self.selected_option {
            1 => { self.app = Box::new(Shapes {}) }
            _ => { self.app = Box::new(Home {}) }
        }
    }

    // random stuff 
    fn home(&mut self)  {
  
        let bounds = Rectangle::new(Point::new(0, 0), Size::new(WIDTH, HEIGHT));

        bounds
            .into_styled(
                PrimitiveStyleBuilder::default()
                .stroke_color(BinaryColor::Off)
                .fill_color(BinaryColor::On)
                .stroke_width(1)
                .build(),
                )
            .draw(&mut self.display)
            .unwrap();

        Text::new(
            "hello world",
            bounds.center() + Point::new(0, 2),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            )
            .draw(&mut self.display)
            .unwrap();

        self.display.partial_update(bounds.try_into().unwrap()).unwrap();

    }
}
