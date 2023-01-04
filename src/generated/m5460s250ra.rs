use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};

pub trait M5460S250RA<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
    /// [Model Number](#model-number) (initial: 46,353)
    fn get_m5460s250ra_model_number(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[0, 2]);
        } else {
            self.send(id, Instruction::Read, &[0, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Model Information](#model-information) (initial: -)
    fn get_m5460s250ra_model_information(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[2, 4]);
        } else {
            self.send(id, Instruction::Read, &[2, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Firmware Version](#firmware-version) (initial: -)
    fn get_m5460s250ra_firmware_version(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[6, 1]);
        } else {
            self.send(id, Instruction::Read, &[6, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [ID](#id) (initial: 1)
    fn get_m5460s250ra_id(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[7, 1]);
        } else {
            self.send(id, Instruction::Read, &[7, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_id(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[7, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[7, 0, params[0]]);
        }
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
    fn get_m5460s250ra_baud_rate(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[8, 1]);
        } else {
            self.send(id, Instruction::Read, &[8, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_baud_rate(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[8, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[8, 0, params[0]]);
        }
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
    fn get_m5460s250ra_return_delay_time(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[9, 1]);
        } else {
            self.send(id, Instruction::Read, &[9, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_return_delay_time(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[9, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[9, 0, params[0]]);
        }
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
    fn get_m5460s250ra_drive_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[10, 1]);
        } else {
            self.send(id, Instruction::Read, &[10, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_drive_mode(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[10, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[10, 0, params[0]]);
        }
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
    fn get_m5460s250ra_operating_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[11, 1]);
        } else {
            self.send(id, Instruction::Read, &[11, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_operating_mode(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[11, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[11, 0, params[0]]);
        }
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
    fn get_m5460s250ra_secondary_id(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[12, 1]);
        } else {
            self.send(id, Instruction::Read, &[12, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_secondary_id(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[12, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[12, 0, params[0]]);
        }
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
    fn get_m5460s250ra_homing_offset(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[20, 4]);
        } else {
            self.send(id, Instruction::Read, &[20, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_homing_offset(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[20, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[20, 0, params[0], params[1], params[2], params[3]],
            );
        }
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// [Moving Threshold](#moving-threshold) (initial: 50)
    fn get_m5460s250ra_moving_threshold(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[24, 4]);
        } else {
            self.send(id, Instruction::Read, &[24, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_moving_threshold(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[24, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[24, 0, params[0], params[1], params[2], params[3]],
            );
        }
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
    fn get_m5460s250ra_temperature_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[31, 1]);
        } else {
            self.send(id, Instruction::Read, &[31, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_temperature_limit(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[31, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[31, 0, params[0]]);
        }
        if self.n_recv() == 2 {
            self.recv::<3>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<1>()?))
        } else {
            Ok(None)
        }
    }
    /// [Max Voltage Limit](#max-voltage-limit) (initial: 350)
    fn get_m5460s250ra_max_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[32, 2]);
        } else {
            self.send(id, Instruction::Read, &[32, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_max_voltage_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[32, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[32, 0, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_min_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[34, 2]);
        } else {
            self.send(id, Instruction::Read, &[34, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_min_voltage_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[34, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[34, 0, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_pwm_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[36, 2]);
        } else {
            self.send(id, Instruction::Read, &[36, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_pwm_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[36, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[36, 0, params[0], params[1]]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// [Current Limit](#current-limit) (initial: 7,980)
    fn get_m5460s250ra_current_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[38, 2]);
        } else {
            self.send(id, Instruction::Read, &[38, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_current_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[38, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[38, 0, params[0], params[1]]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<2>()?))
        } else {
            Ok(None)
        }
    }
    /// [Acceleration Limit](#acceleration-limit) (initial: 11,145)
    fn get_m5460s250ra_acceleration_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[40, 4]);
        } else {
            self.send(id, Instruction::Read, &[40, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_acceleration_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[40, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[40, 0, params[0], params[1], params[2], params[3]],
            );
        }
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// [Velocity Limit](#velocity-limit) (initial: 2,830)
    fn get_m5460s250ra_velocity_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[44, 4]);
        } else {
            self.send(id, Instruction::Read, &[44, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_velocity_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[44, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[44, 0, params[0], params[1], params[2], params[3]],
            );
        }
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// [Max Position Limit](#max-position-limit) (initial: 251,173)
    fn get_m5460s250ra_max_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[48, 4]);
        } else {
            self.send(id, Instruction::Read, &[48, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_max_position_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[48, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[48, 0, params[0], params[1], params[2], params[3]],
            );
        }
        if self.n_recv() == 2 {
            self.recv::<6>()?;
        }
        if self.n_recv() >= 1 {
            Ok(Some(self.recv::<4>()?))
        } else {
            Ok(None)
        }
    }
    /// [Min Position Limit](#min-position-limit) (initial: -251,173)
    fn get_m5460s250ra_min_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[52, 4]);
        } else {
            self.send(id, Instruction::Read, &[52, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_min_position_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[52, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[52, 0, params[0], params[1], params[2], params[3]],
            );
        }
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
    fn get_m5460s250ra_external_port_mode_1(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[56, 1]);
        } else {
            self.send(id, Instruction::Read, &[56, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_external_port_mode_1(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[56, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[56, 0, params[0]]);
        }
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
    fn get_m5460s250ra_external_port_mode_2(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[57, 1]);
        } else {
            self.send(id, Instruction::Read, &[57, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_external_port_mode_2(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[57, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[57, 0, params[0]]);
        }
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
    fn get_m5460s250ra_external_port_mode_3(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[58, 1]);
        } else {
            self.send(id, Instruction::Read, &[58, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_external_port_mode_3(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[58, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[58, 0, params[0]]);
        }
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
    fn get_m5460s250ra_external_port_mode_4(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[59, 1]);
        } else {
            self.send(id, Instruction::Read, &[59, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_external_port_mode_4(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[59, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[59, 0, params[0]]);
        }
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
    fn get_m5460s250ra_shutdown(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[63, 1]);
        } else {
            self.send(id, Instruction::Read, &[63, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_shutdown(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[63, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[63, 0, params[0]]);
        }
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
    fn get_m5460s250ra_indirect_address_1(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[168, 2]);
        } else {
            self.send(id, Instruction::Read, &[168, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_address_1(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[168, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[168, 0, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_indirect_address_2(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[170, 2]);
        } else {
            self.send(id, Instruction::Read, &[170, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_address_2(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[170, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[170, 0, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_indirect_address_3(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[172, 2]);
        } else {
            self.send(id, Instruction::Read, &[172, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_address_3(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[172, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[172, 0, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_indirect_address_128(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[166, 2]);
        } else {
            self.send(id, Instruction::Read, &[166, 1, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_address_128(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[166, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[166, 1, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_torque_enable(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[0, 1]);
        } else {
            self.send(id, Instruction::Read, &[0, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_torque_enable(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[0, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[0, 2, params[0]]);
        }
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
    fn get_m5460s250ra_led_red(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[1, 1]);
        } else {
            self.send(id, Instruction::Read, &[1, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_led_red(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[1, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[1, 2, params[0]]);
        }
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
    fn get_m5460s250ra_led_green(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[2, 1]);
        } else {
            self.send(id, Instruction::Read, &[2, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_led_green(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[2, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[2, 2, params[0]]);
        }
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
    fn get_m5460s250ra_led_blue(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[3, 1]);
        } else {
            self.send(id, Instruction::Read, &[3, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_led_blue(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[3, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[3, 2, params[0]]);
        }
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
    fn get_m5460s250ra_status_return_level(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[4, 1]);
        } else {
            self.send(id, Instruction::Read, &[4, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_status_return_level(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[4, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[4, 2, params[0]]);
        }
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
    fn get_m5460s250ra_registered_instruction(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[5, 1]);
        } else {
            self.send(id, Instruction::Read, &[5, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Hardware Error Status](#hardware-error-status) (initial: 0)
    fn get_m5460s250ra_hardware_error_status(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[6, 1]);
        } else {
            self.send(id, Instruction::Read, &[6, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Velocity I Gain](#velocity-i-gain) (initial: -)
    fn get_m5460s250ra_velocity_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[12, 2]);
        } else {
            self.send(id, Instruction::Read, &[12, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_velocity_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[12, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[12, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_velocity_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[14, 2]);
        } else {
            self.send(id, Instruction::Read, &[14, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_velocity_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[14, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[14, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_position_d_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[16, 2]);
        } else {
            self.send(id, Instruction::Read, &[16, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_position_d_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[16, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[16, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_position_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[20, 2]);
        } else {
            self.send(id, Instruction::Read, &[20, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_position_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[20, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[20, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_position_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[18, 2]);
        } else {
            self.send(id, Instruction::Read, &[18, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_position_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[18, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[18, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[24, 2]);
        } else {
            self.send(id, Instruction::Read, &[24, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_feedforward_2nd_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[24, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[24, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[26, 2]);
        } else {
            self.send(id, Instruction::Read, &[26, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_feedforward_1st_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[26, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[26, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_bus_watchdog(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[34, 1]);
        } else {
            self.send(id, Instruction::Read, &[34, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_bus_watchdog(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[34, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[34, 2, params[0]]);
        }
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
    fn get_m5460s250ra_goal_pwm(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[36, 2]);
        } else {
            self.send(id, Instruction::Read, &[36, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_goal_pwm(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[36, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[36, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_goal_current(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[38, 2]);
        } else {
            self.send(id, Instruction::Read, &[38, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_m5460s250ra_goal_current(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[38, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[38, 2, params[0], params[1]]);
        }
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
    fn get_m5460s250ra_goal_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[40, 4]);
        } else {
            self.send(id, Instruction::Read, &[40, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_goal_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[40, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[40, 2, params[0], params[1], params[2], params[3]],
            );
        }
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
    fn get_m5460s250ra_profile_acceleration(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[44, 4]);
        } else {
            self.send(id, Instruction::Read, &[44, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_profile_acceleration(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[44, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[44, 2, params[0], params[1], params[2], params[3]],
            );
        }
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
    fn get_m5460s250ra_profile_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[48, 4]);
        } else {
            self.send(id, Instruction::Read, &[48, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_profile_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[48, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[48, 2, params[0], params[1], params[2], params[3]],
            );
        }
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
    fn get_m5460s250ra_goal_position(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[52, 4]);
        } else {
            self.send(id, Instruction::Read, &[52, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_m5460s250ra_goal_position(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[52, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[52, 2, params[0], params[1], params[2], params[3]],
            );
        }
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
    fn get_m5460s250ra_realtime_tick(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[56, 2]);
        } else {
            self.send(id, Instruction::Read, &[56, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Moving](#moving) (initial: -)
    fn get_m5460s250ra_moving(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[58, 1]);
        } else {
            self.send(id, Instruction::Read, &[58, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Moving Status](#moving-status) (initial: -)
    fn get_m5460s250ra_moving_status(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[59, 1]);
        } else {
            self.send(id, Instruction::Read, &[59, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Present PWM](#present-pwm) (initial: -)
    fn get_m5460s250ra_present_pwm(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[60, 2]);
        } else {
            self.send(id, Instruction::Read, &[60, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Present Current](#present-current) (initial: -)
    fn get_m5460s250ra_present_current(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[62, 2]);
        } else {
            self.send(id, Instruction::Read, &[62, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Present Velocity](#present-velocity) (initial: -)
    fn get_m5460s250ra_present_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[64, 4]);
        } else {
            self.send(id, Instruction::Read, &[64, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Present Position](#present-position) (initial: -)
    fn get_m5460s250ra_present_position(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[68, 4]);
        } else {
            self.send(id, Instruction::Read, &[68, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Velocity Trajectory](#velocity-trajectory) (initial: -)
    fn get_m5460s250ra_velocity_trajectory(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[72, 4]);
        } else {
            self.send(id, Instruction::Read, &[72, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Position Trajectory](#position-trajectory) (initial: -)
    fn get_m5460s250ra_position_trajectory(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[76, 4]);
        } else {
            self.send(id, Instruction::Read, &[76, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Present Input Voltage](#present-input-voltage) (initial: -)
    fn get_m5460s250ra_present_input_voltage(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[80, 2]);
        } else {
            self.send(id, Instruction::Read, &[80, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Present Temperature](#present-temperature) (initial: -)
    fn get_m5460s250ra_present_temperature(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[82, 1]);
        } else {
            self.send(id, Instruction::Read, &[82, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [External Port Data 1](#external-port-data) (initial: 0)
    fn get_m5460s250ra_external_port_data_1(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[88, 2]);
        } else {
            self.send(id, Instruction::Read, &[88, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [External Port Data 2](#external-port-data) (initial: 0)
    fn get_m5460s250ra_external_port_data_2(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[90, 2]);
        } else {
            self.send(id, Instruction::Read, &[90, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [External Port Data 3](#external-port-data) (initial: 0)
    fn get_m5460s250ra_external_port_data_3(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[92, 2]);
        } else {
            self.send(id, Instruction::Read, &[92, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [External Port Data 4](#external-port-data) (initial: 0)
    fn get_m5460s250ra_external_port_data_4(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[94, 2]);
        } else {
            self.send(id, Instruction::Read, &[94, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Indirect Data 1](#indirect-data) (initial: 0)
    fn get_m5460s250ra_indirect_data_1(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[122, 1]);
        } else {
            self.send(id, Instruction::Read, &[122, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_data_1(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[122, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[122, 2, params[0]]);
        }
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
    fn get_m5460s250ra_indirect_data_2(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[123, 1]);
        } else {
            self.send(id, Instruction::Read, &[123, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_data_2(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[123, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[123, 2, params[0]]);
        }
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
    fn get_m5460s250ra_indirect_data_3(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[124, 1]);
        } else {
            self.send(id, Instruction::Read, &[124, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_data_3(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[124, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[124, 2, params[0]]);
        }
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
    fn get_m5460s250ra_indirect_data_128(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[249, 1]);
        } else {
            self.send(id, Instruction::Read, &[249, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_m5460s250ra_indirect_data_128(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[249, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[249, 2, params[0]]);
        }
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

impl<Serial, Direction> M5460S250RA<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> M5460S250RA<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
