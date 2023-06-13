#![no_std]
#![no_main]

extern crate alloc;

pub mod os;
pub mod home;
pub mod shapes;
pub mod buttons;
pub mod text;

use os::os::{Os, Pins};
use cortex_m::prelude::_embedded_hal_timer_CountDown;
use fugit::{HertzU32, MicrosDurationU32};
use pimoroni_badger2040::entry;
use panic_halt as _;
use pimoroni_badger2040::hal::{pac, Timer, Clock};
use pimoroni_badger2040::hal;
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

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
    let pins = Pins {
        led, 
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

    let mut os = Os::new(pins, display);
    os.run();
}
