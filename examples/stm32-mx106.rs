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
        rprintln!("torque enable {}", dmx.get_mx106_torque_enable(1).unwrap());
        rprintln!(
            "present position {}",
            dmx.get_mx106_present_position(1).unwrap()
        );
        rprintln!("goal position {}", dmx.get_mx106_goal_position(1).unwrap());
        dmx.set_mx106_operating_mode(1, 4)
            .map_err(|e| rprintln!("set operating mode err: {:?}", e)
            .ok();
        dmx.set_mx106_torque_enable(1, 1)
            .map_err(|e| rprintln!("set torque enable err: {:?}", e))
            .ok();
        dmx.set_mx106_led(1, 1)
            .map_err(|e| rprintln!("set led err: {:?}", e))
            .ok();

        //let goal: u32 = 100;
        let goal: u32 = 8000;

        if let Err(e) = dmx.set_mx106_velocity_limit(1, 1) {
            rprintln!("set velocity limit error: {:?}", e);
        }

        dmx.set_mx106_velocity_limit(1, 1)
            .map_err(|e| rprintln!("set velocity err: {:?}", e))
            .ok();
        dmx.set_mx106_profile_acceleration(1, 10)
            .map_err(|e| rprintln!("set profile aceleration err: {:?}", e))
            .ok();
        dmx.set_mx106_profile_velocity(1, 1000)
            .map_err(|e| rprintln!("set profile velocity err: {:?}", e))
            .ok();
        dmx.set_mx106_goal_position(1, goal)
            .map_err(|e| rprintln!("set goal position err: {:?}", e))
            .ok();

        while distance_u32(goal, dmx.get_mx106_present_position(1).unwrap()) > 10 {
            block!(timer.wait()).unwrap();
        }

        dmx.set_mx106_torque_enable(1, 0)
            .map_err(|e| rprintln!("set torque enable err: {:?}", e))
            .ok();
        dmx.set_mx106_led(1, 0)
            .map_err(|e| rprintln!("set led err: {:?}", e))
            .ok();
        rprintln!("torque enable {}", dmx.get_mx106_torque_enable(1).unwrap());
        rprintln!(
            "present position {}",
            dmx.get_mx106_present_position(1).unwrap()
        );
        rprintln!("goal position {}", dmx.get_mx106_goal_position(1).unwrap());

        timer.reset();
        block!(timer.wait()).unwrap();

        for _ in 0..1000 {
            block!(timer.wait()).unwrap();
        }

        block!(timer.wait()).unwrap();
    }
}
