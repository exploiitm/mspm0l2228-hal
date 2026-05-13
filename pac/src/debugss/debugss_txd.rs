#[doc = "Register `DEBUGSS_TXD` reader"]
pub type R = crate::R<DebugssTxdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_txd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssTxdSpec;
impl crate::RegisterSpec for DebugssTxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_txd::R`](R) reader structure"]
impl crate::Readable for DebugssTxdSpec {}
#[doc = "`reset()` method sets DEBUGSS_TXD to value 0"]
impl crate::Resettable for DebugssTxdSpec {}
