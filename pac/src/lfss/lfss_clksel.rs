#[doc = "Register `LFSS_CLKSEL` reader"]
pub type R = crate::R<LfssClkselSpec>;
#[doc = "Selects LFCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfclkSel {
    #[doc = "0: LFCLK is disabled"]
    Disable = 0,
    #[doc = "1: LFCLK is enabled"]
    Enable = 1,
}
impl From<LfclkSel> for bool {
    #[inline(always)]
    fn from(variant: LfclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLK_SEL` reader - Selects LFCLK as clock source if enabled"]
pub type LfclkSelR = crate::BitReader<LfclkSel>;
impl LfclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfclkSel {
        match self.bits {
            false => LfclkSel::Disable,
            true => LfclkSel::Enable,
        }
    }
    #[doc = "LFCLK is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LfclkSel::Disable
    }
    #[doc = "LFCLK is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LfclkSel::Enable
    }
}
impl R {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LfclkSelR {
        LfclkSelR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_clksel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssClkselSpec;
impl crate::RegisterSpec for LfssClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_clksel::R`](R) reader structure"]
impl crate::Readable for LfssClkselSpec {}
#[doc = "`reset()` method sets LFSS_CLKSEL to value 0"]
impl crate::Resettable for LfssClkselSpec {}
