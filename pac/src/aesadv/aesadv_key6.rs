#[doc = "Register `AESADV_KEY6` writer"]
pub type W = crate::W<AesadvKey6Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvKey6Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvKey6Spec;
impl crate::RegisterSpec for AesadvKey6Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_key6::W`](W) writer structure"]
impl crate::Writable for AesadvKey6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_KEY6 to value 0"]
impl crate::Resettable for AesadvKey6Spec {}
