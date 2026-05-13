#[doc = "Register `CRCP0_CRCIN` writer"]
pub type W = crate::W<Crcp0CrcinSpec>;
#[doc = "Field `DATA` writer - Input Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Crcp0CrcinSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "CRC Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crcp0CrcinSpec;
impl crate::RegisterSpec for Crcp0CrcinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcp0_crcin::W`](W) writer structure"]
impl crate::Writable for Crcp0CrcinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCP0_CRCIN to value 0"]
impl crate::Resettable for Crcp0CrcinSpec {}
