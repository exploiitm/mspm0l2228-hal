#[doc = "Register `SYSCTL_LFXTCTL` writer"]
pub type W = crate::W<SysctlLfxtctlSpec>;
#[doc = "Set STARTLFXT to start the low frequency crystal oscillator (LFXT). Once set, STARTLFXT remains set until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startlfxt {
    #[doc = "0: LFXT not started"]
    False = 0,
    #[doc = "1: Start LFXT"]
    True = 1,
}
impl From<Startlfxt> for bool {
    #[inline(always)]
    fn from(variant: Startlfxt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTLFXT` writer - Set STARTLFXT to start the low frequency crystal oscillator (LFXT). Once set, STARTLFXT remains set until the next BOOTRST."]
pub type StartlfxtW<'a, REG> = crate::BitWriter<'a, REG, Startlfxt>;
impl<'a, REG> StartlfxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXT not started"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Startlfxt::False)
    }
    #[doc = "Start LFXT"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Startlfxt::True)
    }
}
#[doc = "Set SETUSELFXT to switch LFCLK to LFXT. Once set, SETUSELFXT remains set until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setuselfxt {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: Use LFXT as the LFCLK source"]
    True = 1,
}
impl From<Setuselfxt> for bool {
    #[inline(always)]
    fn from(variant: Setuselfxt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETUSELFXT` writer - Set SETUSELFXT to switch LFCLK to LFXT. Once set, SETUSELFXT remains set until the next BOOTRST."]
pub type SetuselfxtW<'a, REG> = crate::BitWriter<'a, REG, Setuselfxt>;
impl<'a, REG> SetuselfxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Setuselfxt::False)
    }
    #[doc = "Use LFXT as the LFCLK source"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Setuselfxt::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set STARTLFXT to start the low frequency crystal oscillator (LFXT). Once set, STARTLFXT remains set until the next BOOTRST."]
    #[inline(always)]
    pub fn startlfxt(&mut self) -> StartlfxtW<'_, SysctlLfxtctlSpec> {
        StartlfxtW::new(self, 0)
    }
    #[doc = "Bit 1 - Set SETUSELFXT to switch LFCLK to LFXT. Once set, SETUSELFXT remains set until the next BOOTRST."]
    #[inline(always)]
    pub fn setuselfxt(&mut self) -> SetuselfxtW<'_, SysctlLfxtctlSpec> {
        SetuselfxtW::new(self, 1)
    }
}
#[doc = "LFXT and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_lfxtctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlLfxtctlSpec;
impl crate::RegisterSpec for SysctlLfxtctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_lfxtctl::W`](W) writer structure"]
impl crate::Writable for SysctlLfxtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_LFXTCTL to value 0"]
impl crate::Resettable for SysctlLfxtctlSpec {}
