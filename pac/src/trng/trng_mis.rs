#[doc = "Register `TRNG_MIS` reader"]
pub type R = crate::R<TrngMisSpec>;
#[doc = "Masked interrupt result for HEALTH_FAIL. Indicates to the CPU that any of the health tests have failed for the latest 1024-bit window.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqHealthFail {
    #[doc = "0: IRQ_CAPTURED_READY did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: IRQ_CAPTURED_READY requests an interrupt service routine"]
    Set = 1,
}
impl From<IrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: IrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_HEALTH_FAIL` reader - Masked interrupt result for HEALTH_FAIL. Indicates to the CPU that any of the health tests have failed for the latest 1024-bit window."]
pub type IrqHealthFailR = crate::BitReader<IrqHealthFail>;
impl IrqHealthFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqHealthFail {
        match self.bits {
            false => IrqHealthFail::Clr,
            true => IrqHealthFail::Set,
        }
    }
    #[doc = "IRQ_CAPTURED_READY did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqHealthFail::Clr
    }
    #[doc = "IRQ_CAPTURED_READY requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqHealthFail::Set
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdFail {
    #[doc = "0: IRQ_CMD_FAIL did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: IRQ_CMD_FAIL requests an interrupt service routine"]
    Set = 1,
}
impl From<IrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: IrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CMD_FAIL` reader - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type IrqCmdFailR = crate::BitReader<IrqCmdFail>;
impl IrqCmdFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqCmdFail {
        match self.bits {
            false => IrqCmdFail::Clr,
            true => IrqCmdFail::Set,
        }
    }
    #[doc = "IRQ_CMD_FAIL did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqCmdFail::Clr
    }
    #[doc = "IRQ_CMD_FAIL requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqCmdFail::Set
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdDone {
    #[doc = "0: IRQ_CAPTURED_READY did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: IRQ_CMD_DONE requests an interrupt service routine"]
    Set = 1,
}
impl From<IrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: IrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CMD_DONE` reader - Masked interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
pub type IrqCmdDoneR = crate::BitReader<IrqCmdDone>;
impl IrqCmdDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqCmdDone {
        match self.bits {
            false => IrqCmdDone::Clr,
            true => IrqCmdDone::Set,
        }
    }
    #[doc = "IRQ_CAPTURED_READY did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqCmdDone::Clr
    }
    #[doc = "IRQ_CMD_DONE requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqCmdDone::Set
    }
}
#[doc = "Masked interrupt result for CAPTURED_READY. Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCapturedRdy {
    #[doc = "0: IRQ_CAPTURED_READY did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: IRQ_CAPTURED_READY requests an interrupt service routine"]
    Set = 1,
}
impl From<IrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: IrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CAPTURED_RDY` reader - Masked interrupt result for CAPTURED_READY. Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
pub type IrqCapturedRdyR = crate::BitReader<IrqCapturedRdy>;
impl IrqCapturedRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqCapturedRdy {
        match self.bits {
            false => IrqCapturedRdy::Clr,
            true => IrqCapturedRdy::Set,
        }
    }
    #[doc = "IRQ_CAPTURED_READY did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqCapturedRdy::Clr
    }
    #[doc = "IRQ_CAPTURED_READY requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqCapturedRdy::Set
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt result for HEALTH_FAIL. Indicates to the CPU that any of the health tests have failed for the latest 1024-bit window."]
    #[inline(always)]
    pub fn irq_health_fail(&self) -> IrqHealthFailR {
        IrqHealthFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn irq_cmd_fail(&self) -> IrqCmdFailR {
        IrqCmdFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
    #[inline(always)]
    pub fn irq_cmd_done(&self) -> IrqCmdDoneR {
        IrqCmdDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt result for CAPTURED_READY. Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
    #[inline(always)]
    pub fn irq_captured_rdy(&self) -> IrqCapturedRdyR {
        IrqCapturedRdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngMisSpec;
impl crate::RegisterSpec for TrngMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_mis::R`](R) reader structure"]
impl crate::Readable for TrngMisSpec {}
#[doc = "`reset()` method sets TRNG_MIS to value 0"]
impl crate::Resettable for TrngMisSpec {}
