#[doc = "Register `TRNG_RIS` reader"]
pub type R = crate::R<TrngRisSpec>;
#[doc = "Indicates to the CPU that any of the health tests have failed. Reading the IIDX will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqHealthFail {
    #[doc = "0: IRQ_CAPTURED_READY did not occur"]
    Clr = 0,
    #[doc = "1: IRQ_CAPTURED_READY occurred"]
    Set = 1,
}
impl From<IrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: IrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_HEALTH_FAIL` reader - Indicates to the CPU that any of the health tests have failed. Reading the IIDX will clear this interrupt."]
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
    #[doc = "IRQ_CAPTURED_READY did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqHealthFail::Clr
    }
    #[doc = "IRQ_CAPTURED_READY occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqHealthFail::Set
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdFail {
    #[doc = "0: IRQ_CMD_FAIL did not occur"]
    Clr = 0,
    #[doc = "1: IRQ_CMD_FAIL occurred"]
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
    #[doc = "IRQ_CMD_FAIL did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqCmdFail::Clr
    }
    #[doc = "IRQ_CMD_FAIL occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqCmdFail::Set
    }
}
#[doc = "Raw interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdDone {
    #[doc = "0: IRQ_CMD_DONE did not occur"]
    Clr = 0,
    #[doc = "1: IRQ_CMD_DONE occurred"]
    Set = 1,
}
impl From<IrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: IrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CMD_DONE` reader - Raw interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
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
    #[doc = "IRQ_CMD_DONE did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqCmdDone::Clr
    }
    #[doc = "IRQ_CMD_DONE occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqCmdDone::Set
    }
}
#[doc = "Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCapturedRdy {
    #[doc = "0: IRQ_CAPTURED_READY did not occur"]
    Clr = 0,
    #[doc = "1: IRQ_CAPTURED_READY occurred"]
    Set = 1,
}
impl From<IrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: IrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CAPTURED_RDY` reader - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
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
    #[doc = "IRQ_CAPTURED_READY did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IrqCapturedRdy::Clr
    }
    #[doc = "IRQ_CAPTURED_READY occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IrqCapturedRdy::Set
    }
}
impl R {
    #[doc = "Bit 0 - Indicates to the CPU that any of the health tests have failed. Reading the IIDX will clear this interrupt."]
    #[inline(always)]
    pub fn irq_health_fail(&self) -> IrqHealthFailR {
        IrqHealthFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn irq_cmd_fail(&self) -> IrqCmdFailR {
        IrqCmdFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
    #[inline(always)]
    pub fn irq_cmd_done(&self) -> IrqCmdDoneR {
        IrqCmdDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
    #[inline(always)]
    pub fn irq_captured_rdy(&self) -> IrqCapturedRdyR {
        IrqCapturedRdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngRisSpec;
impl crate::RegisterSpec for TrngRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_ris::R`](R) reader structure"]
impl crate::Readable for TrngRisSpec {}
#[doc = "`reset()` method sets TRNG_RIS to value 0"]
impl crate::Resettable for TrngRisSpec {}
