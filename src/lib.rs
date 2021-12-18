#![no_std]

pub mod generated;
pub mod protocol;
pub mod protocol_1;
pub mod protocol_2;

#[cfg(feature = "std")]
pub mod serialport_embedded_hal;
