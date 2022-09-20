#![no_std]
#![no_main]
use core::iter::once;

use adafruit_kb2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock as _},
        pio::PIOExt as _,
        Sio, Timer, Watchdog,
    },
    pac, Pins, XOSC_CRYSTAL_FREQ,
};
use cortex_m_rt::entry;
use embedded_hal::timer::CountDown;
use embedded_time::duration::Extensions;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

mod panic;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico borad is 12Mhz
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Make a "delay" wrapper from system timer, we can use to track
    // the passing of time
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.neopixel.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut delay_ms = |ms: u32| {
        delay.start(ms.milliseconds());
        let _ = nb::block!(delay.wait());
    };
    loop {
        ws.write(brightness(once(RGB8::new(255, 255, 255)), 32))
            .unwrap();
        delay_ms(250);
        ws.write(brightness(once(RGB8::new(255, 0, 0)), 32))
            .unwrap();
        delay_ms(250);
    }
}
