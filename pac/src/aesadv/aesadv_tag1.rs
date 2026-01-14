#[doc = "Register `AESADV_TAG1` reader"]
pub type R = crate::R<AesadvTag1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvTag1Spec;
impl crate::RegisterSpec for AesadvTag1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_tag1::R`](R) reader structure"]
impl crate::Readable for AesadvTag1Spec {}
#[doc = "`reset()` method sets AESADV_TAG1 to value 0"]
impl crate::Resettable for AesadvTag1Spec {}
