#[doc = "Register `UART3_RXDATA` reader"]
pub type R = crate::R<Uart3RxdataSpec>;
#[doc = "Field `DATA` reader - Received Data. When read, this field contains the data that was received by the UART."]
pub type DataR = crate::FieldReader;
#[doc = "UART Framing Error Writing to this bit has no effect. The flag is cleared by writing 1 to the FRMERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmerr {
    #[doc = "0: No framing error has occurred"]
    Clr = 0,
    #[doc = "1: The received character does not have a valid stop bit sequence, which is one or two stop bits depending on the UARTLCRH.STP2 setting (a valid stop bit is 1)."]
    Set = 1,
}
impl From<Frmerr> for bool {
    #[inline(always)]
    fn from(variant: Frmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` reader - UART Framing Error Writing to this bit has no effect. The flag is cleared by writing 1 to the FRMERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO."]
pub type FrmerrR = crate::BitReader<Frmerr>;
impl FrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frmerr {
        match self.bits {
            false => Frmerr::Clr,
            true => Frmerr::Set,
        }
    }
    #[doc = "No framing error has occurred"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Frmerr::Clr
    }
    #[doc = "The received character does not have a valid stop bit sequence, which is one or two stop bits depending on the UARTLCRH.STP2 setting (a valid stop bit is 1)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Frmerr::Set
    }
}
#[doc = "UART Parity Error Writing to this bit has no effect. The flag is cleared by writing 1 to the PARERR bit in the UART EVENT ICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parerr {
    #[doc = "0: No parity error has occurred"]
    Clr = 0,
    #[doc = "1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register."]
    Set = 1,
}
impl From<Parerr> for bool {
    #[inline(always)]
    fn from(variant: Parerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - UART Parity Error Writing to this bit has no effect. The flag is cleared by writing 1 to the PARERR bit in the UART EVENT ICLR register."]
pub type ParerrR = crate::BitReader<Parerr>;
impl ParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parerr {
        match self.bits {
            false => Parerr::Clr,
            true => Parerr::Set,
        }
    }
    #[doc = "No parity error has occurred"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Parerr::Clr
    }
    #[doc = "The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Parerr::Set
    }
}
#[doc = "UART Break Error Writing to this bit has no effect. The flag is cleared by writing 1 to the BRKERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkerr {
    #[doc = "0: No break condition has occurred"]
    Clr = 0,
    #[doc = "1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits)."]
    Set = 1,
}
impl From<Brkerr> for bool {
    #[inline(always)]
    fn from(variant: Brkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKERR` reader - UART Break Error Writing to this bit has no effect. The flag is cleared by writing 1 to the BRKERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type BrkerrR = crate::BitReader<Brkerr>;
impl BrkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brkerr {
        match self.bits {
            false => Brkerr::Clr,
            true => Brkerr::Set,
        }
    }
    #[doc = "No break condition has occurred"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Brkerr::Clr
    }
    #[doc = "A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Brkerr::Set
    }
}
#[doc = "UART Receive Overrun Error Writing to this bit has no effect. The flag is cleared by writing 1 to the OVRERR bit in the UART EVENT ICLR register. In case of a receive FIFO overflow, the FIFO contents remain valid because no further data is written when the FIFO is full. Only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrerr {
    #[doc = "0: No data has been lost due to a receive overrun."]
    Clr = 0,
    #[doc = "1: New data was received but could not be stored, because the previous data was not read (resulting in data loss)."]
    Set = 1,
}
impl From<Ovrerr> for bool {
    #[inline(always)]
    fn from(variant: Ovrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRERR` reader - UART Receive Overrun Error Writing to this bit has no effect. The flag is cleared by writing 1 to the OVRERR bit in the UART EVENT ICLR register. In case of a receive FIFO overflow, the FIFO contents remain valid because no further data is written when the FIFO is full. Only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO."]
pub type OvrerrR = crate::BitReader<Ovrerr>;
impl OvrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrerr {
        match self.bits {
            false => Ovrerr::Clr,
            true => Ovrerr::Set,
        }
    }
    #[doc = "No data has been lost due to a receive overrun."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ovrerr::Clr
    }
    #[doc = "New data was received but could not be stored, because the previous data was not read (resulting in data loss)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ovrerr::Set
    }
}
#[doc = "Noise Error. Writing to this bit has no effect. The flag is cleared by writing 1 to the NERR bit in the UART EVENT ICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nerr {
    #[doc = "0: No noise error occured"]
    Clr = 0,
    #[doc = "1: Noise error occured during majority voting"]
    Set = 1,
}
impl From<Nerr> for bool {
    #[inline(always)]
    fn from(variant: Nerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NERR` reader - Noise Error. Writing to this bit has no effect. The flag is cleared by writing 1 to the NERR bit in the UART EVENT ICLR register."]
pub type NerrR = crate::BitReader<Nerr>;
impl NerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nerr {
        match self.bits {
            false => Nerr::Clr,
            true => Nerr::Set,
        }
    }
    #[doc = "No noise error occured"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Nerr::Clr
    }
    #[doc = "Noise error occured during majority voting"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Nerr::Set
    }
}
impl R {
    #[doc = "Bits 0:7 - Received Data. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART Framing Error Writing to this bit has no effect. The flag is cleared by writing 1 to the FRMERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Parity Error Writing to this bit has no effect. The flag is cleared by writing 1 to the PARERR bit in the UART EVENT ICLR register."]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Break Error Writing to this bit has no effect. The flag is cleared by writing 1 to the BRKERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub fn brkerr(&self) -> BrkerrR {
        BrkerrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Receive Overrun Error Writing to this bit has no effect. The flag is cleared by writing 1 to the OVRERR bit in the UART EVENT ICLR register. In case of a receive FIFO overflow, the FIFO contents remain valid because no further data is written when the FIFO is full. Only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO."]
    #[inline(always)]
    pub fn ovrerr(&self) -> OvrerrR {
        OvrerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Noise Error. Writing to this bit has no effect. The flag is cleared by writing 1 to the NERR bit in the UART EVENT ICLR register."]
    #[inline(always)]
    pub fn nerr(&self) -> NerrR {
        NerrR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart3RxdataSpec;
impl crate::RegisterSpec for Uart3RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart3_rxdata::R`](R) reader structure"]
impl crate::Readable for Uart3RxdataSpec {}
#[doc = "`reset()` method sets UART3_RXDATA to value 0"]
impl crate::Resettable for Uart3RxdataSpec {}
