#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use pico_traffic_lights::phases::Phase;
use rp_pico::hal::Timer;
use rp_pico::{entry, Pins, XOSC_CRYSTAL_FREQ};

use rp_pico::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    prelude::*,
    sio::Sio,
    watchdog::Watchdog,
};

use smart_leds::{brightness, gamma, SmartLedsWrite, RGB8};

use ws2812_pio::Ws2812;

const BRIGHTNESS: u8 = 255;

const OFF: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
const RED: RGB8 = RGB8 {
    r: 227,
    g: 24,
    b: 55,
};
const AMBER: RGB8 = RGB8 {
    r: 255,
    g: 210,
    b: 0,
};
const GREEN: RGB8 = RGB8 {
    r: 94,
    g: 151,
    b: 50,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

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

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws = Ws2812::new(
        pins.gpio28.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut phase = Phase::new();

    loop {
        let leds = match phase {
            Phase::Red => {
                info!("Red");
                [RED, OFF, OFF]
            }
            Phase::StartingRedAmber => {
                info!("StartingRedAmber");
                [RED, AMBER, OFF]
            }
            Phase::Green => {
                info!("Green");
                led_pin.set_high().unwrap();
                [OFF, OFF, GREEN]
            }
            Phase::LeavingAmber => {
                info!("LeavingAmber");
                led_pin.set_low().unwrap();
                [OFF, AMBER, OFF]
            }
        };
        ws.write(rgb_to_grb(brightness(
            gamma(leds.iter().copied()),
            BRIGHTNESS,
        )))
        .unwrap();
        phase.next(&mut delay);
    }
}

/// The ws-28120-pio create does not provide a mechanism to allow
/// configuration of the channel ordering. For some reason, the LEDs
/// I'm using expect RGB rather than the more typical GRB.
fn rgb_to_grb(input: impl Iterator<Item = RGB8>) -> impl Iterator<Item = RGB8> {
    input.map(|RGB8 { r, g, b }| RGB8 { r: g, g: r, b })
}
