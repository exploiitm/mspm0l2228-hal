#[doc = "Register `SPI1_DMA_TRIG_TX_IIDX` reader"]
pub type R = crate::R<Spi1DmaTrigTxIidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "5: Transmit Event/interrupt pending"]
    TxEvt = 5,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            5 => Some(Stat::TxEvt),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_tx_evt(&self) -> bool {
        *self == Stat::TxEvt
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1DmaTrigTxIidxSpec;
impl crate::RegisterSpec for Spi1DmaTrigTxIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_dma_trig_tx_iidx::R`](R) reader structure"]
impl crate::Readable for Spi1DmaTrigTxIidxSpec {}
#[doc = "`reset()` method sets SPI1_DMA_TRIG_TX_IIDX to value 0"]
impl crate::Resettable for Spi1DmaTrigTxIidxSpec {}
