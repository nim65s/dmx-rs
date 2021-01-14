use crate::convert::*;
use crate::protocol::{Controller, Instruction, Protocol};
use crate::protocol_1::Error1;
use crate::protocol_2::Error2;
use embedded_hal::{digital::v2::OutputPin, serial};
use rtt_target::rprintln;

pub trait MX28<Error>: Protocol<Error> {
    /// Model Number (initial: 30)
    fn get_mx28_model_number(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[0, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Model Information (initial: -)
    fn get_mx28_model_information(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[2, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    /// Firmware Version (initial: -)
    fn get_mx28_firmware_version(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[6, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// DYNAMIXEL ID (initial: 1)
    fn get_mx28_id(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[7, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_id(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[7, 0, params[0]]);
        rprintln!("set_id → {:?}", self.recv()?);
        Ok(())
    }
    /// Communication Baud Rate (initial: 1)
    fn get_mx28_baud_rate(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[8, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_baud_rate(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[8, 0, params[0]]);
        rprintln!("set_baud_rate → {:?}", self.recv()?);
        Ok(())
    }
    /// Response Delay Time (initial: 250)
    fn get_mx28_return_delay_time(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[9, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_return_delay_time(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[9, 0, params[0]]);
        rprintln!("set_return_delay_time → {:?}", self.recv()?);
        Ok(())
    }
    /// Drive Mode (initial: 0)
    fn get_mx28_drive_mode(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[10, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_drive_mode(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[10, 0, params[0]]);
        rprintln!("set_drive_mode → {:?}", self.recv()?);
        Ok(())
    }
    /// Operating Mode (initial: 3)
    fn get_mx28_operating_mode(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[11, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_operating_mode(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[11, 0, params[0]]);
        rprintln!("set_operating_mode → {:?}", self.recv()?);
        Ok(())
    }
    /// Secondary ID (initial: 255)
    fn get_mx28_secondary_id(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[12, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_secondary_id(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[12, 0, params[0]]);
        rprintln!("set_secondary_id → {:?}", self.recv()?);
        Ok(())
    }
    /// Protocol Type (initial: 2)
    fn get_mx28_protocol_type(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[13, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_protocol_type(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[13, 0, params[0]]);
        rprintln!("set_protocol_type → {:?}", self.recv()?);
        Ok(())
    }
    /// Home Position Offset (initial: 0)
    fn get_mx28_homing_offset(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[20, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_homing_offset(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[20, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_homing_offset → {:?}", self.recv()?);
        Ok(())
    }
    /// Velocity Threshold for Movement Detection (initial: 10)
    fn get_mx28_moving_threshold(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[24, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_moving_threshold(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[24, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_moving_threshold → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Internal Temperature Limit (initial: 80)
    fn get_mx28_temperature_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[31, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_temperature_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[31, 0, params[0]]);
        rprintln!("set_temperature_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Input Voltage Limit (initial: 160)
    fn get_mx28_max_voltage_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[32, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_max_voltage_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[32, 0, params[0], params[1]]);
        rprintln!("set_max_voltage_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Minimum Input Voltage Limit (initial: 95)
    fn get_mx28_min_voltage_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[34, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_min_voltage_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[34, 0, params[0], params[1]]);
        rprintln!("set_min_voltage_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum PWM Limit (initial: 885)
    fn get_mx28_pwm_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[36, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_pwm_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[36, 0, params[0], params[1]]);
        rprintln!("set_pwm_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Acceleration Limit (initial: 32767)
    fn get_mx28_acceleration_limit(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[40, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_acceleration_limit(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[40, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_acceleration_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Velocity Limit (initial: 230)
    fn get_mx28_velocity_limit(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[44, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_velocity_limit(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[44, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_velocity_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Position Limit (initial: 4,095)
    fn get_mx28_max_position_limit(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[48, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_max_position_limit(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[48, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_max_position_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Minimum Position Limit (initial: 0)
    fn get_mx28_min_position_limit(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[52, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_min_position_limit(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[52, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_min_position_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Shutdown Error Information (initial: 52)
    fn get_mx28_shutdown(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[63, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_shutdown(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[63, 0, params[0]]);
        rprintln!("set_shutdown → {:?}", self.recv()?);
        Ok(())
    }
    /// Motor Torque On/Off (initial: 0)
    fn get_mx28_torque_enable(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[64, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_torque_enable(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[64, 0, params[0]]);
        rprintln!("set_torque_enable → {:?}", self.recv()?);
        Ok(())
    }
    /// Status LED On/Off (initial: 0)
    fn get_mx28_led(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[65, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_led(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[65, 0, params[0]]);
        rprintln!("set_led → {:?}", self.recv()?);
        Ok(())
    }
    /// Select Types of Status Return (initial: 2)
    fn get_mx28_status_return_level(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[68, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_status_return_level(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[68, 0, params[0]]);
        rprintln!("set_status_return_level → {:?}", self.recv()?);
        Ok(())
    }
    /// REG_WRITE Instruction Flag (initial: 0)
    fn get_mx28_registered_instruction(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[69, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Hardware Error Status (initial: 0)
    fn get_mx28_hardware_error_status(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[70, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// I Gain of Velocity (initial: 1920)
    fn get_mx28_velocity_i_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[76, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_velocity_i_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[76, 0, params[0], params[1]]);
        rprintln!("set_velocity_i_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// P Gain of Velocity (initial: 100)
    fn get_mx28_velocity_p_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[78, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_velocity_p_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[78, 0, params[0], params[1]]);
        rprintln!("set_velocity_p_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// D Gain of Position (initial: 0)
    fn get_mx28_position_d_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[80, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_position_d_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[80, 0, params[0], params[1]]);
        rprintln!("set_position_d_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// I Gain of Position (initial: 0)
    fn get_mx28_position_i_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[82, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_position_i_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[82, 0, params[0], params[1]]);
        rprintln!("set_position_i_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// P Gain of Position (initial: 850)
    fn get_mx28_position_p_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[84, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_position_p_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[84, 0, params[0], params[1]]);
        rprintln!("set_position_p_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// 2nd Gain of Feed-Forward (initial: 0)
    fn get_mx28_feedforward_2nd_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[88, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_feedforward_2nd_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[88, 0, params[0], params[1]]);
        rprintln!("set_feedforward_2nd_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// 1st Gain of Feed-Forward (initial: 0)
    fn get_mx28_feedforward_1st_gain(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[90, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_feedforward_1st_gain(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[90, 0, params[0], params[1]]);
        rprintln!("set_feedforward_1st_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// DYNAMIXEL BUS Watchdog (initial: 0)
    fn get_mx28_bus_watchdog(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[98, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_bus_watchdog(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[98, 0, params[0]]);
        rprintln!("set_bus_watchdog → {:?}", self.recv()?);
        Ok(())
    }
    /// Desired PWM Value (initial: -)
    fn get_mx28_goal_pwm(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[100, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_goal_pwm(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[100, 0, params[0], params[1]]);
        rprintln!("set_goal_pwm → {:?}", self.recv()?);
        Ok(())
    }
    /// Desired Velocity Value (initial: -)
    fn get_mx28_goal_velocity(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[104, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_goal_velocity(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[104, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_goal_velocity → {:?}", self.recv()?);
        Ok(())
    }
    /// Acceleration Value of Profile (initial: 0)
    fn get_mx28_profile_acceleration(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[108, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_profile_acceleration(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[108, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_profile_acceleration → {:?}", self.recv()?);
        Ok(())
    }
    /// Velocity Value of Profile (initial: 0)
    fn get_mx28_profile_velocity(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[112, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_profile_velocity(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[112, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_profile_velocity → {:?}", self.recv()?);
        Ok(())
    }
    /// Desired Position (initial: -)
    fn get_mx28_goal_position(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[116, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    fn set_mx28_goal_position(&mut self, id: u8, params: u32) -> Result<(), Error> {
        let params = u32_to_bytes(params);
        self.send(
            id,
            Instruction::Write,
            &[116, 0, params[0], params[1], params[2], params[3]],
        );
        rprintln!("set_goal_position → {:?}", self.recv()?);
        Ok(())
    }
    /// Count Time in Millisecond (initial: -)
    fn get_mx28_realtime_tick(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[120, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Movement Flag (initial: 0)
    fn get_mx28_moving(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[122, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Detailed Information of Movement Status (initial: 0)
    fn get_mx28_moving_status(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[123, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Present PWM Value (initial: -)
    fn get_mx28_present_pwm(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[124, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Load Value (initial: -)
    fn get_mx28_present_load(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[126, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Velocity Value (initial: -)
    fn get_mx28_present_velocity(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[128, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    /// Present Position Value (initial: -)
    fn get_mx28_present_position(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[132, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    /// Desired Velocity Trajectory from Profile (initial: -)
    fn get_mx28_velocity_trajectory(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[136, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    /// Desired Position Trajectory from Profile (initial: -)
    fn get_mx28_position_trajectory(&mut self, id: u8) -> Result<u32, Error> {
        self.send(id, Instruction::Read, &[140, 0, 4, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u32(&params))
    }
    /// Present Input Voltage (initial: -)
    fn get_mx28_present_input_voltage(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[144, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Internal Temperature (initial: -)
    fn get_mx28_present_temperature(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[146, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
}

impl<Serial, Direction> MX28<Error1<Serial>> for Controller<Serial, Direction, Error1<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> MX28<Error2<Serial>> for Controller<Serial, Direction, Error2<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
