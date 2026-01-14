#[doc = "Register `SYSCTL_HSCLKEN` reader"]
pub type R = crate::R<SysctlHsclkenSpec>;
#[doc = "Register `SYSCTL_HSCLKEN` writer"]
pub type W = crate::W<SysctlHsclkenSpec>;
#[doc = "HFXTEN enables or disables the high frequency crystal oscillator (HFXT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxten {
    #[doc = "0: Disable the HFXT"]
    Disable = 0,
    #[doc = "1: Enable the HFXT"]
    Enable = 1,
}
impl From<Hfxten> for bool {
    #[inline(always)]
    fn from(variant: Hfxten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTEN` reader - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
pub type HfxtenR = crate::BitReader<Hfxten>;
impl HfxtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxten {
        match self.bits {
            false => Hfxten::Disable,
            true => Hfxten::Enable,
        }
    }
    #[doc = "Disable the HFXT"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hfxten::Disable
    }
    #[doc = "Enable the HFXT"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hfxten::Enable
    }
}
#[doc = "Field `HFXTEN` writer - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
pub type HfxtenW<'a, REG> = crate::BitWriter<'a, REG, Hfxten>;
impl<'a, REG> HfxtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the HFXT"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxten::Disable)
    }
    #[doc = "Enable the HFXT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxten::Enable)
    }
}
#[doc = "USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Useexthfclk {
    #[doc = "0: Use HFXT as the HFCLK source"]
    Disable = 0,
    #[doc = "1: Use the HFCLK_IN digital clock input as the HFCLK source"]
    Enable = 1,
}
impl From<Useexthfclk> for bool {
    #[inline(always)]
    fn from(variant: Useexthfclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEEXTHFCLK` reader - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
pub type UseexthfclkR = crate::BitReader<Useexthfclk>;
impl UseexthfclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Useexthfclk {
        match self.bits {
            false => Useexthfclk::Disable,
            true => Useexthfclk::Enable,
        }
    }
    #[doc = "Use HFXT as the HFCLK source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Useexthfclk::Disable
    }
    #[doc = "Use the HFCLK_IN digital clock input as the HFCLK source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Useexthfclk::Enable
    }
}
#[doc = "Field `USEEXTHFCLK` writer - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
pub type UseexthfclkW<'a, REG> = crate::BitWriter<'a, REG, Useexthfclk>;
impl<'a, REG> UseexthfclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use HFXT as the HFCLK source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Useexthfclk::Disable)
    }
    #[doc = "Use the HFCLK_IN digital clock input as the HFCLK source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Useexthfclk::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
    #[inline(always)]
    pub fn hfxten(&self) -> HfxtenR {
        HfxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
    #[inline(always)]
    pub fn useexthfclk(&self) -> UseexthfclkR {
        UseexthfclkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
    #[inline(always)]
    pub fn hfxten(&mut self) -> HfxtenW<'_, SysctlHsclkenSpec> {
        HfxtenW::new(self, 0)
    }
    #[doc = "Bit 16 - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
    #[inline(always)]
    pub fn useexthfclk(&mut self) -> UseexthfclkW<'_, SysctlHsclkenSpec> {
        UseexthfclkW::new(self, 16)
    }
}
#[doc = "High-speed clock (HSCLK) source enable/disable\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_hsclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_hsclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlHsclkenSpec;
impl crate::RegisterSpec for SysctlHsclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_hsclken::R`](R) reader structure"]
impl crate::Readable for SysctlHsclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_hsclken::W`](W) writer structure"]
impl crate::Writable for SysctlHsclkenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_HSCLKEN to value 0"]
impl crate::Resettable for SysctlHsclkenSpec {}
