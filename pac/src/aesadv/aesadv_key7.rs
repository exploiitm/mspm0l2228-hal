#[doc = "Register `AESADV_KEY7` writer"]
pub type W = crate::W<AesadvKey7Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvKey7Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvKey7Spec;
impl crate::RegisterSpec for AesadvKey7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_key7::W`](W) writer structure"]
impl crate::Writable for AesadvKey7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_KEY7 to value 0"]
impl crate::Resettable for AesadvKey7Spec {}
