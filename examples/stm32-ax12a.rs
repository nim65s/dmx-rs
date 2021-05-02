#![deny(unsafe_code)]
#![no_std]
#![no_main]

/*
 * In this example, we short-circuit RX & TX, and plug this to the DATA half-duplex port.
 * A no-op zero-cost Dummy direction pin is provided to the controller.
 * The AX12A can't drive the DATA bus versus the STM32 PushPull on PA9, so this is write-only,
 * and the controller is configured to expect 0 Response after set commands
 *
 * the AX12A has the following configuration: Protocol 1, Baudrate: 115_200, device ID: 1
 */

use cortex_m_rt::entry;
use dmx::{ax12a::AX12A, protocol::Controller};
use dummy_pin::DummyPin;
use nb::block;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial,
    timer::{CountDownTimer, Timer},
};

fn sleep_ms(timer: &mut CountDownTimer<pac::SYST>, ms: usize) {
    timer.reset();
    block!(timer.wait()).unwrap();

    for _ in 0..ms {
        block!(timer.wait()).unwrap();
    }

    block!(timer.wait()).unwrap();
}

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
    let mut dmx = Controller::new_1(serial, dummy_pin, 0);
    sleep_ms(&mut timer, 500);

    for led in 0..6 {
        match dmx.set_ax12a_led(1, led % 2) {
            Err(e) => rprintln!("set led to {} err: {:?}", led % 2, e),
            Ok(_) => rprintln!("set led to {} ok", led % 2),
        }
        sleep_ms(&mut timer, 500);
    }

    match dmx.set_ax12a_torque_enable(1, 1) {
        Err(e) => rprintln!("enable torque err: {:?}", e),
        Ok(_) => rprintln!("enable torque ok"),
    }

    match dmx.set_ax12a_moving_speed(1, 00) {
        Err(e) => rprintln!("set moving speed err: {:?}", e),
        Ok(_) => rprintln!("set moving speed ok"),
    }

    for goal in 0..8 {
        match dmx.set_ax12a_goal_position(1, goal * 127) {
            Err(e) => rprintln!("set goal position {} err: {:?}", goal, e),
            Ok(_) => rprintln!("set goal position {} ok", goal),
        }
        sleep_ms(&mut timer, 1000);
    }

    for goal in 0..=8 {
        match dmx.set_ax12a_goal_position(1, (8 - goal) * 127) {
            Err(e) => rprintln!("set goal position {} err: {:?}", 8 - goal, e),
            Ok(_) => rprintln!("set goal position {} ok", 8 - goal),
        }
        sleep_ms(&mut timer, 1000);
    }

    match dmx.set_ax12a_torque_enable(1, 0) {
        Err(e) => rprintln!("disable torque err: {:?}", e),
        Ok(_) => rprintln!("disable torque ok"),
    }

    loop {
        for led in 0..2 {
            match dmx.set_ax12a_led(1, led % 2) {
                Err(e) => rprintln!("set led to {} err: {:?}", led % 2, e),
                Ok(_) => rprintln!("set led to {} ok", led % 2),
            }
            sleep_ms(&mut timer, 100);
        }
    }
}
