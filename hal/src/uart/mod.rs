mod uart_config;
use crate::dma::{self, dma_config};
use crate::pac;
use core::fmt::Write;
use paste::paste;

pub trait UartWrite {
    fn is_txfifo_full(self: &Self) -> bool;
    fn write_byte(self: &Self, data: u8);
}

pub trait UartRead {
    fn is_rxfifo_empty(&self) -> bool;
    fn read_byte_blocking(&self) -> u8;
}

macro_rules! uart {
    ($UART_NUM:literal, $CHAN_NUM: literal) => {
    paste! {

pub struct [<Uart $UART_NUM>] {
    _uart: pac::[<Uart $UART_NUM>],
    _chan: dma::Channel<$CHAN_NUM>,
}

impl [<Uart $UART_NUM>] {
    pub fn new(uart: pac::[<Uart $UART_NUM>], iomux: &pac::Iomux, baud_rate: u32, chan: dma::Channel<$CHAN_NUM>) -> [<Uart $UART_NUM>] {
        iomux.iomux_pincm(
            match $UART_NUM {
                0 => 24,
                1 => 18,
                _ => panic!()
            }
        ).write(|w| {
            unsafe { w.pf().bits(0x2) }; // UART peripheral selection
            w.pc().connected()
        });

        iomux.iomux_pincm(
            match $UART_NUM {
                0 => 25,
                1 => 19,
                _ => panic!()
            }
        ).write(|w| {
            unsafe { w.pf().bits(0x2) }; // UART peripheral selection
            w.inena().enable();
            w.pc().connected()
        });

        Self::reset(&uart);
        Self::pwren(&uart);
        Self::init(&uart, baud_rate);
        Self::enable(&uart);

        [<Uart $UART_NUM>] { _uart: uart, _chan: chan }
    }
}

impl [<Uart $UART_NUM>] {
    fn init(uart: &pac::[<Uart $UART_NUM>], baud_rate: u32) {
        Self::disable(&uart);

        let mut config = uart_config::UartConfig::default();
        config.baud_rate = match baud_rate {
            115200 => uart_config::BaudRate::preset_115200(),
            230400 => uart_config::BaudRate::preset_230400(),
            _ => panic!()
        };

        uart.[<uart $UART_NUM _clksel>]()
            .write(|w| match config.clock_config.source {
                uart_config::UartClock::LfClk => w.lfclk_sel().enable(),
                uart_config::UartClock::MfClk => w.mfclk_sel().enable(),
                uart_config::UartClock::BusClk => w.busclk_sel().enable(),
            });

        uart.[<uart $UART_NUM _clkdiv>]()
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
        uart.[<uart $UART_NUM _ibrd>]()
            .write(|w| unsafe { w.divint().bits(config.baud_rate.integer_divisor) });
        uart.[<uart $UART_NUM _fbrd>]()
            .write(|w| unsafe { w.divfrac().bits(config.baud_rate.fractional_divisor) });

        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).
        uart.[<uart $UART_NUM _lcrh>]().write(|w| w.brk().disable());

        uart.[<uart $UART_NUM _ctl0>]().write(|w| {
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

        uart.[<uart $UART_NUM _lcrh>]().write(|w| {
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

        uart.[<uart $UART_NUM _ifls>]().write(|w| {
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

    fn enable(uart: &pac::[<Uart $UART_NUM>]) {
        uart.[<uart $UART_NUM _ctl0>]().modify(|_, w| w.enable().enable());
    }

    fn disable(uart: &pac::[<Uart $UART_NUM>]) {
        uart.[<uart $UART_NUM _ctl0>]().write(|w| w.enable().disable());
    }

    fn reset(uart: &pac::[<Uart $UART_NUM>]) {
        uart.[<uart $UART_NUM _gprcm>](0).[<uart $UART_NUM _rstctl>]().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });
    }

    fn pwren(uart: &pac::[<Uart $UART_NUM>]) {
        uart.[<uart $UART_NUM _gprcm>](0).[<uart $UART_NUM _pwren>]().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });
    }

    pub fn is_busy(&self) -> bool {
        self._uart.[<uart $UART_NUM _stat>]().read().busy().is_set()
    }
}

impl UartRead for [<Uart $UART_NUM>] {
    #[inline(always)]
    fn is_rxfifo_empty(&self) -> bool {
        self._uart.[<uart $UART_NUM _stat>]().read().rxfe().bit()
    }

    fn read_byte_blocking(&self) -> u8 {
        while self.is_rxfifo_empty() {}

        self._uart.[<uart $UART_NUM _rxdata>]().read().data().bits()
    }
}

impl [<Uart $UART_NUM>] {
    pub fn read_bytes_dma(&self, dma: &dma::Dma, out_buf: &mut [u8]) {
        self.read_bytes_dma_nonblocking(dma, out_buf);
        self.dma_wait_for_read(dma);
    }

    pub fn read_bytes_dma_nonblocking(&self, dma: &dma::Dma, out_buf: &mut [u8]) {
        let out_len = out_buf.len();
        let chan = &self._chan;
        let uart_rx_addr: u32 =
            self._uart.[<uart $UART_NUM _rxdata>]().as_ptr().addr() as u32;

        self.dma_preconfig(dma, chan);
        let data_out_addr: u32 = out_buf.as_ptr().addr() as u32;
        dma.dma_set(chan, uart_rx_addr, data_out_addr, out_len as u16);
        dma.enable(chan);
    }

    pub fn dma_wait_for_read(&self, dma: &dma::Dma) {
        let chan = &self._chan;
        dma.dma_wait(chan);
    }

    pub fn dma_preconfig(
        &self,
        dma: &dma::Dma,
        chan: &dma::Channel<$CHAN_NUM>,
    ) {
        self._uart.[<uart $UART_NUM _int_event0>](0).[<uart $UART_NUM _int_event0_imask>]().write(|w| w.dma_done_rx().set_());
        self._uart.[<uart $UART_NUM _int_event1>](0).[<uart $UART_NUM _int_event1_imask>]().write(|w| w.rxint().set_());
        dma.disable(chan);

        dma.dma_init(
            chan,
            &dma_config::DmaConfig {
                src_increment: dma_config::Increment::Unchanged,
                dst_increment: dma_config::Increment::Increment,
                src_width: dma_config::Width::Byte,
                dst_width: dma_config::Width::Byte,
                trigger_map: dma_config::DmaTrigger::[<Uart $UART_NUM Rx>],
            },
        );
    }

}

impl UartWrite for [<Uart $UART_NUM>] {
    #[inline(always)]
    fn is_txfifo_full(&self) -> bool {
        self._uart.[<uart $UART_NUM _stat>]().read().txff().bit()
    }
    #[inline(always)]
    fn write_byte(&self, data: u8) {
        while self.is_txfifo_full() {}
        self._uart
            .[<uart $UART_NUM _txdata>]()
            .write(|w| unsafe { w.bits(data as u32) });
        while self.is_busy() {}
    }
}

impl Write for [<Uart $UART_NUM>] {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_byte(c as u8);
        }
        Ok(())
    }
}

        }
    }
}

uart!(0, 2);
uart!(1, 3);
