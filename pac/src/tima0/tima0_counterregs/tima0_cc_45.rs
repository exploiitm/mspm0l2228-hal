#[doc = "Register `TIMA0_CC_45[%s]` reader"]
pub type R = crate::R<Tima0Cc45Spec>;
#[doc = "Register `TIMA0_CC_45[%s]` writer"]
pub type W = crate::W<Tima0Cc45Spec>;
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
    pub fn ccval(&mut self) -> CcvalW<'_, Tima0Cc45Spec> {
        CcvalW::new(self, 0)
    }
}
#[doc = "Compare Register 4 to Compare Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cc_45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cc_45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0Cc45Spec;
impl crate::RegisterSpec for Tima0Cc45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_cc_45::R`](R) reader structure"]
impl crate::Readable for Tima0Cc45Spec {}
#[doc = "`write(|w| ..)` method takes [`tima0_cc_45::W`](W) writer structure"]
impl crate::Writable for Tima0Cc45Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_CC_45[%s] to value 0"]
impl crate::Resettable for Tima0Cc45Spec {}
