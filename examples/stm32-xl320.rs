#![deny(unsafe_code)]
#![no_std]
#![no_main]

/*
 * In this example, we short-circuit RX & TX, and plug this to the DATA half-duplex port.
 * We don't receive sent packets, and that's actually a good thing, but I don't know why.
 * A no-op zero-cost Dummy direction pin is provided to the controller.
 *
 * the XL320 has the following configuration: Baudrate: 115_200, device ID: 1
 */

use cortex_m_rt::entry;
use dmx::{
    protocol::{Controller, Instruction, Protocol},
    xl320::XL320,
};
use dummy_pin::DummyPin;
use nb::block;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, serial, timer::Timer};

#[entry]
fn main() -> ! {
    let id = 2;
    let baudrate = 115_200;

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.khz());

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    // Initialize debugger
    rtt_init_print!();
    rprintln!("rprintln ok");
    rprintln!("Connecting to XL320 ID {} @ {}", id, baudrate);

    // Initialize dynamixel on PA9-10
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;
    let dummy_pin = DummyPin::new_low();

    let serial = serial::Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(baudrate.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let mut dmx = Controller::new_2(serial, dummy_pin, 0);

    loop {
        for led in 0..8 {
            rprintln!("set led {}: {:?}", led, dmx.set_xl320_led(id, led));
            timer.reset();
            block!(timer.wait()).unwrap();

            for _ in 0..1000 {
                block!(timer.wait()).unwrap();
            }

            block!(timer.wait()).unwrap();
        }
    }
}
