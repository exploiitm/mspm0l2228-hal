#[doc = "Register `UART4_INT_EVENT1_ISET` writer"]
pub type W = crate::W<Uart4IntEvent1IsetSpec>;
#[doc = "Set UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Set UARTOUT Receive Time-Out Interrupt."]
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
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
}
#[doc = "Set UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` writer - Set UART Receive Interrupt."]
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
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Set UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Uart4IntEvent1IsetSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 10 - Set UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, Uart4IntEvent1IsetSpec> {
        RxintW::new(self, 10)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart4IntEvent1IsetSpec;
impl crate::RegisterSpec for Uart4IntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart4_int_event1_iset::W`](W) writer structure"]
impl crate::Writable for Uart4IntEvent1IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART4_INT_EVENT1_ISET to value 0"]
impl crate::Resettable for Uart4IntEvent1IsetSpec {}
