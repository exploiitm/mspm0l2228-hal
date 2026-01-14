#[doc = "Register `FLASHCTL_CMDWEPROTC` reader"]
pub type R = crate::R<FlashctlCmdweprotcSpec>;
#[doc = "Register `FLASHCTL_CMDWEPROTC` writer"]
pub type W = crate::W<FlashctlCmdweprotcSpec>;
#[doc = "Field `VAL` reader - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. Note that the sectors protected with this register start at sector 256 in the flash, where the sectors protected by the CMDWEPROTB register end."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. Note that the sectors protected with this register start at sector 256 in the flash, where the sectors protected by the CMDWEPROTB register end."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. Note that the sectors protected with this register start at sector 256 in the flash, where the sectors protected by the CMDWEPROTB register end."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. Note that the sectors protected with this register start at sector 256 in the flash, where the sectors protected by the CMDWEPROTB register end."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmdweprotcSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Write Erase Protect C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprotc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprotc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmdweprotcSpec;
impl crate::RegisterSpec for FlashctlCmdweprotcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmdweprotc::R`](R) reader structure"]
impl crate::Readable for FlashctlCmdweprotcSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmdweprotc::W`](W) writer structure"]
impl crate::Writable for FlashctlCmdweprotcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDWEPROTC to value 0"]
impl crate::Resettable for FlashctlCmdweprotcSpec {}
