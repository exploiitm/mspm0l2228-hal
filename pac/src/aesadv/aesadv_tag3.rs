#[doc = "Register `AESADV_TAG3` reader"]
pub type R = crate::R<AesadvTag3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvTag3Spec;
impl crate::RegisterSpec for AesadvTag3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_tag3::R`](R) reader structure"]
impl crate::Readable for AesadvTag3Spec {}
#[doc = "`reset()` method sets AESADV_TAG3 to value 0"]
impl crate::Resettable for AesadvTag3Spec {}
