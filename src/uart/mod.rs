use crate::pac;
use pac::Iomux;
use pac::Sysctl;

mod clk_config;
mod fifo_config;
mod oversampling_config;
mod uart_config;
use crate::utils::update_reg;

pub struct Uart0 {
    _uart: pac::Uart0,
}

impl Uart0 {
    fn enable(&mut self) {
        self._uart.uart0_ctl0().write(|w| w.enable().enable());
    }
    fn disable(&mut self) {
        self._uart.uart0_ctl0().write(|w| w.enable().disable());
    }

    fn init(&mut self) {
        self.disable();

        // Set baud-rate divisor
        let integer_divisor = 208;
        let fractional_divisor = 21;
        self._uart
            .uart0_ibrd()
            .write(|w| unsafe { w.divint().bits(integer_divisor) });
        self._uart
            .uart0_fbrd()
            .write(|w| unsafe { w.divfrac().bits(fractional_divisor) });
        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).
        self._uart.uart0_lcrh().write(|w| w.brk().disable());

        let config = uart_config::UartConfig {
            mode: uart_config::UartMode::Normal,
            direction: uart_config::UartDirection::TxRx,
            flow_control: uart_config::UartFlowControl::None,
            parity: uart_config::UartParity::None,
            word_length: uart_config::UartWordLength::Bits8,
            stop_bits: uart_config::UartStopBits::One,
        };

        self._uart.uart0_ctl0().write(|w| {
            match config.mode {
                uart_config::UartMode::Normal => w.mode().uart(),
                uart_config::UartMode::Rs485 => w.mode().rs485(),
                uart_config::UartMode::IdleLine => w.mode().idleline(),
                uart_config::UartMode::Addr9Bit => w.mode().addr9bit(),
                uart_config::UartMode::SmartCard => w.mode().smart(),
                uart_config::UartMode::Dali => w.mode().dali(),
            };

            match config.direction {
                uart_config::UartDirection::Tx => w.txe().enable(),
                uart_config::UartDirection::Rx => w.rxe().enable(),
                uart_config::UartDirection::TxRx => {
                    w.rxe().enable();
                    w.txe().enable()
                }
                uart_config::UartDirection::None => {
                    w.rxe().disable();
                    w.txe().disable()
                }
            };

            match config.flow_control {
                uart_config::UartFlowControl::Rts => w.rtsen().enable(),
                uart_config::UartFlowControl::Cts => w.ctsen().enable(),
                uart_config::UartFlowControl::RtsCts => {
                    w.rtsen().enable();
                    w.ctsen().enable()
                }
                uart_config::UartFlowControl::None => {
                    w.rtsen().disable();
                    w.ctsen().disable()
                }
            }
        });

