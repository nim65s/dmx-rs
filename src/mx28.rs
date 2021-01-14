use crate::convert::*;
use crate::protocol::{Controller, Instruction, Protocol};
use crate::protocol_1::Error1;
use crate::protocol_2::Error2;
use embedded_hal::{digital::v2::OutputPin, serial};
use rtt_target::rprintln;

pub trait MX28<Error>: Protocol<Error> {
    /// Model Number (initial: 29)
    fn get_mx28_model_number(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[0, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Firmware Version (initial: -)
    fn get_mx28_firmware_version(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[2, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// DYNAMIXEL ID (initial: 1)
    fn get_mx28_id(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[3, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_id(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[3, 0, params[0]]);
        rprintln!("set_id → {:?}", self.recv()?);
        Ok(())
    }
    /// Communication Speed (initial: 34)
    fn get_mx28_baud_rate(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[4, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_baud_rate(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[4, 0, params[0]]);
        rprintln!("set_baud_rate → {:?}", self.recv()?);
        Ok(())
    }
    /// Response Delay Time (initial: 250)
    fn get_mx28_return_delay_time(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[5, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_return_delay_time(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[5, 0, params[0]]);
        rprintln!("set_return_delay_time → {:?}", self.recv()?);
        Ok(())
    }
    /// Clockwise Angle Limit (initial: 0)
    fn get_mx28_cw_angle_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[6, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_cw_angle_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[6, 0, params[0], params[1]]);
        rprintln!("set_cw_angle_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Counter-Clockwise Angle Limit (initial: 4,095)
    fn get_mx28_ccw_angle_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[8, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_ccw_angle_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[8, 0, params[0], params[1]]);
        rprintln!("set_ccw_angle_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Internal Temperature Limit (initial: 80)
    fn get_mx28_temperature_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[11, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_temperature_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[11, 0, params[0]]);
        rprintln!("set_temperature_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Minimum Input Voltage Limit (initial: 60)
    fn get_mx28_min_voltage_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[12, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_min_voltage_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[12, 0, params[0]]);
        rprintln!("set_min_voltage_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximum Input Voltage Limit (initial: 160)
    fn get_mx28_max_voltage_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[13, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_max_voltage_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[13, 0, params[0]]);
        rprintln!("set_max_voltage_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Maximun Torque (initial: 1023)
    fn get_mx28_max_torque(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[14, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_max_torque(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[14, 0, params[0], params[1]]);
        rprintln!("set_max_torque → {:?}", self.recv()?);
        Ok(())
    }
    /// Select Types of Status Return (initial: 2)
    fn get_mx28_status_return_level(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[16, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_status_return_level(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[16, 0, params[0]]);
        rprintln!("set_status_return_level → {:?}", self.recv()?);
        Ok(())
    }
    /// LED for Alarm (initial: 36)
    fn get_mx28_alarm_led(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[17, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_alarm_led(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[17, 0, params[0]]);
        rprintln!("set_alarm_led → {:?}", self.recv()?);
        Ok(())
    }
    /// Shutdown Error Information (initial: 36)
    fn get_mx28_shutdown(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[18, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_shutdown(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[18, 0, params[0]]);
        rprintln!("set_shutdown → {:?}", self.recv()?);
        Ok(())
    }
    /// Adjust Position with Offset (initial: 0)
    fn get_mx28_multi_turn_offset(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[20, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_multi_turn_offset(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[20, 0, params[0], params[1]]);
        rprintln!("set_multi_turn_offset → {:?}", self.recv()?);
        Ok(())
    }
    /// Divider for Position Resolution (initial: 1)
    fn get_mx28_resolution_divider(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[22, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_resolution_divider(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[22, 0, params[0]]);
        rprintln!("set_resolution_divider → {:?}", self.recv()?);
        Ok(())
    }
    /// Motor Torque On/Off (initial: 0)
    fn get_mx28_torque_enable(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[24, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_torque_enable(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[24, 0, params[0]]);
        rprintln!("set_torque_enable → {:?}", self.recv()?);
        Ok(())
    }
    /// Status LED On/Off (initial: 0)
    fn get_mx28_led(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[25, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_led(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[25, 0, params[0]]);
        rprintln!("set_led → {:?}", self.recv()?);
        Ok(())
    }
    /// Derivative Gain (initial: 0)
    fn get_mx28_d_gain(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[26, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_d_gain(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[26, 0, params[0]]);
        rprintln!("set_d_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// Integral Gain (initial: 0)
    fn get_mx28_i_gain(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[27, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_i_gain(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[27, 0, params[0]]);
        rprintln!("set_i_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// Proportional Gain (initial: 32)
    fn get_mx28_p_gain(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[28, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_p_gain(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[28, 0, params[0]]);
        rprintln!("set_p_gain → {:?}", self.recv()?);
        Ok(())
    }
    /// Desired Position (initial: -)
    fn get_mx28_goal_position(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[30, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_goal_position(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[30, 0, params[0], params[1]]);
        rprintln!("set_goal_position → {:?}", self.recv()?);
        Ok(())
    }
    /// Moving Speed(Moving Velocity) (initial: -)
    fn get_mx28_moving_speed(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[32, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_moving_speed(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[32, 0, params[0], params[1]]);
        rprintln!("set_moving_speed → {:?}", self.recv()?);
        Ok(())
    }
    /// Torque Limit (initial: Max Torque)
    fn get_mx28_torque_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[34, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_torque_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[34, 0, params[0], params[1]]);
        rprintln!("set_torque_limit → {:?}", self.recv()?);
        Ok(())
    }
    /// Present Position (initial: -)
    fn get_mx28_present_position(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[36, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Speed (initial: -)
    fn get_mx28_present_speed(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[38, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Load (initial: -)
    fn get_mx28_present_load(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[40, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Voltage (initial: -)
    fn get_mx28_present_voltage(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[42, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Present Temperature (initial: -)
    fn get_mx28_present_temperature(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[43, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// If Instruction is registered (initial: 0)
    fn get_mx28_registered(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[44, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Movement Status (initial: 0)
    fn get_mx28_moving(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[46, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Locking EEPROM (initial: 0)
    fn get_mx28_lock(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[47, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_lock(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[47, 0, params[0]]);
        rprintln!("set_lock → {:?}", self.recv()?);
        Ok(())
    }
    /// Minimum Current Threshold (initial: 0)
    fn get_mx28_punch(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[48, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_mx28_punch(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[48, 0, params[0], params[1]]);
        rprintln!("set_punch → {:?}", self.recv()?);
        Ok(())
    }
    /// Count Time in millisecond (initial: 0)
    fn get_mx28_realtime_tick(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[50, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Goal Acceleration (initial: 0)
    fn get_mx28_goal_acceleration(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[73, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_mx28_goal_acceleration(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[73, 0, params[0]]);
        rprintln!("set_goal_acceleration → {:?}", self.recv()?);
        Ok(())
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
