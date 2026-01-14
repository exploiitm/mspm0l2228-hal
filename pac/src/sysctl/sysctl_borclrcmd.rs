#[doc = "Register `SYSCTL_BORCLRCMD` writer"]
pub type W = crate::W<SysctlBorclrcmdSpec>;
#[doc = "GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Go {
    #[doc = "1: Issue clear"]
    True = 1,
}
impl From<Go> for bool {
    #[inline(always)]
    fn from(variant: Go) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GO` writer - GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register."]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG, Go>;
impl<'a, REG> GoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Issue clear"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Go::True)
    }
}
impl W {
    #[doc = "Bit 0 - GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register."]
    #[inline(always)]
    pub fn go(&mut self) -> GoW<'_, SysctlBorclrcmdSpec> {
        GoW::new(self, 0)
    }
}
#[doc = "Set the BOR threshold\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_borclrcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlBorclrcmdSpec;
impl crate::RegisterSpec for SysctlBorclrcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_borclrcmd::W`](W) writer structure"]
impl crate::Writable for SysctlBorclrcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_BORCLRCMD to value 0"]
impl crate::Resettable for SysctlBorclrcmdSpec {}
