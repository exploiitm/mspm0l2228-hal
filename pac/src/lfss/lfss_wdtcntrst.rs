#[doc = "Register `LFSS_WDTCNTRST` writer"]
pub type W = crate::W<LfssWdtcntrstSpec>;
#[doc = "Writing 03A7h to this register restarts the WDT Counter. Writing any other value causes a POR level reset. Read as 0x0h.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Restart {
    #[doc = "935: VALUE to restart the WDT counter"]
    Value = 935,
}
impl From<Restart> for u32 {
    #[inline(always)]
    fn from(variant: Restart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Restart {
    type Ux = u32;
}
impl crate::IsEnum for Restart {}
#[doc = "Field `RESTART` writer - Writing 03A7h to this register restarts the WDT Counter. Writing any other value causes a POR level reset. Read as 0x0h."]
pub type RestartW<'a, REG> = crate::FieldWriter<'a, REG, 32, Restart>;
impl<'a, REG> RestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "VALUE to restart the WDT counter"]
    #[inline(always)]
    pub fn value(self) -> &'a mut crate::W<REG> {
        self.variant(Restart::Value)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing 03A7h to this register restarts the WDT Counter. Writing any other value causes a POR level reset. Read as 0x0h."]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, LfssWdtcntrstSpec> {
        RestartW::new(self, 0)
    }
}
#[doc = "Watchdog Timer Counter Reset Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtcntrst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssWdtcntrstSpec;
impl crate::RegisterSpec for LfssWdtcntrstSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lfss_wdtcntrst::W`](W) writer structure"]
impl crate::Writable for LfssWdtcntrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_WDTCNTRST to value 0"]
impl crate::Resettable for LfssWdtcntrstSpec {}
