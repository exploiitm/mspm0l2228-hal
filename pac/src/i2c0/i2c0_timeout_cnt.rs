#[doc = "Register `I2C0_TIMEOUT_CNT` reader"]
pub type R = crate::R<I2c0TimeoutCntSpec>;
#[doc = "Field `TCNTA` reader - Timeout Count A Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter A"]
pub type TcntaR = crate::FieldReader;
#[doc = "Field `TCNTB` reader - Timeout Count B Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter B"]
pub type TcntbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Timeout Count A Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter A"]
    #[inline(always)]
    pub fn tcnta(&self) -> TcntaR {
        TcntaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timeout Count B Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter B"]
    #[inline(always)]
    pub fn tcntb(&self) -> TcntbR {
        TcntbR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "I2C Timeout Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_timeout_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0TimeoutCntSpec;
impl crate::RegisterSpec for I2c0TimeoutCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_timeout_cnt::R`](R) reader structure"]
impl crate::Readable for I2c0TimeoutCntSpec {}
#[doc = "`reset()` method sets I2C0_TIMEOUT_CNT to value 0x0002_0002"]
impl crate::Resettable for I2c0TimeoutCntSpec {
    const RESET_VALUE: u32 = 0x0002_0002;
}
