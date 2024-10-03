#!/usr/bin/env python
"""
Generate getters and setters for a dynamixel motor from a control table TSV copy-pasted
from the documentation website
"""

from pathlib import Path
from subprocess import run

HEAD = """
use crate::protocol::{Controller, Instruction, Protocol, StatusPacket, Error};
use embedded_hal::{digital::v2::OutputPin, serial};
use heapless::Vec;


pub trait MOTOR<Serial, const PROTOCOL_VERSION: u8>: Protocol<Serial, PROTOCOL_VERSION>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
"""

TAIL = """
}

impl<Serial, Direction> MOTOR<Serial, 1> for Controller<Serial, Direction, 1>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}

impl<Serial, Direction> MOTOR<Serial, 2> for Controller<Serial, Direction, 2>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
    Direction: OutputPin,
{
}
"""


def generate(
    address,
    size,
    data_name,
    description,
    access,
    initial_value,
    mini=None,
    maxi=None,
    *,
    motor,
    out,
):
    address = (int(address) & 0xFF, int(address) >> 8)
    size = int(size)
    size_t = (size & 0xFF, size >> 8)
    data_name = (
        data_name.split("]")[0][1:]
        .replace(" ", "_")
        .replace("(Shadow)", "_shadow")
        .lower()
    )
    lines = [
        f"/// {description} (initial: {initial_value})",
        f"fn get_{motor}_{data_name}(&mut self, id: u8) -> "
        f"Result<u{size * 8}, Error<Serial>> {{",
        "    let mut content : Vec<u8, 4> = Vec::new();",
        f"    content.push({address[0]}).map_err(|_| Error::TooSmall)?;",
        "    if PROTOCOL_VERSION == 2 {",
        f"        content.push({address[1]}).map_err(|_| Error::TooSmall)?;",
        "    }",
        f"    content.push({size_t[0]}).map_err(|_| Error::TooSmall)?;",
        "    if PROTOCOL_VERSION == 2 {",
        f"        content.push({size_t[1]}).map_err(|_| Error::TooSmall)?;",
        "    }",
        "    self.send(id, Instruction::Read, content)?;",
        "    if self.n_recv() == 2 { self.recv::<4>()?; }"
        f"    let params = self.recv::<{size}>()?.params;",
        f"    Ok(u{size * 8}::from_le_bytes(params.into_array()"
        ".map_err(|_| Error::TooSmall)?))",
        "}",
    ]
    if access == "RW":
        lines += [
            f"fn set_{motor}_{data_name}(&mut self, id: u8, params: u{size * 8}) -> "
            f"Result<Option<StatusPacket<{size}>>, Error<Serial>> {{",
            f"    let mut content : Vec<u8, {2 + size}> = Vec::new();",
            f"    content.push({address[0]}).map_err(|_| Error::TooSmall)?;",
            "    if PROTOCOL_VERSION == 2 {",
            f"        content.push({address[1]}).map_err(|_| Error::TooSmall)?;",
            "    }",
            "    for byte in params.to_le_bytes() {",
            "        content.push(byte).map_err(|_| Error::TooSmall)?;",
            "    }",
            "    self.send(id, Instruction::Write, content)?;",
            f"    if self.n_recv() == 2 {{ self.recv::<{2 + size}>()?; }}"
            f"    if self.n_recv() >= 1 {{ Ok(Some(self.recv::<{size}>()?)) }} "
            "else { Ok(None) }}",
        ]
    for line in lines:
        print(line, file=out)


def main(motor: Path, motor_name: str):
    generated = f"generated/{motor_name}.rs"
    with motor.open() as mdfile, open(generated, "w") as rsfile:
        has_description = False
        print(HEAD.replace("MOTOR", motor_name.upper()), file=rsfile)
        state, data = 0, False
        for line in mdfile:
            if "control-table-of" in line:
                state += 1
                data = False
            elif state > 0 and "---" in line:
                data = True
            elif data and line.strip() == "":
                data = False
                if state == 2:
                    break
            elif "| Address" in line:
                has_description = "Description" in line
            if (
                state > 0
                and data
                and not any(symbol in line for symbol in ["---", "...", "…"])
            ):
                if has_description:
                    (
                        address,
                        size,
                        data_name,
                        description,
                        access,
                        initial_value,
                        *_,
                    ) = (item.strip() for item in line.split("|")[1:-1])
                else:
                    address, size, data_name, access, initial_value, *_ = (
                        item.strip() for item in line.split("|")[1:-1]
                    )
                    description = data_name
                generate(
                    address,
                    size,
                    data_name,
                    description,
                    access,
                    initial_value,
                    mini=None,
                    maxi=None,
                    motor=motor_name,
                    out=rsfile,
                )
        print(TAIL.replace("MOTOR", motor_name.upper()), file=rsfile)
    run(["rustfmt", generated])


if __name__ == "__main__":
    with open("generated/mod.rs", "w") as mod:
        for serie in Path("../emanual/docs/en/dxl/").iterdir():
            if serie.is_dir() and serie.name != "p":
                for motor in serie.iterdir():
                    if motor.name not in [
                        "2xc430-w250.md",
                        "2xl430-w250.md",
                        "x.md",
                        "pro.md",
                    ]:
                        motor_name = (
                            motor.stem.replace("_", "")
                            .replace("-", "")
                            .replace("+", "plus")
                        )
                        print(motor)
                        main(motor, motor_name)
                        print(f"pub mod {motor_name};", file=mod)
