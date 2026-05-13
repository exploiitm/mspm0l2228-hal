#[doc = "Register `AESADV_GCMCCM_TAG2` writer"]
pub type W = crate::W<AesadvGcmccmTag2Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvGcmccmTag2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_gcmccm_tag2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvGcmccmTag2Spec;
impl crate::RegisterSpec for AesadvGcmccmTag2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_gcmccm_tag2::W`](W) writer structure"]
impl crate::Writable for AesadvGcmccmTag2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_GCMCCM_TAG2 to value 0"]
impl crate::Resettable for AesadvGcmccmTag2Spec {}
