#[doc = "Register `SYSCTL_RESETCMD` writer"]
pub type W = crate::W<SysctlResetcmdSpec>;
#[doc = "Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Go {
    #[doc = "1: Issue reset"]
    True = 1,
}
impl From<Go> for bool {
    #[inline(always)]
    fn from(variant: Go) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GO` writer - Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY."]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG, Go>;
impl<'a, REG> GoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Issue reset"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Go::True)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY."]
    #[inline(always)]
    pub fn go(&mut self) -> GoW<'_, SysctlResetcmdSpec> {
        GoW::new(self, 0)
    }
}
#[doc = "Execute an application-triggered reset command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_resetcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlResetcmdSpec;
impl crate::RegisterSpec for SysctlResetcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_resetcmd::W`](W) writer structure"]
impl crate::Writable for SysctlResetcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_RESETCMD to value 0"]
impl crate::Resettable for SysctlResetcmdSpec {}
