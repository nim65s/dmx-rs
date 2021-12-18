#!/usr/bin/env python
"""
Generate getters and setters for a dynamixel motor from a control table TSV copy-pasted from the documentation website
"""

import csv
from pathlib import Path
from subprocess import run

HEAD = """
use crate::protocol::{Controller, Instruction, Protocol, Response};
use embedded_hal::{digital::v2::OutputPin, serial};


pub trait MOTOR<const PROTOCOL_VERSION: u8>: Protocol<PROTOCOL_VERSION> {
"""

TAIL = """
}

impl<Serial, Direction> MOTOR<1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> MOTOR<2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
"""


def generate(address, size, data_name, description, access, initial_value, mini=None, maxi=None, *, motor, out):
    address = (int(address) & 0xFF, int(address) >> 8)
    size = int(size)
    size_t = (size & 0xFF, size >> 8)
    data_name = data_name.split(']')[0][1:].replace(' ', '_').replace('(Shadow)', '_shadow').lower()
    lines = [
        f'/// {description} (initial: {initial_value})',
        f'fn get_{motor}_{data_name}(&mut self, id: u8) -> Result<u{size * 8}, Self::Error> {{',
        '    if PROTOCOL_VERSION == 1 {',
        f'        self.send(id, Instruction::Read, &[{address[0]}, {size_t[0]}]);',
        '    } else {'
        f'        self.send(id, Instruction::Read, &[{address[0]}, {address[1]}, {size_t[0]}, {size_t[1]}]);',
        '    }'
        '    if self.n_recv() == 2 { self.recv::<4>()?; }'
        f'    let params = self.recv::<{size}>()?.params;',
        f'    Ok(u{size * 8}::from_le_bytes(params))',
        '}',
    ]
    if access == 'RW':
        params = ', '.join(f'params[{i}]' for i in range(size))
        lines += [
            f'fn set_{motor}_{data_name}(&mut self, id: u8, params: u{size * 8}) -> Result<Option<Response<{size}>>, Self::Error> {{',
            f'    let params = params.to_le_bytes();', '    if PROTOCOL_VERSION == 1 {',
            f'        self.send(id, Instruction::Write, &[{address[0]}, {params}]);', '    } else {',
            f'        self.send(id, Instruction::Write, &[{address[0]}, {address[1]}, {params}]);', '    }',
            f'    if self.n_recv() == 2 {{ self.recv::<{ 2 + size }>()?; }}'
            f'    if self.n_recv() >= 1 {{ Ok(Some(self.recv::<{size}>()?)) }} else {{ Ok(None) }}}}'
        ]
    for line in lines:
        print(line, file=out)


def main(motor: Path):
    motor_name = motor.stem.replace('-', '').replace('+', 'plus')
    generated = f'generated/{motor_name}.rs'
    with motor.open() as mdfile, open(generated, 'w') as rsfile:
        print(HEAD.replace('MOTOR', motor_name.upper()), file=rsfile)
        state, data = 0, False
        for line in mdfile:
            if 'control-table-of' in line:
                state += 1
                data = False
            elif state > 0 and '---' in line:
                data = True
            elif data and line.strip() == '':
                data = False
                if state == 2:
                    break
            if state > 0 and data and not any(symbol in line for symbol in ['---', '...', '…']):
                generate(*(item.strip() for item in line.split('|')[1:-1]), motor=motor_name, out=rsfile)
        print(TAIL.replace('MOTOR', motor_name.upper()), file=rsfile)
    run(['rustfmt', generated])


if __name__ == '__main__':
    for serie in Path('../emanual/docs/en/dxl/').iterdir():
        if serie.is_dir() and serie.name != 'p':
            for motor in serie.iterdir():
                if motor.name not in ['2xc430-w250.md', '2xl430-w250.md']:
                    main(motor)
