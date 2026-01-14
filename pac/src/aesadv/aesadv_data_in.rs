#[doc = "Register `AESADV_DATA_IN` writer"]
pub type W = crate::W<AesadvDataInSpec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvDataInSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Data in alias register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data_in::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvDataInSpec;
impl crate::RegisterSpec for AesadvDataInSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_data_in::W`](W) writer structure"]
impl crate::Writable for AesadvDataInSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_DATA_IN to value 0"]
impl crate::Resettable for AesadvDataInSpec {}
