#[doc = "Register `I2C1_TFIFOSR` reader"]
pub type R = crate::R<I2c1TfifosrSpec>;
#[doc = "Field `RXFIFOCNT` reader - Number of Bytes which could be read from the RX FIFO"]
pub type RxfifocntR = crate::FieldReader;
#[doc = "RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxflush {
    #[doc = "0: FIFOFlush not active"]
    Inactive = 0,
    #[doc = "1: FIFO Flush active"]
    Active = 1,
}
impl From<Rxflush> for bool {
    #[inline(always)]
    fn from(variant: Rxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFLUSH` reader - RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop."]
pub type RxflushR = crate::BitReader<Rxflush>;
impl RxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxflush {
        match self.bits {
            false => Rxflush::Inactive,
            true => Rxflush::Active,
        }
    }
    #[doc = "FIFOFlush not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Rxflush::Inactive
    }
    #[doc = "FIFO Flush active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Rxflush::Active
    }
}
#[doc = "Field `TXFIFOCNT` reader - Number of Bytes which could be put into the TX FIFO"]
pub type TxfifocntR = crate::FieldReader;
#[doc = "TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txflush {
    #[doc = "0: FIFO Flush not active"]
    Inactive = 0,
    #[doc = "1: FIFO Flush active"]
    Active = 1,
}
impl From<Txflush> for bool {
    #[inline(always)]
    fn from(variant: Txflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFLUSH` reader - TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop."]
pub type TxflushR = crate::BitReader<Txflush>;
impl TxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txflush {
        match self.bits {
            false => Txflush::Inactive,
            true => Txflush::Active,
        }
    }
    #[doc = "FIFO Flush not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Txflush::Inactive
    }
    #[doc = "FIFO Flush active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Txflush::Active
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Bytes which could be read from the RX FIFO"]
    #[inline(always)]
    pub fn rxfifocnt(&self) -> RxfifocntR {
        RxfifocntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop."]
    #[inline(always)]
    pub fn rxflush(&self) -> RxflushR {
        RxflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bytes which could be put into the TX FIFO"]
    #[inline(always)]
    pub fn txfifocnt(&self) -> TxfifocntR {
        TxfifocntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop."]
    #[inline(always)]
    pub fn txflush(&self) -> TxflushR {
        TxflushR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "I2C Target FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_tfifosr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1TfifosrSpec;
impl crate::RegisterSpec for I2c1TfifosrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_tfifosr::R`](R) reader structure"]
impl crate::Readable for I2c1TfifosrSpec {}
#[doc = "`reset()` method sets I2C1_TFIFOSR to value 0x0800"]
impl crate::Resettable for I2c1TfifosrSpec {
    const RESET_VALUE: u32 = 0x0800;
}
