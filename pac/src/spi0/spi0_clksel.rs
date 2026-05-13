#[doc = "Register `SPI0_CLKSEL` reader"]
pub type R = crate::R<Spi0ClkselSpec>;
#[doc = "Register `SPI0_CLKSEL` writer"]
pub type W = crate::W<Spi0ClkselSpec>;
#[doc = "Selects LFCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfclkSel {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
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
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LfclkSel::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LfclkSel::Enable
    }
}
#[doc = "Field `LFCLK_SEL` writer - Selects LFCLK as clock source if enabled"]
pub type LfclkSelW<'a, REG> = crate::BitWriter<'a, REG, LfclkSel>;
impl<'a, REG> LfclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkSel::Disable)
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkSel::Enable)
    }
}
#[doc = "Selects MFCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfclkSel {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
    Enable = 1,
}
impl From<MfclkSel> for bool {
    #[inline(always)]
    fn from(variant: MfclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFCLK_SEL` reader - Selects MFCLK as clock source if enabled"]
pub type MfclkSelR = crate::BitReader<MfclkSel>;
impl MfclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MfclkSel {
        match self.bits {
            false => MfclkSel::Disable,
            true => MfclkSel::Enable,
        }
    }
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MfclkSel::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MfclkSel::Enable
    }
}
#[doc = "Field `MFCLK_SEL` writer - Selects MFCLK as clock source if enabled"]
pub type MfclkSelW<'a, REG> = crate::BitWriter<'a, REG, MfclkSel>;
impl<'a, REG> MfclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MfclkSel::Disable)
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MfclkSel::Enable)
    }
}
#[doc = "Selects SYSCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysclkSel {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
    Enable = 1,
}
impl From<SysclkSel> for bool {
    #[inline(always)]
    fn from(variant: SysclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCLK_SEL` reader - Selects SYSCLK as clock source if enabled"]
pub type SysclkSelR = crate::BitReader<SysclkSel>;
impl SysclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysclkSel {
        match self.bits {
            false => SysclkSel::Disable,
            true => SysclkSel::Enable,
        }
    }
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SysclkSel::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SysclkSel::Enable
    }
}
#[doc = "Field `SYSCLK_SEL` writer - Selects SYSCLK as clock source if enabled"]
pub type SysclkSelW<'a, REG> = crate::BitWriter<'a, REG, SysclkSel>;
impl<'a, REG> SysclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Disable)
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Enable)
    }
}
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
    #[doc = "Bit 3 - Selects SYSCLK as clock source if enabled"]
    #[inline(always)]
    pub fn sysclk_sel(&self) -> SysclkSelR {
        SysclkSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn lfclk_sel(&mut self) -> LfclkSelW<'_, Spi0ClkselSpec> {
        LfclkSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Selects MFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn mfclk_sel(&mut self) -> MfclkSelW<'_, Spi0ClkselSpec> {
        MfclkSelW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects SYSCLK as clock source if enabled"]
    #[inline(always)]
    pub fn sysclk_sel(&mut self) -> SysclkSelW<'_, Spi0ClkselSpec> {
        SysclkSelW::new(self, 3)
    }
}
#[doc = "Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0ClkselSpec;
impl crate::RegisterSpec for Spi0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_clksel::R`](R) reader structure"]
impl crate::Readable for Spi0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`spi0_clksel::W`](W) writer structure"]
impl crate::Writable for Spi0ClkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0_CLKSEL to value 0"]
impl crate::Resettable for Spi0ClkselSpec {}
