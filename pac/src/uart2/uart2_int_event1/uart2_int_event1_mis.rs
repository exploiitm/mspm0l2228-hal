#[doc = "Register `UART2_INT_EVENT1_MIS` reader"]
pub type R = crate::R<Uart2IntEvent1MisSpec>;
#[doc = "Masked UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - Masked UARTOUT Receive Time-Out Interrupt."]
pub type RtoutR = crate::BitReader<Rtout>;
impl RtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtout {
        match self.bits {
            false => Rtout::Clr,
            true => Rtout::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtout::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtout::Set
    }
}
#[doc = "Masked UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` reader - Masked UART Receive Interrupt."]
pub type RxintR = crate::BitReader<Rxint>;
impl RxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxint {
        match self.bits {
            false => Rxint::Clr,
            true => Rxint::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxint::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxint::Set
    }
}
impl R {
    #[doc = "Bit 0 - Masked UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Masked UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2IntEvent1MisSpec;
impl crate::RegisterSpec for Uart2IntEvent1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_int_event1_mis::R`](R) reader structure"]
impl crate::Readable for Uart2IntEvent1MisSpec {}
#[doc = "`reset()` method sets UART2_INT_EVENT1_MIS to value 0"]
impl crate::Resettable for Uart2IntEvent1MisSpec {}
