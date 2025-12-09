use crate::uart::fifo_config::RxFifoLevel;

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartPulseWidth {
    Ns5 = 0x0000_0000, // Pulses shorter than 5ns length are filtered
    Ns10 = 0x0000_0200, // Pulses shorter than 10ns length are filtered
    Ns25 = 0x0000_0400, // Pulses shorter than 25ns length are filtered
    Ns50 = 0x0000_0600, // Pulses shorter than 50ns length are filtered
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartParity {
    Even = 0x0000_0002 | 0x0000_0004, // Enable even parity generation, checks for even number of 1s
    Odd = 0x0000_0002 | 0x0000_0000, // Enable odd parity generation, checks for odd number of 1s
    StickOne = 0x0000_0002 | 0x0000_0040, // Enable stick parity with parity bit '1'
    StickZero = 0x0000_0002 | 0x0000_0040 | 0x0000_0004, // Stick parity with parity bit '0'
    None = 0x0000_0000, // Disable parity checking and generation
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartWordLength {
    Bits5 = 0x0000_0000, // Word length is 5 bits
    Bits6 = 0x0000_0010, // Word length is 6 bits
    Bits7 = 0x0000_0020, // Word length is 7 bits
    Bits8 = 0x0000_0030, // Word length is 8 bits
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartMode {
    Normal = 0x0000_0000,    // Normal operation
    Rs485 = 0x0000_0100,     // RS485 mode
    IdleLine = 0x0000_0200,  // Idle Line mode
    Addr9Bit = 0x0000_0300,  // 9-bit Address mode
    SmartCard = 0x0000_0400, // ISO7816 Smart Card mode
    Dali = 0x0000_0500,      // DALI mode
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartStopBits {
    One = 0x0000_0000, // One stop bit
    Two = 0x0000_0008, // Two stop bits
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartDirection {
    Tx = 0x0000_0010,                 // Enable UART transmitter
    Rx = 0x0000_0008,                 // Enable UART receiver
    TxRx = 0x0000_0008 | 0x0000_0010, // Enable both transmitter and receiver
    None = 0x0000_0000,               // Disable UART transmitter and receiver
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartFlowControl {
    Rts = 0x0000_2000,                  // Enable request-to-send
    Cts = 0x0000_4000,                  // Enable clear-to-send
    RtsCts = 0x0000_2000 | 0x0000_4000, // Enable both RTS and CTS
    None = 0x0000_0000,                 // Disable flow control
}

#[derive(Copy, Clone)]
pub struct UartConfig {
    pub mode: UartMode, // Communication mode and protocol
    pub direction: UartDirection, // TX/RX enable configuration
    pub flow_control: UartFlowControl, // Flow control configuration
    pub parity: UartParity, // Parity configuration
    pub word_length: UartWordLength, // Word length
    pub stop_bits: UartStopBits, // Stop bits configuration
}
