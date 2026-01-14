#[doc = "Register `TRNG_IIDX` reader"]
pub type R = crate::R<TrngIidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No bit is set means there is no pending interrupt request"]
    NoIntr = 0,
    #[doc = "1: Indicates that a health test has failed. The TRNG is in an error state until the interrupt is cleared."]
    IrqHealthFail = 1,
    #[doc = "2: Indicates that the just issued command was rejected and is not being performed."]
    IrqCmdFail = 2,
    #[doc = "3: Indicates that the current command/mode is done. This may have different meanings based on the mode: OFF --> Power has been turned off PWRUP_DIG --> Digital powerup tests are done PWRUP_ANA --> Analog powerup tests are done NORM_FUNC --> No IRQ, since mode runs indefinitely until a new command is issued"]
    IrqCmdDone = 3,
    #[doc = "4: Indicates that the captured word buffer is ready to be copied to memory"]
    IrqCapturedRdy = 4,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::IrqHealthFail),
            2 => Some(Stat::IrqCmdFail),
            3 => Some(Stat::IrqCmdDone),
            4 => Some(Stat::IrqCapturedRdy),
            _ => None,
        }
    }
    #[doc = "No bit is set means there is no pending interrupt request"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Indicates that a health test has failed. The TRNG is in an error state until the interrupt is cleared."]
    #[inline(always)]
    pub fn is_irq_health_fail(&self) -> bool {
        *self == Stat::IrqHealthFail
    }
    #[doc = "Indicates that the just issued command was rejected and is not being performed."]
    #[inline(always)]
    pub fn is_irq_cmd_fail(&self) -> bool {
        *self == Stat::IrqCmdFail
    }
    #[doc = "Indicates that the current command/mode is done. This may have different meanings based on the mode: OFF --> Power has been turned off PWRUP_DIG --> Digital powerup tests are done PWRUP_ANA --> Analog powerup tests are done NORM_FUNC --> No IRQ, since mode runs indefinitely until a new command is issued"]
    #[inline(always)]
    pub fn is_irq_cmd_done(&self) -> bool {
        *self == Stat::IrqCmdDone
    }
    #[doc = "Indicates that the captured word buffer is ready to be copied to memory"]
    #[inline(always)]
    pub fn is_irq_captured_rdy(&self) -> bool {
        *self == Stat::IrqCapturedRdy
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngIidxSpec;
impl crate::RegisterSpec for TrngIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_iidx::R`](R) reader structure"]
impl crate::Readable for TrngIidxSpec {}
#[doc = "`reset()` method sets TRNG_IIDX to value 0"]
impl crate::Resettable for TrngIidxSpec {}
