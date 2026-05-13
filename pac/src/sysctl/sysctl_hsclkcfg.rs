#[doc = "Register `SYSCTL_HSCLKCFG` reader"]
pub type R = crate::R<SysctlHsclkcfgSpec>;
#[doc = "Register `SYSCTL_HSCLKCFG` writer"]
pub type W = crate::W<SysctlHsclkcfgSpec>;
#[doc = "HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclksel {
    #[doc = "1: HSCLK is sourced from the HFCLK"]
    Hfclkclk = 1,
}
impl From<Hsclksel> for bool {
    #[inline(always)]
    fn from(variant: Hsclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKSEL` reader - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
pub type HsclkselR = crate::BitReader<Hsclksel>;
impl HsclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hsclksel> {
        match self.bits {
            true => Some(Hsclksel::Hfclkclk),
            _ => None,
        }
    }
    #[doc = "HSCLK is sourced from the HFCLK"]
    #[inline(always)]
    pub fn is_hfclkclk(&self) -> bool {
        *self == Hsclksel::Hfclkclk
    }
}
#[doc = "Field `HSCLKSEL` writer - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
pub type HsclkselW<'a, REG> = crate::BitWriter<'a, REG, Hsclksel>;
impl<'a, REG> HsclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSCLK is sourced from the HFCLK"]
    #[inline(always)]
    pub fn hfclkclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclksel::Hfclkclk)
    }
}
impl R {
    #[doc = "Bit 0 - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
    #[inline(always)]
    pub fn hsclksel(&self) -> HsclkselR {
        HsclkselR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
    #[inline(always)]
    pub fn hsclksel(&mut self) -> HsclkselW<'_, SysctlHsclkcfgSpec> {
        HsclkselW::new(self, 0)
    }
}
#[doc = "High-speed clock (HSCLK) source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_hsclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_hsclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlHsclkcfgSpec;
impl crate::RegisterSpec for SysctlHsclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_hsclkcfg::R`](R) reader structure"]
impl crate::Readable for SysctlHsclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_hsclkcfg::W`](W) writer structure"]
impl crate::Writable for SysctlHsclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_HSCLKCFG to value 0"]
impl crate::Resettable for SysctlHsclkcfgSpec {}
