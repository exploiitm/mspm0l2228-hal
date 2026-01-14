#[doc = "Register `AESADV_GHASH_H2` writer"]
pub type W = crate::W<AesadvGhashH2Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvGhashH2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CCM &amp; CBC-MAC second key / GCM Hash Key input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ghash_h2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvGhashH2Spec;
impl crate::RegisterSpec for AesadvGhashH2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_ghash_h2::W`](W) writer structure"]
impl crate::Writable for AesadvGhashH2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_GHASH_H2 to value 0"]
impl crate::Resettable for AesadvGhashH2Spec {}
