#[doc = "Register `LFSS_DBGCTL` reader"]
pub type R = crate::R<LfssDbgctlSpec>;
#[doc = "Register `LFSS_DBGCTL` writer"]
pub type W = crate::W<LfssDbgctlSpec>;
#[doc = "Debug Run.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgrun {
    #[doc = "0: Counter is halted if CPU is in debug state."]
    Halt = 0,
    #[doc = "1: Continue to operate normally ignoring the debug state of the CPU."]
    Run = 1,
}
impl From<Dbgrun> for bool {
    #[inline(always)]
    fn from(variant: Dbgrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGRUN` reader - Debug Run."]
pub type DbgrunR = crate::BitReader<Dbgrun>;
impl DbgrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgrun {
        match self.bits {
            false => Dbgrun::Halt,
            true => Dbgrun::Run,
        }
    }
    #[doc = "Counter is halted if CPU is in debug state."]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Dbgrun::Halt
    }
    #[doc = "Continue to operate normally ignoring the debug state of the CPU."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Dbgrun::Run
    }
}
#[doc = "Field `DBGRUN` writer - Debug Run."]
pub type DbgrunW<'a, REG> = crate::BitWriter<'a, REG, Dbgrun>;
impl<'a, REG> DbgrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is halted if CPU is in debug state."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgrun::Halt)
    }
    #[doc = "Continue to operate normally ignoring the debug state of the CPU."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgrun::Run)
    }
}
#[doc = "Debug Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgint {
    #[doc = "0: Interrupts of the module will not be captured anymore if CPU is in debug state. Which means no update to the RTCRIS, RTCMISC and RTCIIDX register."]
    Off = 0,
    #[doc = "1: Interrupts are enabled in debug mode. Interrupt requests are signaled to the interrupt controller. If the flags are required by software (polling mode) the DGBINT bit need to be set to 1."]
    On = 1,
}
impl From<Dbgint> for bool {
    #[inline(always)]
    fn from(variant: Dbgint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGINT` reader - Debug Interrupt Enable."]
pub type DbgintR = crate::BitReader<Dbgint>;
impl DbgintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgint {
        match self.bits {
            false => Dbgint::Off,
            true => Dbgint::On,
        }
    }
    #[doc = "Interrupts of the module will not be captured anymore if CPU is in debug state. Which means no update to the RTCRIS, RTCMISC and RTCIIDX register."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Dbgint::Off
    }
    #[doc = "Interrupts are enabled in debug mode. Interrupt requests are signaled to the interrupt controller. If the flags are required by software (polling mode) the DGBINT bit need to be set to 1."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Dbgint::On
    }
}
#[doc = "Field `DBGINT` writer - Debug Interrupt Enable."]
pub type DbgintW<'a, REG> = crate::BitWriter<'a, REG, Dbgint>;
impl<'a, REG> DbgintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts of the module will not be captured anymore if CPU is in debug state. Which means no update to the RTCRIS, RTCMISC and RTCIIDX register."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgint::Off)
    }
    #[doc = "Interrupts are enabled in debug mode. Interrupt requests are signaled to the interrupt controller. If the flags are required by software (polling mode) the DGBINT bit need to be set to 1."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgint::On)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Run."]
    #[inline(always)]
    pub fn dbgrun(&self) -> DbgrunR {
        DbgrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Interrupt Enable."]
    #[inline(always)]
    pub fn dbgint(&self) -> DbgintR {
        DbgintR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Run."]
    #[inline(always)]
    pub fn dbgrun(&mut self) -> DbgrunW<'_, LfssDbgctlSpec> {
        DbgrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Interrupt Enable."]
    #[inline(always)]
    pub fn dbgint(&mut self) -> DbgintW<'_, LfssDbgctlSpec> {
        DbgintW::new(self, 1)
    }
}
#[doc = "RTC Module Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_dbgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_dbgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssDbgctlSpec;
impl crate::RegisterSpec for LfssDbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_dbgctl::R`](R) reader structure"]
impl crate::Readable for LfssDbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_dbgctl::W`](W) writer structure"]
impl crate::Writable for LfssDbgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_DBGCTL to value 0"]
impl crate::Resettable for LfssDbgctlSpec {}
