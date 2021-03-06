//! ref <https://emanual.robotis.com/docs/en/dxl/protocol1>

use crate::protocol::*;
use core::{fmt, num::Wrapping};
use embedded_hal::{digital::v2::OutputPin, serial};
use nb::block;

const HEADER: [u8; 2] = [0xFF, 0xFF];

pub enum Error1<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    Communication(<Serial as serial::Read<u8>>::Error),
    ChecksumError,
}

impl<Serial> fmt::Debug for Error1<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error1::Communication(_) => f.write_str("Serial read error"),
            Error1::ChecksumError => f.write_str("Checksum error"),
        }
    }
}

impl<Serial, Direction> Protocol<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    type Error = Error1<Serial>;

    fn n_recv(&self) -> u8 {
        self.n_recv
    }
    fn send(&mut self, id: u8, instruction: Instruction, params: &[u8]) {
        let content = [id, (params.len() + 2) as u8, instruction as u8];
        let mut sumcheck = Wrapping(0);
        for &p in content.iter().chain(params) {
            sumcheck += Wrapping(p)
        }
        let chksum = !(sumcheck.0);

        // send data in half duplex
        self.direction.set_high().ok();
        for &b in HEADER.iter().chain(&content).chain(params).chain(&[chksum]) {
            block!(self.serial.write(b)).ok();
        }
        block!(self.serial.flush()).ok();
        self.direction.set_low().ok();
    }

    fn recv<const PARAMS_SIZE: usize>(&mut self) -> Result<Response<PARAMS_SIZE>, Self::Error> {
        // wait for HEADER
        let mut head = 0;
        loop {
            match block!(self.serial.read()) {
                Ok(b) if b == HEADER[head] => head += 1,
                _ => head = 0,
            }
            if head == HEADER.len() {
                break;
            }
        }

        // read content
        let packet_id: u8 = block!(self.serial.read()).map_err(Error1::Communication)?;
        let length: u8 = block!(self.serial.read()).map_err(Error1::Communication)?;
        let error: u8 = block!(self.serial.read()).map_err(Error1::Communication)?;
        let mut params = [0; PARAMS_SIZE];
        for param in params.iter_mut().take(length as usize - 2) {
            *param = block!(self.serial.read()).map_err(Error1::Communication)?;
        }
        let checksum: u8 = block!(self.serial.read()).map_err(Error1::Communication)?;

        let mut sumcheck = Wrapping(packet_id) + Wrapping(length) + Wrapping(error);
        for &p in params.iter() {
            sumcheck += Wrapping(p);
        }

        if checksum != !(sumcheck.0) {
            Err(Error1::ChecksumError)
        } else {
            let length = length as usize;
            Ok(Response {
                packet_id,
                length,
                params,
                error,
            })
        }
    }
}

impl<Serial, Direction> Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    pub fn new_1(
        serial: Serial,
        direction: Direction,
        n_recv: u8,
    ) -> Controller<Serial, Direction, 1> {
        Controller::new(serial, direction, n_recv)
    }
}
