use embedded_hal::{digital::v2::OutputPin, serial};

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
}

#[derive(Debug)]
pub struct Controller<Serial, Direction, const PROTOCOL_VERSION: u8> {
    pub serial: Serial,
    pub direction: Direction,
    pub n_recv: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Response<const PARAMS_SIZE: usize> {
    pub packet_id: u8,
    pub length: usize,
    pub params: [u8; PARAMS_SIZE],
    pub error: u8,
}

impl<Serial, Direction, const PROTOCOL_VERSION: u8> Controller<Serial, Direction, PROTOCOL_VERSION>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    pub const fn new(
        serial: Serial,
        direction: Direction,
        n_recv: u8,
    ) -> Self {
        Self {
            serial,
            direction,
            n_recv,
        }
    }
}

pub trait Protocol<const PROTOCOL_VERSION: u8> {
    type Error;

    fn send(&mut self, id: u8, instruction: Instruction, params: &[u8]);
    fn recv<const PARAMS_SIZE: usize>(&mut self) -> Result<Response<PARAMS_SIZE>, Self::Error>;
    fn n_recv(&self) -> u8;

    fn ping(&mut self, id: u8) -> bool {
        self.send(id, Instruction::Ping, &[]);
        self.recv::<0>().is_ok()
    }
}
