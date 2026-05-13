#[doc = "Register `CRCP0_CRCOUT` reader"]
pub type R = crate::R<Crcp0CrcoutSpec>;
#[doc = "Field `RESULT` reader - Result"]
pub type ResultR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Result"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits)
    }
}
#[doc = "CRC Output Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_crcout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crcp0CrcoutSpec;
impl crate::RegisterSpec for Crcp0CrcoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcp0_crcout::R`](R) reader structure"]
impl crate::Readable for Crcp0CrcoutSpec {}
#[doc = "`reset()` method sets CRCP0_CRCOUT to value 0"]
impl crate::Resettable for Crcp0CrcoutSpec {}
