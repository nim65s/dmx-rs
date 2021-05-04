#!/usr/bin/env python
"""
Generate getters and setters for a dynamixel motor from a control table TSV copy-pasted from the documentation website
"""

import csv
from pathlib import Path
from subprocess import run

HEAD = """
use crate::convert::*;
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
    data_name = data_name.replace(' ', '_').lower()
    lines = [
        f'/// {description} (initial: {initial_value})',
        f'pub fn get_{motor}_{data_name}(&mut self, id: u8) -> Result<u{size * 8}, Self::Error> {{',
        '    if PROTOCOL_VERSION == 1 {',
        f'        self.send(id, Instruction::Read, &[{address[0]}, {size_t[0]}]);',
        '    } else {'
        f'        self.send(id, Instruction::Read, &[{address[0]}, {address[1]}, {size_t[0]}, {size_t[1]}]);',
        '    }'
        '    if self.n_recv() == 2 { self.recv::<4>()?; }'
        f'    let params = self.recv::<{size}>()?.params;',
        f'    Ok(bytes_to_u{size * 8}(&params))',
        '}',
    ]
    if access == 'RW':
        params = ', '.join(f'params[{i}]' for i in range(size))
        lines += [
            f'pub fn set_{motor}_{data_name}(&mut self, id: u8, params: u{size * 8}) -> Result<Option<Response<{size}>>, Self::Error> {{',
            f'    let params = u{size * 8}_to_bytes(params);', '    if PROTOCOL_VERSION == 1 {',
            f'        self.send(id, Instruction::Write, &[{address[0]}, {params}]);', '    } else {',
            f'        self.send(id, Instruction::Write, &[{address[0]}, {address[1]}, {params}]);', '    }',
            f'    if self.n_recv() == 2 {{ self.recv::<{ 2 + size }>()?; }}'
            f'    if self.n_recv() >= 1 {{ Ok(Some(self.recv::<{size}>()?)) }} else {{ Ok(None) }}}}'
        ]
    for line in lines:
        print(line, file=out)


def main(motor: Path):
    with motor.open() as tsvfile, open(f'{motor.stem}.rs', 'w') as rsfile:
        print(HEAD.replace('MOTOR', motor.stem.upper()), file=rsfile)
        reader = csv.reader(tsvfile, delimiter='\t')
        for row in reader:
            generate(*(item.strip() for item in row), motor=motor.stem, out=rsfile)
        print(TAIL.replace('MOTOR', motor.stem.upper()), file=rsfile)
    run(['rustfmt', f'{motor.stem}.rs'])


if __name__ == '__main__':
    for motor in Path('.').glob('*.tsv'):
        main(motor)
