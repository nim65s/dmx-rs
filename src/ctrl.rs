use embedded_hal::{digital::v2::OutputPin, serial};

use crate::dmx::*;
use crate::protocol::Instruction;

enum Access {
    R,
    RW,
}

/*
impl<Serial, Direction> ControlTable<u16> for Dynamixel<Serial, Direction>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    fn read(&mut self, addr: u16, length: u16) -> u16 {
        0
    }
    fn write(&mut self, _data: u16) {}
}

impl<Serial, Direction> ControlTable<u32> for Dynamixel<Serial, Direction>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
    fn read(&mut self, addr: u16, length: u16) -> u32 {
        0
    }
    fn write(&mut self, _data: u32) {}
}
*/

pub trait ControlTableEntryR<T, S> {
    const DESCRIPTION: &'static str;
    fn read(&mut self) -> S;
    fn write(&mut self, data: S);
}

macro_rules! control_table {
    ($address:expr, 1, $name:ident, $description:expr, $access:expr) => {
        struct $name;

        impl<Serial, Direction> ControlTableEntryR<$name, u8> for Dynamixel<Serial, Direction>
        where
            Serial: serial::Write<u8> + serial::Read<u8>,
            Direction: OutputPin,
        {
            const DESCRIPTION: &'static str = $description;

            fn read(&mut self) -> u8 {

                self.send(
                    0,
                    Instruction::Read,
                    &[$address as u8, ($address >> 8) as u8, 1, 0],
                );

                self.recv().ok().unwrap().params[0]
            }
            fn write(&mut self, data: u8) {

                self.send(
                    0,
                    Instruction::Read,
                    &[$address as u8, ($address >> 8) as u8, 1, 0],
                );


            }
        }
    };
    ($address:expr, 2, $name:ident, $description:expr, R) => {
        struct $name;
    };
}

//use Access::;

// ref https://emanual.robotis.com/docs/en/dxl/mx/mx-106/#control-table-of-eeprom-area
control_table!(0, 2, ModelNumber, "Model Number", R);
control_table!(2, 1, FirmwareVersion, "Firmware Version", R);
control_table!(3, 1, ID, "DYNAMIXEL ID", RW);
control_table!(4, 1, BaudRate, "Communication Speed", RW);
control_table!(5, 1, ReturnDelayTime, "Response Delay Time", RW);
control_table!(6, 2, CWAngleLimit, "Clockwise Angle Limit", RW);
control_table!(8, 2, CCWAngleLimit, "Counter-Clockwise Angle Limit", RW);
control_table!(10, 1, DriveMode, "Dual Mode Setting", RW);
control_table!(
    11,
    1,
    TemperatureLimit,
    "Maximum Internal Temperature Limit",
    RW
);
control_table!(12, 1, MinVoltageLimit, "Minimum Input Voltage Limit", RW);
control_table!(13, 1, MaxVoltageLimit, "Maximum Input Voltage Limit", RW);
control_table!(14, 2, MaxTorque, "Maximun Torque", RW);
control_table!(
    16,
    1,
    StatusReturnLevel,
    "Select Types of Status Return",
    RW
);
control_table!(17, 1, AlarmLED, "LED for Alarm", RW);
control_table!(18, 1, Shutdown, "Shutdown Error Information", RW);
control_table!(20, 2, MultiTurnOffset, "Adjust Position with Offset", RW);
control_table!(
    22,
    1,
    ResolutionDivider,
    "Divider for Position Resolution",
    RW
);

// ref https://emanual.robotis.com/docs/en/dxl/mx/mx-106/#control-table-of-ram-area
control_table!(24, 1, TorqueEnable, "Motor Torque On/Off", RW);
control_table!(25, 1, LED, "Status LED On/Off", RW);
control_table!(26, 1, DGain, "Derivative Gain", RW);
control_table!(27, 1, IGain, "Integral Gain", RW);
control_table!(28, 1, PGain, "Proportional Gain", RW);
control_table!(30, 2, GoalPosition, "Desired Position", RW);
control_table!(32, 2, MovingSpeed, "Moving Speed(Moving Velocity)", RW);
control_table!(34, 2, TorqueLimit, "Torque Limit", RW);
control_table!(36, 2, PresentPosition, "Present Position", R);
control_table!(38, 2, PresentSpeed, "Present Speed", R);
control_table!(40, 2, PresentLoad, "Present Load", R);
control_table!(42, 1, PresentVoltage, "Present Voltage", R);
control_table!(43, 1, PresentTemperature, "Present Temperature", R);
control_table!(44, 1, Registered, "If Instruction is registered", R);
control_table!(46, 1, Moving, "Movement Status", R);
control_table!(47, 1, Lock, "Locking EEPROM", RW);
control_table!(48, 2, Punch, "Minimum Current Threshold", RW);
control_table!(50, 2, RealtimeTick, "Count Time in millisecond", R);
control_table!(68, 2, Current, "Consuming Current", RW);
control_table!(
    70,
    1,
    TorqueCtrlModeEnable,
    "Torque Control Mode On/Off",
    RW
);
control_table!(71, 2, GoalTorque, "Goal Torque", RW);
control_table!(73, 1, GoalAcceleration, "Goal Acceleration", RW);
