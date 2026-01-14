#[doc = "Register `VREF_CLKSEL` reader"]
pub type R = crate::R<VrefClkselSpec>;
#[doc = "Register `VREF_CLKSEL` writer"]
pub type W = crate::W<VrefClkselSpec>;
#[doc = "Field `LFCLK_SEL` reader - Selects LFCLK as clock source if enabled"]
pub type LfclkSelR = crate::BitReader;
#[doc = "Field `LFCLK_SEL` writer - Selects LFCLK as clock source if enabled"]
pub type LfclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFCLK_SEL` reader - Selects MFCLK as clock source if enabled"]
pub type MfclkSelR = crate::BitReader;
#[doc = "Field `MFCLK_SEL` writer - Selects MFCLK as clock source if enabled"]
pub type MfclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSCLK_SEL` reader - Selects BUSCLK as clock source if enabled"]
pub type BusclkSelR = crate::BitReader;
#[doc = "Field `BUSCLK_SEL` writer - Selects BUSCLK as clock source if enabled"]
pub type BusclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LfclkSelR {
        LfclkSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects MFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn mfclk_sel(&self) -> MfclkSelR {
        MfclkSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects BUSCLK as clock source if enabled"]
    #[inline(always)]
    pub fn busclk_sel(&self) -> BusclkSelR {
        BusclkSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn lfclk_sel(&mut self) -> LfclkSelW<'_, VrefClkselSpec> {
        LfclkSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Selects MFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn mfclk_sel(&mut self) -> MfclkSelW<'_, VrefClkselSpec> {
        MfclkSelW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects BUSCLK as clock source if enabled"]
    #[inline(always)]
    pub fn busclk_sel(&mut self) -> BusclkSelW<'_, VrefClkselSpec> {
        BusclkSelW::new(self, 3)
    }
}
#[doc = "Clock Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefClkselSpec;
impl crate::RegisterSpec for VrefClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_clksel::R`](R) reader structure"]
impl crate::Readable for VrefClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`vref_clksel::W`](W) writer structure"]
impl crate::Writable for VrefClkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREF_CLKSEL to value 0"]
impl crate::Resettable for VrefClkselSpec {}
