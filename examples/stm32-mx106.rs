#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use dmx::{
    convert::distance_u32,
    mx106::MX106,
    protocol::{Controller, Protocol},
};
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
    //let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Initialize debugger
    rtt_init_print!();
    rprintln!("rprintln ok");

    // Initialize dynamixel on PA2-4
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;
    let dir = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    let serial = serial::Serial::usart2(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(57_143.bps()),
        clocks,
        &mut rcc.apb1,
    );
    let mut dmx = Controller::new_2(serial, dir);

    loop {
        rprintln!("ping {}", dmx.ping(1));
        rprintln!(
            "torque enable {}",
            dmx.get_mx106_torque_enable(1).ok().unwrap()
        );
        rprintln!(
            "present position {}",
            dmx.get_mx106_present_position(1).ok().unwrap()
        );
        rprintln!(
            "goal position {}",
            dmx.get_mx106_goal_position(1).ok().unwrap()
        );
        dmx.set_mx106_operating_mode(1, 4);
        dmx.set_mx106_torque_enable(1, 1);
        dmx.set_mx106_led(1, 1);

        //let goal: u32 = 100;
        let goal: u32 = 8000;

        dmx.set_mx106_profile_acceleration(1, 10);
        dmx.set_mx106_profile_velocity(1, 1000);
        dmx.set_mx106_goal_position(1, goal);

        while distance_u32(goal, dmx.get_mx106_present_position(1).ok().unwrap()) > 10 {
            block!(timer.wait()).unwrap();
        }

        dmx.set_mx106_torque_enable(1, 0);
        dmx.set_mx106_led(1, 0);
        rprintln!(
            "torque enable {}",
            dmx.get_mx106_torque_enable(1).ok().unwrap()
        );
        rprintln!(
            "present position {}",
            dmx.get_mx106_present_position(1).ok().unwrap()
        );
        rprintln!(
            "goal position {}",
            dmx.get_mx106_goal_position(1).ok().unwrap()
        );

        timer.reset();
        block!(timer.wait()).unwrap();

        for _ in 0..1000 {
            block!(timer.wait()).unwrap();
        }

        block!(timer.wait()).unwrap();
    }
}
