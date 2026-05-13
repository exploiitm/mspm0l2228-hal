#[doc = "Register `I2C2_TTXDATA` reader"]
pub type R = crate::R<I2c2TtxdataSpec>;
#[doc = "Register `I2C2_TTXDATA` writer"]
pub type W = crate::W<I2c2TtxdataSpec>;
#[doc = "Field `VALUE` reader - Transmit Data This byte contains the data to be transferred during the next transaction."]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Transmit Data This byte contains the data to be transferred during the next transaction."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Data This byte contains the data to be transferred during the next transaction."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data This byte contains the data to be transferred during the next transaction."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, I2c2TtxdataSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "I2C Target TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_ttxdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_ttxdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2TtxdataSpec;
impl crate::RegisterSpec for I2c2TtxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2_ttxdata::R`](R) reader structure"]
impl crate::Readable for I2c2TtxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c2_ttxdata::W`](W) writer structure"]
impl crate::Writable for I2c2TtxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2_TTXDATA to value 0"]
impl crate::Resettable for I2c2TtxdataSpec {}
