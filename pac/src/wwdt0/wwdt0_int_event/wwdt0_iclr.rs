#[doc = "Register `WWDT0_ICLR` writer"]
pub type W = crate::W<Wwdt0IclrSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inttim {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Inttim> for bool {
    #[inline(always)]
    fn from(variant: Inttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTTIM` writer - Interval Timer Interrupt."]
pub type InttimW<'a, REG> = crate::BitWriter<'a, REG, Inttim>;
impl<'a, REG> InttimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Inttim::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Inttim::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn inttim(&mut self) -> InttimW<'_, Wwdt0IclrSpec> {
        InttimW::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0IclrSpec;
impl crate::RegisterSpec for Wwdt0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wwdt0_iclr::W`](W) writer structure"]
impl crate::Writable for Wwdt0IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WWDT0_ICLR to value 0"]
impl crate::Resettable for Wwdt0IclrSpec {}
