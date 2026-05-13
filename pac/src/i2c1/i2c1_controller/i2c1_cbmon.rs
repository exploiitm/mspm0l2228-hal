#[doc = "Register `I2C1_CBMON` reader"]
pub type R = crate::R<I2c1CbmonSpec>;
#[doc = "I2C SCL Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scl {
    #[doc = "0: The I2CSCL signal is low."]
    Cleared = 0,
    #[doc = "1: The I2CSCL signal is high. Note: During and right after reset, the SCL pin is in GPIO input mode without the internal pull enabled. For proper I2C operation, the user should have the external pull-up resistor in place before starting any I2C operations."]
    Set = 1,
}
impl From<Scl> for bool {
    #[inline(always)]
    fn from(variant: Scl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCL` reader - I2C SCL Status"]
pub type SclR = crate::BitReader<Scl>;
impl SclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scl {
        match self.bits {
            false => Scl::Cleared,
            true => Scl::Set,
        }
    }
    #[doc = "The I2CSCL signal is low."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Scl::Cleared
    }
    #[doc = "The I2CSCL signal is high. Note: During and right after reset, the SCL pin is in GPIO input mode without the internal pull enabled. For proper I2C operation, the user should have the external pull-up resistor in place before starting any I2C operations."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Scl::Set
    }
}
#[doc = "I2C SDA Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sda {
    #[doc = "0: The I2CSDA signal is low."]
    Cleared = 0,
    #[doc = "1: The I2CSDA signal is high. Note: During and right after reset, the SDA pin is in GPIO input mode without the internal pull enabled. For proper I2C operation, the user should have the external pull-up resistor in place before starting any I2C operations."]
    Set = 1,
}
impl From<Sda> for bool {
    #[inline(always)]
    fn from(variant: Sda) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDA` reader - I2C SDA Status"]
pub type SdaR = crate::BitReader<Sda>;
impl SdaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sda {
        match self.bits {
            false => Sda::Cleared,
            true => Sda::Set,
        }
    }
    #[doc = "The I2CSDA signal is low."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Sda::Cleared
    }
    #[doc = "The I2CSDA signal is high. Note: During and right after reset, the SDA pin is in GPIO input mode without the internal pull enabled. For proper I2C operation, the user should have the external pull-up resistor in place before starting any I2C operations."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Sda::Set
    }
}
impl R {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "I2C Controller Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cbmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1CbmonSpec;
impl crate::RegisterSpec for I2c1CbmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_cbmon::R`](R) reader structure"]
impl crate::Readable for I2c1CbmonSpec {}
#[doc = "`reset()` method sets I2C1_CBMON to value 0x03"]
impl crate::Resettable for I2c1CbmonSpec {
    const RESET_VALUE: u32 = 0x03;
}
