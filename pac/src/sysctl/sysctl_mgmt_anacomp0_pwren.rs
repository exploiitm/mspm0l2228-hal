#[doc = "Register `SYSCTL_MGMT_ANACOMP0_PWREN` reader"]
pub type R = crate::R<SysctlMgmtAnacomp0PwrenSpec>;
#[doc = "Register `SYSCTL_MGMT_ANACOMP0_PWREN` writer"]
pub type W = crate::W<SysctlMgmtAnacomp0PwrenSpec>;
#[doc = "IP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - IP Enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - IP Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - IP Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, SysctlMgmtAnacomp0PwrenSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_anacomp0_pwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_anacomp0_pwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlMgmtAnacomp0PwrenSpec;
impl crate::RegisterSpec for SysctlMgmtAnacomp0PwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_mgmt_anacomp0_pwren::R`](R) reader structure"]
impl crate::Readable for SysctlMgmtAnacomp0PwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_mgmt_anacomp0_pwren::W`](W) writer structure"]
impl crate::Writable for SysctlMgmtAnacomp0PwrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_MGMT_ANACOMP0_PWREN to value 0"]
impl crate::Resettable for SysctlMgmtAnacomp0PwrenSpec {}
