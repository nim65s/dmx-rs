use crate::protocol::{Controller, Error, Instruction, Protocol, StatusPacket};
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;

pub trait XL430W250TEST<Serial, const PROTOCOL_VERSION: u8>:
    Protocol<Serial, PROTOCOL_VERSION>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    /// [Model Number]{: .popup} (initial: 1,060)
    fn get_xl430w250test_model_number(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn get_xl430w250test_odel_information(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    /// [Firmware Version]{: .popup} (initial: -)
    fn get_xl430w250test_firmware_version(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [ID]{: .popup} (initial: 1)
    fn get_xl430w250test_id(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_id(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
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
    /// [Baud Rate]{: .popup} (initial: 1)
    fn get_xl430w250test_baud_rate(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_baud_rate(
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
    /// [Return Delay Time]{: .popup} (initial: 250)
    fn get_xl430w250test_return_delay_time(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_return_delay_time(
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
    /// [Drive Mode]{: .popup} (initial: 0)
    fn get_xl430w250test_drive_mode(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_drive_mode(
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
    /// [Operating Mode]{: .popup} (initial: 3)
    fn get_xl430w250test_operating_mode(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_operating_mode(
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
    /// [Secondary(Shadow) ID]{: .popup} (initial: 255)
    fn get_xl430w250test_secondary_shadow_id(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_secondary_shadow_id(
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
    /// [Protocol Type]{: .popup} (initial: 2)
    fn get_xl430w250test_protocol_type(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_protocol_type(
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
    /// [Homing Offset]{: .popup} (initial: 0)
    fn get_xl430w250test_homing_offset(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_homing_offset(
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
    /// [Moving Threshold]{: .popup} (initial: 10)
    fn get_xl430w250test_moving_threshold(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_moving_threshold(
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
    /// [Temperature Limit]{: .popup} (initial: 72)
    fn get_xl430w250test_temperature_limit(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_temperature_limit(
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
    /// [Max Voltage Limit]{: .popup} (initial: 140)
    fn get_xl430w250test_max_voltage_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_max_voltage_limit(
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
    /// [Min Voltage Limit]{: .popup} (initial: 60)
    fn get_xl430w250test_min_voltage_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_min_voltage_limit(
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
    /// [PWM Limit]{: .popup} (initial: 885)
    fn get_xl430w250test_pwm_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_pwm_limit(
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
    /// [Velocity Limit]{: .popup} (initial: 265)
    fn get_xl430w250test_velocity_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_velocity_limit(
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
    /// [Max Position Limit]{: .popup} (initial: 4,095)
    fn get_xl430w250test_max_position_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_max_position_limit(
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
    /// [Min Position Limit]{: .popup} (initial: 0)
    fn get_xl430w250test_min_position_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_min_position_limit(
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
    /// [Shutdown]{: .popup} (initial: 52)
    fn get_xl430w250test_shutdown(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_shutdown(
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
    /// [Torque Enable](#torque-enable) (initial: 0)
    fn get_xl430w250test_torque_enable(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_torque_enable(
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
    /// [LED](#led) (initial: 0)
    fn get_xl430w250test_led(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_led(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
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
    /// [Status Return Level](#status-return-level) (initial: 2)
    fn get_xl430w250test_status_return_level(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_status_return_level(
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
    /// [Registered Instruction](#registered-instruction) (initial: 0)
    fn get_xl430w250test_registered_instruction(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [Hardware Error Status](#hardware-error-status) (initial: 0)
    fn get_xl430w250test_hardware_error_status(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [Velocity I Gain](#velocity-i-gain) (initial: 1,000)
    fn get_xl430w250test_velocity_i_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_velocity_i_gain(
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
    /// [Velocity P Gain](#velocity-p-gain) (initial: 100)
    fn get_xl430w250test_velocity_p_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_velocity_p_gain(
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
    /// [Position D Gain](#position-d-gain) (initial: 4,000)
    fn get_xl430w250test_position_d_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_position_d_gain(
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
    /// [Position I Gain](#position-i-gain) (initial: 0)
    fn get_xl430w250test_position_i_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_position_i_gain(
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
    /// [Position P Gain](#position-p-gain) (initial: 640)
    fn get_xl430w250test_position_p_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_position_p_gain(
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
    /// [Feedforward 2nd Gain](#feedforward-2nd-gain) (initial: 0)
    fn get_xl430w250test_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_feedforward_2nd_gain(
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
    /// [Feedforward 1st Gain](#feedforward-1st-gain) (initial: 0)
    fn get_xl430w250test_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_feedforward_1st_gain(
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
    /// [Bus Watchdog](#bus-watchdog) (initial: 0)
    fn get_xl430w250test_bus_watchdog(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_bus_watchdog(
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
    /// [Goal PWM](#goal-pwm) (initial: -)
    fn get_xl430w250test_goal_pwm(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_goal_pwm(
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
    /// [Goal Velocity](#goal-velocity) (initial: -)
    fn get_xl430w250test_goal_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_goal_velocity(
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
    /// [Profile Acceleration](#profile-acceleration) (initial: 0)
    fn get_xl430w250test_profile_acceleration(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_profile_acceleration(
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
    /// [Profile Velocity](#profile-velocity) (initial: 0)
    fn get_xl430w250test_profile_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_profile_velocity(
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
    /// [Goal Position](#goal-position) (initial: -)
    fn get_xl430w250test_goal_position(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_xl430w250test_goal_position(
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
    /// [Realtime Tick](#realtime-tick) (initial: -)
    fn get_xl430w250test_realtime_tick(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    /// [Moving](#moving) (initial: 0)
    fn get_xl430w250test_moving(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [Moving Status](#moving-status) (initial: 0)
    fn get_xl430w250test_moving_status(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [Present PWM](#present-pwm) (initial: -)
    fn get_xl430w250test_present_pwm(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    /// [Present Load](#present-load) (initial: -)
    fn get_xl430w250test_present_load(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    /// [Present Velocity](#present-velocity) (initial: -)
    fn get_xl430w250test_present_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    /// [Present Position](#present-position) (initial: -)
    fn get_xl430w250test_present_position(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    /// [Velocity Trajectory](#velocity-trajectory) (initial: -)
    fn get_xl430w250test_velocity_trajectory(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    /// [Position Trajectory](#position-trajectory) (initial: -)
    fn get_xl430w250test_position_trajectory(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    /// [Present Input Voltage](#present-input-voltage) (initial: -)
    fn get_xl430w250test_present_input_voltage(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    /// [Present Temperature](#present-temperature) (initial: -)
    fn get_xl430w250test_present_temperature(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [Indirect Address 1](#indirect-address) (initial: 224)
    fn get_xl430w250test_indirect_address_1(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_1(
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
    /// [Indirect Address 2](#indirect-address) (initial: 225)
    fn get_xl430w250test_indirect_address_2(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_2(
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
    /// [Indirect Address 3](#indirect-address) (initial: 226)
    fn get_xl430w250test_indirect_address_3(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_3(
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
    /// [Indirect Address 26](#indirect-address) (initial: 249)
    fn get_xl430w250test_indirect_address_26(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_26(
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
    /// [Indirect Address 27](#indirect-address) (initial: 250)
    fn get_xl430w250test_indirect_address_27(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_27(
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
    /// [Indirect Address 28](#indirect-address) (initial: 251)
    fn get_xl430w250test_indirect_address_28(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_28(
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
    /// [Indirect Data 1](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_1(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_1(
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
    /// [Indirect Data 2](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_2(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_2(
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
    /// [Indirect Data 3](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_3(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_3(
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
    /// [Indirect Data 26](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_26(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_26(
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
    /// [Indirect Data 27](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_27(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_27(
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
    /// [Indirect Data 28](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_28(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_28(
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
    /// [Indirect Address 29](#indirect-address) (initial: 634)
    fn get_xl430w250test_indirect_address_29(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_29(
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
    /// [Indirect Address 30](#indirect-address) (initial: 635)
    fn get_xl430w250test_indirect_address_30(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_30(
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
    /// [Indirect Address 31](#indirect-address) (initial: 636)
    fn get_xl430w250test_indirect_address_31(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_31(
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
    /// [Indirect Address 54](#indirect-address) (initial: 659)
    fn get_xl430w250test_indirect_address_54(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_54(
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
    /// [Indirect Address 55](#indirect-address) (initial: 660)
    fn get_xl430w250test_indirect_address_55(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_55(
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
    /// [Indirect Address 56](#indirect-address) (initial: 661)
    fn get_xl430w250test_indirect_address_56(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_xl430w250test_indirect_address_56(
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
    /// [Indirect Data 29](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_29(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_29(
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
    /// [Indirect Data 30](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_30(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_30(
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
    /// [Indirect Data 31](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_31(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_31(
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
    /// [Indirect Data 54](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_54(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_54(
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
    /// [Indirect Data 55](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_55(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_55(
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
    /// [Indirect Data 56](#indirect-data) (initial: 0)
    fn get_xl430w250test_indirect_data_56(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_xl430w250test_indirect_data_56(
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

impl<Serial, Direction> XL430W250TEST<Serial, 1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> XL430W250TEST<Serial, 2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
