use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};

pub trait M5440S250R<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
    /// R (initial: -)
    fn get_m5440s250r_model_number(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn get_m5440s250r_model_information(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn get_m5440s250r_firmware_version(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn get_m5440s250r_id(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 13)
    fn get_m5440s250r_baud_rate(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_return_delay_time(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0, 1, 3, 4)
    fn get_m5440s250r_operating_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: -2,147,483,648 ~<br> 2,147,483,647)
    fn get_m5440s250r_homing_offset(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[13, 4]);
        } else {
            self.send(id, Instruction::Read, &[13, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 2,147,483,647)
    fn get_m5440s250r_moving_threshold(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[17, 4]);
        } else {
            self.send(id, Instruction::Read, &[17, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 100)
    fn get_m5440s250r_temperature_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[21, 1]);
        } else {
            self.send(id, Instruction::Read, &[21, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 400)
    fn get_m5440s250r_max_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[22, 2]);
        } else {
            self.send(id, Instruction::Read, &[22, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 400)
    fn get_m5440s250r_min_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[24, 2]);
        } else {
            self.send(id, Instruction::Read, &[24, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 2,147,483,647)
    fn get_m5440s250r_acceleration_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[26, 4]);
        } else {
            self.send(id, Instruction::Read, &[26, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 360)
    fn get_m5440s250r_torque_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~ 2,147,483,647)
    fn get_m5440s250r_velocity_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[32, 4]);
        } else {
            self.send(id, Instruction::Read, &[32, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: -2,147,483,648 ~<br> 2,147,483,647)
    fn get_m5440s250r_max_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[36, 4]);
        } else {
            self.send(id, Instruction::Read, &[36, 0, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: -2,147,483,648 ~<br> 2,147,483,647)
    fn get_m5440s250r_min_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// RW (initial: 0 ~ 3)
    fn get_m5440s250r_external_port_mode_1(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 3)
    fn get_m5440s250r_external_port_mode_2(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[45, 1]);
        } else {
            self.send(id, Instruction::Read, &[45, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 3)
    fn get_m5440s250r_external_port_mode_3(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 3)
    fn get_m5440s250r_external_port_mode_4(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_shutdown(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[48, 1]);
        } else {
            self.send(id, Instruction::Read, &[48, 0, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 562 ~ 949)
    fn get_m5440s250r_indirect_address_1(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[49, 2]);
        } else {
            self.send(id, Instruction::Read, &[49, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 562 ~ 949)
    fn get_m5440s250r_indirect_address_2(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[51, 2]);
        } else {
            self.send(id, Instruction::Read, &[51, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 562 ~ 949)
    fn get_m5440s250r_indirect_address_3(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[53, 2]);
        } else {
            self.send(id, Instruction::Read, &[53, 0, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 562 ~ 949)
    fn get_m5440s250r_indirect_address_256(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[47, 2]);
        } else {
            self.send(id, Instruction::Read, &[47, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 1)
    fn get_m5440s250r_torque_enable(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[50, 1]);
        } else {
            self.send(id, Instruction::Read, &[50, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_led_red(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[51, 1]);
        } else {
            self.send(id, Instruction::Read, &[51, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_led_green(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[52, 1]);
        } else {
            self.send(id, Instruction::Read, &[52, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_led_blue(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[53, 1]);
        } else {
            self.send(id, Instruction::Read, &[53, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 65535)
    fn get_m5440s250r_velocity_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[74, 2]);
        } else {
            self.send(id, Instruction::Read, &[74, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 65535)
    fn get_m5440s250r_velocity_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[76, 2]);
        } else {
            self.send(id, Instruction::Read, &[76, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 65535)
    fn get_m5440s250r_position_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[82, 2]);
        } else {
            self.send(id, Instruction::Read, &[82, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: Min Position Limit(40) ~<br> Max Position Limit(36))
    fn get_m5440s250r_goal_position(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[84, 4]);
        } else {
            self.send(id, Instruction::Read, &[84, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: -Velocity Limit(32) ~<br> Velocity Limit(32))
    fn get_m5440s250r_goal_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[88, 4]);
        } else {
            self.send(id, Instruction::Read, &[88, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// RW (initial: -Torque Limit(30) ~<br> Torque Limit(30))
    fn get_m5440s250r_goal_torque(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// RW (initial: 0 ~<br> Acceleration Limit(26))
    fn get_m5440s250r_goal_acceleration(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[94, 4]);
        } else {
            self.send(id, Instruction::Read, &[94, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_moving(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[98, 1]);
        } else {
            self.send(id, Instruction::Read, &[98, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_present_position(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[99, 4]);
        } else {
            self.send(id, Instruction::Read, &[99, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_present_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[103, 4]);
        } else {
            self.send(id, Instruction::Read, &[103, 2, 4, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<4>()?.params;
        Ok(u32::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_present_current(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[109, 2]);
        } else {
            self.send(id, Instruction::Read, &[109, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_present_input_voltage(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[111, 2]);
        } else {
            self.send(id, Instruction::Read, &[111, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_present_temperature(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[113, 1]);
        } else {
            self.send(id, Instruction::Read, &[113, 2, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// R/RW (initial: 0 ~ 4,095)
    fn get_m5440s250r_external_port_data_1(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[114, 2]);
        } else {
            self.send(id, Instruction::Read, &[114, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// R/RW (initial: 0 ~ 4,095)
    fn get_m5440s250r_external_port_data_2(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[116, 2]);
        } else {
            self.send(id, Instruction::Read, &[116, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// R/RW (initial: 0 ~ 4,095)
    fn get_m5440s250r_external_port_data_3(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[118, 2]);
        } else {
            self.send(id, Instruction::Read, &[118, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// R/RW (initial: 0 ~ 4,095)
    fn get_m5440s250r_external_port_data_4(&mut self, id: u8) -> Result<u16, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[120, 2]);
        } else {
            self.send(id, Instruction::Read, &[120, 2, 2, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<2>()?.params;
        Ok(u16::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_indirect_data_1(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_indirect_data_2(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_indirect_data_3(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// RW (initial: 0 ~ 255)
    fn get_m5440s250r_indirect_data_256(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[121, 1]);
        } else {
            self.send(id, Instruction::Read, &[121, 3, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_registered_instruction(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[122, 1]);
        } else {
            self.send(id, Instruction::Read, &[122, 3, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// RW (initial: 0 ~ 2)
    fn get_m5440s250r_status_return_level(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[123, 1]);
        } else {
            self.send(id, Instruction::Read, &[123, 3, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
    /// R (initial: -)
    fn get_m5440s250r_hardware_error_status(&mut self, id: u8) -> Result<u8, Self::Error> {
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Read, &[124, 1]);
        } else {
            self.send(id, Instruction::Read, &[124, 3, 1, 0]);
        }
        if self.n_recv() == 2 {
            self.recv::<4>()?;
        }
        let params = self.recv::<1>()?.params;
        Ok(u8::from_le_bytes(params))
    }
}

impl<Serial, Direction> M5440S250R<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> M5440S250R<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
