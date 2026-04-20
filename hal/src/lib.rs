#![no_std]

pub mod aes;
pub mod dma;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod mpu;
pub mod trng;
pub mod uart;
mod utils;

pub use mspm0l2228_pac as pac;
