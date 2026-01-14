#[repr(C)]
#[doc = "DMA_GEN_EVENT\\[%s\\]"]
#[doc(alias = "DMA_GEN_EVENT")]
pub struct DmaGenEvent {
    dma_gen_event_iidx: DmaGenEventIidx,
    _reserved1: [u8; 0x04],
    dma_gen_event_imask: DmaGenEventImask,
    _reserved2: [u8; 0x04],
    dma_gen_event_ris: DmaGenEventRis,
    _reserved3: [u8; 0x04],
    dma_gen_event_mis: DmaGenEventMis,
    _reserved4: [u8; 0x04],
    dma_gen_event_iset: DmaGenEventIset,
    _reserved5: [u8; 0x04],
    dma_gen_event_iclr: DmaGenEventIclr,
}
impl DmaGenEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn dma_gen_event_iidx(&self) -> &DmaGenEventIidx {
        &self.dma_gen_event_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn dma_gen_event_imask(&self) -> &DmaGenEventImask {
        &self.dma_gen_event_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn dma_gen_event_ris(&self) -> &DmaGenEventRis {
        &self.dma_gen_event_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn dma_gen_event_mis(&self) -> &DmaGenEventMis {
        &self.dma_gen_event_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn dma_gen_event_iset(&self) -> &DmaGenEventIset {
        &self.dma_gen_event_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn dma_gen_event_iclr(&self) -> &DmaGenEventIclr {
        &self.dma_gen_event_iclr
    }
}
#[doc = "DMA_GEN_EVENT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_gen_event_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gen_event_iidx`] module"]
#[doc(alias = "DMA_GEN_EVENT_IIDX")]
pub type DmaGenEventIidx = crate::Reg<dma_gen_event_iidx::DmaGenEventIidxSpec>;
#[doc = "Interrupt index"]
pub mod dma_gen_event_iidx;
#[doc = "DMA_GEN_EVENT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_gen_event_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_gen_event_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gen_event_imask`] module"]
#[doc(alias = "DMA_GEN_EVENT_IMASK")]
pub type DmaGenEventImask = crate::Reg<dma_gen_event_imask::DmaGenEventImaskSpec>;
#[doc = "Interrupt mask"]
pub mod dma_gen_event_imask;
#[doc = "DMA_GEN_EVENT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_gen_event_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gen_event_ris`] module"]
#[doc(alias = "DMA_GEN_EVENT_RIS")]
pub type DmaGenEventRis = crate::Reg<dma_gen_event_ris::DmaGenEventRisSpec>;
#[doc = "Raw interrupt status"]
pub mod dma_gen_event_ris;
#[doc = "DMA_GEN_EVENT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_gen_event_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gen_event_mis`] module"]
#[doc(alias = "DMA_GEN_EVENT_MIS")]
pub type DmaGenEventMis = crate::Reg<dma_gen_event_mis::DmaGenEventMisSpec>;
#[doc = "Masked interrupt status"]
pub mod dma_gen_event_mis;
#[doc = "DMA_GEN_EVENT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_gen_event_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gen_event_iset`] module"]
#[doc(alias = "DMA_GEN_EVENT_ISET")]
pub type DmaGenEventIset = crate::Reg<dma_gen_event_iset::DmaGenEventIsetSpec>;
#[doc = "Interrupt set"]
pub mod dma_gen_event_iset;
#[doc = "DMA_GEN_EVENT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_gen_event_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gen_event_iclr`] module"]
#[doc(alias = "DMA_GEN_EVENT_ICLR")]
pub type DmaGenEventIclr = crate::Reg<dma_gen_event_iclr::DmaGenEventIclrSpec>;
#[doc = "Interrupt clear"]
pub mod dma_gen_event_iclr;
