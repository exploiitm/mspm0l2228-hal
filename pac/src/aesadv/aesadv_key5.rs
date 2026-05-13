#[doc = "Register `AESADV_KEY5` writer"]
pub type W = crate::W<AesadvKey5Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvKey5Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvKey5Spec;
impl crate::RegisterSpec for AesadvKey5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_key5::W`](W) writer structure"]
impl crate::Writable for AesadvKey5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_KEY5 to value 0"]
impl crate::Resettable for AesadvKey5Spec {}
