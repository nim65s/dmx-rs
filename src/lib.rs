#![no_std]

pub mod generated;
pub mod protocol;
pub mod protocol_1;
pub mod protocol_2;
pub use generated::*;

#[cfg(feature = "std")]
pub mod serialport_embedded_hal;
