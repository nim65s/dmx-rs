use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};

pub trait XC330T181<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
    /// [Model Number](#model-number) (initial: 1,210)
    fn get_xc330t181_model_number(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn get_xc330t181_model_information(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn get_xc330t181_firmware_version(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn get_xc330t181_id(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_id(&mut self, id: u8, params: u8) -> Result<Option<Response<1>>, Self::Error> {
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
    fn get_xc330t181_baud_rate(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_baud_rate(
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
    fn get_xc330t181_return_delay_time(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_return_delay_time(
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
    fn get_xc330t181_drive_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_drive_mode(
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
    fn get_xc330t181_operating_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_operating_mode(
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
    /// [Secondary(Shadow) ID](#secondaryshadow-id12) (initial: 255)
    fn get_xc330t181_secondary_shadow_id(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_secondary_shadow_id(
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
    /// [Protocol Type](#protocol-type13) (initial: 2)
    fn get_xc330t181_protocol_type(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[13, 1]);
        } else {
            self.send(id, Instruction::Read, &[13, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_protocol_type(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[13, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[13, 0, params[0]]);
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
    fn get_xc330t181_homing_offset(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_xc330t181_homing_offset(
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
    /// [Moving Threshold](#moving-threshold) (initial: 10)
    fn get_xc330t181_moving_threshold(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_xc330t181_moving_threshold(
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
    /// [Temperature Limit](#temperature-limit) (initial: 70)
    fn get_xc330t181_temperature_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_temperature_limit(
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
    /// [Max Voltage Limit](#max-voltage-limit) (initial: 140)
    fn get_xc330t181_max_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_max_voltage_limit(
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
    /// [Min Voltage Limit](#min-voltage-limit) (initial: 55)
    fn get_xc330t181_min_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_min_voltage_limit(
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
    /// [PWM Limit](#pwm-limit) (initial: 885)
    fn get_xc330t181_pwm_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_pwm_limit(
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
    /// [Current Limit](#current-limit) (initial: 910)
    fn get_xc330t181_current_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_current_limit(
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
    /// [Velocity Limit](#velocity-limit) (initial: 510)
    fn get_xc330t181_velocity_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_xc330t181_velocity_limit(
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
    /// [Max Position Limit](#max-position-limit) (initial: 4,095)
    fn get_xc330t181_max_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_xc330t181_max_position_limit(
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
    /// [Min Position Limit](#min-position-limit) (initial: 0)
    fn get_xc330t181_min_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_xc330t181_min_position_limit(
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
    /// [Startup Configuration](#startup-configuration) (initial: 0)
    fn get_xc330t181_startup_configuration(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[60, 1]);
        } else {
            self.send(id, Instruction::Read, &[60, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_startup_configuration(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[60, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[60, 0, params[0]]);
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
    /// [PWM Slope](#pwm-slope) (initial: 255)
    fn get_xc330t181_pwm_slope(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[62, 1]);
        } else {
            self.send(id, Instruction::Read, &[62, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_pwm_slope(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[62, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[62, 0, params[0]]);
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
    fn get_xc330t181_shutdown(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_xc330t181_shutdown(
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
    /// [Torque Enable](#torque-enable) (initial: 0)
    fn get_xc330t181_torque_enable(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[64, 1]);
        } else {
            self.send(id, Instruction::Read, &[64, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_torque_enable(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[64, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[64, 0, params[0]]);
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
    /// [LED](#led) (initial: 0)
    fn get_xc330t181_led(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[65, 1]);
        } else {
            self.send(id, Instruction::Read, &[65, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_led(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[65, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[65, 0, params[0]]);
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
    fn get_xc330t181_status_return_level(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[68, 1]);
        } else {
            self.send(id, Instruction::Read, &[68, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_status_return_level(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[68, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[68, 0, params[0]]);
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
    fn get_xc330t181_registered_instruction(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[69, 1]);
        } else {
            self.send(id, Instruction::Read, &[69, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Hardware Error Status](#hardware-error-status) (initial: 0)
    fn get_xc330t181_hardware_error_status(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[70, 1]);
        } else {
            self.send(id, Instruction::Read, &[70, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Velocity I Gain](#velocity-i-gain) (initial: 1,200)
    fn get_xc330t181_velocity_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[76, 2]);
        } else {
            self.send(id, Instruction::Read, &[76, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_velocity_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[76, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[76, 0, params[0], params[1]]);
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
    /// [Velocity P Gain](#velocity-p-gain) (initial: 40)
    fn get_xc330t181_velocity_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[78, 2]);
        } else {
            self.send(id, Instruction::Read, &[78, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_velocity_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[78, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[78, 0, params[0], params[1]]);
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
    /// [Position D Gain](#position-d-gain) (initial: 0)
    fn get_xc330t181_position_d_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[80, 2]);
        } else {
            self.send(id, Instruction::Read, &[80, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_position_d_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[80, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[80, 0, params[0], params[1]]);
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
    /// [Position I Gain](#position-i-gain) (initial: 0)
    fn get_xc330t181_position_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[82, 2]);
        } else {
            self.send(id, Instruction::Read, &[82, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_position_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[82, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[82, 0, params[0], params[1]]);
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
    /// [Position P Gain](#position-p-gain) (initial: 900)
    fn get_xc330t181_position_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[84, 2]);
        } else {
            self.send(id, Instruction::Read, &[84, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_position_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[84, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[84, 0, params[0], params[1]]);
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
    /// [Feedforward 2nd Gain](#feedforward-2nd-gain) (initial: 0)
    fn get_xc330t181_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[88, 2]);
        } else {
            self.send(id, Instruction::Read, &[88, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_feedforward_2nd_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[88, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[88, 0, params[0], params[1]]);
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
    /// [Feedforward 1st Gain](#feedforward-1st-gain) (initial: 0)
    fn get_xc330t181_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[90, 2]);
        } else {
            self.send(id, Instruction::Read, &[90, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_feedforward_1st_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[90, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[90, 0, params[0], params[1]]);
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
    /// [Bus Watchdog](#bus-watchdog) (initial: 0)
    fn get_xc330t181_bus_watchdog(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[98, 1]);
        } else {
            self.send(id, Instruction::Read, &[98, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_bus_watchdog(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[98, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[98, 0, params[0]]);
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
    fn get_xc330t181_goal_pwm(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[100, 2]);
        } else {
            self.send(id, Instruction::Read, &[100, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_goal_pwm(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[100, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[100, 0, params[0], params[1]]);
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
    fn get_xc330t181_goal_current(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[102, 2]);
        } else {
            self.send(id, Instruction::Read, &[102, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_goal_current(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[102, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[102, 0, params[0], params[1]]);
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
    fn get_xc330t181_goal_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[104, 4]);
        } else {
            self.send(id, Instruction::Read, &[104, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_xc330t181_goal_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[104, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[104, 0, params[0], params[1], params[2], params[3]],
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
    /// [Profile Acceleration](#profile-acceleration) (initial: 0)
    fn get_xc330t181_profile_acceleration(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[108, 4]);
        } else {
            self.send(id, Instruction::Read, &[108, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_xc330t181_profile_acceleration(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[108, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[108, 0, params[0], params[1], params[2], params[3]],
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
    /// [Profile Velocity](#profile-velocity) (initial: 0)
    fn get_xc330t181_profile_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[112, 4]);
        } else {
            self.send(id, Instruction::Read, &[112, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_xc330t181_profile_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[112, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[112, 0, params[0], params[1], params[2], params[3]],
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
    fn get_xc330t181_goal_position(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[116, 4]);
        } else {
            self.send(id, Instruction::Read, &[116, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    fn set_xc330t181_goal_position(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[116, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[116, 0, params[0], params[1], params[2], params[3]],
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
    fn get_xc330t181_realtime_tick(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[120, 2]);
        } else {
            self.send(id, Instruction::Read, &[120, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Moving](#moving) (initial: 0)
    fn get_xc330t181_moving(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[122, 1]);
        } else {
            self.send(id, Instruction::Read, &[122, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Moving Status](#moving-status) (initial: 0)
    fn get_xc330t181_moving_status(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[123, 1]);
        } else {
            self.send(id, Instruction::Read, &[123, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Present PWM](#present-pwm) (initial: -)
    fn get_xc330t181_present_pwm(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[124, 2]);
        } else {
            self.send(id, Instruction::Read, &[124, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Present Current](#present-current) (initial: -)
    fn get_xc330t181_present_current(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[126, 2]);
        } else {
            self.send(id, Instruction::Read, &[126, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Present Velocity](#present-velocity) (initial: -)
    fn get_xc330t181_present_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[128, 4]);
        } else {
            self.send(id, Instruction::Read, &[128, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Present Position](#present-position) (initial: -)
    fn get_xc330t181_present_position(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[132, 4]);
        } else {
            self.send(id, Instruction::Read, &[132, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Velocity Trajectory](#velocity-trajectory) (initial: -)
    fn get_xc330t181_velocity_trajectory(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[136, 4]);
        } else {
            self.send(id, Instruction::Read, &[136, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Position Trajectory](#position-trajectory) (initial: -)
    fn get_xc330t181_position_trajectory(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[140, 4]);
        } else {
            self.send(id, Instruction::Read, &[140, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// [Present Input Voltage](#present-input-voltage) (initial: -)
    fn get_xc330t181_present_input_voltage(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[144, 2]);
        } else {
            self.send(id, Instruction::Read, &[144, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// [Present Temperature](#present-temperature) (initial: -)
    fn get_xc330t181_present_temperature(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[146, 1]);
        } else {
            self.send(id, Instruction::Read, &[146, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Backup Ready](#backup-ready) (initial: -)
    fn get_xc330t181_backup_ready(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[147, 1]);
        } else {
            self.send(id, Instruction::Read, &[147, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// [Indirect Address 1](#indirect-address) (initial: 208)
    fn get_xc330t181_indirect_address_1(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_indirect_address_1(
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
    /// [Indirect Address 2](#indirect-address) (initial: 209)
    fn get_xc330t181_indirect_address_2(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_indirect_address_2(
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
    /// [Indirect Address 3](#indirect-address) (initial: 210)
    fn get_xc330t181_indirect_address_3(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_xc330t181_indirect_address_3(
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
    /// [Indirect Address 18](#indirect-address) (initial: 225)
    fn get_xc330t181_indirect_address_18(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[202, 2]);
        } else {
            self.send(id, Instruction::Read, &[202, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_address_18(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[202, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[202, 0, params[0], params[1]]);
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
    /// [Indirect Address 19](#indirect-address) (initial: 226)
    fn get_xc330t181_indirect_address_19(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[204, 2]);
        } else {
            self.send(id, Instruction::Read, &[204, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_address_19(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[204, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[204, 0, params[0], params[1]]);
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
    /// [Indirect Address 20](#indirect-address) (initial: 227)
    fn get_xc330t181_indirect_address_20(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[206, 2]);
        } else {
            self.send(id, Instruction::Read, &[206, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_address_20(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[206, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[206, 0, params[0], params[1]]);
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
    /// [Indirect Data 1](#indirect-data) (initial: ···)
    fn get_xc330t181_indirect_data_1(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[208, 1]);
        } else {
            self.send(id, Instruction::Read, &[208, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_data_1(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[208, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[208, 0, params[0]]);
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
    fn get_xc330t181_indirect_data_2(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[209, 1]);
        } else {
            self.send(id, Instruction::Read, &[209, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_data_2(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[209, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[209, 0, params[0]]);
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
    fn get_xc330t181_indirect_data_3(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[210, 1]);
        } else {
            self.send(id, Instruction::Read, &[210, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_data_3(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[210, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[210, 0, params[0]]);
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
    /// [Indirect Data 18](#indirect-data) (initial: 0)
    fn get_xc330t181_indirect_data_18(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[225, 1]);
        } else {
            self.send(id, Instruction::Read, &[225, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_data_18(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[225, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[225, 0, params[0]]);
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
    /// [Indirect Data 19](#indirect-data) (initial: 0)
    fn get_xc330t181_indirect_data_19(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[226, 1]);
        } else {
            self.send(id, Instruction::Read, &[226, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_data_19(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[226, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[226, 0, params[0]]);
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
    /// [Indirect Data 20](#indirect-data) (initial: 0)
    fn get_xc330t181_indirect_data_20(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[227, 1]);
        } else {
            self.send(id, Instruction::Read, &[227, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_xc330t181_indirect_data_20(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[227, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[227, 0, params[0]]);
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

impl<Serial, Direction> XC330T181<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> XC330T181<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
