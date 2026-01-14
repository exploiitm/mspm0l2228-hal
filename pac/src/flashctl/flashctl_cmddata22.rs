#[doc = "Register `FLASHCTL_CMDDATA22` reader"]
pub type R = crate::R<FlashctlCmddata22Spec>;
#[doc = "Register `FLASHCTL_CMDDATA22` writer"]
pub type W = crate::W<FlashctlCmddata22Spec>;
#[doc = "Field `VAL` reader - A 32-bit data value is placed in this field."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - A 32-bit data value is placed in this field."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmddata22Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmddata22Spec;
impl crate::RegisterSpec for FlashctlCmddata22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmddata22::R`](R) reader structure"]
impl crate::Readable for FlashctlCmddata22Spec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmddata22::W`](W) writer structure"]
impl crate::Writable for FlashctlCmddata22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDDATA22 to value 0"]
impl crate::Resettable for FlashctlCmddata22Spec {}
