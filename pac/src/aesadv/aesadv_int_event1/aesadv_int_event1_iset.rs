#[doc = "Register `AESADV_INT_EVENT1_ISET` writer"]
pub type W = crate::W<AesadvIntEvent1IsetSpec>;
#[doc = "TRIG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig0 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Trig0> for bool {
    #[inline(always)]
    fn from(variant: Trig0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG0` writer - TRIG0"]
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
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trig0::Set)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG0"]
    #[inline(always)]
    pub fn trig0(&mut self) -> Trig0W<'_, AesadvIntEvent1IsetSpec> {
        Trig0W::new(self, 0)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent1IsetSpec;
impl crate::RegisterSpec for AesadvIntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_int_event1_iset::W`](W) writer structure"]
impl crate::Writable for AesadvIntEvent1IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_INT_EVENT1_ISET to value 0"]
impl crate::Resettable for AesadvIntEvent1IsetSpec {}
