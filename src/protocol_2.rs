//! ref <https://emanual.robotis.com/docs/en/dxl/protocol2>

use crate::protocol::*;
use core::{convert::Into, fmt};
use embedded_hal::{digital::v2::OutputPin, serial};
use nb::block;

const HEADER: [u8; 4] = [0xFF, 0xFF, 0xFD, 0x00];

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Status {
    ResultFail = 0x01,       // Failed to process the sent Instruction Packet
    InstructionError = 0x02, // Undefined Instruction has been used / Action has been used without Reg Write
    CrcError = 0x03,         // CRC of the sent Packet does not match
    DataRangeError = 0x04, // Data to be written in the corresponding Address is outside the range of the minimum/maximum value
    DataLengthError = 0x05, // Attempt to write Data that is shorter than the data length of the corresponding Address (ex: when you attempt to only use 2 bytes of a item that has been defined as 4 bytes)
    DataLimitError = 0x06, // Data to be written in the corresponding Address is outside of the Limit value
    AccessError = 0x07, // Attempt to write a value in an Address that is Read Only or has not been defined / Attempt to read a value in an Address that is Write Only or has not been defined / Attempt to write a value in the ROM domain while in a state of Torque Enable(ROM Lock)
    ProtocolError = 0x08, // An illegal status was received
}

impl Into<Status> for u8 {
    fn into(self) -> Status {
        match self {
            0x01 => Status::ResultFail,
            0x02 => Status::InstructionError,
            0x03 => Status::CrcError,
            0x04 => Status::DataRangeError,
            0x05 => Status::DataLengthError,
            0x06 => Status::DataLimitError,
            0x07 => Status::AccessError,
            _ => Status::ProtocolError,
        }
    }
}

pub enum Error2<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    Communication(<Serial as serial::Read<u8>>::Error),
    CrcError,
    InstructionReceived,
}

impl<Serial> fmt::Debug for Error2<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error2::Communication(_) => f.write_str("Serial read error"),
            e => f.write_fmt(format_args!("{:?}", e)),
        }
    }
}

impl<Serial, Direction> Protocol<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    type Error = Error2<Serial>;

    fn n_recv(&self) -> u8 {
        self.n_recv
    }
    fn send(&mut self, id: u8, instruction: Instruction, params: &[u8]) {
        let content = [
            id,
            (params.len() + 3) as u8,
            ((params.len() + 3) >> 8) as u8,
            instruction as u8,
        ];
        let mut crc = crc16::State::<crc16::BUYPASS>::new();
        crc.update(&HEADER);
        crc.update(&content);
        crc.update(params);
        let crc = crc.get();
        let crc = [crc as u8, (crc >> 8) as u8];

        // send data in half duplex
        self.direction.set_high().ok();
        for &b in HEADER.iter().chain(&content).chain(params).chain(&crc) {
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
        let packet_id: u8 = block!(self.serial.read()).map_err(Error2::Communication)?;
        let length_l: u8 = block!(self.serial.read()).map_err(Error2::Communication)?;
        let length_h: u8 = block!(self.serial.read()).map_err(Error2::Communication)?;
        let instruction: u8 = block!(self.serial.read()).map_err(Error2::Communication)?;
        if instruction != Instruction::StatusReturn as u8 {
            return Err(Error2::InstructionReceived);
        }
        let length: usize = length_l as usize + ((length_h as usize) << 8) - 4;
        let error: u8 = block!(self.serial.read()).map_err(Error2::Communication)?;
        let mut params = [0; PARAMS_SIZE];
        for param in params.iter_mut().take(length) {
            *param = block!(self.serial.read()).map_err(Error2::Communication)?;
        }
        let mut crcs = [0; 2];
        crcs[0] = block!(self.serial.read()).map_err(Error2::Communication)?;
        crcs[1] = block!(self.serial.read()).map_err(Error2::Communication)?;

        // compute CRC
        let mut crc = crc16::State::<crc16::BUYPASS>::new();
        crc.update(&HEADER);
        crc.update(&[packet_id, length_l, length_h, instruction, error]);
        for &param in params.iter().take(length) {
            crc.update(&[param]);
        }
        let crc = crc.get();

        if crc as u8 != crcs[0] || (crc >> 8) as u8 != crcs[1] {
            Err(Error2::CrcError)
        } else {
            Ok(Response {
                packet_id,
                length,
                params,
                error,
            })
        }
    }
}

impl<Serial, Direction> Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    pub fn new_2(
        serial: Serial,
        direction: Direction,
        n_recv: u8,
    ) -> Controller<Serial, Direction, 2> {
        Controller::new(serial, direction, n_recv)
    }
}
