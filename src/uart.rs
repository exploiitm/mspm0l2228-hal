use crate::pac;
use core::marker::PhantomData;
use pac::Iomux;
use pac::Sysctl;
use paste::paste;

pub struct uart0 {
    _uart: pac::Uart0,
}

/// UART clock source selection
#[repr(u32)]
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
    source: UartClock,
    divider: UartClockDivide,
}

// uart_config.rs
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UartPulseWidth {
    Ns5 = 0x0000_0000, // Pulses shorter than 5ns length are filtered
    Ns10 = 0x0000_0200, // Pulses shorter than 10ns length are filtered
    Ns25 = 0x0000_0400, // Pulses shorter than 25ns length are filtered
    Ns50 = 0x0000_0600, // Pulses shorter than 50ns length are filtered
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UartParity {
    Even = 0x0000_0002 | 0x0000_0004, // Enable even parity generation, checks for even number of 1s
    Odd = 0x0000_0002 | 0x0000_0000, // Enable odd parity generation, checks for odd number of 1s
    StickOne = 0x0000_0002 | 0x0000_0040, // Enable stick parity with parity bit '1'
    StickZero = 0x0000_0002 | 0x0000_0040 | 0x0000_0004, // Stick parity with parity bit '0'
    None = 0x0000_0000, // Disable parity checking and generation
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UartWordLength {
    Bits5 = 0x0000_0000, // Word length is 5 bits
    Bits6 = 0x0000_0010, // Word length is 6 bits
    Bits7 = 0x0000_0020, // Word length is 7 bits
    Bits8 = 0x0000_0030, // Word length is 8 bits
}

#[repr(u32)]
#[derive(Copy, Clone)]
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
pub enum UartStopBits {
    One = 0x0000_0000, // One stop bit
    Two = 0x0000_0008, // Two stop bits
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UartDirection {
    Tx = 0x0000_0010,                 // Enable UART transmitter
    Rx = 0x0000_0008,                 // Enable UART receiver
    TxRx = 0x0000_0008 | 0x0000_0010, // Enable both transmitter and receiver
    None = 0x0000_0000,               // Disable UART transmitter and receiver
}

#[repr(u32)]
#[derive(Copy, Clone)]
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

// oversampling_config.rs
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UartOversamplingRate {
    Rate16x = 0x0000_0000, // Set oversampling rate to 16x
    Rate8x = 0x0000_8000,  // Set oversampling rate to 8x
    Rate3x = 0x0001_0000,  // Set oversampling rate to 3x
                           // Note: IrDA, Manchester, and DALI are not supported when 3x oversampling is enabled.
}

// fifo_config.rs
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TxFifoLevel {
    /// Interrupt triggers when FIFO ≤ 3/4 empty
    ThreeQuartersEmpty = 0x00000001,
    /// Interrupt triggers when FIFO ≤ 1/2 empty
    HalfEmpty = 0x00000002,
    /// Interrupt triggers when FIFO ≤ 1/4 empty
    QuarterEmpty = 0x00000003,
    /// Interrupt triggers when FIFO is empty
    Empty = 0x00000005,
    /// Interrupt triggers when FIFO ≥ 1 entry
    OneEntry = 0x00000007,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum RxFifoLevel {
    /// Interrupt triggers when FIFO ≥ 1 entry available (required for DMA trigger)
    OneEntry = 0x00000070,
    /// Interrupt triggers when FIFO is full
    Full = 0x00000050,
    /// Interrupt triggers when FIFO ≥ 3/4 full
    ThreeQuartersFull = 0x00000030,
    /// Interrupt triggers when FIFO ≥ 1/2 full
    HalfFull = 0x00000020,
    /// Interrupt triggers when FIFO ≥ 1/4 full
    QuarterFull = 0x00000010,
}

#[inline(always)]
fn update_reg(reg: u32, val: u32, mask: u32) -> u32 {
    let tmp = reg & (!mask);
    tmp | (val & mask)
}

impl uart0 {
    fn enable(&mut self) {
        self._uart.uart0_ctl0().write(|w| unsafe { w.bits(0x1) });
    }
    fn disable(&mut self) {
        let value = self._uart.uart0_ctl0().read().bits() as u32;
        self._uart
            .uart0_ctl0()
            .write(|w| unsafe { w.bits(value & !0x1) });
    }

    pub fn new(uart: pac::Uart0) -> Self {
        let mut result = Self { _uart: uart };

        const RSTCTL_KEY_UNLOCK: u32 = 0xB100_0000;
        const RSTCTL_STKYCLR: u32 = 0x0000_0002;
        const RSTCTL_ASSERT: u32 = 0x0000_0001;

        result
            ._uart
            .uart0_gprcm(0)
            .uart0_rstctl()
            .write(|w| unsafe {
                w.bits(RSTCTL_KEY_UNLOCK | RSTCTL_STKYCLR | RSTCTL_ASSERT)
            });

        const PWREN_KEY_UNLOCK: u32 = 0x2600_0000;
        const PWREN_ENABLE: u32 = 0x0000_0001;

        result
            ._uart
            .uart0_gprcm(0)
            .uart0_pwren()
            .write(|w| unsafe { w.bits(PWREN_KEY_UNLOCK | PWREN_ENABLE) });

        let clock_config = UartClockConfig {
            source: UartClock::BusClk,
            divider: UartClockDivide::Div1,
        };

        result
            ._uart
            .uart0_clksel()
            .write(|w| unsafe { w.bits(clock_config.source as u32) });
        result
            ._uart
            .uart0_clkdiv()
            .write(|w| unsafe { w.bits(clock_config.divider as u32) });

        result.disable();

        const CTL0_RXE_MASK: u32 = 0x0000_0008;
        const CTL0_TXE_MASK: u32 = 0x0000_0010;
        const CTL0_MODE_MASK: u32 = 0x0000_0700;
        const CTL0_RTSEN_MASK: u32 = 0x0000_2000;
        const CTL0_CTSEN_MASK: u32 = 0x0000_4000;
        const CTL0_FEN_MASK: u32 = 0x0002_0000;

        let config = UartConfig {
            mode: UartMode::Normal,
            direction: UartDirection::TxRx,
            flow_control: UartFlowControl::None,
            parity: UartParity::None,
            word_length: UartWordLength::Bits8,
            stop_bits: UartStopBits::One,
        };

        result._uart.uart0_ctl0().write(|w| unsafe {
            let val = result._uart.uart0_ctl0().read().bits();

            w.bits(update_reg(
                val,
                config.mode as u32
                    | config.direction as u32
                    | config.flow_control as u32,
                CTL0_RXE_MASK
                    | CTL0_TXE_MASK
                    | CTL0_MODE_MASK
                    | CTL0_RTSEN_MASK
                    | CTL0_CTSEN_MASK
                    | CTL0_FEN_MASK,
            ))
        });

        const LCRH_PEN_ENABLE: u32 = 0x0000_0002;
        const LCRH_EPS_MASK: u32 = 0x0000_0004;
        const LCRH_SPS_MASK: u32 = 0x0000_0040;
        const LCRH_WLEN_MASK: u32 = 0x0000_0030;
        const LCRH_STP2_MASK: u32 = 0x0000_0008;

        result._uart.uart0_lcrh().write(|w| unsafe {
            let val = result._uart.uart0_lcrh().read().bits();

            w.bits(update_reg(
                val,
                (config.parity as u32)
                    | (config.word_length as u32)
                    | (config.stop_bits as u32),
                LCRH_PEN_ENABLE
                    | LCRH_EPS_MASK
                    | LCRH_SPS_MASK
                    | LCRH_WLEN_MASK
                    | LCRH_STP2_MASK,
            ))
        });

        const CTL0_HSE_MASK: u32 = 0x0001_8000;
        let rate = UartOversamplingRate::Rate16x;
        result._uart.uart0_ctl0().write(|w| unsafe {
            let val = result._uart.uart0_ctl0().read().bits();

            w.bits(update_reg(val, rate as u32, CTL0_HSE_MASK))
        });

        const UART_IBRD_DIVINT_MASK: u32 = 0x0000FFFF;
        const UART_FBRD_DIVFRAC_MASK: u32 = 0x0000003F;
        const UART_LCRH_BRK_MASK: u32 = 0x00000001;
        let integer_divisor = 208;
        let fractional_divisor = 21;
        result._uart.uart0_ibrd().write(|w| unsafe {
            let val = result._uart.uart0_ibrd().read().bits();

            w.bits(update_reg(val, integer_divisor, UART_IBRD_DIVINT_MASK))
        });
        result._uart.uart0_fbrd().write(|w| unsafe {
            let val = result._uart.uart0_fbrd().read().bits();

            w.bits(update_reg(val, fractional_divisor, UART_FBRD_DIVFRAC_MASK))
        });

        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).

        result._uart.uart0_lcrh().write(|w| unsafe {
            let val = result._uart.uart0_fbrd().read().bits();
            let value = val & UART_LCRH_BRK_MASK;

            w.bits(update_reg(value, value, UART_LCRH_BRK_MASK))
        });

        const UART_CTL0_FEN_ENABLE: u32 = 0x0002_0000;

        result._uart.uart0_ctl0().write(|w| unsafe {
            let val = result._uart.uart0_ctl0().read().bits();

            w.bits(val | UART_CTL0_FEN_ENABLE)
        });

        const UART_IFLS_RXIFLSEL_MASK: u32 = 0x0000_0070;
        let threshold = RxFifoLevel::Full;
        result._uart.uart0_ifls().write(|w| unsafe {
            let val = result._uart.uart0_ifls().read().bits();

            w.bits(update_reg(val, threshold as u32, UART_IFLS_RXIFLSEL_MASK))
        });
        let threshold = TxFifoLevel::Empty;
        const UART_IFLS_TXIFLSEL_MASK: u32 = 0x0000_0007;
        result._uart.uart0_ifls().write(|w| unsafe {
            let val = result._uart.uart0_ifls().read().bits();

            w.bits(update_reg(val, threshold as u32, UART_IFLS_TXIFLSEL_MASK))
        });

        result.enable();

        let iomux = unsafe { &*Iomux::ptr() };
        iomux
            .iomux_pincm(24)
            .write(|w| unsafe { w.bits(0x80 | 0x2) });
        iomux
            .iomux_pincm(25)
            .write(|w| unsafe { w.bits(0x80 | 0x2 | 0x0004_0000) });

        let sysctl = unsafe { &*Sysctl::ptr() };
        sysctl.sysctl_borthreshold().write(|w| unsafe { w.bits(0) });
        // SYSCTL.soc_lock.bor_threshold = 0;

        sysctl.sysctl_sysosccfg().modify(|r, w| unsafe {
            w.bits((r.bits() & !(0x3)) | (r.bits() & 0x3))
        });
        // update_reg(&mut SYSCTL.soc_lock.sysosc_cfg, 0, 0x3);
        // SYSCTL.soc_lock.hsclk_en &= !(1 as u32);
        sysctl
            .sysctl_hsclken()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 as u32)) });

        result
    }

    pub fn transmit(&mut self, data: u8) {
        self._uart
            .uart0_txdata()
            .write(|w| unsafe { w.bits(data as u32) });
    }
}
