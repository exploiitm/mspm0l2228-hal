#[doc = "Register `UART3_INT_EVENT2_ICLR` writer"]
pub type W = crate::W<Uart3IntEvent2IclrSpec>;
#[doc = "Clear UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` writer - Clear UART Transmit Interrupt."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Clr)
    }
}
impl W {
    #[doc = "Bit 11 - Clear UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, Uart3IntEvent2IclrSpec> {
        TxintW::new(self, 11)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart3IntEvent2IclrSpec;
impl crate::RegisterSpec for Uart3IntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart3_int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for Uart3IntEvent2IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART3_INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for Uart3IntEvent2IclrSpec {}
