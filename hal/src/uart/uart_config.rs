pub struct UartConfig {
    pub mode: UartMode,                // Communication mode and protocol
    pub direction: UartDirection,      // TX/RX enable configuration
    pub flow_control: UartFlowControl, // Flow control configuration
    pub parity: UartParity,            // Parity configuration
    pub word_length: UartWordLength,   // Word length
    pub stop_bits: UartStopBits,       // Stop bits configuration
    pub baud_rate: BaudRate,
    pub enable_fifo: bool,
    pub oversampling_rate: UartOversamplingRate,
    pub rxfifo_level: RxFifoLevel,
    pub txfifo_level: TxFifoLevel,
    pub clock_config: UartClockConfig,
}

impl Default for UartConfig {
    fn default() -> Self {
        UartConfig {
            mode: UartMode::Normal,
            direction: UartDirection::TxRx,
            flow_control: UartFlowControl::None,
            parity: UartParity::None,
            word_length: UartWordLength::Bits8,
            stop_bits: UartStopBits::One,
            baud_rate: BaudRate::preset_115200(),
            enable_fifo: true,
            oversampling_rate: UartOversamplingRate::Rate16x,
            rxfifo_level: RxFifoLevel::Full,
            txfifo_level: TxFifoLevel::Empty,
            clock_config: UartClockConfig::default(),
        }
    }
}

#[allow(dead_code)]
pub enum UartPulseWidth {
    Ns5,  // Pulses shorter than 5ns length are filtered
    Ns10, // Pulses shorter than 10ns length are filtered
    Ns25, // Pulses shorter than 25ns length are filtered
    Ns50, // Pulses shorter than 50ns length are filtered
}

#[allow(dead_code)]
pub enum UartParity {
    Even,      // Enable even parity generation, checks for even number of 1s
    Odd,       // Enable odd parity generation, checks for odd number of 1s
    StickOne,  // Enable stick parity with parity bit '1'
    StickZero, // Stick parity with parity bit '0'
    None,      // Disable parity checking and generation
}

#[allow(dead_code)]
pub enum UartWordLength {
    Bits5, // Word length is 5 bits
    Bits6, // Word length is 6 bits
    Bits7, // Word length is 7 bits
    Bits8, // Word length is 8 bits
}

#[allow(dead_code)]
pub enum UartMode {
    Normal,    // Normal operation
    Rs485,     // RS485 mode
    IdleLine,  // Idle Line mode
    Addr9Bit,  // 9-bit Address mode
    SmartCard, // ISO7816 Smart Card mode
    Dali,      // DALI mode
}

#[allow(dead_code)]
pub enum UartStopBits {
    One, // One stop bit
    Two, // Two stop bits
}

#[allow(dead_code)]
pub enum UartDirection {
    Tx,   // Enable UART transmitter
    Rx,   // Enable UART receiver
    TxRx, // Enable both transmitter and receiver
    None, // Disable UART transmitter and receiver
}

#[allow(dead_code)]
pub enum UartFlowControl {
    Rts,    // Enable request-to-send
    Cts,    // Enable clear-to-send
    RtsCts, // Enable both RTS and CTS
    None,   // Disable flow control
}

#[allow(dead_code)]
pub enum UartOversamplingRate {
    Rate16x, // Set oversampling rate to 16x
    Rate8x,  // Set oversampling rate to 8x
    Rate3x,  // Set oversampling rate to 3x
             // Note: IrDA, Manchester, and DALI are not supported when 3x oversampling is enabled.
}

#[allow(dead_code)]
pub enum TxFifoLevel {
    ThreeQuartersEmpty, // Interrupt triggers when FIFO ≤ 3/4 empty
    HalfEmpty,          // Interrupt triggers when FIFO ≤ 1/2 empty
    QuarterEmpty,       // Interrupt triggers when FIFO ≤ 1/4 empty
    Empty,              // Interrupt triggers when FIFO is empty
    OneEntry,           // Interrupt triggers when FIFO ≥ 1 entry
}

#[allow(dead_code)]
pub enum RxFifoLevel {
    OneEntry, // Interrupt triggers when FIFO ≥ 1 entry available (required for DMA trigger)
    Full,     // Interrupt triggers when FIFO is full
    ThreeQuartersFull, // Interrupt triggers when FIFO ≥ 3/4 full
    HalfFull, // Interrupt triggers when FIFO ≥ 1/2 full
    QuarterFull, // Interrupt triggers when FIFO ≥ 1/4 full
}

#[allow(dead_code)]
pub enum UartClock {
    BusClk, // Selects BUSCLK as the clock source
    MfClk,  // Selects MFCLK as the clock source
    LfClk,  // Selects LFCLK as the clock source
}

/// UART clock divide ratio

#[allow(dead_code)]
pub enum UartClockDivide {
    Div1, // UART source clock divide ratio set to 1
    Div2, // UART source clock divide ratio set to 2
    Div3, // UART source clock divide ratio set to 3
    Div4, // UART source clock divide ratio set to 4
    Div5, // UART source clock divide ratio set to 5
    Div6, // UART source clock divide ratio set to 6
    Div7, // UART source clock divide ratio set to 7
    Div8, // UART source clock divide ratio set to 8
}

pub struct UartClockConfig {
    pub source: UartClock,
    pub divider: UartClockDivide,
}

impl Default for UartClockConfig {
    fn default() -> Self {
        UartClockConfig {
            source: UartClock::BusClk,
            divider: UartClockDivide::Div1,
        }
    }
}

pub struct BaudRate {
    pub integer_divisor: u16,
    pub fractional_divisor: u8,
}

impl BaudRate {
    // Works for default clock, over_sampling and clockdiv values
    pub fn preset_115200() -> Self {
        BaudRate {
            integer_divisor: 17,
            fractional_divisor: 23,
        }
    }

    pub fn preset_230400() -> Self {
        BaudRate {
            integer_divisor: 8,
            fractional_divisor: 44,
        }
    }
}
