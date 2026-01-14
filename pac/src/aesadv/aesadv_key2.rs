#[doc = "Register `AESADV_KEY2` writer"]
pub type W = crate::W<AesadvKey2Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvKey2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvKey2Spec;
impl crate::RegisterSpec for AesadvKey2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_key2::W`](W) writer structure"]
impl crate::Writable for AesadvKey2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_KEY2 to value 0"]
impl crate::Resettable for AesadvKey2Spec {}
