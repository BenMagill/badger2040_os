use embedded_graphics::{primitives::{Rectangle, PrimitiveStyleBuilder, Primitive}, prelude::{Point, Size}, pixelcolor::BinaryColor, Drawable, text::Text, mono_font::{MonoTextStyle, ascii::{FONT_6X13, FONT_10X20}}};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use pimoroni_badger2040::{Pins, hal::{gpio::{bank0::*, Output, PushPull, Pin, PullDown, Input, PullUp}, spi::Enabled}, pac::SPI0};
use uc8151::{Uc8151, WIDTH, HEIGHT};
use pimoroni_badger2040::hal::Spi;

static TOTAL_OPTIONS: u32 = 5;

type UcDisplay = Uc8151<Spi<Enabled, SPI0, 8>, Pin<Gpio17, Output<PushPull>>, Pin<Gpio20, Output<PushPull>>, Pin<Gpio26, Input<PullUp>>, Pin<Gpio21, Output<PushPull>>>;
type LED = Pin<Gpio25, Output<PushPull>>;

pub struct Buttons {
    pub a: Pin<Gpio12, Input<PullDown>>,
    pub b: Pin<Gpio13, Input<PullDown>>,
    pub c: Pin<Gpio14, Input<PullDown>>,
    pub up: Pin<Gpio15, Input<PullDown>>,
    pub down: Pin<Gpio11, Input<PullDown>>,
}

pub struct Os {
    led: Pin<Gpio25, Output<PushPull>>,
    buttons: Buttons,
    options: &'static[&'static str; 5],
    selected_option: u32,
    display: UcDisplay,
}

impl Os {
    pub fn new(buttons: Buttons, led: LED, display: UcDisplay) -> Os {
        return Os {
            buttons,
            led,
            options: &[
                "Home",
                "Shapes",
                "Text",
                "Buttons",
                "Images",
            ],
            selected_option: 0,
            display,
        }
    }

    pub fn run(&mut self) -> ! {
        self.led.set_high();

        self.draw_sidebar();

        loop {
            let mut option_changed = false;
            if self.buttons.down.is_high().unwrap() {
                if self.selected_option != TOTAL_OPTIONS -1 {
                    self.selected_option += 1;
                    option_changed = true;
                } 
            } else if self.buttons.up.is_high().unwrap() {
                if self.selected_option != 0 {
                    self.selected_option -= 1;
                    option_changed = true;
                } 
            }

            if option_changed {
               self.draw_sidebar();
               // todo: also change which app is currently selected
            }

            self.run_app();
        }
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
            .draw(&mut self.display);

        for i in 0..=4 {
            self.draw_option_box(i, self.options[i as usize], i == self.selected_option as i32);
        }
        
        self.display.partial_update(bounds.try_into().unwrap()).unwrap();
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
            .draw(&mut self.display);
        Text::new(
            text,
            bounds.center() + Point::new(0, 2),
            MonoTextStyle::new(&FONT_6X13, fill.invert()),
            )
            .draw(&mut self.display);
    }

    fn run_app(&mut self) {
        // questions
        //  how can an "app" know when it needs to re-render or not
        //  buttons app needs to be continuously called to know when buttons pressed
        //  but the others wont need to 
        //  I guess a flag could be send saying if this is the first render or not?
        //      so basically allow for an "init" phase and the "render" cycle
        if self.selected_option == 321 {
        self.home();
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
