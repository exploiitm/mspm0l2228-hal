#![no_std]

pub mod aes;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod trng;
pub mod uart;
pub mod mpu;
mod utils;

pub use mspm0l2228_pac as pac;
