#[repr(u32)]
#[allow(dead_code)]
pub enum GpioInversion {
    Enable = 0x04000000,
    Disable = 0x00000000,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum GpioResistor {
    None = 0x0,
    PullUp = 131072,
    PullDown = 65536,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum GpioHysteresis {
    Enable = 0,
    Disable = 524288,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum GpioWakeup {
    Enable = 0x08000000,
    Disable = 0x00000000,
    WakeupOn2 = 0x8000000 | 0x10000000,
}

#[repr(u32)]
pub enum I2cControllerDirction {
    Transmit = 0,
    Recieve = 1,
}
