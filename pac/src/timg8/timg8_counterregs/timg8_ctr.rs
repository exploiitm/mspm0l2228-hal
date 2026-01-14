#[doc = "Register `TIMG8_CTR` reader"]
pub type R = crate::R<Timg8CtrSpec>;
#[doc = "Register `TIMG8_CTR` writer"]
pub type W = crate::W<Timg8CtrSpec>;
#[doc = "Field `CCTR` reader - Current Counter value"]
pub type CctrR = crate::FieldReader<u16>;
#[doc = "Field `CCTR` writer - Current Counter value"]
pub type CctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current Counter value"]
    #[inline(always)]
    pub fn cctr(&self) -> CctrR {
        CctrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current Counter value"]
    #[inline(always)]
    pub fn cctr(&mut self) -> CctrW<'_, Timg8CtrSpec> {
        CctrW::new(self, 0)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg8CtrSpec;
impl crate::RegisterSpec for Timg8CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg8_ctr::R`](R) reader structure"]
impl crate::Readable for Timg8CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`timg8_ctr::W`](W) writer structure"]
impl crate::Writable for Timg8CtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG8_CTR to value 0"]
impl crate::Resettable for Timg8CtrSpec {}
