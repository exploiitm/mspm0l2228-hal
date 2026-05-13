#[doc = "Register `AESADV_GHASH_H3` writer"]
pub type W = crate::W<AesadvGhashH3Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvGhashH3Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ghash_h3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvGhashH3Spec;
impl crate::RegisterSpec for AesadvGhashH3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_ghash_h3::W`](W) writer structure"]
impl crate::Writable for AesadvGhashH3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_GHASH_H3 to value 0"]
impl crate::Resettable for AesadvGhashH3Spec {}
