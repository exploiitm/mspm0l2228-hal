#[doc = "Register `GPIOC_CLKOVR` reader"]
pub type R = crate::R<GpiocClkovrSpec>;
#[doc = "Register `GPIOC_CLKOVR` writer"]
pub type W = crate::W<GpiocClkovrSpec>;
#[doc = "Unlocks the functionality of \\[RUN_STOP\\] to override the automatic peripheral clock request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Override {
    #[doc = "0: Override disabled"]
    Disabled = 0,
    #[doc = "1: Override enabled"]
    Enabled = 1,
}
impl From<Override> for bool {
    #[inline(always)]
    fn from(variant: Override) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRIDE` reader - Unlocks the functionality of \\[RUN_STOP\\] to override the automatic peripheral clock request"]
pub type OverrideR = crate::BitReader<Override>;
impl OverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Override {
        match self.bits {
            false => Override::Disabled,
            true => Override::Enabled,
        }
    }
    #[doc = "Override disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Override::Disabled
    }
    #[doc = "Override enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Override::Enabled
    }
}
#[doc = "Field `OVERRIDE` writer - Unlocks the functionality of \\[RUN_STOP\\] to override the automatic peripheral clock request"]
pub type OverrideW<'a, REG> = crate::BitWriter<'a, REG, Override>;
impl<'a, REG> OverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Override::Disabled)
    }
    #[doc = "Override enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Override::Enabled)
    }
}
#[doc = "If \\[OVERRIDE\\] is enabled, this register is used to manually control the peripheral's clock request to the system\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStop {
    #[doc = "0: Run/ungate functional clock"]
    Run = 0,
    #[doc = "1: Stop/gate functional clock"]
    Stop = 1,
}
impl From<RunStop> for bool {
    #[inline(always)]
    fn from(variant: RunStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN_STOP` reader - If \\[OVERRIDE\\] is enabled, this register is used to manually control the peripheral's clock request to the system"]
pub type RunStopR = crate::BitReader<RunStop>;
impl RunStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RunStop {
        match self.bits {
            false => RunStop::Run,
            true => RunStop::Stop,
        }
    }
    #[doc = "Run/ungate functional clock"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RunStop::Run
    }
    #[doc = "Stop/gate functional clock"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RunStop::Stop
    }
}
#[doc = "Field `RUN_STOP` writer - If \\[OVERRIDE\\] is enabled, this register is used to manually control the peripheral's clock request to the system"]
pub type RunStopW<'a, REG> = crate::BitWriter<'a, REG, RunStop>;
impl<'a, REG> RunStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Run/ungate functional clock"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(RunStop::Run)
    }
    #[doc = "Stop/gate functional clock"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(RunStop::Stop)
    }
}
impl R {
    #[doc = "Bit 0 - Unlocks the functionality of \\[RUN_STOP\\] to override the automatic peripheral clock request"]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If \\[OVERRIDE\\] is enabled, this register is used to manually control the peripheral's clock request to the system"]
    #[inline(always)]
    pub fn run_stop(&self) -> RunStopR {
        RunStopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Unlocks the functionality of \\[RUN_STOP\\] to override the automatic peripheral clock request"]
    #[inline(always)]
    pub fn override_(&mut self) -> OverrideW<'_, GpiocClkovrSpec> {
        OverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - If \\[OVERRIDE\\] is enabled, this register is used to manually control the peripheral's clock request to the system"]
    #[inline(always)]
    pub fn run_stop(&mut self) -> RunStopW<'_, GpiocClkovrSpec> {
        RunStopW::new(self, 1)
    }
}
#[doc = "Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_clkovr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_clkovr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocClkovrSpec;
impl crate::RegisterSpec for GpiocClkovrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_clkovr::R`](R) reader structure"]
impl crate::Readable for GpiocClkovrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioc_clkovr::W`](W) writer structure"]
impl crate::Writable for GpiocClkovrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_CLKOVR to value 0"]
impl crate::Resettable for GpiocClkovrSpec {}
