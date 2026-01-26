use crate::pac;
use crate::uart::uart_config;

pub struct Uart0 {
    pub _uart: pac::Uart0,
}

impl Uart0 {
    pub fn new(uart: pac::Uart0, iomux: &pac::Iomux) -> Self {
        iomux.iomux_pincm(24).write(|w| {
            unsafe { w.pf().bits(0x2) }; // UART peripheral selection
            w.pc().connected()
        });

        iomux.iomux_pincm(25).write(|w| {
            unsafe { w.pf().bits(0x2) }; // UART peripheral selection
            w.inena().enable();
            w.pc().connected()
        });

        Self::reset(&uart);
        Self::pwren(&uart);
        Self::init(&uart);
        Self::enable(&uart);

        Self { _uart: uart }
    }

    fn init(uart: &pac::Uart0) {
        Self::disable(&uart);

        let config = uart_config::UartConfig::default();

        uart.uart0_clksel()
            .write(|w| match config.clock_config.source {
                uart_config::UartClock::LfClk => w.lfclk_sel().enable(),
                uart_config::UartClock::MfClk => w.mfclk_sel().enable(),
                uart_config::UartClock::BusClk => w.busclk_sel().enable(),
            });

        uart.uart0_clkdiv()
            .write(|w| match config.clock_config.divider {
                uart_config::UartClockDivide::Div1 => w.ratio().div_by_1(),
                uart_config::UartClockDivide::Div2 => w.ratio().div_by_2(),
                uart_config::UartClockDivide::Div3 => w.ratio().div_by_3(),
                uart_config::UartClockDivide::Div4 => w.ratio().div_by_4(),
                uart_config::UartClockDivide::Div5 => w.ratio().div_by_5(),
                uart_config::UartClockDivide::Div6 => w.ratio().div_by_6(),
                uart_config::UartClockDivide::Div7 => w.ratio().div_by_7(),
                uart_config::UartClockDivide::Div8 => w.ratio().div_by_8(),
            });

        // Set baud-rate divisor
        uart.uart0_ibrd()
            .write(|w| unsafe { w.divint().bits(config.integer_divisor) });
        uart.uart0_fbrd()
            .write(|w| unsafe { w.divfrac().bits(config.fractional_divisor) });

        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).
        uart.uart0_lcrh().write(|w| w.brk().disable());

        uart.uart0_ctl0().write(|w| {
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
            };

            match config.enable_fifo {
                false => w.fen().disable(),
                true => w.fen().enable(),
            };

            match config.oversampling_rate {
                uart_config::UartOversamplingRate::Rate16x => w.hse().ovs16(),
                uart_config::UartOversamplingRate::Rate8x => w.hse().ovs8(),
                uart_config::UartOversamplingRate::Rate3x => w.hse().ovs3(),
            }
        });

        uart.uart0_lcrh().write(|w| {
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

        uart.uart0_ifls().write(|w| {
            match config.rxfifo_level {
                uart_config::RxFifoLevel::OneEntry => w.rxiflsel().lvl_1(),
                uart_config::RxFifoLevel::Full => w.rxiflsel().lvl_full(),
                uart_config::RxFifoLevel::ThreeQuartersFull => {
                    w.rxiflsel().lvl_3_4()
                }
                uart_config::RxFifoLevel::HalfFull => w.rxiflsel().lvl_1_2(),
                uart_config::RxFifoLevel::QuarterFull => w.rxiflsel().lvl_1_4(),
            };

            match config.txfifo_level {
                uart_config::TxFifoLevel::OneEntry => w.txiflsel().lvl_1(),
                uart_config::TxFifoLevel::Empty => w.txiflsel().lvl_empty(),
                uart_config::TxFifoLevel::ThreeQuartersEmpty => {
                    w.txiflsel().lvl_3_4()
                }
                uart_config::TxFifoLevel::HalfEmpty => w.txiflsel().lvl_1_2(),
                uart_config::TxFifoLevel::QuarterEmpty => {
                    w.txiflsel().lvl_1_4()
                }
            }
        });
    }

    fn enable(uart: &pac::Uart0) {
        uart.uart0_ctl0().modify(|_, w| w.enable().enable());
    }

    fn disable(uart: &pac::Uart0) {
        uart.uart0_ctl0().write(|w| w.enable().disable());
    }

    fn reset(uart: &pac::Uart0) {
        uart.uart0_gprcm(0).uart0_rstctl().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });
    }

    fn pwren(uart: &pac::Uart0) {
        uart.uart0_gprcm(0).uart0_pwren().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });
    }
}
