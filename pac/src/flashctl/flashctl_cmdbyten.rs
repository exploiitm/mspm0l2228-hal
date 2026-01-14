#[doc = "Register `FLASHCTL_CMDBYTEN` reader"]
pub type R = crate::R<FlashctlCmdbytenSpec>;
#[doc = "Register `FLASHCTL_CMDBYTEN` writer"]
pub type W = crate::W<FlashctlCmdbytenSpec>;
#[doc = "Field `VAL` reader - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmdbytenSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Program Byte Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdbyten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdbyten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmdbytenSpec;
impl crate::RegisterSpec for FlashctlCmdbytenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmdbyten::R`](R) reader structure"]
impl crate::Readable for FlashctlCmdbytenSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmdbyten::W`](W) writer structure"]
impl crate::Writable for FlashctlCmdbytenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDBYTEN to value 0"]
impl crate::Resettable for FlashctlCmdbytenSpec {}
