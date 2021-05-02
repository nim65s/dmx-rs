#![deny(unsafe_code)]
#![no_std]

/*
 * In this example, we short-circuit RX & TX, and plug this to the DATA half-duplex port.
 * A no-op zero-cost Dummy direction pin is provided to the controller.
 * We'll therefore receive all the packets we send, before actually receiving the real response.
 * and the controller is configured to expect 2 Response after set commands
 *
 * the AX12A has the default configuration: Protocol 1, Baudrate: 1M, device ID: 1
 */

extern crate std;
use std::{path, println, thread, time};

use dummy_pin::DummyPin;
use linux_embedded_hal::Serial;

use dmx::{ax12a::AX12A, protocol::Controller};

fn main() {
    let serial = Serial::open(path::Path::new("/dev/ttyACM0")).unwrap();
    let dummy_pin = DummyPin::new_low();
    let mut dmx = Controller::new_1(serial, dummy_pin, 0);

    for led in 0..6 {
        match dmx.set_ax12a_led(1, led % 2) {
            Err(e) => println!("set led to {} err: {:?}", led % 2, e),
            Ok(_) => println!("set led to {} ok", led % 2),
        }
        thread::sleep(time::Duration::from_millis(500));
    }

    match dmx.set_ax12a_torque_enable(1, 1) {
        Err(e) => println!("enable torque err: {:?}", e),
        Ok(_) => println!("enable torque ok"),
    }

    match dmx.set_ax12a_moving_speed(1, 00) {
        Err(e) => println!("set moving speed err: {:?}", e),
        Ok(_) => println!("set moving speed ok"),
    }

    for goal in 0..8 {
        match dmx.set_ax12a_goal_position(1, goal * 127) {
            Err(e) => println!("set goal position {} err: {:?}", goal, e),
            Ok(_) => println!("set goal position {} ok", goal),
        }
        thread::sleep(time::Duration::from_millis(1000));
    }

    for goal in 0..=8 {
        match dmx.set_ax12a_goal_position(1, (8 - goal) * 127) {
            Err(e) => println!("set goal position {} err: {:?}", 8 - goal, e),
            Ok(_) => println!("set goal position {} ok", 8 - goal),
        }
        thread::sleep(time::Duration::from_millis(1000));
    }

    match dmx.set_ax12a_torque_enable(1, 0) {
        Err(e) => println!("disable torque err: {:?}", e),
        Ok(_) => println!("disable torque ok"),
    }

    loop {
        for led in 0..2 {
            match dmx.set_ax12a_led(1, led % 2) {
                Err(e) => println!("set led to {} err: {:?}", led % 2, e),
                Ok(_) => println!("set led to {} ok", led % 2),
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
