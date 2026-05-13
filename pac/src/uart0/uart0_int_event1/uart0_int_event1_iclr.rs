#[doc = "Register `UART0_INT_EVENT1_ICLR` writer"]
pub type W = crate::W<Uart0IntEvent1IclrSpec>;
#[doc = "Clear UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Clear UARTOUT Receive Time-Out Interrupt."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
}
#[doc = "Clear UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` writer - Clear UART Receive Interrupt."]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG, Rxint>;
impl<'a, REG> RxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Uart0IntEvent1IclrSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 10 - Clear UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, Uart0IntEvent1IclrSpec> {
        RxintW::new(self, 10)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0IntEvent1IclrSpec;
impl crate::RegisterSpec for Uart0IntEvent1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart0_int_event1_iclr::W`](W) writer structure"]
impl crate::Writable for Uart0IntEvent1IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART0_INT_EVENT1_ICLR to value 0"]
impl crate::Resettable for Uart0IntEvent1IclrSpec {}
