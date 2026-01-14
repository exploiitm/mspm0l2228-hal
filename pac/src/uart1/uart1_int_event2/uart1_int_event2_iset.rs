#[doc = "Register `UART1_INT_EVENT2_ISET` writer"]
pub type W = crate::W<Uart1IntEvent2IsetSpec>;
#[doc = "Set UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` writer - Set UART Transmit Interrupt."]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG, Txint>;
impl<'a, REG> TxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Set)
    }
}
impl W {
    #[doc = "Bit 11 - Set UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, Uart1IntEvent2IsetSpec> {
        TxintW::new(self, 11)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_int_event2_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1IntEvent2IsetSpec;
impl crate::RegisterSpec for Uart1IntEvent2IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart1_int_event2_iset::W`](W) writer structure"]
impl crate::Writable for Uart1IntEvent2IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_INT_EVENT2_ISET to value 0"]
impl crate::Resettable for Uart1IntEvent2IsetSpec {}
