#![deny(unsafe_code)]
#![no_std]
#![no_main]

/*
 * In this example, we short-circuit RX & TX, and plug this to the DATA half-duplex port.
 * We don't receive sent packets, and that's actually a good thing, but I don't know why.
 * A no-op zero-cost Dummy direction pin is provided to the controller.
 *
 * the AX12A has the following configuration: Protocol 1, Baudrate: 115_200, device ID: 1
 */

use cortex_m_rt::entry;
use dmx::{
    ax12a::AX12A,
    convert::distance_u16,
    protocol::{Controller, Instruction, Protocol},
    protocol_1::Error1,
};
use dummy_pin::DummyPin;
use nb::block;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, serial, timer::Timer};

#[entry]
fn main() -> ! {
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

    // Initialize dynamixel on PA9-10
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;
    let dummy_pin = DummyPin::new_low();

    let serial = serial::Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(115_200.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let mut dmx = Controller::new_1(serial, dummy_pin);

    rprintln!("ping {:?}", dmx.send(1, Instruction::Ping, &[]));
    match dmx.recv() {
        Ok(resp) => rprintln!("Ok received {:?}", resp),
        Err(Error1::Communication(_)) => rprintln!("Err communication error"),
        Err(Error1::Protocol(status)) => rprintln!("Err protocol error: {:?}", status),
        Err(Error1::ChecksumError) => rprintln!("Err checksum error"),
    }

    for led in 0..10 {
        rprintln!("set led to {}", led % 2);
        if let Err(_) = dmx.set_ax12a_led(1, led % 2) {
            rprintln!("error setting led");
        }
        timer.reset();
        block!(timer.wait()).unwrap();

        for _ in 0..1000 {
            block!(timer.wait()).unwrap();
        }

        block!(timer.wait()).unwrap();
    }

    loop {
        rprintln!("ping {}", dmx.ping(1));
        rprintln!(
            "torque enable {}",
            dmx.get_ax12a_torque_enable(1).ok().unwrap()
        );
        rprintln!(
            "present position {}",
            dmx.get_ax12a_present_position(1).ok().unwrap()
        );
        rprintln!(
            "goal position {}",
            dmx.get_ax12a_goal_position(1).ok().unwrap()
        );
        dmx.set_ax12a_torque_enable(1, 1)
            .map_err(|e| rprintln!("set torque enable err: {:?}", e))
            .ok();
        dmx.set_ax12a_led(1, 1)
            .map_err(|e| rprintln!("set led err: {:?}", e))
            .ok();

        //let goal: u16 = 100;
        let goal: u16 = 8000;

        dmx.set_ax12a_goal_position(1, goal)
            .map_err(|e| rprintln!("set goal position: {:?}", e))
            .ok();

        while distance_u16(goal, dmx.get_ax12a_present_position(1).ok().unwrap()) > 10 {
            block!(timer.wait()).unwrap();
        }

        dmx.set_ax12a_torque_enable(1, 0)
            .map_err(|e| rprintln!("set torque enable err: {:?}", e))
            .ok();
        dmx.set_ax12a_led(1, 0)
            .map_err(|e| rprintln!("set led err: {:?}", e))
            .ok();
        rprintln!(
            "torque enable {}",
            dmx.get_ax12a_torque_enable(1).ok().unwrap()
        );
        rprintln!(
            "present position {}",
            dmx.get_ax12a_present_position(1).ok().unwrap()
        );
        rprintln!(
            "goal position {}",
            dmx.get_ax12a_goal_position(1).ok().unwrap()
        );

        timer.reset();
        block!(timer.wait()).unwrap();

        for _ in 0..1000 {
            block!(timer.wait()).unwrap();
        }

        block!(timer.wait()).unwrap();
    }
}
