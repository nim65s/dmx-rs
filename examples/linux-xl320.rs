#![deny(unsafe_code)]
#![no_std]

/*
 * In this example, we short-circuit RX & TX, and plug this to the DATA half-duplex port.
 * We'll therefore receive all the packets we send, before actually receiving the real response.
 * A no-op zero-cost Dummy direction pin is provided to the controller.
 *
 * the XL320 has the default configuration: Baudrate: 1M, device ID: 1
 */

extern crate std;
use std::{println, thread, time};

use dummy_pin::DummyPin;
use heapless::Vec;

use dmx::{
    protocol::{Controller, Instruction, Protocol},
    serialport_embedded_hal::Serial,
    xl320::XL320,
};

fn main() {
    let id = 1;
    let baudrate = 115_200;

    let port = serialport::new("/dev/ttyACM0", baudrate).timeout(time::Duration::from_millis(100));
    let serial = Serial::new(port);

    let dummy_pin = DummyPin::new_low();
    let mut dmx = Controller::new_2(serial, dummy_pin, 0);

    dmx.send(id, Instruction::Ping, Vec::<u8, 0>::new())
        .unwrap();
    println!("recv: {:?}", dmx.recv::<4>()); // Err Instruction Received
    println!("recv: {:?}", dmx.recv::<4>()); // Ok received response {…}

    loop {
        for led in 0..8 {
            println!("set led {}: {:?}", led, dmx.set_xl320_led(id, led));
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
