#[doc = "Register `WWDT0_WWDTSTAT` reader"]
pub type R = crate::R<Wwdt0WwdtstatSpec>;
#[doc = "Watchdog running status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    #[doc = "0: Watchdog counter stopped."]
    Off = 0,
    #[doc = "1: Watchdog running."]
    On = 1,
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
            false => Run::Off,
            true => Run::On,
        }
    }
    #[doc = "Watchdog counter stopped."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Run::Off
    }
    #[doc = "Watchdog running."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Run::On
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog running status flag."]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new((self.bits & 1) != 0)
    }
}
#[doc = "Window Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0WwdtstatSpec;
impl crate::RegisterSpec for Wwdt0WwdtstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_wwdtstat::R`](R) reader structure"]
impl crate::Readable for Wwdt0WwdtstatSpec {}
#[doc = "`reset()` method sets WWDT0_WWDTSTAT to value 0"]
impl crate::Resettable for Wwdt0WwdtstatSpec {}
