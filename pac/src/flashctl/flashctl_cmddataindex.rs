#[doc = "Register `FLASHCTL_CMDDATAINDEX` reader"]
pub type R = crate::R<FlashctlCmddataindexSpec>;
#[doc = "Register `FLASHCTL_CMDDATAINDEX` writer"]
pub type W = crate::W<FlashctlCmddataindexSpec>;
#[doc = "Field `VAL` reader - Data register index"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Data register index"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Data register index"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data register index"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmddataindexSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmddataindexSpec;
impl crate::RegisterSpec for FlashctlCmddataindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmddataindex::R`](R) reader structure"]
impl crate::Readable for FlashctlCmddataindexSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmddataindex::W`](W) writer structure"]
impl crate::Writable for FlashctlCmddataindexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDDATAINDEX to value 0"]
impl crate::Resettable for FlashctlCmddataindexSpec {}
