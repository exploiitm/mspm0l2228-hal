#[doc = "Register `TRNG_DATA_CAPTURE` reader"]
pub type R = crate::R<TrngDataCaptureSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Captured word buffer of RNG data\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_data_capture::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngDataCaptureSpec;
impl crate::RegisterSpec for TrngDataCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_data_capture::R`](R) reader structure"]
impl crate::Readable for TrngDataCaptureSpec {}
#[doc = "`reset()` method sets TRNG_DATA_CAPTURE to value 0"]
impl crate::Resettable for TrngDataCaptureSpec {}
