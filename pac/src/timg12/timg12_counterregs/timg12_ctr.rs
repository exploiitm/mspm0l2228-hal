#[doc = "Register `TIMG12_CTR` reader"]
pub type R = crate::R<Timg12CtrSpec>;
#[doc = "Register `TIMG12_CTR` writer"]
pub type W = crate::W<Timg12CtrSpec>;
#[doc = "Field `CCTR` reader - Current Counter value"]
pub type CctrR = crate::FieldReader<u32>;
#[doc = "Field `CCTR` writer - Current Counter value"]
pub type CctrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Counter value"]
    #[inline(always)]
    pub fn cctr(&self) -> CctrR {
        CctrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Counter value"]
    #[inline(always)]
    pub fn cctr(&mut self) -> CctrW<'_, Timg12CtrSpec> {
        CctrW::new(self, 0)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg12CtrSpec;
impl crate::RegisterSpec for Timg12CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg12_ctr::R`](R) reader structure"]
impl crate::Readable for Timg12CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`timg12_ctr::W`](W) writer structure"]
impl crate::Writable for Timg12CtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG12_CTR to value 0"]
impl crate::Resettable for Timg12CtrSpec {}
