//! ref <https://emanual.robotis.com/docs/en/dxl/protocol1>

use crate::protocol::{Controller, Error, Instruction, Protocol, Response};
use core::convert::TryInto;
use core::num::Wrapping;
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;
use nb::block;

const HEADER: [u8; 2] = [0xFF, 0xFF];

impl<Serial, Direction> Protocol<Serial, 1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    fn n_recv(&self) -> u8 {
        self.n_recv
    }
    fn send<const MAX_PARAMS_SIZE: usize>(
        &mut self,
        id: u8,
        instruction: Instruction,
        params: Vec<u8, MAX_PARAMS_SIZE>,
    ) -> Result<(), Error<Serial>> {
        let length = (params.len() + 2)
            .try_into()
            .map_err(|_| Error::TooManyParams)?;
        let content = [id, length, instruction as u8];
        let mut sumcheck = Wrapping(0);

        self.direction.set_high().ok();
        for &b in &HEADER {
            block!(self.serial.write(b)).ok();
        }
        for &p in content.iter().chain(params.as_slice()) {
            sumcheck += Wrapping(p);
            block!(self.serial.write(p)).ok();
        }
        block!(self.serial.write(!(sumcheck.0))).ok();
        block!(self.serial.flush()).ok();
        self.direction.set_low().ok();
        Ok(())
    }

    fn recv<const MAX_PARAMS_SIZE: usize>(
        &mut self,
    ) -> Result<Response<MAX_PARAMS_SIZE>, Error<Serial>> {
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
        let packet_id: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        let length: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        if usize::from(length) - 2 < MAX_PARAMS_SIZE {
            return Err(Error::TooSmall);
        }
        let error: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        let mut params = Vec::new();
        for _ in 0..(length - 2) {
            params
                .push(block!(self.serial.read()).map_err(Error::Communication)?)
                .map_err(|_| Error::TooSmall)?;
        }
        let checksum: u8 = block!(self.serial.read()).map_err(Error::Communication)?;

        let mut sumcheck = Wrapping(packet_id) + Wrapping(length) + Wrapping(error);
        for &p in params.iter() {
            sumcheck += Wrapping(p);
        }

        if checksum == !(sumcheck.0) {
            let length = length as usize;
            let instruction = None;
            Ok(Response {
                packet_id,
                length,
                instruction,
                error,
                params,
            })
        } else {
            Err(Error::CrcError)
        }
    }
}

impl<Serial, Direction> Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    pub const fn new_1(serial: Serial, direction: Direction, n_recv: u8) -> Self {
        Self::new(serial, direction, n_recv)
    }
}
