#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use pico_traffic_lights::phases::Phase;
use rp_pico::{entry, Pins, XOSC_CRYSTAL_FREQ};

use rp_pico::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
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

    let mut phase = Phase::new();

    loop {
        match phase {
            Phase::Red => {
                info!("Red");
            }
            Phase::StartingRedAmber => {
                info!("StartingRedAmber");
            }
            Phase::Green => {
                info!("Green");
                led_pin.set_high().unwrap();
            }
            Phase::LeavingAmber => {
                info!("LeavingAmber");
                led_pin.set_low().unwrap();
            }
        }
        phase.next(&mut delay);
    }
}
