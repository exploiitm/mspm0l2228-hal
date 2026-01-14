#[doc = "Register `LFSS_WDTSTAT` reader"]
pub type R = crate::R<LfssWdtstatSpec>;
#[doc = "Watchdog running status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    #[doc = "0: Watchdog counter stopped."]
    Stop = 0,
    #[doc = "1: Watchdog running."]
    Run = 1,
}
impl From<Run> for bool {
    #[inline(always)]
    fn from(variant: Run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - Watchdog running status flag."]
pub type RunR = crate::BitReader<Run>;
impl RunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run {
        match self.bits {
            false => Run::Stop,
            true => Run::Run,
        }
    }
    #[doc = "Watchdog counter stopped."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Run::Stop
    }
    #[doc = "Watchdog running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Run::Run
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog running status flag."]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssWdtstatSpec;
impl crate::RegisterSpec for LfssWdtstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_wdtstat::R`](R) reader structure"]
impl crate::Readable for LfssWdtstatSpec {}
#[doc = "`reset()` method sets LFSS_WDTSTAT to value 0"]
impl crate::Resettable for LfssWdtstatSpec {}
