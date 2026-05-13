#[doc = "Register `AESADV_KEY4` writer"]
pub type W = crate::W<AesadvKey4Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvKey4Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvKey4Spec;
impl crate::RegisterSpec for AesadvKey4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_key4::W`](W) writer structure"]
impl crate::Writable for AesadvKey4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_KEY4 to value 0"]
impl crate::Resettable for AesadvKey4Spec {}
