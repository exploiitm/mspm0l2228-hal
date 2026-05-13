#[doc = "Register `AESADV_KEY0` writer"]
pub type W = crate::W<AesadvKey0Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvKey0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvKey0Spec;
impl crate::RegisterSpec for AesadvKey0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_key0::W`](W) writer structure"]
impl crate::Writable for AesadvKey0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_KEY0 to value 0"]
impl crate::Resettable for AesadvKey0Spec {}
