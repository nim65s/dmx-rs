use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};

pub trait DX117<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
    /// Model Number (initial: 117)
    fn get_dx117_model_number(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// Firmware Version (initial: -)
    fn get_dx117_firmware_version(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[2, 1]);
        } else {
            self.send(id, Instruction::Read, &[2, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// DYNAMIXEL ID (initial: 1)
    fn get_dx117_id(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[3, 1]);
        } else {
            self.send(id, Instruction::Read, &[3, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_id(&mut self, id: u8, params: u8) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[3, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[3, 0, params[0]]);
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
    /// Communication Speed (initial: 34)
    fn get_dx117_baud_rate(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[4, 1]);
        } else {
            self.send(id, Instruction::Read, &[4, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_baud_rate(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[4, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[4, 0, params[0]]);
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
    /// Response Delay Time (initial: 250)
    fn get_dx117_return_delay_time(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[5, 1]);
        } else {
            self.send(id, Instruction::Read, &[5, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_return_delay_time(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[5, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[5, 0, params[0]]);
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
    /// Clockwise Angle Limit (initial: 0)
    fn get_dx117_cw_angle_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[6, 2]);
        } else {
            self.send(id, Instruction::Read, &[6, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_dx117_cw_angle_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[6, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[6, 0, params[0], params[1]]);
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
    /// Counter-Clockwise Angle Limit (initial: 1023)
    fn get_dx117_ccw_angle_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[8, 2]);
        } else {
            self.send(id, Instruction::Read, &[8, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_dx117_ccw_angle_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[8, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[8, 0, params[0], params[1]]);
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
    /// Maximum Internal Temperature Limit (initial: 80)
    fn get_dx117_temperature_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_dx117_temperature_limit(
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
    /// Minimum Input Voltage Limit (initial: 60)
    fn get_dx117_min_voltage_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_dx117_min_voltage_limit(
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
    /// Maximum Input Voltage Limit (initial: 190)
    fn get_dx117_max_voltage_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_dx117_max_voltage_limit(
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
    /// Maximun Torque (initial: 1023)
    fn get_dx117_max_torque(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[14, 2]);
        } else {
            self.send(id, Instruction::Read, &[14, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_dx117_max_torque(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[14, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[14, 0, params[0], params[1]]);
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
    /// Select Types of Status Return (initial: 2)
    fn get_dx117_status_return_level(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[16, 1]);
        } else {
            self.send(id, Instruction::Read, &[16, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_status_return_level(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[16, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[16, 0, params[0]]);
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
    /// LED for Alarm (initial: 36)
    fn get_dx117_alarm_led(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[17, 1]);
        } else {
            self.send(id, Instruction::Read, &[17, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_alarm_led(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[17, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[17, 0, params[0]]);
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
    /// Shutdown Error Information (initial: 36)
    fn get_dx117_shutdown(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[18, 1]);
        } else {
            self.send(id, Instruction::Read, &[18, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_shutdown(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[18, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[18, 0, params[0]]);
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
    /// Motor Torque On/Off (initial: 0)
    fn get_dx117_torque_enable(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[24, 1]);
        } else {
            self.send(id, Instruction::Read, &[24, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_torque_enable(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[24, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[24, 0, params[0]]);
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
    /// Status LED On/Off (initial: 0)
    fn get_dx117_led(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[25, 1]);
        } else {
            self.send(id, Instruction::Read, &[25, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_led(&mut self, id: u8, params: u8) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[25, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[25, 0, params[0]]);
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
    /// CW Compliance Margin (initial: 1)
    fn get_dx117_cw_compliance_margin(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[26, 1]);
        } else {
            self.send(id, Instruction::Read, &[26, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_cw_compliance_margin(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[26, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[26, 0, params[0]]);
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
    /// CCW Compliance Margin (initial: 1)
    fn get_dx117_ccw_compliance_margin(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[27, 1]);
        } else {
            self.send(id, Instruction::Read, &[27, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_ccw_compliance_margin(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[27, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[27, 0, params[0]]);
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
    /// CW Compliance Slope (initial: 32)
    fn get_dx117_cw_compliance_slope(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[28, 1]);
        } else {
            self.send(id, Instruction::Read, &[28, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_cw_compliance_slope(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[28, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[28, 0, params[0]]);
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
    /// CCW Compliance Slope (initial: 32)
    fn get_dx117_ccw_compliance_slope(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[29, 1]);
        } else {
            self.send(id, Instruction::Read, &[29, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_ccw_compliance_slope(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[29, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[29, 0, params[0]]);
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
    /// Target Position (initial: -)
    fn get_dx117_goal_position(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[30, 2]);
        } else {
            self.send(id, Instruction::Read, &[30, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_dx117_goal_position(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[30, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[30, 0, params[0], params[1]]);
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
    /// Moving Speed (initial: -)
    fn get_dx117_moving_speed(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_dx117_moving_speed(
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
    /// Torque Limit (initial: Max Torque)
    fn get_dx117_torque_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_dx117_torque_limit(
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
    /// Present Position (initial: -)
    fn get_dx117_present_position(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// Present Speed (initial: -)
    fn get_dx117_present_speed(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// Present Load (initial: -)
    fn get_dx117_present_load(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[40, 2]);
        } else {
            self.send(id, Instruction::Read, &[40, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// Present Voltage (initial: -)
    fn get_dx117_present_voltage(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[42, 1]);
        } else {
            self.send(id, Instruction::Read, &[42, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// Present Temperature (initial: -)
    fn get_dx117_present_temperature(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[43, 1]);
        } else {
            self.send(id, Instruction::Read, &[43, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// If Instruction is registered (initial: 0)
    fn get_dx117_registered(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[44, 1]);
        } else {
            self.send(id, Instruction::Read, &[44, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// Movement Status (initial: 0)
    fn get_dx117_moving(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[46, 1]);
        } else {
            self.send(id, Instruction::Read, &[46, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// Locking EEPROM (initial: 0)
    fn get_dx117_lock(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[47, 1]);
        } else {
            self.send(id, Instruction::Read, &[47, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    fn set_dx117_lock(&mut self, id: u8, params: u8) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[47, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[47, 0, params[0]]);
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
    /// Minimum Current Threshold (initial: 32)
    fn get_dx117_punch(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[48, 2]);
        } else {
            self.send(id, Instruction::Read, &[48, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    fn set_dx117_punch(&mut self, id: u8, params: u16) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[48, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[48, 0, params[0], params[1]]);
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
}

impl<Serial, Direction> DX117<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> DX117<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
