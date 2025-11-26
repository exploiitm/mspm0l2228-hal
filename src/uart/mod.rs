use crate::pac;
use core::marker::PhantomData;
use pac::Iomux;
use pac::Sysctl;
use paste::paste;

mod clk_config;
mod fifo_config;
mod oversampling_config;
mod uart_config;

pub struct uart0 {
    _uart: pac::Uart0,
}

/// UART clock source selection

// uart_config.rs

// oversampling_config.rs

// fifo_config.rs

macro_rules! update_reg {
    ($r:ident, $w:ident, $threshold:expr, $mask:expr) => {
        $w.bits(($r.bits() & !($mask)) | (($threshold) & ($mask)))
    };
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

    fn init(&mut self) {
        self.disable();

        const CTL0_RXE_MASK: u32 = 0x0000_0008;
        const CTL0_TXE_MASK: u32 = 0x0000_0010;
        const CTL0_MODE_MASK: u32 = 0x0000_0700;
        const CTL0_RTSEN_MASK: u32 = 0x0000_2000;
        const CTL0_CTSEN_MASK: u32 = 0x0000_4000;
        const CTL0_FEN_MASK: u32 = 0x0002_0000;

        let config = uart_config::UartConfig {
            mode: uart_config::UartMode::Normal,
            direction: uart_config::UartDirection::TxRx,
            flow_control: uart_config::UartFlowControl::None,
            parity: uart_config::UartParity::None,
            word_length: uart_config::UartWordLength::Bits8,
            stop_bits: uart_config::UartStopBits::One,
        };

        self._uart.uart0_ctl0().modify(|r, w| unsafe {
            update_reg!(
                r,
                w,
                config.mode as u32
                    | config.direction as u32
                    | config.flow_control as u32,
                CTL0_RXE_MASK
                    | CTL0_TXE_MASK
                    | CTL0_MODE_MASK
                    | CTL0_RTSEN_MASK
                    | CTL0_CTSEN_MASK
                    | CTL0_FEN_MASK
            )
        });

        const LCRH_PEN_ENABLE: u32 = 0x0000_0002;
        const LCRH_EPS_MASK: u32 = 0x0000_0004;
        const LCRH_SPS_MASK: u32 = 0x0000_0040;
        const LCRH_WLEN_MASK: u32 = 0x0000_0030;
        const LCRH_STP2_MASK: u32 = 0x0000_0008;

        self._uart.uart0_lcrh().modify(|r, w| unsafe {
            update_reg!(
                r,
                w,
                (config.parity as u32)
                    | (config.word_length as u32)
                    | (config.stop_bits as u32),
                LCRH_PEN_ENABLE
                    | LCRH_EPS_MASK
                    | LCRH_SPS_MASK
                    | LCRH_WLEN_MASK
                    | LCRH_STP2_MASK
            )
        });

        self.enable();
    }

    pub fn new(uart: pac::Uart0) -> Self {
        let mut result = Self { _uart: uart };

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

        result.init();

        // Set oversampling rate
        const CTL0_HSE_MASK: u32 = 0x0001_8000;
        let rate = oversampling_config::UartOversamplingRate::Rate16x;
        result._uart.uart0_ctl0().modify(|r, w| unsafe {
            update_reg!(r, w, rate as u32, CTL0_HSE_MASK)
        });

        // Set baud-rate divisor
        const UART_IBRD_DIVINT_MASK: u32 = 0x0000FFFF;
        const UART_FBRD_DIVFRAC_MASK: u32 = 0x0000003F;
        const UART_LCRH_BRK_MASK: u32 = 0x00000001;
        let integer_divisor = 208;
        let fractional_divisor = 21;
        result._uart.uart0_ibrd().modify(|r, w| unsafe {
            update_reg!(r, w, integer_divisor, UART_IBRD_DIVINT_MASK)
        });
        result._uart.uart0_fbrd().modify(|r, w| unsafe {
            update_reg!(r, w, fractional_divisor, UART_FBRD_DIVFRAC_MASK)
        });
        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).
        result._uart.uart0_lcrh().modify(|r, w| unsafe {
            let value = r.bits() & UART_LCRH_BRK_MASK;

            update_reg!(r, w, value, UART_LCRH_BRK_MASK)
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

        sysctl
            .sysctl_sysosccfg()
            .modify(|r, w| unsafe { w.bits((r.bits() & !(0x3)) | (0 & 0x3)) });
        // update_reg(&mut SYSCTL.soc_lock.sysosc_cfg, 0, 0x3);
        // SYSCTL.soc_lock.hsclk_en &= !(1 as u32);
        sysctl
            .sysctl_hsclken()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 as u32)) });

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

    pub fn transmit(&mut self, data: u8) {
        self._uart
            .uart0_txdata()
            .write(|w| unsafe { w.bits(data as u32) });
    }
}
