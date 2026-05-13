#[doc = "Register `SYSCTL_SHDNIOREL` writer"]
pub type W = crate::W<SysctlShdniorelSpec>;
#[doc = "Set RELEASE to release the IO after a SHUTDOWN mode exit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Release {
    #[doc = "1: Release IO"]
    True = 1,
}
impl From<Release> for bool {
    #[inline(always)]
    fn from(variant: Release) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELEASE` writer - Set RELEASE to release the IO after a SHUTDOWN mode exit."]
pub type ReleaseW<'a, REG> = crate::BitWriter<'a, REG, Release>;
impl<'a, REG> ReleaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Release IO"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Release::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set RELEASE to release the IO after a SHUTDOWN mode exit."]
    #[inline(always)]
    pub fn release(&mut self) -> ReleaseW<'_, SysctlShdniorelSpec> {
        ReleaseW::new(self, 0)
    }
}
#[doc = "SHUTDOWN IO release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shdniorel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlShdniorelSpec;
impl crate::RegisterSpec for SysctlShdniorelSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_shdniorel::W`](W) writer structure"]
impl crate::Writable for SysctlShdniorelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SHDNIOREL to value 0"]
impl crate::Resettable for SysctlShdniorelSpec {}
