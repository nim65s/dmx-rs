#!/usr/bin/env python

import asyncio
import shutil
import string
import sys

from aioserial import AioSerial
from aioconsole import ainput

PRINTABLES = set(string.printable) - set(string.whitespace)


def crc16(data, accum=0):
    """
    Compute CRC-16 (IBM/ANSI)
    ref https://emanual.robotis.com/docs/en/dxl/crc/

    >>> crc16([0xFF, 0xFF, 0xFD, 0, 1, 6, 0, 3, 0x19, 0, 1])
    25135
    """
    table = [
        0x0000, 0x8005, 0x800F, 0x000A, 0x801B, 0x001E, 0x0014, 0x8011, 0x8033, 0x0036, 0x003C, 0x8039, 0x0028, 0x802D,
        0x8027, 0x0022, 0x8063, 0x0066, 0x006C, 0x8069, 0x0078, 0x807D, 0x8077, 0x0072, 0x0050, 0x8055, 0x805F, 0x005A,
        0x804B, 0x004E, 0x0044, 0x8041, 0x80C3, 0x00C6, 0x00CC, 0x80C9, 0x00D8, 0x80DD, 0x80D7, 0x00D2, 0x00F0, 0x80F5,
        0x80FF, 0x00FA, 0x80EB, 0x00EE, 0x00E4, 0x80E1, 0x00A0, 0x80A5, 0x80AF, 0x00AA, 0x80BB, 0x00BE, 0x00B4, 0x80B1,
        0x8093, 0x0096, 0x009C, 0x8099, 0x0088, 0x808D, 0x8087, 0x0082, 0x8183, 0x0186, 0x018C, 0x8189, 0x0198, 0x819D,
        0x8197, 0x0192, 0x01B0, 0x81B5, 0x81BF, 0x01BA, 0x81AB, 0x01AE, 0x01A4, 0x81A1, 0x01E0, 0x81E5, 0x81EF, 0x01EA,
        0x81FB, 0x01FE, 0x01F4, 0x81F1, 0x81D3, 0x01D6, 0x01DC, 0x81D9, 0x01C8, 0x81CD, 0x81C7, 0x01C2, 0x0140, 0x8145,
        0x814F, 0x014A, 0x815B, 0x015E, 0x0154, 0x8151, 0x8173, 0x0176, 0x017C, 0x8179, 0x0168, 0x816D, 0x8167, 0x0162,
        0x8123, 0x0126, 0x012C, 0x8129, 0x0138, 0x813D, 0x8137, 0x0132, 0x0110, 0x8115, 0x811F, 0x011A, 0x810B, 0x010E,
        0x0104, 0x8101, 0x8303, 0x0306, 0x030C, 0x8309, 0x0318, 0x831D, 0x8317, 0x0312, 0x0330, 0x8335, 0x833F, 0x033A,
        0x832B, 0x032E, 0x0324, 0x8321, 0x0360, 0x8365, 0x836F, 0x036A, 0x837B, 0x037E, 0x0374, 0x8371, 0x8353, 0x0356,
        0x035C, 0x8359, 0x0348, 0x834D, 0x8347, 0x0342, 0x03C0, 0x83C5, 0x83CF, 0x03CA, 0x83DB, 0x03DE, 0x03D4, 0x83D1,
        0x83F3, 0x03F6, 0x03FC, 0x83F9, 0x03E8, 0x83ED, 0x83E7, 0x03E2, 0x83A3, 0x03A6, 0x03AC, 0x83A9, 0x03B8, 0x83BD,
        0x83B7, 0x03B2, 0x0390, 0x8395, 0x839F, 0x039A, 0x838B, 0x038E, 0x0384, 0x8381, 0x0280, 0x8285, 0x828F, 0x028A,
        0x829B, 0x029E, 0x0294, 0x8291, 0x82B3, 0x02B6, 0x02BC, 0x82B9, 0x02A8, 0x82AD, 0x82A7, 0x02A2, 0x82E3, 0x02E6,
        0x02EC, 0x82E9, 0x02F8, 0x82FD, 0x82F7, 0x02F2, 0x02D0, 0x82D5, 0x82DF, 0x02DA, 0x82CB, 0x02CE, 0x02C4, 0x82C1,
        0x8243, 0x0246, 0x024C, 0x8249, 0x0258, 0x825D, 0x8257, 0x0252, 0x0270, 0x8275, 0x827F, 0x027A, 0x826B, 0x026E,
        0x0264, 0x8261, 0x0220, 0x8225, 0x822F, 0x022A, 0x823B, 0x023E, 0x0234, 0x8231, 0x8213, 0x0216, 0x021C, 0x8219,
        0x0208, 0x820D, 0x8207, 0x0202
    ]

    for d in data:
        i = ((accum >> 8) ^ d) & 0xFF
        accum = ((accum << 8) ^ table[i]) & 0xFFFF

    return accum


