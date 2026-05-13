#[doc = "Register `TRNG_ICLR` writer"]
pub type W = crate::W<TrngIclrSpec>;
#[doc = "Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqHealthFail {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to CAPTURED_READY is cleared"]
    Clr = 1,
}
impl From<IrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: IrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_HEALTH_FAIL` writer - Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
pub type IrqHealthFailW<'a, REG> = crate::BitWriter<'a, REG, IrqHealthFail>;
impl<'a, REG> IrqHealthFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IrqHealthFail::NoEffect)
    }
    #[doc = "RIS bit corresponding to CAPTURED_READY is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(IrqHealthFail::Clr)
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdFail {
    #[doc = "0: Writing a 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to CMD_FAIL is cleared"]
    Clr = 1,
}
impl From<IrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: IrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CMD_FAIL` writer - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type IrqCmdFailW<'a, REG> = crate::BitWriter<'a, REG, IrqCmdFail>;
impl<'a, REG> IrqCmdFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdFail::NoEffect)
    }
    #[doc = "RIS bit corresponding to CMD_FAIL is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdFail::Clr)
    }
}
#[doc = "Write to turn off CMD_DONE IRQ. Indicates that the last issued TRNG command has finished.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdDone {
    #[doc = "0: Writing a 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to CMD_DONE is cleared"]
    Clr = 1,
}
impl From<IrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: IrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CMD_DONE` writer - Write to turn off CMD_DONE IRQ. Indicates that the last issued TRNG command has finished."]
pub type IrqCmdDoneW<'a, REG> = crate::BitWriter<'a, REG, IrqCmdDone>;
impl<'a, REG> IrqCmdDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdDone::NoEffect)
    }
    #[doc = "RIS bit corresponding to CMD_DONE is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdDone::Clr)
    }
}
#[doc = "Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCapturedRdy {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to CAPTURED_READY is cleared"]
    Clr = 1,
}
impl From<IrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: IrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CAPTURED_RDY` writer - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
pub type IrqCapturedRdyW<'a, REG> = crate::BitWriter<'a, REG, IrqCapturedRdy>;
impl<'a, REG> IrqCapturedRdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCapturedRdy::NoEffect)
    }
    #[doc = "RIS bit corresponding to CAPTURED_READY is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCapturedRdy::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
    #[inline(always)]
    pub fn irq_health_fail(&mut self) -> IrqHealthFailW<'_, TrngIclrSpec> {
        IrqHealthFailW::new(self, 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn irq_cmd_fail(&mut self) -> IrqCmdFailW<'_, TrngIclrSpec> {
        IrqCmdFailW::new(self, 1)
    }
    #[doc = "Bit 2 - Write to turn off CMD_DONE IRQ. Indicates that the last issued TRNG command has finished."]
    #[inline(always)]
    pub fn irq_cmd_done(&mut self) -> IrqCmdDoneW<'_, TrngIclrSpec> {
        IrqCmdDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
    #[inline(always)]
    pub fn irq_captured_rdy(&mut self) -> IrqCapturedRdyW<'_, TrngIclrSpec> {
        IrqCapturedRdyW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngIclrSpec;
impl crate::RegisterSpec for TrngIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trng_iclr::W`](W) writer structure"]
impl crate::Writable for TrngIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRNG_ICLR to value 0"]
impl crate::Resettable for TrngIclrSpec {}
