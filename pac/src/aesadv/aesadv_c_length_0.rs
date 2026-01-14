#[doc = "Register `AESADV_C_LENGTH_0` writer"]
pub type W = crate::W<AesadvCLength0Spec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvCLength0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Crypto data length (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_c_length_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvCLength0Spec;
impl crate::RegisterSpec for AesadvCLength0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_c_length_0::W`](W) writer structure"]
impl crate::Writable for AesadvCLength0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_C_LENGTH_0 to value 0"]
impl crate::Resettable for AesadvCLength0Spec {}
