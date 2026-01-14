#[doc = "Register `SYSCTL_FCCCMD` writer"]
pub type W = crate::W<SysctlFcccmdSpec>;
#[doc = "Set GO to start a capture with the frequency clock counter (FCC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Go {
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Go> for bool {
    #[inline(always)]
    fn from(variant: Go) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GO` writer - Set GO to start a capture with the frequency clock counter (FCC)."]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG, Go>;
impl<'a, REG> GoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Go::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set GO to start a capture with the frequency clock counter (FCC)."]
    #[inline(always)]
    pub fn go(&mut self) -> GoW<'_, SysctlFcccmdSpec> {
        GoW::new(self, 0)
    }
}
#[doc = "Frequency clock counter start capture\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fcccmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFcccmdSpec;
impl crate::RegisterSpec for SysctlFcccmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_fcccmd::W`](W) writer structure"]
impl crate::Writable for SysctlFcccmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FCCCMD to value 0"]
impl crate::Resettable for SysctlFcccmdSpec {}
