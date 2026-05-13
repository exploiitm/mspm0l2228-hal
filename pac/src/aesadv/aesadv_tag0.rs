#[doc = "Register `AESADV_TAG0` reader"]
pub type R = crate::R<AesadvTag0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvTag0Spec;
impl crate::RegisterSpec for AesadvTag0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_tag0::R`](R) reader structure"]
impl crate::Readable for AesadvTag0Spec {}
#[doc = "`reset()` method sets AESADV_TAG0 to value 0"]
impl crate::Resettable for AesadvTag0Spec {}
