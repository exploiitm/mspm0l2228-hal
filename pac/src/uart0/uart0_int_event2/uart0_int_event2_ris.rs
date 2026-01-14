#[doc = "Register `UART0_INT_EVENT2_RIS` reader"]
pub type R = crate::R<Uart0IntEvent2RisSpec>;
#[doc = "UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` reader - UART Transmit Interrupt."]
pub type TxintR = crate::BitReader<Txint>;
impl TxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txint {
        match self.bits {
            false => Txint::Clr,
            true => Txint::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txint::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txint::Set
    }
}
impl R {
    #[doc = "Bit 11 - UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_int_event2_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0IntEvent2RisSpec;
impl crate::RegisterSpec for Uart0IntEvent2RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_int_event2_ris::R`](R) reader structure"]
impl crate::Readable for Uart0IntEvent2RisSpec {}
#[doc = "`reset()` method sets UART0_INT_EVENT2_RIS to value 0"]
impl crate::Resettable for Uart0IntEvent2RisSpec {}
