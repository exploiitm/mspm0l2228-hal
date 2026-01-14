#[doc = "Register `TIMG4_CC_01[%s]` reader"]
pub type R = crate::R<Timg4Cc01Spec>;
#[doc = "Register `TIMG4_CC_01[%s]` writer"]
pub type W = crate::W<Timg4Cc01Spec>;
#[doc = "Field `CCVAL` reader - Capture or compare value"]
pub type CcvalR = crate::FieldReader<u16>;
#[doc = "Field `CCVAL` writer - Capture or compare value"]
pub type CcvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value"]
    #[inline(always)]
    pub fn ccval(&self) -> CcvalR {
        CcvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value"]
    #[inline(always)]
    pub fn ccval(&mut self) -> CcvalW<'_, Timg4Cc01Spec> {
        CcvalW::new(self, 0)
    }
}
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_cc_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cc_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg4Cc01Spec;
impl crate::RegisterSpec for Timg4Cc01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg4_cc_01::R`](R) reader structure"]
impl crate::Readable for Timg4Cc01Spec {}
#[doc = "`write(|w| ..)` method takes [`timg4_cc_01::W`](W) writer structure"]
impl crate::Writable for Timg4Cc01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG4_CC_01[%s] to value 0"]
impl crate::Resettable for Timg4Cc01Spec {}
