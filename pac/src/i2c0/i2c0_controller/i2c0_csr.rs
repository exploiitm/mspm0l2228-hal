#[doc = "Register `I2C0_CSR` reader"]
pub type R = crate::R<I2c0CsrSpec>;
#[doc = "I2C Controller FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: The controller is idle."]
    Cleared = 0,
    #[doc = "1: The controller is busy."]
    Set = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - I2C Controller FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Cleared,
            true => Busy::Set,
        }
    }
    #[doc = "The controller is idle."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Busy::Cleared
    }
    #[doc = "The controller is busy."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Busy::Set
    }
}
#[doc = "Error The error can be from the Target address not being acknowledged or the transmit data not being acknowledged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err {
    #[doc = "0: No error was detected on the last operation."]
    Cleared = 0,
    #[doc = "1: An error occurred on the last operation."]
    Set = 1,
}
impl From<Err> for bool {
    #[inline(always)]
    fn from(variant: Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - Error The error can be from the Target address not being acknowledged or the transmit data not being acknowledged."]
pub type ErrR = crate::BitReader<Err>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err {
        match self.bits {
            false => Err::Cleared,
            true => Err::Set,
        }
    }
    #[doc = "No error was detected on the last operation."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Err::Cleared
    }
    #[doc = "An error occurred on the last operation."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Err::Set
    }
}
#[doc = "Acknowledge Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrack {
    #[doc = "0: The transmitted address was acknowledged"]
    Cleared = 0,
    #[doc = "1: The transmitted address was not acknowledged."]
    Set = 1,
}
impl From<Adrack> for bool {
    #[inline(always)]
    fn from(variant: Adrack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRACK` reader - Acknowledge Address"]
pub type AdrackR = crate::BitReader<Adrack>;
impl AdrackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrack {
        match self.bits {
            false => Adrack::Cleared,
            true => Adrack::Set,
        }
    }
    #[doc = "The transmitted address was acknowledged"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Adrack::Cleared
    }
    #[doc = "The transmitted address was not acknowledged."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Adrack::Set
    }
}
#[doc = "Acknowledge Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datack {
    #[doc = "0: The transmitted data was acknowledged"]
    Cleared = 0,
    #[doc = "1: The transmitted data was not acknowledged."]
    Set = 1,
}
impl From<Datack> for bool {
    #[inline(always)]
    fn from(variant: Datack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATACK` reader - Acknowledge Data"]
pub type DatackR = crate::BitReader<Datack>;
impl DatackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datack {
        match self.bits {
            false => Datack::Cleared,
            true => Datack::Set,
        }
    }
    #[doc = "The transmitted data was acknowledged"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Datack::Cleared
    }
    #[doc = "The transmitted data was not acknowledged."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Datack::Set
    }
}
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arblst {
    #[doc = "0: The I2C controller won arbitration."]
    Cleared = 0,
    #[doc = "1: The I2C controller lost arbitration."]
    Set = 1,
}
impl From<Arblst> for bool {
    #[inline(always)]
    fn from(variant: Arblst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBLST` reader - Arbitration Lost"]
pub type ArblstR = crate::BitReader<Arblst>;
impl ArblstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arblst {
        match self.bits {
            false => Arblst::Cleared,
            true => Arblst::Set,
        }
    }
    #[doc = "The I2C controller won arbitration."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Arblst::Cleared
    }
    #[doc = "The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Arblst::Set
    }
}
#[doc = "I2C Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: The I2C controller is not idle."]
    Cleared = 0,
    #[doc = "1: The I2C controller is idle."]
    Set = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - I2C Idle"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::Cleared,
            true => Idle::Set,
        }
    }
    #[doc = "The I2C controller is not idle."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Idle::Cleared
    }
    #[doc = "The I2C controller is idle."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idle::Set
    }
}
#[doc = "I2C Bus is Busy Controller State Machine will wait until this bit is cleared before starting a transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busbsy {
    #[doc = "0: The I2C bus is idle."]
    Cleared = 0,
    #[doc = "1: 'This Status bit is set on a START or when SCL goes low. It is cleared on a STOP, or when a SCL high bus busy timeout occurs and SCL and SDA are both high. This status is cleared when the ACTIVE bit is low. Note that the Controller State Machine will wait until this bit is cleared before starting an I2C transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
    Set = 1,
}
impl From<Busbsy> for bool {
    #[inline(always)]
    fn from(variant: Busbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSBSY` reader - I2C Bus is Busy Controller State Machine will wait until this bit is cleared before starting a transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
pub type BusbsyR = crate::BitReader<Busbsy>;
impl BusbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busbsy {
        match self.bits {
            false => Busbsy::Cleared,
            true => Busbsy::Set,
        }
    }
    #[doc = "The I2C bus is idle."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Busbsy::Cleared
    }
    #[doc = "'This Status bit is set on a START or when SCL goes low. It is cleared on a STOP, or when a SCL high bus busy timeout occurs and SCL and SDA are both high. This status is cleared when the ACTIVE bit is low. Note that the Controller State Machine will wait until this bit is cleared before starting an I2C transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Busbsy::Set
    }
}
#[doc = "Field `CBCNT` reader - I2C Controller Transaction Count This field contains the current count-down value of the transaction."]
pub type CbcntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - I2C Controller FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error The error can be from the Target address not being acknowledged or the transmit data not being acknowledged."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn adrack(&self) -> AdrackR {
        AdrackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn datack(&self) -> DatackR {
        DatackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus is Busy Controller State Machine will wait until this bit is cleared before starting a transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:27 - I2C Controller Transaction Count This field contains the current count-down value of the transaction."]
    #[inline(always)]
    pub fn cbcnt(&self) -> CbcntR {
        CbcntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0CsrSpec;
impl crate::RegisterSpec for I2c0CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_csr::R`](R) reader structure"]
impl crate::Readable for I2c0CsrSpec {}
#[doc = "`reset()` method sets I2C0_CSR to value 0"]
impl crate::Resettable for I2c0CsrSpec {}
