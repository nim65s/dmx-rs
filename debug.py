#!/usr/bin/env python

import asyncio

from aioserial import AioSerial
from aioconsole import ainput


class Debug:
    def __init__(self):
        self.serial = AioSerial(port='/dev/ttyACM0', baudrate=1_000_000)

    async def write(self):
        while True:
            data = await ainput()
            await self.serial.write_async([int(i, base=16) & 0xFF for i in data.split()])

    async def read(self):
        while True:
            data = await self.serial.read_async(1)
            data = int.from_bytes(data, 'big')
            print("Received byte: ", hex(data), data)

    async def run(self):
        await asyncio.gather(self.write(), self.read())


if __name__ == '__main__':
    print('xl320 led 1: FF FF FD 0 1 6 0 3 19 0 1 2F 62')
    print('xl320 led 1: FF FF FD 0 1 6 0 3 19 0 2 25 62')
    print('xl320 led 3: FF FF FD 0 1 6 0 3 19 0 3 20 E2')
    asyncio.run(Debug().run())
