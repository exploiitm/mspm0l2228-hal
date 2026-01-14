#[doc = "Register `I2C0_DMA_TRIG0_RIS` reader"]
pub type R = crate::R<I2c0DmaTrig0RisSpec>;
#[doc = "Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Crxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Crxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOTRG` reader - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
pub type CrxfifotrgR = crate::BitReader<Crxfifotrg>;
impl CrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crxfifotrg {
        match self.bits {
            false => Crxfifotrg::Clr,
            true => Crxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxfifotrg::Set
    }
}
#[doc = "Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ctxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ctxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXFIFOTRG` reader - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
pub type CtxfifotrgR = crate::BitReader<Ctxfifotrg>;
impl CtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctxfifotrg {
        match self.bits {
            false => Ctxfifotrg::Clr,
            true => Ctxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxfifotrg::Set
    }
}
#[doc = "Target Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Trxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Trxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOTRG` reader - Target Receive FIFO Trigger"]
pub type TrxfifotrgR = crate::BitReader<Trxfifotrg>;
impl TrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trxfifotrg {
        match self.bits {
            false => Trxfifotrg::Clr,
            true => Trxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxfifotrg::Set
    }
}
#[doc = "Target Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ttxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ttxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXFIFOTRG` reader - Target Transmit FIFO Trigger"]
pub type TtxfifotrgR = crate::BitReader<Ttxfifotrg>;
impl TtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttxfifotrg {
        match self.bits {
            false => Ttxfifotrg::Clr,
            true => Ttxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxfifotrg::Set
    }
}
impl R {
    #[doc = "Bit 0 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&self) -> CrxfifotrgR {
        CrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&self) -> CtxfifotrgR {
        CtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&self) -> TrxfifotrgR {
        TrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&self) -> TtxfifotrgR {
        TtxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0DmaTrig0RisSpec;
impl crate::RegisterSpec for I2c0DmaTrig0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_dma_trig0_ris::R`](R) reader structure"]
impl crate::Readable for I2c0DmaTrig0RisSpec {}
#[doc = "`reset()` method sets I2C0_DMA_TRIG0_RIS to value 0"]
impl crate::Resettable for I2c0DmaTrig0RisSpec {}
