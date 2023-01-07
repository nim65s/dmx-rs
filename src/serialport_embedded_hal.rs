//! Implementation of [`Serial`](https://docs.rs/embedded-hal/0.2.5/embedded_hal/serial/index.html)
//! for [`SerialPort`](https://docs.rs/serialport/4.0.1/serialport/trait.SerialPort.html)

extern crate std;
use serialport::SerialPort;
use std::io::{Read, Write};
use std::println;

#[cfg(unix)]
type Port = serialport::TTYPort;

#[cfg(windows)]
type Port = serialport::COMPort;

pub struct Serial {
    port: Port,
}

impl Serial {
    pub fn new(port: serialport::SerialPortBuilder) -> Self {
        Serial {
            port: Port::open(&port).unwrap(),
        }
    }
}

pub enum IoSerialError {
    Io(std::io::Error),
    Serial(serialport::Error),
}

pub type Error = nb::Error<IoSerialError>;

fn from_io(e: std::io::Error) -> Error {
    nb::Error::Other(IoSerialError::Io(e))
}

fn from_serial(e: serialport::Error) -> Error {
    nb::Error::Other(IoSerialError::Serial(e))
}

impl From<std::io::Error> for IoSerialError {
    fn from(e: std::io::Error) -> Self {
        IoSerialError::Io(e)
    }
}

impl embedded_hal::serial::Read<u8> for Serial {
    type Error = IoSerialError;

    fn read(&mut self) -> Result<u8, Error> {
        if self.port.bytes_to_read().map_err(from_serial)? == 0 {
            return Err(nb::Error::WouldBlock);
        }
        let mut byte: [u8; 1] = [0];
        self.port.read(&mut byte).map_err(from_io)?;
        //println!("SEH read {:?}", byte);
        Ok(byte[0])
    }
}

impl embedded_hal::serial::Write<u8> for Serial {
    type Error = IoSerialError;

    fn write(&mut self, word: u8) -> Result<(), Error> {
        //println!("SEH writing {}", word);
        while self.port.write(&[word]).map_err(from_io)? != 1 {
            println!("writing {word} again");
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Error> {
        //println!("SEH flushing");
        if self.port.bytes_to_write().map_err(from_serial)? != 0 {
            return Err(nb::Error::WouldBlock);
        }
        self.port.flush().map_err(from_io)
    }
}
