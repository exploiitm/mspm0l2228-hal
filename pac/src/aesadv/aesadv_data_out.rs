#[doc = "Register `AESADV_DATA_OUT` reader"]
pub type R = crate::R<AesadvDataOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Data out alias register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvDataOutSpec;
impl crate::RegisterSpec for AesadvDataOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_data_out::R`](R) reader structure"]
impl crate::Readable for AesadvDataOutSpec {}
#[doc = "`reset()` method sets AESADV_DATA_OUT to value 0"]
impl crate::Resettable for AesadvDataOutSpec {}
