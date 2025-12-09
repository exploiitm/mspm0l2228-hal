#[repr(u32)]
#[allow(dead_code)]
pub enum UartClock {
    /// Selects BUSCLK as the clock source
    BusClk = 0x0000_0008,
    /// Selects MFCLK as the clock source
    MfClk = 0x0000_0004,
    /// Selects LFCLK as the clock source
    LfClk = 0x0000_0002,
}

/// UART clock divide ratio
#[repr(u32)]
#[allow(dead_code)]
pub enum UartClockDivide {
    /// UART source clock divide ratio set to 1
    Div1 = 0,
    /// UART source clock divide ratio set to 2
    Div2 = 1,
    /// UART source clock divide ratio set to 3
    Div3 = 2,
    /// UART source clock divide ratio set to 4
    Div4 = 3,
    /// UART source clock divide ratio set to 5
    Div5 = 4,
    /// UART source clock divide ratio set to 6
    Div6 = 5,
    /// UART source clock divide ratio set to 7
    Div7 = 6,
    /// UART source clock divide ratio set to 8
    Div8 = 7,
}

pub struct UartClockConfig {
    pub source: UartClock,
    pub divider: UartClockDivide,
}
