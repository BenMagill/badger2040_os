#![no_std]
#![no_main]

pub mod app;
use app::app::{Os, Buttons};
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

    let led = pins.led.into_push_pull_output();
    let a = pins.sw_a.into_pull_down_input();
    let b = pins.sw_b.into_pull_down_input();
    let c = pins.sw_c.into_pull_down_input();
    let up = pins.sw_up.into_pull_down_input();
    let down = pins.sw_down.into_pull_down_input();
    let buttons = Buttons {
        a,
        b,
        c,
        up,
        down,
    };

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut countdown = timer.count_down();
    
    let spi = spi.init(
        &mut pac.RESETS, 
        &clocks.peripheral_clock, 
        HertzU32::Hz(1000000), 
        &embedded_hal::spi::MODE_0);

    let mut display = uc8151::Uc8151::new(spi, cs, dc, busy, reset);

    // clear display
    display.disable();
    countdown.start(MicrosDurationU32::micros(10));
    let _ = nb::block!(countdown.wait());
    display.enable();
    countdown.start(MicrosDurationU32::micros(10));
    let _ = nb::block!(countdown.wait());
    while display.is_busy() {}

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.system_clock.freq().to_Hz());

    // Initialise display.
    display.setup(&mut delay, uc8151::LUT::Fast).unwrap();

    display.update().unwrap();

    let mut os = Os::new(buttons, led, display);
        
    os.run();
}
