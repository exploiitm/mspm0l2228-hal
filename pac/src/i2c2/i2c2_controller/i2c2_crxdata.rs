#[doc = "Register `I2C2_CRXDATA` reader"]
pub type R = crate::R<I2c2CrxdataSpec>;
#[doc = "Field `VALUE` reader - Received Data. This field contains the last received data."]
pub type ValueR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Data. This field contains the last received data."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Controller RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_crxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2CrxdataSpec;
impl crate::RegisterSpec for I2c2CrxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2_crxdata::R`](R) reader structure"]
impl crate::Readable for I2c2CrxdataSpec {}
#[doc = "`reset()` method sets I2C2_CRXDATA to value 0"]
impl crate::Resettable for I2c2CrxdataSpec {}
