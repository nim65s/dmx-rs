//! ref <https://emanual.robotis.com/docs/en/dxl/protocol2>

use crate::protocol::{Controller, Error, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;
use nb::block;

const HEADER: [u8; 4] = [0xFF, 0xFF, 0xFD, 0x00];

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Status {
    Ok = 0x00,
    ResultFail = 0x01,       // Failed to process the sent Instruction Packet
    InstructionError = 0x02, // Undefined Instruction has been used / Action has been used without Reg Write
    CrcError = 0x03,         // CRC of the sent Packet does not match
    DataRangeError = 0x04, // Data to be written in the corresponding Address is outside the range of the minimum/maximum value
    DataLengthError = 0x05, // Attempt to write Data that is shorter than the data length of the corresponding Address (ex: when you attempt to only use 2 bytes of a item that has been defined as 4 bytes)
    DataLimitError = 0x06, // Data to be written in the corresponding Address is outside of the Limit value
    AccessError = 0x07, // Attempt to write a value in an Address that is Read Only or has not been defined / Attempt to read a value in an Address that is Write Only or has not been defined / Attempt to write a value in the ROM domain while in a state of Torque Enable(ROM Lock)
    ProtocolError = 0x08, // An illegal status was received
}

impl From<u8> for Status {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Self::Ok,
            0x01 => Self::ResultFail,
            0x02 => Self::InstructionError,
            0x03 => Self::CrcError,
            0x04 => Self::DataRangeError,
            0x05 => Self::DataLengthError,
            0x06 => Self::DataLimitError,
            0x07 => Self::AccessError,
            _ => Self::ProtocolError,
        }
    }
}

impl<Serial, Direction> Protocol<Serial, 2> for Controller<Serial, Direction, 2>
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
        let length = (params.len() + 3).to_le_bytes();
        let content = [id, length[0], length[1], instruction as u8];
        let mut crc = crc16::State::<crc16::BUYPASS>::new();
        crc.update(&HEADER);
        crc.update(&content);
        crc.update(params.as_slice());
        let crc = crc.get().to_le_bytes();
        let crc = [crc[0], crc[1]];

        // send data in half duplex
        self.direction.set_high().ok();
        for &b in HEADER
            .iter()
            .chain(&content)
            .chain(params.as_slice())
            .chain(&crc)
        {
            block!(self.serial.write(b)).ok();
        }
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
        let length_l: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        let length_h: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        let instruction: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        if instruction != Instruction::StatusReturn as u8 {
            return Err(Error::InstructionReceived);
        }
        let length = u16::from_le_bytes([length_l, length_h]);
        let length = usize::from(length) - 4; // inst + err + crc1 + crc2: 4 bytes.
        let error: u8 = block!(self.serial.read()).map_err(Error::Communication)?;
        let mut params = Vec::new();
        for _ in 0..length {
            let byte = block!(self.serial.read()).map_err(Error::Communication)?;
            params.push(byte).map_err(|_| Error::TooSmall)?;
        }
        let mut crcs = [0; 2];
        crcs[0] = block!(self.serial.read()).map_err(Error::Communication)?;
        crcs[1] = block!(self.serial.read()).map_err(Error::Communication)?;

        // compute CRC
        let mut crc = crc16::State::<crc16::BUYPASS>::new();
        crc.update(&HEADER);
        crc.update(&[packet_id, length_l, length_h, instruction, error]);
        for &param in &params {
            crc.update(&[param]);
        }
        let instruction = Some(instruction.into());

        if crc.get().to_le_bytes() == crcs {
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

impl<Serial, Direction> Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    pub const fn new_2(serial: Serial, direction: Direction, n_recv: u8) -> Self {
        Self::new(serial, direction, n_recv)
    }
}
