#[doc = "Register `UART1_STAT` reader"]
pub type R = crate::R<Uart1StatSpec>;
#[doc = "UART Busy This bit is set as soon as the transmit FIFO or TXDATA register becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: The UART is not busy."]
    Cleared = 0,
    #[doc = "1: The UART is busy transmitting data. This bit remains set until the complete byte, including all stop bits, has been sent/received from/into the shift register."]
    Set = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - UART Busy This bit is set as soon as the transmit FIFO or TXDATA register becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Cleared,
            true => Busy::Set,
        }
    }
    #[doc = "The UART is not busy."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Busy::Cleared
    }
    #[doc = "The UART is busy transmitting data. This bit remains set until the complete byte, including all stop bits, has been sent/received from/into the shift register."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Busy::Set
    }
}
#[doc = "UART Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfe {
    #[doc = "0: The receiver is not empty."]
    Cleared = 0,
    #[doc = "1: If the FIFO is disabled (FEN is 0), the receive holding register is empty. If the FIFO is enabled (FEN is 1), the receive FIFO is empty."]
    Set = 1,
}
impl From<Rxfe> for bool {
    #[inline(always)]
    fn from(variant: Rxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - UART Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type RxfeR = crate::BitReader<Rxfe>;
impl RxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfe {
        match self.bits {
            false => Rxfe::Cleared,
            true => Rxfe::Set,
        }
    }
    #[doc = "The receiver is not empty."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Rxfe::Cleared
    }
    #[doc = "If the FIFO is disabled (FEN is 0), the receive holding register is empty. If the FIFO is enabled (FEN is 1), the receive FIFO is empty."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxfe::Set
    }
}
#[doc = "UART Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxff {
    #[doc = "0: The receiver can receive data."]
    Cleared = 0,
    #[doc = "1: If the FIFO is disabled (FEN is 0), the receive holding register is full. If the FIFO is enabled (FEN is 1), the receive FIFO is full."]
    Set = 1,
}
impl From<Rxff> for bool {
    #[inline(always)]
    fn from(variant: Rxff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFF` reader - UART Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type RxffR = crate::BitReader<Rxff>;
impl RxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxff {
        match self.bits {
            false => Rxff::Cleared,
            true => Rxff::Set,
        }
    }
    #[doc = "The receiver can receive data."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Rxff::Cleared
    }
    #[doc = "If the FIFO is disabled (FEN is 0), the receive holding register is full. If the FIFO is enabled (FEN is 1), the receive FIFO is full."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxff::Set
    }
}
#[doc = "UART Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfe {
    #[doc = "0: The transmitter has data to transmit."]
    Cleared = 0,
    #[doc = "1: If the FIFO is disabled (FEN is 0), the transmit holding register is empty. If the FIFO is enabled (FEN is 1), the transmit FIFO is empty."]
    Set = 1,
}
impl From<Txfe> for bool {
    #[inline(always)]
    fn from(variant: Txfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - UART Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type TxfeR = crate::BitReader<Txfe>;
impl TxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfe {
        match self.bits {
            false => Txfe::Cleared,
            true => Txfe::Set,
        }
    }
    #[doc = "The transmitter has data to transmit."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Txfe::Cleared
    }
    #[doc = "If the FIFO is disabled (FEN is 0), the transmit holding register is empty. If the FIFO is enabled (FEN is 1), the transmit FIFO is empty."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txfe::Set
    }
}
#[doc = "UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txff {
    #[doc = "0: The transmitter is not full."]
    Cleared = 0,
    #[doc = "1: If the FIFO is disabled (FEN is 0), the transmit holding register is full. If the FIFO is enabled (FEN is 1), the transmit FIFO is full."]
    Set = 1,
}
impl From<Txff> for bool {
    #[inline(always)]
    fn from(variant: Txff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFF` reader - UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type TxffR = crate::BitReader<Txff>;
impl TxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txff {
        match self.bits {
            false => Txff::Cleared,
            true => Txff::Set,
        }
    }
    #[doc = "The transmitter is not full."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Txff::Cleared
    }
    #[doc = "If the FIFO is disabled (FEN is 0), the transmit holding register is full. If the FIFO is enabled (FEN is 1), the transmit FIFO is full."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txff::Set
    }
}
#[doc = "Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: The CTS signal is not asserted (high)."]
    Cleared = 0,
    #[doc = "1: The CTS signal is asserted (low)."]
    Set = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Clear To Send"]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::Cleared,
            true => Cts::Set,
        }
    }
    #[doc = "The CTS signal is not asserted (high)."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Cts::Cleared
    }
    #[doc = "The CTS signal is asserted (low)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cts::Set
    }
}
#[doc = "IDLE mode has been detected in Idleline-Multiprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: IDLE has not been detected before last received character. (In idle-line multiprocessor mode)."]
    Cleared = 0,
    #[doc = "1: IDLE has been detected before last received character. (In idle-line multiprocessor mode)."]
    Set = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - IDLE mode has been detected in Idleline-Multiprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::Cleared,
            true => Idle::Set,
        }
    }
    #[doc = "IDLE has not been detected before last received character. (In idle-line multiprocessor mode)."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Idle::Cleared
    }
    #[doc = "IDLE has been detected before last received character. (In idle-line multiprocessor mode)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idle::Set
    }
}
impl R {
    #[doc = "Bit 0 - UART Busy This bit is set as soon as the transmit FIFO or TXDATA register becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - UART Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDLE mode has been detected in Idleline-Multiprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1StatSpec;
impl crate::RegisterSpec for Uart1StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_stat::R`](R) reader structure"]
impl crate::Readable for Uart1StatSpec {}
#[doc = "`reset()` method sets UART1_STAT to value 0"]
impl crate::Resettable for Uart1StatSpec {}
