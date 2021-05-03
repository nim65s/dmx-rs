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
    rprintln!("Connecting to MX106 ID {} @ {}", id, baudrate);

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

    loop {
        rprintln!("ping {}", dmx.ping(1));
        rprintln!("torque enable {:?}", dmx.get_mx106_torque_enable(id));
        rprintln!("present position {:?}", dmx.get_mx106_present_position(id));
        rprintln!("goal position {:?}", dmx.get_mx106_goal_position(id));
        dmx.set_mx106_operating_mode(id, 4)
            .map_err(|e| rprintln!("set operating mode err: {:?}", e))
            .ok();
        dmx.set_mx106_torque_enable(id, 1)
            .map_err(|e| rprintln!("set torque enable err: {:?}", e))
            .ok();
        dmx.set_mx106_led(id, 1)
            .map_err(|e| rprintln!("set led err: {:?}", e))
            .ok();

        let goal: u32 = 3000;

        if let Err(e) = dmx.set_mx106_velocity_limit(id, 1) {
            rprintln!("set velocity limit error: {:?}", e);
        }

        dmx.set_mx106_velocity_limit(id, 1)
            .map_err(|e| rprintln!("set velocity err: {:?}", e))
            .ok();
        dmx.set_mx106_profile_acceleration(id, 10)
            .map_err(|e| rprintln!("set profile aceleration err: {:?}", e))
            .ok();
        dmx.set_mx106_profile_velocity(id, 1000)
            .map_err(|e| rprintln!("set profile velocity err: {:?}", e))
            .ok();
        dmx.set_mx106_goal_position(id, goal)
            .map_err(|e| rprintln!("set goal position err: {:?}", e))
            .ok();

        while distance_u32(goal, dmx.get_mx106_present_position(id).unwrap()) > 10 {
            block!(timer.wait()).ok();
        }

        dmx.set_mx106_torque_enable(id, 0)
            .map_err(|e| rprintln!("set torque enable err: {:?}", e))
            .ok();
        dmx.set_mx106_led(id, 0)
            .map_err(|e| rprintln!("set led err: {:?}", e))
            .ok();
        rprintln!("torque enable {:?}", dmx.get_mx106_torque_enable(id));
        rprintln!("present position {:?}", dmx.get_mx106_present_position(id));
        rprintln!("goal position {:?}", dmx.get_mx106_goal_position(id));

        timer.reset();
        block!(timer.wait()).ok();

        for _ in 0..1000 {
            block!(timer.wait()).ok();
        }

        block!(timer.wait()).ok();
    }
}
