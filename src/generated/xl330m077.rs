use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};

pub trait XL330M077<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
    /// R (initial: -)
    fn get_xl330m077_model_number(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// R (initial: -)
    fn get_xl330m077_model_information(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// R (initial: -)
    fn get_xl330m077_firmware_version(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 252)
    fn get_xl330m077_id(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 6)
    fn get_xl330m077_baud_rate(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 254)
    fn get_xl330m077_return_delay_time(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 5)
    fn get_xl330m077_drive_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 16)
    fn get_xl330m077_operating_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 252)
    fn get_xl330m077_secondary_shadow_id(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 2 ~ 22)
    fn get_xl330m077_protocol_type(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: -1,044,479 ~<br>1,044,479)
    fn get_xl330m077_homing_offset(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 1,023)
    fn get_xl330m077_moving_threshold(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 100)
    fn get_xl330m077_temperature_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 31 ~ 70)
    fn get_xl330m077_max_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 31 ~ 70)
    fn get_xl330m077_min_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 885)
    fn get_xl330m077_pwm_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 1,750)
    fn get_xl330m077_current_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 2,047)
    fn get_xl330m077_velocity_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 4,095)
    fn get_xl330m077_max_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 4,095)
    fn get_xl330m077_min_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 3)
    fn get_xl330m077_startup_configuration(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 1 ~ 255)
    fn get_xl330m077_pwm_slope(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: -)
    fn get_xl330m077_shutdown(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 1)
    fn get_xl330m077_torque_enable(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 1)
    fn get_xl330m077_led(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 2)
    fn get_xl330m077_status_return_level(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// R (initial: 0 ~ 1)
    fn get_xl330m077_registered_instruction(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_hardware_error_status(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_velocity_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_velocity_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_position_d_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_position_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_position_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 16,383)
    fn get_xl330m077_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 1 ~ 127)
    fn get_xl330m077_bus_watchdog(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: \-PWM Limit(36) ~<br>PWM Limit(36))
    fn get_xl330m077_goal_pwm(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: \-Current Limit(38) ~<br>Current Limit(38))
    fn get_xl330m077_goal_current(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: \-Velocity Limit(44) ~<br>Velocity Limit(44))
    fn get_xl330m077_goal_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 32,767<br>0 ~ 32,737)
    fn get_xl330m077_profile_acceleration(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 32,767)
    fn get_xl330m077_profile_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: Min Position Limit(52) ~<br>Max Position Limit(48))
    fn get_xl330m077_goal_position(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// R (initial: 0 ~ 32,767)
    fn get_xl330m077_realtime_tick(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// R (initial: 0 ~ 1)
    fn get_xl330m077_moving(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_moving_status(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_present_pwm(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_present_current(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_present_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_present_position(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_velocity_trajectory(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_position_trajectory(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_present_input_voltage(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// R (initial: \-)
    fn get_xl330m077_present_temperature(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// R (initial: 0 ~ 1)
    fn get_xl330m077_backup_ready(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 64 ~ 227)
    fn get_xl330m077_indirect_address_1(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 64 ~ 227)
    fn get_xl330m077_indirect_address_2(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 64 ~ 227)
    fn get_xl330m077_indirect_address_3(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 64 ~ 227)
    fn get_xl330m077_indirect_address_18(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 64 ~ 227)
    fn get_xl330m077_indirect_address_19(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 64 ~ 227)
    fn get_xl330m077_indirect_address_20(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_xl330m077_indirect_data_1(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_xl330m077_indirect_data_2(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_xl330m077_indirect_data_3(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_xl330m077_indirect_data_18(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_xl330m077_indirect_data_19(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_xl330m077_indirect_data_20(&mut self, id: u8) -> Result<u8, Self::Error> {
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
}

impl<Serial, Direction> XL330M077<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> XL330M077<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
