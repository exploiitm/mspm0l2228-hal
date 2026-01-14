#[doc = "Register `AESADV_AAD_LENGTH` writer"]
pub type W = crate::W<AesadvAadLengthSpec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvAadLengthSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "AAD Data Length\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_aad_length::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvAadLengthSpec;
impl crate::RegisterSpec for AesadvAadLengthSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_aad_length::W`](W) writer structure"]
impl crate::Writable for AesadvAadLengthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_AAD_LENGTH to value 0"]
impl crate::Resettable for AesadvAadLengthSpec {}
