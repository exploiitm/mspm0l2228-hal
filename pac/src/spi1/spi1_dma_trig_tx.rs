#[repr(C)]
#[doc = "SPI1_DMA_TRIG_TX\\[%s\\]"]
#[doc(alias = "SPI1_DMA_TRIG_TX")]
pub struct Spi1DmaTrigTx {
    spi1_dma_trig_tx_iidx: Spi1DmaTrigTxIidx,
    _reserved1: [u8; 0x04],
    spi1_dma_trig_tx_imask: Spi1DmaTrigTxImask,
    _reserved2: [u8; 0x04],
    spi1_dma_trig_tx_ris: Spi1DmaTrigTxRis,
    _reserved3: [u8; 0x04],
    spi1_dma_trig_tx_mis: Spi1DmaTrigTxMis,
    _reserved4: [u8; 0x04],
    spi1_dma_trig_tx_iset: Spi1DmaTrigTxIset,
    _reserved5: [u8; 0x04],
    spi1_dma_trig_tx_iclr: Spi1DmaTrigTxIclr,
}
impl Spi1DmaTrigTx {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx_iidx(&self) -> &Spi1DmaTrigTxIidx {
        &self.spi1_dma_trig_tx_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx_imask(&self) -> &Spi1DmaTrigTxImask {
        &self.spi1_dma_trig_tx_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx_ris(&self) -> &Spi1DmaTrigTxRis {
        &self.spi1_dma_trig_tx_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx_mis(&self) -> &Spi1DmaTrigTxMis {
        &self.spi1_dma_trig_tx_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx_iset(&self) -> &Spi1DmaTrigTxIset {
        &self.spi1_dma_trig_tx_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx_iclr(&self) -> &Spi1DmaTrigTxIclr {
        &self.spi1_dma_trig_tx_iclr
    }
}
#[doc = "SPI1_DMA_TRIG_TX_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_tx_iidx`] module"]
#[doc(alias = "SPI1_DMA_TRIG_TX_IIDX")]
pub type Spi1DmaTrigTxIidx = crate::Reg<spi1_dma_trig_tx_iidx::Spi1DmaTrigTxIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod spi1_dma_trig_tx_iidx;
#[doc = "SPI1_DMA_TRIG_TX_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_tx_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_tx_imask`] module"]
#[doc(alias = "SPI1_DMA_TRIG_TX_IMASK")]
pub type Spi1DmaTrigTxImask = crate::Reg<spi1_dma_trig_tx_imask::Spi1DmaTrigTxImaskSpec>;
#[doc = "Interrupt mask"]
pub mod spi1_dma_trig_tx_imask;
#[doc = "SPI1_DMA_TRIG_TX_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_tx_ris`] module"]
#[doc(alias = "SPI1_DMA_TRIG_TX_RIS")]
pub type Spi1DmaTrigTxRis = crate::Reg<spi1_dma_trig_tx_ris::Spi1DmaTrigTxRisSpec>;
#[doc = "Raw interrupt status"]
pub mod spi1_dma_trig_tx_ris;
#[doc = "SPI1_DMA_TRIG_TX_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_tx_mis`] module"]
#[doc(alias = "SPI1_DMA_TRIG_TX_MIS")]
pub type Spi1DmaTrigTxMis = crate::Reg<spi1_dma_trig_tx_mis::Spi1DmaTrigTxMisSpec>;
#[doc = "Masked interrupt status"]
pub mod spi1_dma_trig_tx_mis;
#[doc = "SPI1_DMA_TRIG_TX_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_tx_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_tx_iset`] module"]
#[doc(alias = "SPI1_DMA_TRIG_TX_ISET")]
pub type Spi1DmaTrigTxIset = crate::Reg<spi1_dma_trig_tx_iset::Spi1DmaTrigTxIsetSpec>;
#[doc = "Interrupt set"]
pub mod spi1_dma_trig_tx_iset;
#[doc = "SPI1_DMA_TRIG_TX_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_tx_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_tx_iclr`] module"]
#[doc(alias = "SPI1_DMA_TRIG_TX_ICLR")]
pub type Spi1DmaTrigTxIclr = crate::Reg<spi1_dma_trig_tx_iclr::Spi1DmaTrigTxIclrSpec>;
#[doc = "Interrupt clear"]
pub mod spi1_dma_trig_tx_iclr;
