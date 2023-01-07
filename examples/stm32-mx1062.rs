#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use dmx::{
    mx1062::MX1062,
    protocol::{Controller, Protocol},
};
use nb::block;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, serial, timer::Timer};

#[entry]
fn main() -> ! {
    let id = 1;
    let baudrate = 57_143;

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
    rprintln!("Connecting to mx1062 ID {} @ {}", id, baudrate);

    // Initialize dynamixel on PA2-4
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;
    let dir = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    let serial = serial::Serial::usart2(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(baudrate.bps()),
        clocks,
        &mut rcc.apb1,
    );
    let mut dmx = Controller::new_2(serial, dir, 1);

    let mut i = false;

    loop {
        rprintln!("ping {}", dmx.ping(1));
        rprintln!("get torque enable: {:?}", dmx.get_mx1062_torque_enable(id));
        rprintln!(
            "get present position: {:?}",
            dmx.get_mx1062_present_position(id)
        );
        rprintln!("get goal position: {:?}", dmx.get_mx1062_goal_position(id));
        rprintln!(
            "set operationg mode {}: {:?}",
            4,
            dmx.set_mx1062_operating_mode(id, 4)
        );
        rprintln!(
            "set torque enable {}: {:?}",
            1,
            dmx.set_mx1062_torque_enable(id, 1)
        );
        rprintln!("set led {}: {:?}", 1, dmx.set_mx1062_led(id, 1));

        let goal: u32 = if i { 3000 } else { 2000 };
        i = !i;

        rprintln!(
            "set velocity limit {}: {:?}",
            1,
            dmx.set_mx1062_velocity_limit(id, 1)
        );
        rprintln!(
            "set profile acceleration {}: {:?}",
            10,
            dmx.set_mx1062_profile_acceleration(id, 10)
        );
        rprintln!(
            "set profile velocity {}: {:?}",
            1000,
            dmx.set_mx1062_profile_velocity(id, 1000)
        );
        rprintln!(
            "set goal position {}: {:?}",
            goal,
            dmx.set_mx1062_goal_position(id, goal)
        );

        loop {
            let pose = dmx.get_mx1062_present_position(id);
            rprintln!("get present_position : {:?}", pose);
            if goal.abs_diff(pose.unwrap()) < 10 {
                break;
            }
            block!(timer.wait()).ok();
        }

        rprintln!(
            "set torque enable {}: {:?}",
            0,
            dmx.set_mx1062_torque_enable(id, 0)
        );
        rprintln!("set led {}: {:?}", 0, dmx.set_mx1062_led(id, 0));
        rprintln!("get torque enable {:?}", dmx.get_mx1062_torque_enable(id));
        rprintln!(
            "get present position {:?}",
            dmx.get_mx1062_present_position(id)
        );
        rprintln!("get goal position {:?}", dmx.get_mx1062_goal_position(id));

        timer.reset();
        block!(timer.wait()).ok();

        for _ in 0..1000 {
            block!(timer.wait()).ok();
        }

        block!(timer.wait()).ok();
    }
}
