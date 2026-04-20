#[doc = "Register `I2C1_TRXDATA` reader"]
pub type R = crate::R<I2c1TrxdataSpec>;
#[doc = "Field `VALUE` reader - Received Data. This field contains the last received data."]
pub type ValueR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Data. This field contains the last received data."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Target RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_trxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1TrxdataSpec;
impl crate::RegisterSpec for I2c1TrxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_trxdata::R`](R) reader structure"]
impl crate::Readable for I2c1TrxdataSpec {}
#[doc = "`reset()` method sets I2C1_TRXDATA to value 0"]
impl crate::Resettable for I2c1TrxdataSpec {}
