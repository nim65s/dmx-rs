use crate::protocol::{Controller, Error, Instruction, Protocol, StatusPacket};
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;

pub trait MX282<Serial, const PROTOCOL_VERSION: u8>: Protocol<Serial, PROTOCOL_VERSION>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    /// Model Number (initial: 30)
    fn get_mx282_model_number(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(0).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Model Information (initial: -)
    fn get_mx282_model_information(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Firmware Version (initial: -)
    fn get_mx282_firmware_version(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(6).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// DYNAMIXEL ID (initial: 1)
    fn get_mx282_id(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(7).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_id(&mut self, id: u8, params: u8) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(7).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Communication Baud Rate (initial: 1)
    fn get_mx282_baud_rate(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(8).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_baud_rate(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(8).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// StatusPacket Delay Time (initial: 250)
    fn get_mx282_return_delay_time(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(9).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_return_delay_time(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(9).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Drive Mode (initial: 0)
    fn get_mx282_drive_mode(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(10).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_drive_mode(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(10).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Operating Mode (initial: 3)
    fn get_mx282_operating_mode(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(11).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_operating_mode(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(11).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Secondary ID (initial: 255)
    fn get_mx282_secondary_shadow_id(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(12).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_secondary_shadow_id(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(12).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Protocol Type (initial: 2)
    fn get_mx282_protocol_type(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(13).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_protocol_type(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(13).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Home Position Offset (initial: 0)
    fn get_mx282_homing_offset(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(20).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_homing_offset(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(20).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Velocity Threshold for Movement Detection (initial: 10)
    fn get_mx282_moving_threshold(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(24).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_moving_threshold(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(24).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Maximum Internal Temperature Limit (initial: 80)
    fn get_mx282_temperature_limit(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(31).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_temperature_limit(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(31).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Maximum Input Voltage Limit (initial: 160)
    fn get_mx282_max_voltage_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(32).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_max_voltage_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(32).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Minimum Input Voltage Limit (initial: 95)
    fn get_mx282_min_voltage_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(34).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_min_voltage_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(34).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Maximum PWM Limit (initial: 885)
    fn get_mx282_pwm_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(36).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_pwm_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(36).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Maximum Acceleration Limit (initial: 32767)
    fn get_mx282_acceleration_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(40).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_acceleration_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(40).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Maximum Velocity Limit (initial: 230)
    fn get_mx282_velocity_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(44).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_velocity_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(44).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Maximum Position Limit (initial: 4,095)
    fn get_mx282_max_position_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(48).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_max_position_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(48).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Minimum Position Limit (initial: 0)
    fn get_mx282_min_position_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(52).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_min_position_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(52).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Shutdown Error Information (initial: 52)
    fn get_mx282_shutdown(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(63).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_shutdown(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(63).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Motor Torque On/Off (initial: 0)
    fn get_mx282_torque_enable(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(64).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_torque_enable(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(64).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Status LED On/Off (initial: 0)
    fn get_mx282_led(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(65).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_led(&mut self, id: u8, params: u8) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(65).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Select Types of Status Return (initial: 2)
    fn get_mx282_status_return_level(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(68).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_status_return_level(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(68).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// REG_WRITE Instruction Flag (initial: 0)
    fn get_mx282_registered_instruction(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(69).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Hardware Error Status (initial: 0)
    fn get_mx282_hardware_error_status(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(70).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// I Gain of Velocity (initial: 1920)
    fn get_mx282_velocity_i_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(76).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_velocity_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(76).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// P Gain of Velocity (initial: 100)
    fn get_mx282_velocity_p_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(78).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_velocity_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(78).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// D Gain of Position (initial: 0)
    fn get_mx282_position_d_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(80).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_position_d_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(80).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// I Gain of Position (initial: 0)
    fn get_mx282_position_i_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(82).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_position_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(82).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// P Gain of Position (initial: 850)
    fn get_mx282_position_p_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(84).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_position_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(84).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// 2nd Gain of Feed-Forward (initial: 0)
    fn get_mx282_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(88).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_feedforward_2nd_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(88).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// 1st Gain of Feed-Forward (initial: 0)
    fn get_mx282_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(90).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_feedforward_1st_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(90).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// DYNAMIXEL BUS Watchdog (initial: 0)
    fn get_mx282_bus_watchdog(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(98).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_bus_watchdog(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(98).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Desired PWM Value (initial: -)
    fn get_mx282_goal_pwm(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(100).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_goal_pwm(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(100).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Desired Velocity Value (initial: -)
    fn get_mx282_goal_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(104).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_goal_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(104).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Acceleration Value of Profile (initial: 0)
    fn get_mx282_profile_acceleration(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(108).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_profile_acceleration(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(108).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Velocity Value of Profile (initial: 0)
    fn get_mx282_profile_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(112).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_profile_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(112).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Desired Position (initial: -)
    fn get_mx282_goal_position(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(116).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_goal_position(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(116).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// Count Time in Millisecond (initial: -)
    fn get_mx282_realtime_tick(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(120).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Movement Flag (initial: 0)
    fn get_mx282_moving(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(122).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Detailed Information of Movement Status (initial: 0)
    fn get_mx282_moving_status(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(123).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Present PWM Value (initial: -)
    fn get_mx282_present_pwm(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(124).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Present Load Value (initial: -)
    fn get_mx282_present_load(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(126).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Present Velocity Value (initial: -)
    fn get_mx282_present_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(128).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Present Position Value (initial: -)
    fn get_mx282_present_position(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(132).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Desired Velocity Trajectory from Profile (initial: -)
    fn get_mx282_velocity_trajectory(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(136).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Desired Position Trajectory from Profile (initial: -)
    fn get_mx282_position_trajectory(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(140).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(4).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Present Input Voltage (initial: -)
    fn get_mx282_present_input_voltage(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(144).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Present Internal Temperature (initial: -)
    fn get_mx282_present_temperature(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(146).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    /// Indirect Address 1 (initial: 224)
    fn get_mx282_indirect_address_1(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(168).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_1(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(168).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 2 (initial: 225)
    fn get_mx282_indirect_address_2(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(170).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_2(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(170).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 3 (initial: 226)
    fn get_mx282_indirect_address_3(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(172).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_3(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(172).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 26 (initial: 249)
    fn get_mx282_indirect_address_26(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(218).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_26(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(218).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 27 (initial: 250)
    fn get_mx282_indirect_address_27(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(220).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_27(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(220).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 28 (initial: 251)
    fn get_mx282_indirect_address_28(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(222).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_28(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(222).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 1 (initial: 0)
    fn get_mx282_indirect_data_1(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(224).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_1(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(224).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 2 (initial: 0)
    fn get_mx282_indirect_data_2(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(225).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_2(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(225).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 3 (initial: 0)
    fn get_mx282_indirect_data_3(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(226).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_3(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(226).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 26 (initial: 0)
    fn get_mx282_indirect_data_26(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(249).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_26(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(249).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 27 (initial: 0)
    fn get_mx282_indirect_data_27(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(250).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_27(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(250).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 28 (initial: 0)
    fn get_mx282_indirect_data_28(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(251).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_28(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(251).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 29 (initial: 634)
    fn get_mx282_indirect_address_29(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(66).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_29(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(66).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 30 (initial: 635)
    fn get_mx282_indirect_address_30(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(68).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_30(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(68).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 31 (initial: 636)
    fn get_mx282_indirect_address_31(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(70).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_31(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(70).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 54 (initial: 659)
    fn get_mx282_indirect_address_54(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(116).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_54(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(116).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 55 (initial: 660)
    fn get_mx282_indirect_address_55(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(118).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_55(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(118).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Address 56 (initial: 661)
    fn get_mx282_indirect_address_56(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(120).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(2).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_address_56(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(120).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 29 (initial: 0)
    fn get_mx282_indirect_data_29(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(122).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_29(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(122).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 30 (initial: 0)
    fn get_mx282_indirect_data_30(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(123).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_30(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(123).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 31 (initial: 0)
    fn get_mx282_indirect_data_31(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(124).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_31(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(124).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 54 (initial: 0)
    fn get_mx282_indirect_data_54(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(147).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_54(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(147).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 55 (initial: 0)
    fn get_mx282_indirect_data_55(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(148).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_55(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(148).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// Indirect Data 56 (initial: 0)
    fn get_mx282_indirect_data_56(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(149).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        content.push(1).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(0).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Read, content)?;
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(
            params.into_array().map_err(|_| Error::TooSmall)?,
        ))
    }
    fn set_mx282_indirect_data_56(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(149).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
        }
        for byte in params.to_le_bytes() {
            content.push(byte).map_err(|_| Error::TooSmall)?;
        }
        self.send(id, Instruction::Write, content)?;
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
}

impl<Serial, Direction> MX282<Serial, 1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> MX282<Serial, 2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
