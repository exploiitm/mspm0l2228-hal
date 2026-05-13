#[doc = "Register `UART0_INT_EVENT2_IIDX` reader"]
pub type R = crate::R<Uart0IntEvent2IidxSpec>;
#[doc = "UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "12: UART transmit interrupt; Interrupt Flag: TX"]
    Txifg = 12,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            12 => Some(Stat::Txifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "UART transmit interrupt; Interrupt Flag: TX"]
    #[inline(always)]
    pub fn is_txifg(&self) -> bool {
        *self == Stat::Txifg
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_int_event2_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0IntEvent2IidxSpec;
impl crate::RegisterSpec for Uart0IntEvent2IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_int_event2_iidx::R`](R) reader structure"]
impl crate::Readable for Uart0IntEvent2IidxSpec {}
#[doc = "`reset()` method sets UART0_INT_EVENT2_IIDX to value 0"]
impl crate::Resettable for Uart0IntEvent2IidxSpec {}
