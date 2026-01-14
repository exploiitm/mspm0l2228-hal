#[repr(C)]
#[doc = "SPI1_DMA_TRIG_RX\\[%s\\]"]
#[doc(alias = "SPI1_DMA_TRIG_RX")]
pub struct Spi1DmaTrigRx {
    spi1_dma_trig_rx_iidx: Spi1DmaTrigRxIidx,
    _reserved1: [u8; 0x04],
    spi1_dma_trig_rx_imask: Spi1DmaTrigRxImask,
    _reserved2: [u8; 0x04],
    spi1_dma_trig_rx_ris: Spi1DmaTrigRxRis,
    _reserved3: [u8; 0x04],
    spi1_dma_trig_rx_mis: Spi1DmaTrigRxMis,
    _reserved4: [u8; 0x04],
    spi1_dma_trig_rx_iset: Spi1DmaTrigRxIset,
    _reserved5: [u8; 0x04],
    spi1_dma_trig_rx_iclr: Spi1DmaTrigRxIclr,
}
impl Spi1DmaTrigRx {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx_iidx(&self) -> &Spi1DmaTrigRxIidx {
        &self.spi1_dma_trig_rx_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx_imask(&self) -> &Spi1DmaTrigRxImask {
        &self.spi1_dma_trig_rx_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx_ris(&self) -> &Spi1DmaTrigRxRis {
        &self.spi1_dma_trig_rx_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx_mis(&self) -> &Spi1DmaTrigRxMis {
        &self.spi1_dma_trig_rx_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx_iset(&self) -> &Spi1DmaTrigRxIset {
        &self.spi1_dma_trig_rx_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx_iclr(&self) -> &Spi1DmaTrigRxIclr {
        &self.spi1_dma_trig_rx_iclr
    }
}
#[doc = "SPI1_DMA_TRIG_RX_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_rx_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_rx_iidx`] module"]
#[doc(alias = "SPI1_DMA_TRIG_RX_IIDX")]
pub type Spi1DmaTrigRxIidx = crate::Reg<spi1_dma_trig_rx_iidx::Spi1DmaTrigRxIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod spi1_dma_trig_rx_iidx;
#[doc = "SPI1_DMA_TRIG_RX_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_rx_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_rx_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_rx_imask`] module"]
#[doc(alias = "SPI1_DMA_TRIG_RX_IMASK")]
pub type Spi1DmaTrigRxImask = crate::Reg<spi1_dma_trig_rx_imask::Spi1DmaTrigRxImaskSpec>;
#[doc = "Interrupt mask"]
pub mod spi1_dma_trig_rx_imask;
#[doc = "SPI1_DMA_TRIG_RX_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_rx_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_rx_ris`] module"]
#[doc(alias = "SPI1_DMA_TRIG_RX_RIS")]
pub type Spi1DmaTrigRxRis = crate::Reg<spi1_dma_trig_rx_ris::Spi1DmaTrigRxRisSpec>;
#[doc = "Raw interrupt status"]
pub mod spi1_dma_trig_rx_ris;
#[doc = "SPI1_DMA_TRIG_RX_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_rx_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_rx_mis`] module"]
#[doc(alias = "SPI1_DMA_TRIG_RX_MIS")]
pub type Spi1DmaTrigRxMis = crate::Reg<spi1_dma_trig_rx_mis::Spi1DmaTrigRxMisSpec>;
#[doc = "Masked interrupt status"]
pub mod spi1_dma_trig_rx_mis;
#[doc = "SPI1_DMA_TRIG_RX_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_rx_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_rx_iset`] module"]
#[doc(alias = "SPI1_DMA_TRIG_RX_ISET")]
pub type Spi1DmaTrigRxIset = crate::Reg<spi1_dma_trig_rx_iset::Spi1DmaTrigRxIsetSpec>;
#[doc = "Interrupt set"]
pub mod spi1_dma_trig_rx_iset;
#[doc = "SPI1_DMA_TRIG_RX_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_rx_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_dma_trig_rx_iclr`] module"]
#[doc(alias = "SPI1_DMA_TRIG_RX_ICLR")]
pub type Spi1DmaTrigRxIclr = crate::Reg<spi1_dma_trig_rx_iclr::Spi1DmaTrigRxIclrSpec>;
#[doc = "Interrupt clear"]
pub mod spi1_dma_trig_rx_iclr;
