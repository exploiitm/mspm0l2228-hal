#[doc = "Register `TRNG_TEST_RESULTS` reader"]
pub type R = crate::R<TrngTestResultsSpec>;
#[doc = "Field `DIG_TEST` reader - Bit 0 indicates if the first decimation rate test and health test(verifies conditioning, decimation, and captured buffer) fails and Bit 1 indicates if the second decimation test and health test fails Bit 0 - decim_test0 (decim = 0x0) Bit 1 - decim_test1 (decim = 0x1) ..."]
pub type DigTestR = crate::FieldReader;
#[doc = "Field `ANA_TEST` reader - Runs through 4096 samples from an enabled entropy source and verifies that none of the health tests failed, indicating sufficient entropy was produced by the analog components"]
pub type AnaTestR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Bit 0 indicates if the first decimation rate test and health test(verifies conditioning, decimation, and captured buffer) fails and Bit 1 indicates if the second decimation test and health test fails Bit 0 - decim_test0 (decim = 0x0) Bit 1 - decim_test1 (decim = 0x1) ..."]
    #[inline(always)]
    pub fn dig_test(&self) -> DigTestR {
        DigTestR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Runs through 4096 samples from an enabled entropy source and verifies that none of the health tests failed, indicating sufficient entropy was produced by the analog components"]
    #[inline(always)]
    pub fn ana_test(&self) -> AnaTestR {
        AnaTestR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Test results from TEST_ANA and TEST_DIG\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_test_results::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngTestResultsSpec;
impl crate::RegisterSpec for TrngTestResultsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_test_results::R`](R) reader structure"]
impl crate::Readable for TrngTestResultsSpec {}
#[doc = "`reset()` method sets TRNG_TEST_RESULTS to value 0"]
impl crate::Resettable for TrngTestResultsSpec {}
