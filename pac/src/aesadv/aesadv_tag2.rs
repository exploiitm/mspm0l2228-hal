#[doc = "Register `AESADV_TAG2` reader"]
pub type R = crate::R<AesadvTag2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvTag2Spec;
impl crate::RegisterSpec for AesadvTag2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_tag2::R`](R) reader structure"]
impl crate::Readable for AesadvTag2Spec {}
#[doc = "`reset()` method sets AESADV_TAG2 to value 0"]
impl crate::Resettable for AesadvTag2Spec {}
