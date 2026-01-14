#[doc = "Register `TIMG5_CTR` reader"]
pub type R = crate::R<Timg5CtrSpec>;
#[doc = "Register `TIMG5_CTR` writer"]
pub type W = crate::W<Timg5CtrSpec>;
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
    pub fn cctr(&mut self) -> CctrW<'_, Timg5CtrSpec> {
        CctrW::new(self, 0)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg5CtrSpec;
impl crate::RegisterSpec for Timg5CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg5_ctr::R`](R) reader structure"]
impl crate::Readable for Timg5CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`timg5_ctr::W`](W) writer structure"]
impl crate::Writable for Timg5CtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG5_CTR to value 0"]
impl crate::Resettable for Timg5CtrSpec {}
