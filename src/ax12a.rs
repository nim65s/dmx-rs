use crate::convert::*;
use crate::protocol::{Controller, Instruction, Protocol, Response};
use crate::protocol_1::Error1;
use crate::protocol_2::Error2;
use embedded_hal::{digital::v2::OutputPin, serial};
//use rtt_target::rprintln;

pub trait AX12A<Error>: Protocol<Error> {
    /// Model Number (initial: 12)
    fn get_ax12a_model_number(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[0, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Firmware Version (initial: -)
    fn get_ax12a_firmware_version(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[2, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// DYNAMIXEL ID (initial: 1)
    fn get_ax12a_id(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[3, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_id(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[3, 0, params[0]]);
        self.recv()
    }
    /// Communication Speed (initial: 1)
    fn get_ax12a_baud_rate(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[4, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_baud_rate(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[4, 0, params[0]]);
        self.recv()
    }
    /// Response Delay Time (initial: 250)
    fn get_ax12a_return_delay_time(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[5, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_return_delay_time(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[5, 0, params[0]]);
        self.recv()
    }
    /// Clockwise Angle Limit (initial: 0)
    fn get_ax12a_cw_angle_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[6, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_cw_angle_limit(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[6, 0, params[0], params[1]]);
        self.recv()
    }
    /// Counter-Clockwise Angle Limit (initial: 1023)
    fn get_ax12a_ccw_angle_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[8, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_ccw_angle_limit(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[8, 0, params[0], params[1]]);
        self.recv()
    }
    /// Maximum Internal Temperature Limit (initial: 70)
    fn get_ax12a_temperature_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[11, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_temperature_limit(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[11, 0, params[0]]);
        self.recv()
    }
    /// Minimum Input Voltage Limit (initial: 60)
    fn get_ax12a_min_voltage_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[12, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_min_voltage_limit(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[12, 0, params[0]]);
        self.recv()
    }
    /// Maximum Input Voltage Limit (initial: 140)
    fn get_ax12a_max_voltage_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[13, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_max_voltage_limit(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[13, 0, params[0]]);
        self.recv()
    }
    /// Maximun Torque (initial: 1023)
    fn get_ax12a_max_torque(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[14, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_max_torque(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[14, 0, params[0], params[1]]);
        self.recv()
    }
    /// Select Types of Status Return (initial: 2)
    fn get_ax12a_status_return_level(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[16, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_status_return_level(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[16, 0, params[0]]);
        self.recv()
    }
    /// LED for Alarm (initial: 36)
    fn get_ax12a_alarm_led(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[17, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_alarm_led(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[17, 0, params[0]]);
        self.recv()
    }
    /// Shutdown Error Information (initial: 36)
    fn get_ax12a_shutdown(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[18, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_shutdown(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[18, 0, params[0]]);
        self.recv()
    }
    /// Motor Torque On/Off (initial: 0)
    fn get_ax12a_torque_enable(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[24, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_torque_enable(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[24, 0, params[0]]);
        self.recv()
    }
    /// Status LED On/Off (initial: 0)
    fn get_ax12a_led(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[25, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_led(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[25, 0, params[0]]);
        self.recv()
    }
    /// CW Compliance Margin (initial: 1)
    fn get_ax12a_cw_compliance_margin(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[26, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_cw_compliance_margin(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[26, 0, params[0]]);
        self.recv()
    }
    /// CCW Compliance Margin (initial: 1)
    fn get_ax12a_ccw_compliance_margin(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[27, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_ccw_compliance_margin(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[27, 0, params[0]]);
        self.recv()
    }
    /// CW Compliance Slope (initial: 32)
    fn get_ax12a_cw_compliance_slope(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[28, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_cw_compliance_slope(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[28, 0, params[0]]);
        self.recv()
    }
    /// CCW Compliance Slope (initial: 32)
    fn get_ax12a_ccw_compliance_slope(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[29, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_ccw_compliance_slope(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[29, 0, params[0]]);
        self.recv()
    }
    /// Target Position (initial: -)
    fn get_ax12a_goal_position(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[30, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_goal_position(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[30, 0, params[0], params[1]]);
        self.recv()
    }
    /// Moving Speed (initial: -)
    fn get_ax12a_moving_speed(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[32, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_moving_speed(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[32, 0, params[0], params[1]]);
        self.recv()
    }
    /// Torque Limit (initial: Max Torque)
    fn get_ax12a_torque_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[34, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_torque_limit(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[34, 0, params[0], params[1]]);
        self.recv()
    }
    /// Present Position (initial: -)
    fn get_ax12a_present_position(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[36, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Speed (initial: -)
    fn get_ax12a_present_speed(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[38, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Load (initial: -)
    fn get_ax12a_present_load(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[40, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Voltage (initial: -)
    fn get_ax12a_present_voltage(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[42, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Present Temperature (initial: -)
    fn get_ax12a_present_temperature(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[43, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// If Instruction is registered (initial: 0)
    fn get_ax12a_registered(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[44, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Movement Status (initial: 0)
    fn get_ax12a_moving(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[46, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Locking EEPROM (initial: 0)
    fn get_ax12a_lock(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[47, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_ax12a_lock(&mut self, id: u8, params: u8) -> Result<Response, Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[47, 0, params[0]]);
        self.recv()
    }
    /// Minimum Current Threshold (initial: 32)
    fn get_ax12a_punch(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[48, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_ax12a_punch(&mut self, id: u8, params: u16) -> Result<Response, Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[48, 0, params[0], params[1]]);
        self.recv()
    }
}

impl<Serial, Direction> AX12A<Error1<Serial>> for Controller<Serial, Direction, Error1<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> AX12A<Error2<Serial>> for Controller<Serial, Direction, Error2<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
