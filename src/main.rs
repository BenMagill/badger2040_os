#![no_std]
#![no_main]

use cortex_m::prelude::{_embedded_hal_timer_CountDown, _embedded_hal_blocking_spi_Write, _embedded_hal_serial_Write};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_6X13};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Point, Size, DrawTarget};
use embedded_graphics::{primitives::*, Drawable};
use embedded_graphics::text::Text;
use fugit::{HertzU32, MicrosDurationU32};
use pimoroni_badger2040::entry;

use embedded_hal::digital::v2::{OutputPin, InputPin};
use panic_halt as _;

use pimoroni_badger2040::hal::gpio::{Pin, Output, PushPull, Input, PullUp};
use pimoroni_badger2040::hal::gpio::bank0::*;
use pimoroni_badger2040::hal::{pac, Spi, Timer, Clock};
use pimoroni_badger2040::hal;
use pimoroni_badger2040::hal::spi::{Disabled, Enabled};
use pimoroni_badger2040::pac::SPI0;
use uc8151::{WIDTH, HEIGHT, Uc8151};

#[entry]
fn main() -> ! {
    let ON = BinaryColor::Off;
    let OFF = BinaryColor::On;

    let mut pac = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_badger2040::XOSC_CRYSTAL_FREQ, 
        pac.XOSC, 
        pac.CLOCKS, 
        pac.PLL_SYS, 
        pac.PLL_USB, 
        &mut pac.RESETS, 
        &mut watchdog,
    ).ok().unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = pimoroni_badger2040::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS
    );
    
    pins.sclk.into_mode::<hal::gpio::FunctionSpi>();
    pins.mosi.into_mode::<hal::gpio::FunctionSpi>();
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI0);
    let dc = pins.inky_dc.into_push_pull_output();
    let cs = pins.inky_cs_gpio.into_push_pull_output();
    let busy = pins.inky_busy.into_pull_up_input();
    let reset = pins.inky_res.into_push_pull_output();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut countdown = timer.count_down();
    
    let spi = spi.init(
        &mut pac.RESETS, 
        &clocks.peripheral_clock, 
        HertzU32::Hz(1000000), 
        &embedded_hal::spi::MODE_0);

    let mut display = uc8151::Uc8151::new(spi, cs, dc, busy, reset);

    let mut led_pin = pins.led.into_push_pull_output();
    let button_a_pin = pins.sw_a.into_pull_down_input();
    let button_b_pin = pins.sw_b.into_pull_down_input();
    let button_c_pin = pins.sw_c.into_pull_down_input();
    let button_up_pin = pins.sw_up.into_pull_down_input();
    let button_down_pin = pins.sw_down.into_pull_down_input();

    display.disable();
    countdown.start(MicrosDurationU32::micros(10));
    let _ = nb::block!(countdown.wait());
    display.enable();
    countdown.start(MicrosDurationU32::micros(10));
    let _ = nb::block!(countdown.wait());
    // Wait for the screen to finish reset
    while display.is_busy() {}

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.system_clock.freq().to_Hz());

    // Initialise display. Using the default LUT speed setting
    display.setup(&mut delay, uc8151::LUT::Fast).unwrap();

    display.update().unwrap();

    let bounds = Rectangle::new(Point::new(0, 0), Size::new(WIDTH, HEIGHT));

    bounds
        .into_styled(
            PrimitiveStyleBuilder::default()
            .stroke_color(BinaryColor::Off)
            .fill_color(BinaryColor::On)
            .stroke_width(1)
            .build(),
            )
        .draw(&mut display)
        .unwrap();

    Text::new(
        "hello world",
        bounds.center() + Point::new(0, 2),
        MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
        )
        .draw(&mut display)
        .unwrap();

    display.partial_update(bounds.try_into().unwrap()).unwrap();

    // todo remove?
    countdown.start(MicrosDurationU32::millis(500));
    let _ = nb::block!(countdown.wait());

    // todo: 
    // - create list of options for menu
    // - variable for which option is chosen
    let options = &[
        "Home",
        "Shapes",
        "Text",
        "Buttons",
        "Images",
    ];
    let mut selected_option = 0u32;
    let total_options: u32 = options.len().try_into().unwrap_or(1);

    // drawing sidebar
    let bounds = Rectangle::new(Point::new(0, 0), Size::new(WIDTH/3, HEIGHT));
    bounds.into_styled(
        PrimitiveStyleBuilder::default()
            .stroke_color(ON)
            .fill_color(OFF)
            .stroke_width(2)
            .build(),
        )
        .draw(&mut display)
        .unwrap();

    for i in 0..=4 {
        draw_option_box(&mut display, i, selected_option as i32 == i, options[i as usize]);
    }
    
    display.partial_update(bounds.try_into().unwrap()).unwrap();

    loop {
        let mut option_changed = false;
        // todo:
        // - check if up or down pressed
            // - if so change selected option
        // - render side menu
        // - render selection opions UI
        if button_down_pin.is_high().unwrap() {
            if selected_option != total_options -1 {
                selected_option += 1;
                option_changed = true;
            } 
        } else if button_up_pin.is_high().unwrap() {
            if selected_option != 0 {
                selected_option -= 1;
                option_changed = true;
            } 
        }

        if option_changed {
            // on is apparently off
            //display.clear(BinaryColor::On);  
            //Text::new(
            //    value_text(selected_option),
            //    bounds.center() + Point::new(0, 2),
            //    MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            //    )
            //    .draw(&mut display)
            //    .unwrap();
            //display.update();
            render(&mut display, selected_option)
        }
        // execute_app(selected_option);
    }
}
type UcDisplay = Uc8151<Spi<Enabled, SPI0, 8>, Pin<Gpio17, Output<PushPull>>, Pin<Gpio20, Output<PushPull>>, Pin<Gpio26, Input<PullUp>>, Pin<Gpio21, Output<PushPull>>>;
fn render(display: &mut UcDisplay, selected_option: u32) {
    let options = &[
        "Home",
        "Shapes",
        "Text",
        "Buttons",
        "Images",
    ];
    let bounds = Rectangle::new(Point::new(0, 0), Size::new(WIDTH/3, HEIGHT));
    bounds.into_styled(
        PrimitiveStyleBuilder::default()
            .stroke_color(BinaryColor::Off)
            .fill_color(BinaryColor::On)
            .stroke_width(2)
            .build(),
        )
        .draw(display);

    for i in 0..=4 {
        draw_option_box(display, i, selected_option as i32 == i, options[i as usize]);
    }
    
    display.partial_update(bounds.try_into().unwrap()).unwrap();
}

fn draw_option_box<D: DrawTarget<Color = BinaryColor>>(display: &mut D, n: i32, selected: bool, text: &str) {
        let size = HEIGHT/5;
        let bounds = Rectangle::new(Point::new(0, n*(size as i32)), Size::new(WIDTH/3, size));
        let stroke = match selected {
           true => BinaryColor::On,
           false => BinaryColor::Off
        };
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
            .draw(display);
        Text::new(
            text,
            bounds.center() + Point::new(0, 2),
            MonoTextStyle::new(&FONT_6X13, stroke),
            )
            .draw(display);
}

fn execute_app(option: u32) {
   match option {
      0 => {},
      1 => {},
      2 => {},
      3 => {},
      _ => {},
   } 
}

fn value_text(value: u32) -> &'static str {
    const CHANNEL_NUM: &[&str] = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
    ];

    #[allow(clippy::cast_sign_loss)]
    CHANNEL_NUM[(value % 16) as usize]
}
