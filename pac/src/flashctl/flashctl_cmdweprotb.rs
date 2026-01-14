#[doc = "Register `FLASHCTL_CMDWEPROTB` reader"]
pub type R = crate::R<FlashctlCmdweprotbSpec>;
#[doc = "Register `FLASHCTL_CMDWEPROTB` writer"]
pub type W = crate::W<FlashctlCmdweprotbSpec>;
#[doc = "Field `VAL` reader - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmdweprotbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Write Erase Protect B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprotb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprotb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmdweprotbSpec;
impl crate::RegisterSpec for FlashctlCmdweprotbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmdweprotb::R`](R) reader structure"]
impl crate::Readable for FlashctlCmdweprotbSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmdweprotb::W`](W) writer structure"]
impl crate::Writable for FlashctlCmdweprotbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDWEPROTB to value 0"]
impl crate::Resettable for FlashctlCmdweprotbSpec {}
