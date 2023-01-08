#![deny(unsafe_code)]

/*
 * In this example, we short-circuit RX & TX, and plug this to the DATA half-duplex port.
 * A no-op zero-cost Dummy direction pin is provided to the controller.
 * We'll therefore receive all the packets we send, before actually receiving the real response.
 * and the controller is configured to expect 2 Response after set commands
 *
 * the AX12A has the default configuration: Protocol 1, Baudrate: 115_200, device ID: 1
 */

extern crate std;
use std::{println, thread, time};

use dummy_pin::DummyPin;
use heapless::Vec;

use dmx::{
    ax12a::AX12A,
    protocol::{Controller, Instruction, Protocol},
    serialport_embedded_hal::Serial,
};

fn main() {
    let id = 1;
    let baudrate = 115_200;

    let port = serialport::new("/dev/ttyACM0", baudrate).timeout(time::Duration::from_millis(100));
    let serial = Serial::new(port);

    let dummy_pin = DummyPin::new_low();
    let mut dmx = Controller::new_1(serial, dummy_pin, 0);

    println!("auto ping: {:?}", dmx.ping(id));

    println!("manual ping:");
    dmx.send(id, Instruction::Ping, Vec::<u8, 0>::new())
        .unwrap();
    println!("recv: {:?}", dmx.recv::<0>());
    println!("recv: {:?}", dmx.recv::<0>());

    println!("manual read:");
    dmx.send(
        id,
        Instruction::Read,
        Vec::<u8, 2>::from_slice(&[36, 2]).unwrap(),
    )
    .unwrap();
    println!("recv: {:?}", dmx.recv::<2>());
    println!("recv: {:?}", dmx.recv::<2>());

    println!("get pose: {:?}", dmx.get_ax12a_present_position(id));

    for led in 0..6 {
        println!("set led {}: {:?}", led % 2, dmx.set_ax12a_led(id, led % 2));
        thread::sleep(time::Duration::from_millis(500));
    }

    println!(
        "torque enable {}: {:?}",
        1,
        dmx.set_ax12a_torque_enable(id, 1)
    );
    println!(
        "moving speed {}: {:?}",
        100,
        dmx.set_ax12a_moving_speed(id, 100)
    );

    for goal in 0..8 {
        println!(
            "goal position {}: {:?}",
            goal,
            dmx.set_ax12a_goal_position(id, goal * 127)
        );
        thread::sleep(time::Duration::from_millis(1000));
    }

    for goal in 0..=8 {
        println!(
            "goal position {}: {:?}",
            8 - goal,
            dmx.set_ax12a_goal_position(id, (8 - goal) * 127)
        );
        thread::sleep(time::Duration::from_millis(1000));
    }

    println!(
        "set torque enable {}: {:?}",
        0,
        dmx.set_ax12a_torque_enable(id, 0)
    );

    loop {
        for led in 0..2 {
            println!("set led {}: {:?}", led % 2, dmx.set_ax12a_led(id, led % 2));
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
