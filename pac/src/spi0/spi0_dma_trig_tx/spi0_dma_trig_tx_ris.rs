#[doc = "Register `SPI0_DMA_TRIG_TX_RIS` reader"]
pub type R = crate::R<Spi0DmaTrigTxRisSpec>;
#[doc = "Transmit FIFO event: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `TX` reader - Transmit FIFO event: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
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
    #[doc = "Bit 4 - Transmit FIFO event: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_tx_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0DmaTrigTxRisSpec;
impl crate::RegisterSpec for Spi0DmaTrigTxRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_dma_trig_tx_ris::R`](R) reader structure"]
impl crate::Readable for Spi0DmaTrigTxRisSpec {}
#[doc = "`reset()` method sets SPI0_DMA_TRIG_TX_RIS to value 0"]
impl crate::Resettable for Spi0DmaTrigTxRisSpec {}
