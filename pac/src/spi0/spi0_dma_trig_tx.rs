#[repr(C)]
#[doc = "SPI0_DMA_TRIG_TX\\[%s\\]"]
#[doc(alias = "SPI0_DMA_TRIG_TX")]
pub struct Spi0DmaTrigTx {
    spi0_dma_trig_tx_iidx: Spi0DmaTrigTxIidx,
    _reserved1: [u8; 0x04],
    spi0_dma_trig_tx_imask: Spi0DmaTrigTxImask,
    _reserved2: [u8; 0x04],
    spi0_dma_trig_tx_ris: Spi0DmaTrigTxRis,
    _reserved3: [u8; 0x04],
    spi0_dma_trig_tx_mis: Spi0DmaTrigTxMis,
    _reserved4: [u8; 0x04],
    spi0_dma_trig_tx_iset: Spi0DmaTrigTxIset,
    _reserved5: [u8; 0x04],
    spi0_dma_trig_tx_iclr: Spi0DmaTrigTxIclr,
}
impl Spi0DmaTrigTx {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx_iidx(&self) -> &Spi0DmaTrigTxIidx {
        &self.spi0_dma_trig_tx_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx_imask(&self) -> &Spi0DmaTrigTxImask {
        &self.spi0_dma_trig_tx_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx_ris(&self) -> &Spi0DmaTrigTxRis {
        &self.spi0_dma_trig_tx_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx_mis(&self) -> &Spi0DmaTrigTxMis {
        &self.spi0_dma_trig_tx_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx_iset(&self) -> &Spi0DmaTrigTxIset {
        &self.spi0_dma_trig_tx_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx_iclr(&self) -> &Spi0DmaTrigTxIclr {
        &self.spi0_dma_trig_tx_iclr
    }
}
#[doc = "SPI0_DMA_TRIG_TX_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_tx_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_tx_iidx`] module"]
#[doc(alias = "SPI0_DMA_TRIG_TX_IIDX")]
pub type Spi0DmaTrigTxIidx = crate::Reg<spi0_dma_trig_tx_iidx::Spi0DmaTrigTxIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod spi0_dma_trig_tx_iidx;
#[doc = "SPI0_DMA_TRIG_TX_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_tx_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_tx_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_tx_imask`] module"]
#[doc(alias = "SPI0_DMA_TRIG_TX_IMASK")]
pub type Spi0DmaTrigTxImask = crate::Reg<spi0_dma_trig_tx_imask::Spi0DmaTrigTxImaskSpec>;
#[doc = "Interrupt mask"]
pub mod spi0_dma_trig_tx_imask;
#[doc = "SPI0_DMA_TRIG_TX_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_tx_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_tx_ris`] module"]
#[doc(alias = "SPI0_DMA_TRIG_TX_RIS")]
pub type Spi0DmaTrigTxRis = crate::Reg<spi0_dma_trig_tx_ris::Spi0DmaTrigTxRisSpec>;
#[doc = "Raw interrupt status"]
pub mod spi0_dma_trig_tx_ris;
#[doc = "SPI0_DMA_TRIG_TX_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_tx_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_tx_mis`] module"]
#[doc(alias = "SPI0_DMA_TRIG_TX_MIS")]
pub type Spi0DmaTrigTxMis = crate::Reg<spi0_dma_trig_tx_mis::Spi0DmaTrigTxMisSpec>;
#[doc = "Masked interrupt status"]
pub mod spi0_dma_trig_tx_mis;
#[doc = "SPI0_DMA_TRIG_TX_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_tx_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_tx_iset`] module"]
#[doc(alias = "SPI0_DMA_TRIG_TX_ISET")]
pub type Spi0DmaTrigTxIset = crate::Reg<spi0_dma_trig_tx_iset::Spi0DmaTrigTxIsetSpec>;
#[doc = "Interrupt set"]
pub mod spi0_dma_trig_tx_iset;
#[doc = "SPI0_DMA_TRIG_TX_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_tx_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_tx_iclr`] module"]
#[doc(alias = "SPI0_DMA_TRIG_TX_ICLR")]
pub type Spi0DmaTrigTxIclr = crate::Reg<spi0_dma_trig_tx_iclr::Spi0DmaTrigTxIclrSpec>;
#[doc = "Interrupt clear"]
pub mod spi0_dma_trig_tx_iclr;
