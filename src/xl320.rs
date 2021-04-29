use crate::convert::*;
use crate::protocol::{Controller, Instruction, Protocol};
use crate::protocol_1::Error1;
use crate::protocol_2::Error2;
use embedded_hal::{digital::v2::OutputPin, serial};
//use rtt_target::rprintln;

pub trait XL320<Error>: Protocol<Error> {
    /// Model Number (initial: 350)
    fn get_xl320_model_number(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[0, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Firmware Version (initial: -)
    fn get_xl320_firmware_version(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[2, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// DYNAMIXEL ID (initial: 1)
    fn get_xl320_id(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[3, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_id(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[3, 0, params[0]]);
        Ok(())
    }
    /// Communication Speed (initial: 3)
    fn get_xl320_baud_rate(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[4, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_baud_rate(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[4, 0, params[0]]);
        Ok(())
    }
    /// Response Delay Time (initial: 250)
    fn get_xl320_return_delay_time(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[5, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_return_delay_time(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[5, 0, params[0]]);
        Ok(())
    }
    /// Clockwise Angle Limit (initial: 0)
    fn get_xl320_cw_angle_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[6, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_cw_angle_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[6, 0, params[0], params[1]]);
        Ok(())
    }
    /// Counter-Clockwise Angle Limit (initial: 1023)
    fn get_xl320_ccw_angle_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[8, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_ccw_angle_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[8, 0, params[0], params[1]]);
        Ok(())
    }
    /// Control Mode (initial: 2)
    fn get_xl320_control_mode(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[11, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_control_mode(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[11, 0, params[0]]);
        Ok(())
    }
    /// Maximum Internal Temperature Limit (initial: 65)
    fn get_xl320_temperature_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[12, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_temperature_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[12, 0, params[0]]);
        Ok(())
    }
    /// Minimum Input Voltage Limit (initial: 60)
    fn get_xl320_min_voltage_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[13, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_min_voltage_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[13, 0, params[0]]);
        Ok(())
    }
    /// Maximum Input Voltage Limit (initial: 90)
    fn get_xl320_max_voltage_limit(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[14, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_max_voltage_limit(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[14, 0, params[0]]);
        Ok(())
    }
    /// Maximun Torque (initial: 1023)
    fn get_xl320_max_torque(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[15, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_max_torque(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[15, 0, params[0], params[1]]);
        Ok(())
    }
    /// Select Types of Status Return (initial: 2)
    fn get_xl320_status_return_level(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[17, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_status_return_level(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[17, 0, params[0]]);
        Ok(())
    }
    /// Shutdown Error Information (initial: 3)
    fn get_xl320_shutdown(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[18, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_shutdown(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[18, 0, params[0]]);
        Ok(())
    }
    /// Motor Torque On/Off (initial: 0)
    fn get_xl320_torque_enable(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[24, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_torque_enable(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[24, 0, params[0]]);
        Ok(())
    }
    /// Status LED On/Off (initial: 0)
    fn get_xl320_led(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[25, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_led(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[25, 0, params[0]]);
        Ok(())
    }
    /// Derivative Gain (initial: 0)
    fn get_xl320_d_gain(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[27, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_d_gain(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[27, 0, params[0]]);
        Ok(())
    }
    /// Integral Gain (initial: 0)
    fn get_xl320_i_gain(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[28, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_i_gain(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[28, 0, params[0]]);
        Ok(())
    }
    /// Proportional Gain (initial: 32)
    fn get_xl320_p_gain(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[29, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    fn set_xl320_p_gain(&mut self, id: u8, params: u8) -> Result<(), Error> {
        let params = u8_to_bytes(params);
        self.send(id, Instruction::Write, &[29, 0, params[0]]);
        Ok(())
    }
    /// Desired Position (initial: -)
    fn get_xl320_goal_position(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[30, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_goal_position(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[30, 0, params[0], params[1]]);
        Ok(())
    }
    /// Moving Speed(Moving Velocity) (initial: -)
    fn get_xl320_moving_speed(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[32, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_moving_speed(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[32, 0, params[0], params[1]]);
        Ok(())
    }
    /// Torque Limit (initial: -)
    fn get_xl320_torque_limit(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[35, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_torque_limit(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[35, 0, params[0], params[1]]);
        Ok(())
    }
    /// Present Position (initial: -)
    fn get_xl320_present_position(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[37, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Speed (initial: -)
    fn get_xl320_present_speed(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[39, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Load (initial: -)
    fn get_xl320_present_load(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[41, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    /// Present Voltage (initial: -)
    fn get_xl320_present_voltage(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[45, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Present Temperature (initial: -)
    fn get_xl320_present_temperature(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[46, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// If Instruction is registered (initial: 0)
    fn get_xl320_registered_instruction(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[47, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Movement Status (initial: 0)
    fn get_xl320_moving(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[49, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Hardware Error Status (initial: 0)
    fn get_xl320_hardware_error_status(&mut self, id: u8) -> Result<u8, Error> {
        self.send(id, Instruction::Read, &[50, 0, 1, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u8(&params))
    }
    /// Minimum Current Threshold (initial: 32)
    fn get_xl320_punch(&mut self, id: u8) -> Result<u16, Error> {
        self.send(id, Instruction::Read, &[51, 0, 2, 0]);
        let params = self.recv()?.params;
        Ok(bytes_to_u16(&params))
    }
    fn set_xl320_punch(&mut self, id: u8, params: u16) -> Result<(), Error> {
        let params = u16_to_bytes(params);
        self.send(id, Instruction::Write, &[51, 0, params[0], params[1]]);
        Ok(())
    }
}

impl<Serial, Direction> XL320<Error1<Serial>> for Controller<Serial, Direction, Error1<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> XL320<Error2<Serial>> for Controller<Serial, Direction, Error2<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
