#[doc = "Register `UART3_RSTCTL` writer"]
pub type W = crate::W<Uart3RstctlSpec>;
#[doc = "Assert reset to the peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetassert {
    #[doc = "0: Writing 0 has no effect"]
    Nop = 0,
    #[doc = "1: Assert reset"]
    Assert = 1,
}
impl From<Resetassert> for bool {
    #[inline(always)]
    fn from(variant: Resetassert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETASSERT` writer - Assert reset to the peripheral"]
pub type ResetassertW<'a, REG> = crate::BitWriter<'a, REG, Resetassert>;
impl<'a, REG> ResetassertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Resetassert::Nop)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Resetassert::Assert)
    }
}
#[doc = "Clear the RESETSTKY bit in the STAT register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetstkyclr {
    #[doc = "0: Writing 0 has no effect"]
    Nop = 0,
    #[doc = "1: Clear reset sticky bit"]
    Clr = 1,
}
impl From<Resetstkyclr> for bool {
    #[inline(always)]
    fn from(variant: Resetstkyclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETSTKYCLR` writer - Clear the RESETSTKY bit in the STAT register"]
pub type ResetstkyclrW<'a, REG> = crate::BitWriter<'a, REG, Resetstkyclr>;
impl<'a, REG> ResetstkyclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstkyclr::Nop)
    }
    #[doc = "Clear reset sticky bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstkyclr::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Assert reset to the peripheral"]
    #[inline(always)]
    pub fn resetassert(&mut self) -> ResetassertW<'_, Uart3RstctlSpec> {
        ResetassertW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the RESETSTKY bit in the STAT register"]
    #[inline(always)]
    pub fn resetstkyclr(&mut self) -> ResetstkyclrW<'_, Uart3RstctlSpec> {
        ResetstkyclrW::new(self, 1)
    }
}
#[doc = "Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_rstctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart3RstctlSpec;
impl crate::RegisterSpec for Uart3RstctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart3_rstctl::W`](W) writer structure"]
impl crate::Writable for Uart3RstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART3_RSTCTL to value 0"]
impl crate::Resettable for Uart3RstctlSpec {}
