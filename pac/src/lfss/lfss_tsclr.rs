#[doc = "Register `LFSS_TSCLR` writer"]
pub type W = crate::W<LfssTsclrSpec>;
#[doc = "Clear time stamp and status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: clear time stamp event"]
    Clr = 1,
}
impl From<Clr> for bool {
    #[inline(always)]
    fn from(variant: Clr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR` writer - Clear time stamp and status register."]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG, Clr>;
impl<'a, REG> ClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Clr::NoEffect)
    }
    #[doc = "clear time stamp event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Clr::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear time stamp and status register."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, LfssTsclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Time Stamp Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tsclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsclrSpec;
impl crate::RegisterSpec for LfssTsclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lfss_tsclr::W`](W) writer structure"]
impl crate::Writable for LfssTsclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TSCLR to value 0"]
impl crate::Resettable for LfssTsclrSpec {}
