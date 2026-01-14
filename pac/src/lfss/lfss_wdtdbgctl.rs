#[doc = "Register `LFSS_WDTDBGCTL` reader"]
pub type R = crate::R<LfssWdtdbgctlSpec>;
#[doc = "Register `LFSS_WDTDBGCTL` writer"]
pub type W = crate::W<LfssWdtdbgctlSpec>;
#[doc = "Free run control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Free {
    #[doc = "0: The WDT freezes functionality while the CPU is Halted during debug and resumes when the CPU is active."]
    Stop = 0,
    #[doc = "1: The WDT ignores the state of the CPU Halted debug state."]
    Run = 1,
}
impl From<Free> for bool {
    #[inline(always)]
    fn from(variant: Free) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREE` reader - Free run control"]
pub type FreeR = crate::BitReader<Free>;
impl FreeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Free {
        match self.bits {
            false => Free::Stop,
            true => Free::Run,
        }
    }
    #[doc = "The WDT freezes functionality while the CPU is Halted during debug and resumes when the CPU is active."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Free::Stop
    }
    #[doc = "The WDT ignores the state of the CPU Halted debug state."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Free::Run
    }
}
#[doc = "Field `FREE` writer - Free run control"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG, Free>;
impl<'a, REG> FreeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WDT freezes functionality while the CPU is Halted during debug and resumes when the CPU is active."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Free::Stop)
    }
    #[doc = "The WDT ignores the state of the CPU Halted debug state."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Free::Run)
    }
}
impl R {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&mut self) -> FreeW<'_, LfssWdtdbgctlSpec> {
        FreeW::new(self, 0)
    }
}
#[doc = "Watchdog Timer Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtdbgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtdbgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssWdtdbgctlSpec;
impl crate::RegisterSpec for LfssWdtdbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_wdtdbgctl::R`](R) reader structure"]
impl crate::Readable for LfssWdtdbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_wdtdbgctl::W`](W) writer structure"]
impl crate::Writable for LfssWdtdbgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_WDTDBGCTL to value 0"]
impl crate::Resettable for LfssWdtdbgctlSpec {}
