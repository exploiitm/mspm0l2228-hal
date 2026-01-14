#[doc = "Register `SPI1_STAT` reader"]
pub type R = crate::R<Spi1StatSpec>;
#[doc = "Transmit FIFO empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfe {
    #[doc = "0: Transmit FIFO is not empty."]
    NotEmpty = 0,
    #[doc = "1: Transmit FIFO is empty."]
    Empty = 1,
}
impl From<Tfe> for bool {
    #[inline(always)]
    fn from(variant: Tfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO empty."]
pub type TfeR = crate::BitReader<Tfe>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfe {
        match self.bits {
            false => Tfe::NotEmpty,
            true => Tfe::Empty,
        }
    }
    #[doc = "Transmit FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Tfe::NotEmpty
    }
    #[doc = "Transmit FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tfe::Empty
    }
}
#[doc = "Transmit FIFO not full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnf {
    #[doc = "0: Transmit FIFO is full."]
    Full = 0,
    #[doc = "1: Transmit FIFO is not full."]
    NotFull = 1,
}
impl From<Tnf> for bool {
    #[inline(always)]
    fn from(variant: Tnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNF` reader - Transmit FIFO not full"]
pub type TnfR = crate::BitReader<Tnf>;
impl TnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnf {
        match self.bits {
            false => Tnf::Full,
            true => Tnf::NotFull,
        }
    }
    #[doc = "Transmit FIFO is full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Tnf::Full
    }
    #[doc = "Transmit FIFO is not full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Tnf::NotFull
    }
}
#[doc = "Receive FIFO empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    #[doc = "0: Receive FIFO is not empty."]
    NotEmpty = 0,
    #[doc = "1: Receive FIFO is empty."]
    Empty = 1,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO empty."]
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            false => Rfe::NotEmpty,
            true => Rfe::Empty,
        }
    }
    #[doc = "Receive FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Rfe::NotEmpty
    }
    #[doc = "Receive FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rfe::Empty
    }
}
#[doc = "Receive FIFO not full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnf {
    #[doc = "0: Receive FIFO is full."]
    Full = 0,
    #[doc = "1: Receive FIFO is not full."]
    NotFull = 1,
}
impl From<Rnf> for bool {
    #[inline(always)]
    fn from(variant: Rnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNF` reader - Receive FIFO not full"]
pub type RnfR = crate::BitReader<Rnf>;
impl RnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rnf {
        match self.bits {
            false => Rnf::Full,
            true => Rnf::NotFull,
        }
    }
    #[doc = "Receive FIFO is full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rnf::Full
    }
    #[doc = "Receive FIFO is not full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Rnf::NotFull
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: SPI is in idle mode."]
    Idle = 0,
    #[doc = "1: SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    Active = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Active,
        }
    }
    #[doc = "SPI is in idle mode."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO empty."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO not full"]
    #[inline(always)]
    pub fn rnf(&self) -> RnfR {
        RnfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1StatSpec;
impl crate::RegisterSpec for Spi1StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_stat::R`](R) reader structure"]
impl crate::Readable for Spi1StatSpec {}
#[doc = "`reset()` method sets SPI1_STAT to value 0"]
impl crate::Resettable for Spi1StatSpec {}
