#[doc = "Register `FLASHCTL_IMASK` reader"]
pub type R = crate::R<FlashctlImaskSpec>;
#[doc = "Register `FLASHCTL_IMASK` writer"]
pub type W = crate::W<FlashctlImaskSpec>;
#[doc = "Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Interrupt is masked out"]
    Disabled = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in \\[IPSTANDARD.MIS\\] will be set"]
    Enabled = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Disabled,
            true => Done::Enabled,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in \\[IPSTANDARD.MIS\\] will be set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
}
#[doc = "Field `DONE` writer - Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Disabled)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in \\[IPSTANDARD.MIS\\] will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, FlashctlImaskSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlImaskSpec;
impl crate::RegisterSpec for FlashctlImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_imask::R`](R) reader structure"]
impl crate::Readable for FlashctlImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_imask::W`](W) writer structure"]
impl crate::Writable for FlashctlImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_IMASK to value 0"]
impl crate::Resettable for FlashctlImaskSpec {}
