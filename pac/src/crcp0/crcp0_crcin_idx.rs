#[doc = "Register `CRCP0_CRCIN_IDX[%s]` writer"]
pub type W = crate::W<Crcp0CrcinIdxSpec>;
#[doc = "Field `DATA` writer - Input Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Crcp0CrcinIdxSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "CRC Input Data Array Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcin_idx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crcp0CrcinIdxSpec;
impl crate::RegisterSpec for Crcp0CrcinIdxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcp0_crcin_idx::W`](W) writer structure"]
impl crate::Writable for Crcp0CrcinIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCP0_CRCIN_IDX[%s] to value 0"]
impl crate::Resettable for Crcp0CrcinIdxSpec {}
