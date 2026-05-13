#[doc = "Register `AESADV_INT_EVENT1_ICLR` writer"]
pub type W = crate::W<AesadvIntEvent1IclrSpec>;
#[doc = "TRIG0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig0 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Trig0> for bool {
    #[inline(always)]
    fn from(variant: Trig0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG0` writer - TRIG0 event"]
pub type Trig0W<'a, REG> = crate::BitWriter<'a, REG, Trig0>;
impl<'a, REG> Trig0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trig0::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trig0::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG0 event"]
    #[inline(always)]
    pub fn trig0(&mut self) -> Trig0W<'_, AesadvIntEvent1IclrSpec> {
        Trig0W::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent1IclrSpec;
impl crate::RegisterSpec for AesadvIntEvent1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_int_event1_iclr::W`](W) writer structure"]
impl crate::Writable for AesadvIntEvent1IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_INT_EVENT1_ICLR to value 0"]
impl crate::Resettable for AesadvIntEvent1IclrSpec {}
