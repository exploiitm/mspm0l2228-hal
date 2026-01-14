#[doc = "Register `SPI1_DMA_TRIG_TX_MIS` reader"]
pub type R = crate::R<Spi1DmaTrigTxMisSpec>;
#[doc = "Masked Transmit FIFO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Masked Transmit FIFO event"]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::Clr,
            true => Tx::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tx::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tx::Set
    }
}
impl R {
    #[doc = "Bit 4 - Masked Transmit FIFO event"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1DmaTrigTxMisSpec;
impl crate::RegisterSpec for Spi1DmaTrigTxMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_dma_trig_tx_mis::R`](R) reader structure"]
impl crate::Readable for Spi1DmaTrigTxMisSpec {}
#[doc = "`reset()` method sets SPI1_DMA_TRIG_TX_MIS to value 0"]
impl crate::Resettable for Spi1DmaTrigTxMisSpec {}
