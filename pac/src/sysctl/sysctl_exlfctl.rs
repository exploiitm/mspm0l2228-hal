#[doc = "Register `SYSCTL_EXLFCTL` writer"]
pub type W = crate::W<SysctlExlfctlSpec>;
#[doc = "Set SETUSEEXLF to switch LFCLK to the LFCLK_IN digital clock input. Once set, SETUSEEXLF remains set until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setuseexlf {
    #[doc = "1: Use LFCLK_IN as the LFCLK source"]
    True = 1,
}
impl From<Setuseexlf> for bool {
    #[inline(always)]
    fn from(variant: Setuseexlf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETUSEEXLF` writer - Set SETUSEEXLF to switch LFCLK to the LFCLK_IN digital clock input. Once set, SETUSEEXLF remains set until the next BOOTRST."]
pub type SetuseexlfW<'a, REG> = crate::BitWriter<'a, REG, Setuseexlf>;
impl<'a, REG> SetuseexlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use LFCLK_IN as the LFCLK source"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Setuseexlf::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set SETUSEEXLF to switch LFCLK to the LFCLK_IN digital clock input. Once set, SETUSEEXLF remains set until the next BOOTRST."]
    #[inline(always)]
    pub fn setuseexlf(&mut self) -> SetuseexlfW<'_, SysctlExlfctlSpec> {
        SetuseexlfW::new(self, 0)
    }
}
#[doc = "LFCLK_IN and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_exlfctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlExlfctlSpec;
impl crate::RegisterSpec for SysctlExlfctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_exlfctl::W`](W) writer structure"]
impl crate::Writable for SysctlExlfctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_EXLFCTL to value 0"]
impl crate::Resettable for SysctlExlfctlSpec {}
