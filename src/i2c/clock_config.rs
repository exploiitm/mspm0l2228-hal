#[repr(u32)]
#[allow(dead_code)]
pub enum I2cClock {
    BusClk = 0x0000_0008, // Selects BUSCLK as the clock source
    MfClk = 0x0000_0004,  // Selects MFCLK as the clock source
    LfClk = 0x0000_0002,  // Selects LFCLK as the clock source
}

/// UART clock divide ratio
#[repr(u32)]
#[allow(dead_code)]
pub enum I2cClockDivide {
    Div1 = 0, // I2C source clock divide ratio set to 1
    Div2 = 1, // I2C source clock divide ratio set to 2
    Div3 = 2, // I2C source clock divide ratio set to 3
    Div4 = 3, // I2C source clock divide ratio set to 4
    Div5 = 4, // I2C source clock divide ratio set to 5
    Div6 = 5, // I2C source clock divide ratio set to 6
    Div7 = 6, // I2C source clock divide ratio set to 7
    Div8 = 7, // I2C source clock divide ratio set to 8
}

pub struct I2cClockConfig {
    pub source: I2cClock,
    pub divider: I2cClockDivide,
}
