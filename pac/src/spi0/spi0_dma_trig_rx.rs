#[repr(C)]
#[doc = "SPI0_DMA_TRIG_RX\\[%s\\]"]
#[doc(alias = "SPI0_DMA_TRIG_RX")]
pub struct Spi0DmaTrigRx {
    spi0_dma_trig_rx_iidx: Spi0DmaTrigRxIidx,
    _reserved1: [u8; 0x04],
    spi0_dma_trig_rx_imask: Spi0DmaTrigRxImask,
    _reserved2: [u8; 0x04],
    spi0_dma_trig_rx_ris: Spi0DmaTrigRxRis,
    _reserved3: [u8; 0x04],
    spi0_dma_trig_rx_mis: Spi0DmaTrigRxMis,
    _reserved4: [u8; 0x04],
    spi0_dma_trig_rx_iset: Spi0DmaTrigRxIset,
    _reserved5: [u8; 0x04],
    spi0_dma_trig_rx_iclr: Spi0DmaTrigRxIclr,
}
impl Spi0DmaTrigRx {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx_iidx(&self) -> &Spi0DmaTrigRxIidx {
        &self.spi0_dma_trig_rx_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx_imask(&self) -> &Spi0DmaTrigRxImask {
        &self.spi0_dma_trig_rx_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx_ris(&self) -> &Spi0DmaTrigRxRis {
        &self.spi0_dma_trig_rx_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx_mis(&self) -> &Spi0DmaTrigRxMis {
        &self.spi0_dma_trig_rx_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx_iset(&self) -> &Spi0DmaTrigRxIset {
        &self.spi0_dma_trig_rx_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx_iclr(&self) -> &Spi0DmaTrigRxIclr {
        &self.spi0_dma_trig_rx_iclr
    }
}
#[doc = "SPI0_DMA_TRIG_RX_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_rx_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_rx_iidx`] module"]
#[doc(alias = "SPI0_DMA_TRIG_RX_IIDX")]
pub type Spi0DmaTrigRxIidx = crate::Reg<spi0_dma_trig_rx_iidx::Spi0DmaTrigRxIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod spi0_dma_trig_rx_iidx;
#[doc = "SPI0_DMA_TRIG_RX_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_rx_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_rx_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_rx_imask`] module"]
#[doc(alias = "SPI0_DMA_TRIG_RX_IMASK")]
pub type Spi0DmaTrigRxImask = crate::Reg<spi0_dma_trig_rx_imask::Spi0DmaTrigRxImaskSpec>;
#[doc = "Interrupt mask"]
pub mod spi0_dma_trig_rx_imask;
#[doc = "SPI0_DMA_TRIG_RX_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_rx_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_rx_ris`] module"]
#[doc(alias = "SPI0_DMA_TRIG_RX_RIS")]
pub type Spi0DmaTrigRxRis = crate::Reg<spi0_dma_trig_rx_ris::Spi0DmaTrigRxRisSpec>;
#[doc = "Raw interrupt status"]
pub mod spi0_dma_trig_rx_ris;
#[doc = "SPI0_DMA_TRIG_RX_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_rx_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_rx_mis`] module"]
#[doc(alias = "SPI0_DMA_TRIG_RX_MIS")]
pub type Spi0DmaTrigRxMis = crate::Reg<spi0_dma_trig_rx_mis::Spi0DmaTrigRxMisSpec>;
#[doc = "Masked interrupt status"]
pub mod spi0_dma_trig_rx_mis;
#[doc = "SPI0_DMA_TRIG_RX_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_rx_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_rx_iset`] module"]
#[doc(alias = "SPI0_DMA_TRIG_RX_ISET")]
pub type Spi0DmaTrigRxIset = crate::Reg<spi0_dma_trig_rx_iset::Spi0DmaTrigRxIsetSpec>;
#[doc = "Interrupt set"]
pub mod spi0_dma_trig_rx_iset;
#[doc = "SPI0_DMA_TRIG_RX_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_rx_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_dma_trig_rx_iclr`] module"]
#[doc(alias = "SPI0_DMA_TRIG_RX_ICLR")]
pub type Spi0DmaTrigRxIclr = crate::Reg<spi0_dma_trig_rx_iclr::Spi0DmaTrigRxIclrSpec>;
#[doc = "Interrupt clear"]
pub mod spi0_dma_trig_rx_iclr;
