#[doc = "Register `FLASHCTL_CMDEXEC` reader"]
pub type R = crate::R<FlashctlCmdexecSpec>;
#[doc = "Register `FLASHCTL_CMDEXEC` writer"]
pub type W = crate::W<FlashctlCmdexecSpec>;
#[doc = "Command Execute value Initiates execution of the command specified in the CMDTYPE register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "0: Command will not execute or is not executing in flash wrapper"]
    Noexecute = 0,
    #[doc = "1: Command will execute or is executing in flash wrapper"]
    Execute = 1,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            false => Val::Noexecute,
            true => Val::Execute,
        }
    }
    #[doc = "Command will not execute or is not executing in flash wrapper"]
    #[inline(always)]
    pub fn is_noexecute(&self) -> bool {
        *self == Val::Noexecute
    }
    #[doc = "Command will execute or is executing in flash wrapper"]
    #[inline(always)]
    pub fn is_execute(&self) -> bool {
        *self == Val::Execute
    }
}
#[doc = "Field `VAL` writer - Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command will not execute or is not executing in flash wrapper"]
    #[inline(always)]
    pub fn noexecute(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Noexecute)
    }
    #[doc = "Command will execute or is executing in flash wrapper"]
    #[inline(always)]
    pub fn execute(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Execute)
    }
}
impl R {
    #[doc = "Bit 0 - Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmdexecSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Execute Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdexec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdexec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmdexecSpec;
impl crate::RegisterSpec for FlashctlCmdexecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmdexec::R`](R) reader structure"]
impl crate::Readable for FlashctlCmdexecSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmdexec::W`](W) writer structure"]
impl crate::Writable for FlashctlCmdexecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDEXEC to value 0"]
impl crate::Resettable for FlashctlCmdexecSpec {}
