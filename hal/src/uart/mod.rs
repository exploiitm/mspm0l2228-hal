mod uart0;
mod uart1;
mod uart_config;

pub use uart0::Uart0;
pub use uart1::Uart1;

pub trait UartRead {
    fn is_rxfifo_empty(self: &Self) -> bool;
    fn read_byte_blocking(self: &Self) -> u8;
    fn read_byte(self: &Self) -> Option<u8>;
}

pub trait UartWrite {
    fn is_txfifo_full(self: &Self) -> bool;
    fn write_byte(self: &Self, data: u8);
}

impl UartRead for Uart0 {
    #[inline(always)]
    fn is_rxfifo_empty(&self) -> bool {
        self._uart.uart0_stat().read().rxfe().bit()
    }

    fn read_byte_blocking(&self) -> u8 {
        while self.is_rxfifo_empty() {}

        self._uart.uart0_rxdata().read().data().bits()
    }

    fn read_byte(&self) -> Option<u8> {
        if self.is_rxfifo_empty() {
            None
        } else {
            Some(self._uart.uart0_rxdata().read().data().bits())
        }
    }
}

impl UartRead for Uart1 {
    #[inline(always)]
    fn is_rxfifo_empty(&self) -> bool {
        self._uart.uart1_stat().read().rxfe().bit()
    }

    fn read_byte_blocking(&self) -> u8 {
        while self.is_rxfifo_empty() {}

        self._uart.uart1_rxdata().read().data().bits()
    }

    fn read_byte(&self) -> Option<u8> {
        if self.is_rxfifo_empty() {
            None
        } else {
            Some(self._uart.uart1_rxdata().read().data().bits())
        }
    }
}
impl UartWrite for Uart0 {
    #[inline(always)]
    fn is_txfifo_full(&self) -> bool {
        self._uart.uart0_stat().read().txff().bit()
    }
    #[inline(always)]
    fn write_byte(&self, data: u8) {
        while self.is_txfifo_full() {}
        self._uart
            .uart0_txdata()
            .write(|w| unsafe { w.bits(data as u32) });
    }
}

impl UartWrite for Uart1 {
    #[inline(always)]
    fn is_txfifo_full(&self) -> bool {
        self._uart.uart1_stat().read().txff().bit()
    }
    #[inline(always)]
    fn write_byte(&self, data: u8) {
        while self.is_txfifo_full() {}
        self._uart
            .uart1_txdata()
            .write(|w| unsafe { w.bits(data as u32) });
    }
}

use core::fmt::Write;
impl Write for Uart0 {
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

impl Write for Uart1 {
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
