#[doc = "Register `SYSCTL_MGMT_GPTIMER32B2CC0_RSTCTL` writer"]
pub type W = crate::W<SysctlMgmtGptimer32b2cc0RstctlSpec>;
#[doc = "Assert Reset to IP Domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetassert {
    #[doc = "1: `1`"]
    Do = 1,
}
impl From<Resetassert> for bool {
    #[inline(always)]
    fn from(variant: Resetassert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETASSERT` writer - Assert Reset to IP Domain."]
pub type ResetassertW<'a, REG> = crate::BitWriter<'a, REG, Resetassert>;
impl<'a, REG> ResetassertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn do_(self) -> &'a mut crate::W<REG> {
        self.variant(Resetassert::Do)
    }
}
#[doc = "Clear the RESET STICKY Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetstkyclr {
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Resetstkyclr> for bool {
    #[inline(always)]
    fn from(variant: Resetstkyclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETSTKYCLR` writer - Clear the RESET STICKY Bit"]
pub type ResetstkyclrW<'a, REG> = crate::BitWriter<'a, REG, Resetstkyclr>;
impl<'a, REG> ResetstkyclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstkyclr::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Assert Reset to IP Domain."]
    #[inline(always)]
    pub fn resetassert(&mut self) -> ResetassertW<'_, SysctlMgmtGptimer32b2cc0RstctlSpec> {
        ResetassertW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the RESET STICKY Bit"]
    #[inline(always)]
    pub fn resetstkyclr(&mut self) -> ResetstkyclrW<'_, SysctlMgmtGptimer32b2cc0RstctlSpec> {
        ResetstkyclrW::new(self, 1)
    }
}
#[doc = "Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer32b2cc0_rstctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlMgmtGptimer32b2cc0RstctlSpec;
impl crate::RegisterSpec for SysctlMgmtGptimer32b2cc0RstctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_mgmt_gptimer32b2cc0_rstctl::W`](W) writer structure"]
impl crate::Writable for SysctlMgmtGptimer32b2cc0RstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_MGMT_GPTIMER32B2CC0_RSTCTL to value 0"]
impl crate::Resettable for SysctlMgmtGptimer32b2cc0RstctlSpec {}
