#[doc = "Register `LFSS_CLKCTL` reader"]
pub type R = crate::R<LfssClkctlSpec>;
#[doc = "Register `LFSS_CLKCTL` writer"]
pub type W = crate::W<LfssClkctlSpec>;
#[doc = "This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modclken {
    #[doc = "0: 32kHz clock is not supplied to the RTC."]
    Disable = 0,
    #[doc = "1: 32kHz clock is supplied to the RTC."]
    Enable = 1,
}
impl From<Modclken> for bool {
    #[inline(always)]
    fn from(variant: Modclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODCLKEN` reader - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
pub type ModclkenR = crate::BitReader<Modclken>;
impl ModclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modclken {
        match self.bits {
            false => Modclken::Disable,
            true => Modclken::Enable,
        }
    }
    #[doc = "32kHz clock is not supplied to the RTC."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Modclken::Disable
    }
    #[doc = "32kHz clock is supplied to the RTC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Modclken::Enable
    }
}
#[doc = "Field `MODCLKEN` writer - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
pub type ModclkenW<'a, REG> = crate::BitWriter<'a, REG, Modclken>;
impl<'a, REG> ModclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32kHz clock is not supplied to the RTC."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Modclken::Disable)
    }
    #[doc = "32kHz clock is supplied to the RTC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Modclken::Enable)
    }
}
impl R {
    #[doc = "Bit 31 - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
    #[inline(always)]
    pub fn modclken(&self) -> ModclkenR {
        ModclkenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
    #[inline(always)]
    pub fn modclken(&mut self) -> ModclkenW<'_, LfssClkctlSpec> {
        ModclkenW::new(self, 31)
    }
}
#[doc = "RTC Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssClkctlSpec;
impl crate::RegisterSpec for LfssClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_clkctl::R`](R) reader structure"]
impl crate::Readable for LfssClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_clkctl::W`](W) writer structure"]
impl crate::Writable for LfssClkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_CLKCTL to value 0"]
impl crate::Resettable for LfssClkctlSpec {}
