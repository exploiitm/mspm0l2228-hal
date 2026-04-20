#[doc = "Register `I2C2_TARGET_PECSR` reader"]
pub type R = crate::R<I2c2TargetPecsrSpec>;
#[doc = "Field `PECBYTECNT` reader - This is the current PEC Byte Count of the Target State Machine."]
pub type PecbytecntR = crate::FieldReader<u16>;
#[doc = "This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PecstsCheck {
    #[doc = "0: Indicates PEC was not checked in the transaction that occurred before the last Stop"]
    Cleared = 0,
    #[doc = "1: Indicates PEC was checked in the transaction that occurred before the last Stop"]
    Set = 1,
}
impl From<PecstsCheck> for bool {
    #[inline(always)]
    fn from(variant: PecstsCheck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECSTS_CHECK` reader - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
pub type PecstsCheckR = crate::BitReader<PecstsCheck>;
impl PecstsCheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PecstsCheck {
        match self.bits {
            false => PecstsCheck::Cleared,
            true => PecstsCheck::Set,
        }
    }
    #[doc = "Indicates PEC was not checked in the transaction that occurred before the last Stop"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == PecstsCheck::Cleared
    }
    #[doc = "Indicates PEC was checked in the transaction that occurred before the last Stop"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PecstsCheck::Set
    }
}
#[doc = "This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PecstsError {
    #[doc = "0: Indicates PEC check error did not occurr in the transaction that occurred before the last Stop"]
    Cleared = 0,
    #[doc = "1: Indicates PEC check error occurred in the transaction that occurred before the last Stop"]
    Set = 1,
}
impl From<PecstsError> for bool {
    #[inline(always)]
    fn from(variant: PecstsError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECSTS_ERROR` reader - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
pub type PecstsErrorR = crate::BitReader<PecstsError>;
impl PecstsErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PecstsError {
        match self.bits {
            false => PecstsError::Cleared,
            true => PecstsError::Set,
        }
    }
    #[doc = "Indicates PEC check error did not occurr in the transaction that occurred before the last Stop"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == PecstsError::Cleared
    }
    #[doc = "Indicates PEC check error occurred in the transaction that occurred before the last Stop"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PecstsError::Set
    }
}
impl R {
    #[doc = "Bits 0:8 - This is the current PEC Byte Count of the Target State Machine."]
    #[inline(always)]
    pub fn pecbytecnt(&self) -> PecbytecntR {
        PecbytecntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn pecsts_check(&self) -> PecstsCheckR {
        PecstsCheckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn pecsts_error(&self) -> PecstsErrorR {
        PecstsErrorR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "I2C Target PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_target_pecsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2TargetPecsrSpec;
impl crate::RegisterSpec for I2c2TargetPecsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2_target_pecsr::R`](R) reader structure"]
impl crate::Readable for I2c2TargetPecsrSpec {}
#[doc = "`reset()` method sets I2C2_TARGET_PECSR to value 0"]
impl crate::Resettable for I2c2TargetPecsrSpec {}
