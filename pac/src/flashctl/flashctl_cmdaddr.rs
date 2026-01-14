#[doc = "Register `FLASHCTL_CMDADDR` reader"]
pub type R = crate::R<FlashctlCmdaddrSpec>;
#[doc = "Register `FLASHCTL_CMDADDR` writer"]
pub type W = crate::W<FlashctlCmdaddrSpec>;
#[doc = "Field `VAL` reader - Address value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Address value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmdaddrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmdaddrSpec;
impl crate::RegisterSpec for FlashctlCmdaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmdaddr::R`](R) reader structure"]
impl crate::Readable for FlashctlCmdaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmdaddr::W`](W) writer structure"]
impl crate::Writable for FlashctlCmdaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDADDR to value 0"]
impl crate::Resettable for FlashctlCmdaddrSpec {}
