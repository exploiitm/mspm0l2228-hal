#[doc = "Register `AESADV_GCMCCM_TAG1` writer"]
pub type W = crate::W<AesadvGcmccmTag1Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvGcmccmTag1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_gcmccm_tag1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvGcmccmTag1Spec;
impl crate::RegisterSpec for AesadvGcmccmTag1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_gcmccm_tag1::W`](W) writer structure"]
impl crate::Writable for AesadvGcmccmTag1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_GCMCCM_TAG1 to value 0"]
impl crate::Resettable for AesadvGcmccmTag1Spec {}
