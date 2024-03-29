use core::fmt;
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Instruction {
    Ping = 0x01, // Instruction that checks whether the Packet has arrived to a device with the same ID as Packet ID
    Read = 0x02, // Instruction to read data from the Device
    Write = 0x03, // Instruction to write data on the Device
    RegWrite = 0x04, // Instruction that registers the Instruction Packet to a standby status; Packet is later executed through the Action command
    Action = 0x05, // Instruction that executes the Packet that was registered beforehand using Reg Write
    FactoryReset = 0x06, // Instruction that resets the Control Table to its initial factory default settings
    Reboot = 0x08,       // Instruction to reboot the Device
    Clear = 0x10,        // Instruction to reset certain information
    ControlTableBackup = 0x20, // Instruction to store current Control Table status data to a Backup area or to restore EEPROM data.
    StatusReturn = 0x55,       // Return Instruction for the Instruction Packet
    SyncRead = 0x82, // For multiple devices, Instruction to read data from the same Address with the same length at once
    SyncWrite = 0x83, // For multiple devices, Instruction to write data on the same Address with the same length at once
    FastSyncRead = 0x8A, // For multiple devices, Instruction to read data from the same Address with the same length at once
    BulkRead = 0x92, // For multiple devices, Instruction to read data from different Addresses with different lengths at once
    BulkWrite = 0x93, // For multiple devices, Instruction to write data on different Addresses with different lengths at once
    FastBulkRead = 0x9A, // For multiple devices, Instruction to read data from different Addresses with different lengths at once
    WrongInstruction = 0xFF,
}

impl From<u8> for Instruction {
    fn from(val: u8) -> Self {
        match val {
            0x01 => Self::Ping,
            0x02 => Self::Read,
            0x03 => Self::Write,
            0x04 => Self::RegWrite,
            0x05 => Self::Action,
            0x06 => Self::FactoryReset,
            0x08 => Self::Reboot,
            0x10 => Self::Clear,
            0x20 => Self::ControlTableBackup,
            0x55 => Self::StatusReturn,
            0x82 => Self::SyncRead,
            0x83 => Self::SyncWrite,
            0x8A => Self::FastSyncRead,
            0x92 => Self::BulkRead,
            0x93 => Self::BulkWrite,
            0x9A => Self::FastBulkRead,
            _ => Self::WrongInstruction,
        }
    }
}

#[derive(Debug)]
pub struct Controller<Serial, Direction, const PROTOCOL_VERSION: u8> {
    pub serial: Serial,
    pub direction: Direction,
    pub n_recv: u8,
}

#[derive(Debug, Clone)]
pub struct StatusPacket<const MAX_PARAMS_SIZE: usize = 4> {
    pub packet_id: u8,
    pub length: usize,
    pub instruction: Option<Instruction>,
    pub error: u8,
    pub params: Vec<u8, MAX_PARAMS_SIZE>,
}

impl<Serial, Direction, const PROTOCOL_VERSION: u8> Controller<Serial, Direction, PROTOCOL_VERSION>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    pub const fn new(serial: Serial, direction: Direction, n_recv: u8) -> Self {
        Self {
            serial,
            direction,
            n_recv,
        }
    }
}

pub enum Error<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    Communication(<Serial as serial::Read<u8>>::Error),
    TooSmall,
    TooManyParams,
    CrcError,
    InstructionReceived,
}

impl<Serial> fmt::Debug for Error<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Communication(_) => f.write_str("Serial read error"),
            Self::TooSmall => f.write_str("MAX_PARAMS_SIZE too small for this packet"),
            Self::TooManyParams => f.write_str("too many params"),
            e => f.write_fmt(format_args!("{e:?}")),
        }
    }
}

pub trait Protocol<Serial, const PROTOCOL_VERSION: u8 = 4>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    fn send<const MAX_PARAMS_SIZE: usize>(
        &mut self,
        id: u8,
        instruction: Instruction,
        params: Vec<u8, MAX_PARAMS_SIZE>,
    ) -> Result<(), Error<Serial>>;
    fn recv<const MAX_PARAMS_SIZE: usize>(
        &mut self,
    ) -> Result<StatusPacket<MAX_PARAMS_SIZE>, Error<Serial>>;
    fn n_recv(&self) -> u8;

    fn ping(&mut self, id: u8) -> Result<bool, Error<Serial>> {
        self.send(id, Instruction::Ping, Vec::<u8, 0>::new())?;
        Ok(self.recv::<3>().is_ok())
    }
}
