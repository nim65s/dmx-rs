use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};

pub trait L5450S500R<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
    /// [Model Number](#model-number) (initial: 38,152)
    fn get_l5450s500r_model_number(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn get_l5450s500r_model_information(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn get_l5450s500r_firmware_version(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn get_l5450s500r_id(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_id(
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
    fn get_l5450s500r_baud_rate(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_baud_rate(
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
    fn get_l5450s500r_return_delay_time(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_return_delay_time(
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
    /// [Operating Mode](#operating-mode) (initial: 3)
    fn get_l5450s500r_operating_mode(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_operating_mode(
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
    /// [Homing Offset](#homing-offset) (initial: 0)
    fn get_l5450s500r_homing_offset(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_homing_offset(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[13, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[13, 0, params[0], params[1], params[2], params[3]],
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
    fn get_l5450s500r_moving_threshold(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_moving_threshold(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[17, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[17, 0, params[0], params[1], params[2], params[3]],
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
    fn get_l5450s500r_temperature_limit(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_temperature_limit(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[21, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[21, 0, params[0]]);
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
    /// [Max Voltage Limit](#max-voltage-limit) (initial: 400)
    fn get_l5450s500r_max_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_max_voltage_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[22, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[22, 0, params[0], params[1]]);
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
    fn get_l5450s500r_min_voltage_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_min_voltage_limit(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[24, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[24, 0, params[0], params[1]]);
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
    /// [Acceleration Limit](#acceleration-limit) (initial: -)
    fn get_l5450s500r_acceleration_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_acceleration_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[26, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[26, 0, params[0], params[1], params[2], params[3]],
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
    /// [Torque Limit](#torque-limit) (initial: 120)
    fn get_l5450s500r_torque_limit(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_torque_limit(
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
    /// [Velocity Limit](#velocity-limit) (initial: 8,000)
    fn get_l5450s500r_velocity_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_velocity_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[32, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[32, 0, params[0], params[1], params[2], params[3]],
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
    /// [Max Position Limit](#max-position-limit) (initial: 180,692)
    fn get_l5450s500r_max_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_max_position_limit(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[36, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[36, 0, params[0], params[1], params[2], params[3]],
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
    /// [Min Position Limit](#min-position-limit) (initial: -180,692)
    fn get_l5450s500r_min_position_limit(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_min_position_limit(
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
    /// [External Port Mode 1](#external-port-mode) (initial: 0)
    fn get_l5450s500r_external_port_mode_1(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_external_port_mode_1(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[44, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[44, 0, params[0]]);
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
    /// [External Port Mode 2](#external-port-mode) (initial: 0)
    fn get_l5450s500r_external_port_mode_2(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_external_port_mode_2(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[45, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[45, 0, params[0]]);
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
    /// [External Port Mode 3](#external-port-mode) (initial: 0)
    fn get_l5450s500r_external_port_mode_3(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_external_port_mode_3(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[46, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[46, 0, params[0]]);
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
    /// [External Port Mode 4](#external-port-mode) (initial: 0)
    fn get_l5450s500r_external_port_mode_4(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_external_port_mode_4(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
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
    /// [Shutdown](#shutdown) (initial: 58)
    fn get_l5450s500r_shutdown(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_shutdown(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[48, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[48, 0, params[0]]);
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
    fn get_l5450s500r_indirect_address_1(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_indirect_address_1(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[49, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[49, 0, params[0], params[1]]);
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
    fn get_l5450s500r_indirect_address_2(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_indirect_address_2(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[51, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[51, 0, params[0], params[1]]);
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
    fn get_l5450s500r_indirect_address_3(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_indirect_address_3(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[53, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[53, 0, params[0], params[1]]);
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
    /// [Indirect Address 256](#indirect-address) (initial: 889)
    fn get_l5450s500r_indirect_address_256(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_indirect_address_256(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[47, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[47, 2, params[0], params[1]]);
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
    fn get_l5450s500r_torque_enable(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_torque_enable(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[50, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[50, 2, params[0]]);
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
    fn get_l5450s500r_led_red(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_led_red(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[51, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[51, 2, params[0]]);
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
    fn get_l5450s500r_led_green(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_led_green(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[52, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[52, 2, params[0]]);
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
    fn get_l5450s500r_led_blue(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_led_blue(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[53, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[53, 2, params[0]]);
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
    /// [Velocity I Gain](#velocity-i-gain) (initial: 16)
    fn get_l5450s500r_velocity_i_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_velocity_i_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[74, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[74, 2, params[0], params[1]]);
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
    /// [Velocity P Gain](#velocity-p-gain) (initial: 256)
    fn get_l5450s500r_velocity_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_velocity_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[76, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[76, 2, params[0], params[1]]);
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
    /// [Position P Gain](#position-p-gain) (initial: 32)
    fn get_l5450s500r_position_p_gain(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_position_p_gain(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[82, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[82, 2, params[0], params[1]]);
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
    /// [Goal Position](#goal-position) (initial: -)
    fn get_l5450s500r_goal_position(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_goal_position(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[84, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[84, 2, params[0], params[1], params[2], params[3]],
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
    /// [Goal Velocity](#goal-velocity) (initial: 0)
    fn get_l5450s500r_goal_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_goal_velocity(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[88, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[88, 2, params[0], params[1], params[2], params[3]],
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
    /// [Goal Torque](#goal-torque) (initial: 0)
    fn get_l5450s500r_goal_torque(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    fn set_l5450s500r_goal_torque(
        &mut self,
        id: u8,
        params: u16,
    ) -> Result<Option<Response<2>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[92, params[0], params[1]]);
        } else {
            self.send(id, Instruction::Write, &[92, 2, params[0], params[1]]);
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
    /// [Goal Acceleration](#goal-acceleration) (initial: 0)
    fn get_l5450s500r_goal_acceleration(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    fn set_l5450s500r_goal_acceleration(
        &mut self,
        id: u8,
        params: u32,
    ) -> Result<Option<Response<4>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(
                id,
                Instruction::Write,
                &[94, params[0], params[1], params[2], params[3]],
            );
        } else {
            self.send(
                id,
                Instruction::Write,
                &[94, 2, params[0], params[1], params[2], params[3]],
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
    /// [Moving](#moving) (initial: -)
    fn get_l5450s500r_moving(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// [Present Position](#present-position) (initial: -)
    fn get_l5450s500r_present_position(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// [Present Velocity](#present-velocity) (initial: -)
    fn get_l5450s500r_present_velocity(&mut self, id: u8) -> Result<u32, Self::Error> {
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
    /// [Present Current](#present-current) (initial: -)
    fn get_l5450s500r_present_current(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// [Present Input Voltage](#present-input-voltage) (initial: -)
    fn get_l5450s500r_present_input_voltage(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// [Present Temperature](#present-temperature) (initial: -)
    fn get_l5450s500r_present_temperature(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// [External Port Data 1](#external-port-data) (initial: 0)
    fn get_l5450s500r_external_port_data_1(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// [External Port Data 2](#external-port-data) (initial: 0)
    fn get_l5450s500r_external_port_data_2(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// [External Port Data 3](#external-port-data) (initial: 0)
    fn get_l5450s500r_external_port_data_3(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// [External Port Data 4](#external-port-data) (initial: 0)
    fn get_l5450s500r_external_port_data_4(&mut self, id: u8) -> Result<u16, Self::Error> {
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
    /// [Indirect Data 1](#indirect-data) (initial: 0)
    fn get_l5450s500r_indirect_data_1(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_indirect_data_1(
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
    fn get_l5450s500r_indirect_data_2(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_indirect_data_2(
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
    fn get_l5450s500r_indirect_data_3(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_indirect_data_3(
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
    /// [Indirect Data 256](#indirect-data) (initial: 0)
    fn get_l5450s500r_indirect_data_256(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_indirect_data_256(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[121, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[121, 3, params[0]]);
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
    fn get_l5450s500r_registered_instruction(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    /// [Status Return Level](#status-return-level) (initial: 2)
    fn get_l5450s500r_status_return_level(&mut self, id: u8) -> Result<u8, Self::Error> {
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
    fn set_l5450s500r_status_return_level(
        &mut self,
        id: u8,
        params: u8,
    ) -> Result<Option<Response<1>>, Self::Error> {
        let params = params.to_le_bytes();
        if PROTOCOL_VERSION == 1 {
            self.send(id, Instruction::Write, &[123, params[0]]);
        } else {
            self.send(id, Instruction::Write, &[123, 3, params[0]]);
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
    /// [Hardware Error Status](#hardware-error-status) (initial: 0)
    fn get_l5450s500r_hardware_error_status(&mut self, id: u8) -> Result<u8, Self::Error> {
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

impl<Serial, Direction> L5450S500R<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> L5450S500R<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
