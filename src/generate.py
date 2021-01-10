#!/usr/bin/env python
"""
Generate getters and setters for a dynamixel motor from a control table TSV copy-pasted from the documentation website
"""

import csv
from subprocess import run
from pathlib import Path

HEAD = """
use crate::convert::*;
use crate::protocol::{Controller, Instruction, Protocol};
use crate::protocol_1::Error1;
use crate::protocol_2::Error2;
use embedded_hal::{digital::v2::OutputPin, serial};
use rtt_target::rprintln;


pub trait MOTOR<Error>: Protocol<Error> {
"""

TAIL = """
}

impl<Serial, Direction> MOTOR<Error1<Serial>> for Controller<Serial, Direction, Error1<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> MOTOR<Error2<Serial>> for Controller<Serial, Direction, Error2<Serial>>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
"""


def generate(address, size, data_name, description, access, initial_value, *, motor, out):
    address = (int(address) & 0xFF, int(address) >> 8)
    size = int(size)
    size_t = (size & 0xFF, size >> 8)
    data_name = data_name.replace(' ', '_').lower()
    lines = [
        f'/// {description} (initial: {initial_value})',
        f'pub fn get_{motor}_{data_name}(&mut self, id: u8) -> Result<u{size * 8}, Error> {{',
        f'    self.send(id, Instruction::Read, &[{address[0]}, {address[1]}, {size_t[0]}, {size_t[1]}]);',
        '    let params = self.recv()?.params;',
        f'    Ok(bytes_to_u{size * 8}(&params))',
        '}',
    ]
    if access == 'RW':
        lines += [
            f'pub fn set_{motor}_{data_name}(&mut self, id: u8, params: u{size * 8}) {{',
            f'    let params = u{size * 8}_to_bytes(params);',
            f'    self.send(id, Instruction::Write, &[{address[0]}, {address[1]}, ',
            ', '.join(f'params[{i}]' for i in range(size)) + ']);',
            f'    rprintln!("set_{data_name} → {{:?}}", self.recv().ok().unwrap());}}',
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
