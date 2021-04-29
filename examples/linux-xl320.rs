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

use dummy_pin::DummyPin;
use linux_embedded_hal::Serial;
use std::path;
use std::println;
use std::thread;
use std::time;

use dmx::{
    protocol::{Controller, Instruction, Protocol, Response},
    protocol_2::Error2,
    xl320::XL320,
};

fn print_err2(err: Error2<Serial>) {
    match err {
        Error2::Communication(_) => println!("Err communication error"),
        Error2::Protocol(status) => println!("Err protocol error: {:?}", status),
        Error2::CRCError => println!("Err CRC Error"),
        Error2::InstructionReceived => println!("Err Instruction Received"),
    }
}

fn print_resp(resp: Result<Response, Error2<Serial>>) {
    match resp {
        Ok(resp) => println!("Ok received {:?}", resp),
        Err(err) => print_err2(err),
    }
}

fn main() {
    let serial = Serial::open(path::Path::new("/dev/ttyACM0")).unwrap();
    let dummy_pin = DummyPin::new_low();
    let mut dmx = Controller::new_2(serial, dummy_pin);

    dmx.send(1, Instruction::Ping, &[]);
    print_resp(dmx.recv()); // Err Instruction Received
    print_resp(dmx.recv()); // Ok received response {…}

    loop {
        for led in 0..8 {
            println!("set led to {}", led);
            if let Err(err) = dmx.set_xl320_led(1, led) {
                print_err2(err);
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
