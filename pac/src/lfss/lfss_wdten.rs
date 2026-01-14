#[doc = "Register `LFSS_WDTEN` reader"]
pub type R = crate::R<LfssWdtenSpec>;
#[doc = "Register `LFSS_WDTEN` writer"]
pub type W = crate::W<LfssWdtenSpec>;
#[doc = "Enable bit for the WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable WDT"]
    Clr = 0,
    #[doc = "1: Enable WDT"]
    Set = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable bit for the WDT."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Clr,
            true => Enable::Set,
        }
    }
    #[doc = "Disable WDT"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Enable::Clr
    }
    #[doc = "Enable WDT"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enable::Set
    }
}
#[doc = "Field `ENABLE` writer - Enable bit for the WDT."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable WDT"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Clr)
    }
    #[doc = "Enable WDT"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bit for the WDT."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit for the WDT."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, LfssWdtenSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Watchdog Timer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssWdtenSpec;
impl crate::RegisterSpec for LfssWdtenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_wdten::R`](R) reader structure"]
impl crate::Readable for LfssWdtenSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_wdten::W`](W) writer structure"]
impl crate::Writable for LfssWdtenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_WDTEN to value 0"]
impl crate::Resettable for LfssWdtenSpec {}
