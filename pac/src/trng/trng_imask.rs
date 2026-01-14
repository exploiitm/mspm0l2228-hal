#[doc = "Register `TRNG_IMASK` reader"]
pub type R = crate::R<TrngImaskSpec>;
#[doc = "Register `TRNG_IMASK` writer"]
pub type W = crate::W<TrngImaskSpec>;
#[doc = "Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqHealthFail {
    #[doc = "0: Interrupt is masked out"]
    Disabled = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Enabled = 1,
}
impl From<IrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: IrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_HEALTH_FAIL` reader - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
pub type IrqHealthFailR = crate::BitReader<IrqHealthFail>;
impl IrqHealthFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqHealthFail {
        match self.bits {
            false => IrqHealthFail::Disabled,
            true => IrqHealthFail::Enabled,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IrqHealthFail::Disabled
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IrqHealthFail::Enabled
    }
}
#[doc = "Field `IRQ_HEALTH_FAIL` writer - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
pub type IrqHealthFailW<'a, REG> = crate::BitWriter<'a, REG, IrqHealthFail>;
impl<'a, REG> IrqHealthFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqHealthFail::Disabled)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqHealthFail::Enabled)
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdFail {
    #[doc = "0: Interrupt is masked out"]
    Disabled = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Enabled = 1,
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
            false => IrqCmdFail::Disabled,
            true => IrqCmdFail::Enabled,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IrqCmdFail::Disabled
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IrqCmdFail::Enabled
    }
}
#[doc = "Field `IRQ_CMD_FAIL` writer - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type IrqCmdFailW<'a, REG> = crate::BitWriter<'a, REG, IrqCmdFail>;
impl<'a, REG> IrqCmdFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdFail::Disabled)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdFail::Enabled)
    }
}
#[doc = "Mask for IRQ_CMD_DONE. Indicates that a command has finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCmdDone {
    #[doc = "0: Interrupt is masked out"]
    Disabled = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Enabled = 1,
}
impl From<IrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: IrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CMD_DONE` reader - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
pub type IrqCmdDoneR = crate::BitReader<IrqCmdDone>;
impl IrqCmdDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqCmdDone {
        match self.bits {
            false => IrqCmdDone::Disabled,
            true => IrqCmdDone::Enabled,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IrqCmdDone::Disabled
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IrqCmdDone::Enabled
    }
}
#[doc = "Field `IRQ_CMD_DONE` writer - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
pub type IrqCmdDoneW<'a, REG> = crate::BitWriter<'a, REG, IrqCmdDone>;
impl<'a, REG> IrqCmdDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdDone::Disabled)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCmdDone::Enabled)
    }
}
#[doc = "Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqCapturedRdy {
    #[doc = "0: Interrupt is masked out"]
    Disabled = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Enabled = 1,
}
impl From<IrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: IrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_CAPTURED_RDY` reader - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
pub type IrqCapturedRdyR = crate::BitReader<IrqCapturedRdy>;
impl IrqCapturedRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqCapturedRdy {
        match self.bits {
            false => IrqCapturedRdy::Disabled,
            true => IrqCapturedRdy::Enabled,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IrqCapturedRdy::Disabled
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IrqCapturedRdy::Enabled
    }
}
#[doc = "Field `IRQ_CAPTURED_RDY` writer - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
pub type IrqCapturedRdyW<'a, REG> = crate::BitWriter<'a, REG, IrqCapturedRdy>;
impl<'a, REG> IrqCapturedRdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCapturedRdy::Disabled)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IrqCapturedRdy::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
    #[inline(always)]
    pub fn irq_health_fail(&self) -> IrqHealthFailR {
        IrqHealthFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn irq_cmd_fail(&self) -> IrqCmdFailR {
        IrqCmdFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
    #[inline(always)]
    pub fn irq_cmd_done(&self) -> IrqCmdDoneR {
        IrqCmdDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
    #[inline(always)]
    pub fn irq_captured_rdy(&self) -> IrqCapturedRdyR {
        IrqCapturedRdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
    #[inline(always)]
    pub fn irq_health_fail(&mut self) -> IrqHealthFailW<'_, TrngImaskSpec> {
        IrqHealthFailW::new(self, 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn irq_cmd_fail(&mut self) -> IrqCmdFailW<'_, TrngImaskSpec> {
        IrqCmdFailW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
    #[inline(always)]
    pub fn irq_cmd_done(&mut self) -> IrqCmdDoneW<'_, TrngImaskSpec> {
        IrqCmdDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
    #[inline(always)]
    pub fn irq_captured_rdy(&mut self) -> IrqCapturedRdyW<'_, TrngImaskSpec> {
        IrqCapturedRdyW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngImaskSpec;
impl crate::RegisterSpec for TrngImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_imask::R`](R) reader structure"]
impl crate::Readable for TrngImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`trng_imask::W`](W) writer structure"]
impl crate::Writable for TrngImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRNG_IMASK to value 0"]
impl crate::Resettable for TrngImaskSpec {}
