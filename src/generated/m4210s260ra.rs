use crate::protocol::{Controller, Error, Instruction, Protocol, StatusPacket};
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;

pub trait M4210S260RA<Serial, const PROTOCOL_VERSION: u8>:
    Protocol<Serial, PROTOCOL_VERSION>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    /// [Model Number](#model-number) (initial: 43,289)
    fn get_m4210s260ra_model_number(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    /// [Model Information](#model-information) (initial: -)
    fn get_m4210s260ra_model_information(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    /// [Firmware Version](#firmware-version) (initial: -)
    fn get_m4210s260ra_firmware_version(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    /// [ID](#id) (initial: 1)
    fn get_m4210s260ra_id(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_id(
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
    /// [Baud Rate](#baud-rate) (initial: 1)
    fn get_m4210s260ra_baud_rate(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_baud_rate(
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
    /// [Return Delay Time](#return-delay-time) (initial: 250)
    fn get_m4210s260ra_return_delay_time(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_return_delay_time(
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
    /// [Drive Mode](#drive-mode) (initial: 0)
    fn get_m4210s260ra_drive_mode(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_drive_mode(
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
    /// [Operating Mode](#operating-mode) (initial: 3)
    fn get_m4210s260ra_operating_mode(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_operating_mode(
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
    /// [Secondary ID](#secondary-id) (initial: 255)
    fn get_m4210s260ra_secondary_id(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_secondary_id(
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
    /// [Homing Offset](#homing-offset) (initial: 0)
    fn get_m4210s260ra_homing_offset(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_m4210s260ra_homing_offset(
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
    /// [Moving Threshold](#moving-threshold) (initial: 20)
    fn get_m4210s260ra_moving_threshold(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_m4210s260ra_moving_threshold(
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
    /// [Temperature Limit](#temperature-limit) (initial: 80)
    fn get_m4210s260ra_temperature_limit(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_temperature_limit(
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
    /// [Max Voltage Limit](#max-voltage-limit) (initial: 300)
    fn get_m4210s260ra_max_voltage_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_m4210s260ra_max_voltage_limit(
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
    /// [Min Voltage Limit](#min-voltage-limit) (initial: 150)
    fn get_m4210s260ra_min_voltage_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_m4210s260ra_min_voltage_limit(
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
    /// [PWM Limit](#pwm-limit) (initial: 2,009)
    fn get_m4210s260ra_pwm_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_m4210s260ra_pwm_limit(
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
    /// [Current Limit](#current-limit) (initial: 1,461)
    fn get_m4210s260ra_current_limit(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(38).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_current_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(38).map_err(|_| Error::TooSmall)?;
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
    /// [Acceleration Limit](#acceleration-limit) (initial: 10,867)
    fn get_m4210s260ra_acceleration_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_m4210s260ra_acceleration_limit(
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
    /// [Velocity Limit](#velocity-limit) (initial: 2,600)
    fn get_m4210s260ra_velocity_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_m4210s260ra_velocity_limit(
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
    /// [Max Position Limit](#max-position-limit) (initial: 262,931)
    fn get_m4210s260ra_max_position_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_m4210s260ra_max_position_limit(
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
    /// [Min Position Limit](#min-position-limit) (initial: -262,931)
    fn get_m4210s260ra_min_position_limit(&mut self, id: u8) -> Result<u32, Error<Serial>> {
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
    fn set_m4210s260ra_min_position_limit(
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
    /// [External Port Mode 1](#external-port-mode) (initial: 3)
    fn get_m4210s260ra_external_port_mode_1(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(56).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_external_port_mode_1(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(56).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Mode 2](#external-port-mode) (initial: 3)
    fn get_m4210s260ra_external_port_mode_2(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(57).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_external_port_mode_2(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(57).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Mode 3](#external-port-mode) (initial: 3)
    fn get_m4210s260ra_external_port_mode_3(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(58).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_external_port_mode_3(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(58).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Mode 4](#external-port-mode) (initial: 3)
    fn get_m4210s260ra_external_port_mode_4(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(59).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_external_port_mode_4(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(59).map_err(|_| Error::TooSmall)?;
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
    /// [Shutdown](#shutdown) (initial: 52)
    fn get_m4210s260ra_shutdown(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_shutdown(
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
    /// [Indirect Address 1](#indirect-address) (initial: 634)
    fn get_m4210s260ra_indirect_address_1(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_m4210s260ra_indirect_address_1(
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
    /// [Indirect Address 2](#indirect-address) (initial: 635)
    fn get_m4210s260ra_indirect_address_2(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_m4210s260ra_indirect_address_2(
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
    /// [Indirect Address 3](#indirect-address) (initial: 636)
    fn get_m4210s260ra_indirect_address_3(&mut self, id: u8) -> Result<u16, Error<Serial>> {
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
    fn set_m4210s260ra_indirect_address_3(
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
    /// [Indirect Address 128](#indirect-address) (initial: 761)
    fn get_m4210s260ra_indirect_address_128(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(166).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(1).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_indirect_address_128(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(166).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(1).map_err(|_| Error::TooSmall)?;
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
    /// [Torque Enable](#torque-enable) (initial: 0)
    fn get_m4210s260ra_torque_enable(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(0).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_torque_enable(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(0).map_err(|_| Error::TooSmall)?;
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
    /// [LED Red](#led) (initial: 0)
    fn get_m4210s260ra_led_red(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(1).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_led_red(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(1).map_err(|_| Error::TooSmall)?;
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
    /// [LED Green](#led) (initial: 0)
    fn get_m4210s260ra_led_green(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_led_green(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(2).map_err(|_| Error::TooSmall)?;
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
    /// [LED Blue](#led) (initial: 0)
    fn get_m4210s260ra_led_blue(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(3).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_led_blue(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(3).map_err(|_| Error::TooSmall)?;
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
    /// [Status Return Level](#status-return-level) (initial: 2)
    fn get_m4210s260ra_status_return_level(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(4).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_status_return_level(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(4).map_err(|_| Error::TooSmall)?;
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
    /// [Registered Instruction](#registered-instruction) (initial: 0)
    fn get_m4210s260ra_registered_instruction(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(5).map_err(|_| Error::TooSmall)?;
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
    /// [Hardware Error Status](#hardware-error-status) (initial: 0)
    fn get_m4210s260ra_hardware_error_status(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(6).map_err(|_| Error::TooSmall)?;
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
    /// [Velocity I Gain](#velocity-i-gain) (initial: -)
    fn get_m4210s260ra_velocity_i_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(12).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_velocity_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(12).map_err(|_| Error::TooSmall)?;
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
    /// [Velocity P Gain](#velocity-p-gain) (initial: -)
    fn get_m4210s260ra_velocity_p_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(14).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_velocity_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(14).map_err(|_| Error::TooSmall)?;
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
    /// [Position D Gain](#position-p-gain) (initial: -)
    fn get_m4210s260ra_position_d_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(16).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_position_d_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(16).map_err(|_| Error::TooSmall)?;
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
    /// [Position P Gain](#position-p-gain) (initial: -)
    fn get_m4210s260ra_position_p_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(20).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_position_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(20).map_err(|_| Error::TooSmall)?;
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
    /// [Position I Gain](#position-p-gain) (initial: -)
    fn get_m4210s260ra_position_i_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(18).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_position_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(18).map_err(|_| Error::TooSmall)?;
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
    /// [Feedforward 2nd Gain](#feedforward-2nd-gain) (initial: -)
    fn get_m4210s260ra_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(24).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_feedforward_2nd_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(24).map_err(|_| Error::TooSmall)?;
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
    /// [Feedforward 1st Gain](#feedforward-1st-gain) (initial: -)
    fn get_m4210s260ra_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(26).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_feedforward_1st_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(26).map_err(|_| Error::TooSmall)?;
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
    /// [Bus Watchdog](#bus-watchdog) (initial: -)
    fn get_m4210s260ra_bus_watchdog(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(34).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_bus_watchdog(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(34).map_err(|_| Error::TooSmall)?;
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
    /// [Goal PWM](#goal-pwm) (initial: -)
    fn get_m4210s260ra_goal_pwm(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(36).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_goal_pwm(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(36).map_err(|_| Error::TooSmall)?;
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
    /// [Goal Current](#goal-current) (initial: -)
    fn get_m4210s260ra_goal_current(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(38).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_goal_current(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<StatusPacket<2>>, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(38).map_err(|_| Error::TooSmall)?;
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
    /// [Goal Velocity](#goal-velocity) (initial: -)
    fn get_m4210s260ra_goal_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(40).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_goal_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(40).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    /// [Profile Acceleration](#profile-acceleration) (initial: -)
    fn get_m4210s260ra_profile_acceleration(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(44).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_profile_acceleration(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(44).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    /// [Profile Velocity](#profile-velocity) (initial: -)
    fn get_m4210s260ra_profile_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(48).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_profile_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(48).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn get_m4210s260ra_goal_position(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(52).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_goal_position(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<StatusPacket<4>>, Error<Serial>> {
        let mut content: Vec<u8, 6> = Vec::new();
        content.push(52).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn get_m4210s260ra_realtime_tick(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(56).map_err(|_| Error::TooSmall)?;
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
    /// [Moving](#moving) (initial: -)
    fn get_m4210s260ra_moving(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(58).map_err(|_| Error::TooSmall)?;
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
    /// [Moving Status](#moving-status) (initial: -)
    fn get_m4210s260ra_moving_status(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(59).map_err(|_| Error::TooSmall)?;
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
    /// [Present PWM](#present-pwm) (initial: -)
    fn get_m4210s260ra_present_pwm(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(60).map_err(|_| Error::TooSmall)?;
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
    /// [Present Current](#present-current) (initial: -)
    fn get_m4210s260ra_present_current(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(62).map_err(|_| Error::TooSmall)?;
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
    /// [Present Velocity](#present-velocity) (initial: -)
    fn get_m4210s260ra_present_velocity(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(64).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn get_m4210s260ra_present_position(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(68).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn get_m4210s260ra_velocity_trajectory(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(72).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn get_m4210s260ra_position_trajectory(&mut self, id: u8) -> Result<u32, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(76).map_err(|_| Error::TooSmall)?;
        if PROTOCOL_VERSION == 2 {
            content.push(2).map_err(|_| Error::TooSmall)?;
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
    fn get_m4210s260ra_present_input_voltage(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(80).map_err(|_| Error::TooSmall)?;
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
    /// [Present Temperature](#present-temperature) (initial: -)
    fn get_m4210s260ra_present_temperature(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(82).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Data 1](#external-port-data) (initial: 0)
    fn get_m4210s260ra_external_port_data_1(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(88).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Data 2](#external-port-data) (initial: 0)
    fn get_m4210s260ra_external_port_data_2(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(90).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Data 3](#external-port-data) (initial: 0)
    fn get_m4210s260ra_external_port_data_3(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(92).map_err(|_| Error::TooSmall)?;
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
    /// [External Port Data 4](#external-port-data) (initial: 0)
    fn get_m4210s260ra_external_port_data_4(&mut self, id: u8) -> Result<u16, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(94).map_err(|_| Error::TooSmall)?;
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
    /// [Indirect Data 1](#indirect-data) (initial: 0)
    fn get_m4210s260ra_indirect_data_1(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_indirect_data_1(
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
    /// [Indirect Data 2](#indirect-data) (initial: 0)
    fn get_m4210s260ra_indirect_data_2(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_indirect_data_2(
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
    /// [Indirect Data 3](#indirect-data) (initial: 0)
    fn get_m4210s260ra_indirect_data_3(&mut self, id: u8) -> Result<u8, Error<Serial>> {
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
    fn set_m4210s260ra_indirect_data_3(
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
    /// [Indirect Data 128](#indirect-data) (initial: 0)
    fn get_m4210s260ra_indirect_data_128(&mut self, id: u8) -> Result<u8, Error<Serial>> {
        let mut content: Vec<u8, 4> = Vec::new();
        content.push(249).map_err(|_| Error::TooSmall)?;
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
    fn set_m4210s260ra_indirect_data_128(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<StatusPacket<1>>, Error<Serial>> {
        let mut content: Vec<u8, 3> = Vec::new();
        content.push(249).map_err(|_| Error::TooSmall)?;
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

impl<Serial, Direction> M4210S260RA<Serial, 1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> M4210S260RA<Serial, 2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