        self._uart.uart0_lcrh().write(|w| {
            match config.parity {
                uart_config::UartParity::Even => {
                    w.pen().enable();
                    w.eps().even()
                }
                uart_config::UartParity::Odd => {
                    w.pen().enable();
                    w.eps().odd()
                }
                uart_config::UartParity::StickOne => {
                    w.pen().enable();
                    w.sps().enable();
                    w.eps().odd()
                }
                uart_config::UartParity::StickZero => {
                    w.pen().enable();
                    w.sps().enable();
                    w.eps().even()
                }
                uart_config::UartParity::None => w.pen().disable(),
            };

            match config.word_length {
                uart_config::UartWordLength::Bits5 => w.wlen().databit5(),
                uart_config::UartWordLength::Bits6 => w.wlen().databit6(),
                uart_config::UartWordLength::Bits7 => w.wlen().databit7(),
                uart_config::UartWordLength::Bits8 => w.wlen().databit8(),
            };

            match config.stop_bits {
                uart_config::UartStopBits::One => w.stp2().disable(),
                uart_config::UartStopBits::Two => w.stp2().enable(),
            }
        });
    }

    pub fn new(uart: pac::Uart0) -> Self {
        let mut result = Self { _uart: uart };

        let iomux = unsafe { &*Iomux::ptr() };
        iomux
            .iomux_pincm(24)
            .write(|w| unsafe { w.bits(0x80 | 0x2) });
        iomux
            .iomux_pincm(25)
            .write(|w| unsafe { w.bits(0x80 | 0x2 | 0x0004_0000) });

        // Reset
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

        // Enable power
        const PWREN_KEY_UNLOCK: u32 = 0x2600_0000;
        const PWREN_ENABLE: u32 = 0x0000_0001;

        result
            ._uart
            .uart0_gprcm(0)
            .uart0_pwren()
            .write(|w| unsafe { w.bits(PWREN_KEY_UNLOCK | PWREN_ENABLE) });

        // set clock config
        let clock_config = clk_config::UartClockConfig {
            source: clk_config::UartClock::BusClk,
            divider: clk_config::UartClockDivide::Div1,
        };

        result
            ._uart
            .uart0_clksel()
            .write(|w| unsafe { w.bits(clock_config.source as u32) });
        result
            ._uart
            .uart0_clkdiv()
            .write(|w| unsafe { w.bits(clock_config.divider as u32) });

        // Disable UART
        // Also set baud-rate
        result.init();

        // Set oversampling rate
        const CTL0_HSE_MASK: u32 = 0x0001_8000;
        let rate = oversampling_config::UartOversamplingRate::Rate16x;
        result._uart.uart0_ctl0().modify(|r, w| unsafe {
            update_reg!(r, w, rate as u32, CTL0_HSE_MASK)
        });

        // Enable FIFOs
        const UART_CTL0_FEN_ENABLE: u32 = 0x0002_0000;

        result._uart.uart0_ctl0().modify(|r, w| unsafe {
            let val = r.bits();

            w.bits(val | UART_CTL0_FEN_ENABLE)
        });

        let threshold = fifo_config::RxFifoLevel::Full;
        result.set_rx_fifo_threshold(threshold);
        let threshold = fifo_config::TxFifoLevel::Empty;
        result.set_tx_fifo_threshold(threshold);

        let sysctl = unsafe { &*Sysctl::ptr() };
        sysctl.sysctl_borthreshold().write(|w| unsafe { w.bits(0) });
        // SYSCTL.soc_lock.bor_threshold = 0;

        sysctl
            .sysctl_sysosccfg()
            .modify(|r, w| unsafe { w.bits((r.bits() & !(0x3)) | (0 & 0x3)) });
        // update_reg(&mut SYSCTL.soc_lock.sysosc_cfg, 0, 0x3);
        // SYSCTL.soc_lock.hsclk_en &= !(1 as u32);
        sysctl
            .sysctl_hsclken()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 as u32)) });

        result.enable();

        result
    }

    fn set_rx_fifo_threshold(&mut self, threshold: fifo_config::RxFifoLevel) {
        const UART_IFLS_RXIFLSEL_MASK: u32 = 0x0000_0070;
        self._uart.uart0_ifls().modify(|r, w| unsafe {
            update_reg!(r, w, threshold as u32, UART_IFLS_RXIFLSEL_MASK)
        });
    }

    fn set_tx_fifo_threshold(&mut self, threshold: fifo_config::TxFifoLevel) {
        const UART_IFLS_TXIFLSEL_MASK: u32 = 0x0000_0007;
        self._uart.uart0_ifls().modify(|r, w| unsafe {
            update_reg!(r, w, threshold as u32, UART_IFLS_TXIFLSEL_MASK)
        });
    }
}

pub trait UartRead {
    fn is_rxfifo_empty(self: &Self) -> bool;
    fn read_byte_blocking(self: &Self) -> u8;
    fn read_byte(self: &Self) -> Option<u8>;
}

pub trait UartWrite {
    fn is_txfifo_full(self: &Self) -> bool;
    fn write_byte(self: &mut Self, data: u8);
}

impl UartRead for Uart0 {
    #[inline(always)]
    fn is_rxfifo_empty(&self) -> bool {
        const UART_STAT_RXFE_MASK: u32 = 0x00000004;
        const UART_STAT_RXFE_SET: u32 = 0x00000004;

        (self._uart.uart0_stat().read().bits() & UART_STAT_RXFE_MASK)
            == UART_STAT_RXFE_SET
    }

    fn read_byte_blocking(&self) -> u8 {
        const UART_RXDATA_DATA_MASK: u32 = 0x000000FF;
        while self.is_rxfifo_empty() {}

        (self._uart.uart0_rxdata().read().bits() & UART_RXDATA_DATA_MASK) as u8
    }

    fn read_byte(&self) -> Option<u8> {
        if self.is_rxfifo_empty() {
            None
        } else {
            const UART_RXDATA_DATA_MASK: u32 = 0x000000FF;
            Some(
                (self._uart.uart0_rxdata().read().bits()
                    & UART_RXDATA_DATA_MASK) as u8,
            )
        }
    }
}

impl UartWrite for Uart0 {
    #[inline(always)]
    fn is_txfifo_full(&self) -> bool {
        const UART_STAT_TXFF_MASK: u32 = 0x00000080;
        const UART_STAT_TXFF_SET: u32 = 0x00000080;

        (self._uart.uart0_stat().read().bits() & UART_STAT_TXFF_MASK)
            == UART_STAT_TXFF_SET
    }
    fn write_byte(&mut self, data: u8) {
        while self.is_txfifo_full() {}
        self._uart
            .uart0_txdata()
            .write(|w| unsafe { w.bits(data as u32) });
    }
}