def u16_to_bytes(data):
    """
    Split a u16 into two u8

    >>> u16_to_bytes(0x622F)
    [47, 98]
    """
    return [data & 0xFF, data >> 8]


def protocol_1(data):
    """
    Add header and checksum to some data to make a protocol 1 packet
    ref https://emanual.robotis.com/docs/en/dxl/protocol1/

    >>> protocol_1([1, 2, 1])
    [255, 255, 1, 2, 1, 251]
    >>> protocol_1([1, 2, 0])
    [255, 255, 1, 2, 0, 252]
    """

    return [0xFF, 0xFF] + data + [~sum(data) & 0xFF]


def protocol_2(data):
    """
    Add header and CRC to some data to make a protocol 2 packet
    ref https://emanual.robotis.com/docs/en/dxl/protocol2/

    >>> protocol_2([1, 3, 0, 1])
    [255, 255, 253, 0, 1, 3, 0, 1, 25, 78]
    >>> protocol_2([1, 7, 0, 0x55, 0, 6, 4, 0x26])
    [255, 255, 253, 0, 1, 7, 0, 85, 0, 6, 4, 38, 101, 93]
    """

    data = [0xFF, 0xFF, 0xFD, 0] + data
    return data + u16_to_bytes(crc16(data))


def show(i: int, end: bool = False) -> str:
    """
    Show an byte, in decimal, hexadecimal, and ascii
    aligned inj the center or the end of the line
    """
    h = hex(i)
    if len(h) == 3:  # add padding
        h = f"0x0{h[-1]}"
    c = chr(i)
    if c not in PRINTABLES:
        c = ' '
    ret = f"{i:3} {h} {c}"
    w = shutil.get_terminal_size((80, 20)).columns - 1
    print(f"←{ret:>{w}}" if end else f"→{ret:^{w}}")


class Debug:

    def __init__(self, baudrate=1_000_000, ignore_same=True):
        self.serial = AioSerial(port='/dev/ttyUSB0', baudrate=baudrate, timeout=0.01, cancel_read_timeout=0.1)
        self.loop = True
        self.ignore_same = True
        self.sent = None
        self.ready = False

    async def write(self, data: str):
        data = [int(i, base=16) & 0xFF for i in data.split()]
        if data[0] == 1:
            data = protocol_1(data[1:])
        elif data[0] == 2:
            data = protocol_2(data[1:])
        await self.serial.write_async(data)
        for d in data:
            show(d)
            await self.sent.put(d)

    async def keyboard(self):
        while self.loop:
            data = await ainput()
            if data.lower().startswith('q'):
                await self.quit()
            else:
                await self.write(data)

    async def auto(self, data):
        await self.write(data)
        await asyncio.sleep(0.01)
        await self.quit()

    async def read(self):
        while self.loop:
            data = await self.serial.read_async(1)
            data = int.from_bytes(data, 'big')
            try:
                q = self.sent.get_nowait()
                if q == data:
                    print('⇔')
                else:
                    print('old:', q)
                    show(data, True)

            except asyncio.QueueEmpty:
                show(data, True)

    async def quit(self):
        print('exiting...')
        self.loop = False

    async def run(self, data=''):
        self.sent = asyncio.Queue()
        self.loop = True
        while self.serial.read():
            print('…')
        if data:
            await asyncio.gather(self.read(), self.auto(data))
        else:
            await asyncio.gather(self.read(), self.keyboard())


if __name__ == '__main__':
    print('xl320 led 1: 2 1 6 0 3 19 0 1')
    print('xl320 led 1: 2 1 6 0 3 19 0 2')
    print('xl320 led 3: 2 1 6 0 3 19 0 3')
    print('ax12a led 0: 1 1 4 3 19 0')
    print('ax12a led 1: 1 1 4 3 19 1')
    asyncio.run(Debug().run(' '.join(sys.argv[1:])))
