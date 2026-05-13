#[repr(C)]
#[doc = "DMA_CPU_INT\\[%s\\]"]
#[doc(alias = "DMA_CPU_INT")]
pub struct DmaCpuInt {
    dma_cpu_int_iidx: DmaCpuIntIidx,
    _reserved1: [u8; 0x04],
    dma_cpu_int_imask: DmaCpuIntImask,
    _reserved2: [u8; 0x04],
    dma_cpu_int_ris: DmaCpuIntRis,
    _reserved3: [u8; 0x04],
    dma_cpu_int_mis: DmaCpuIntMis,
    _reserved4: [u8; 0x04],
    dma_cpu_int_iset: DmaCpuIntIset,
    _reserved5: [u8; 0x04],
    dma_cpu_int_iclr: DmaCpuIntIclr,
}
impl DmaCpuInt {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn dma_cpu_int_iidx(&self) -> &DmaCpuIntIidx {
        &self.dma_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn dma_cpu_int_imask(&self) -> &DmaCpuIntImask {
        &self.dma_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn dma_cpu_int_ris(&self) -> &DmaCpuIntRis {
        &self.dma_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn dma_cpu_int_mis(&self) -> &DmaCpuIntMis {
        &self.dma_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn dma_cpu_int_iset(&self) -> &DmaCpuIntIset {
        &self.dma_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn dma_cpu_int_iclr(&self) -> &DmaCpuIntIclr {
        &self.dma_cpu_int_iclr
    }
}
#[doc = "DMA_CPU_INT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpu_int_iidx`] module"]
#[doc(alias = "DMA_CPU_INT_IIDX")]
pub type DmaCpuIntIidx = crate::Reg<dma_cpu_int_iidx::DmaCpuIntIidxSpec>;
#[doc = "Interrupt index"]
pub mod dma_cpu_int_iidx;
#[doc = "DMA_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpu_int_imask`] module"]
#[doc(alias = "DMA_CPU_INT_IMASK")]
pub type DmaCpuIntImask = crate::Reg<dma_cpu_int_imask::DmaCpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod dma_cpu_int_imask;
#[doc = "DMA_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpu_int_ris`] module"]
#[doc(alias = "DMA_CPU_INT_RIS")]
pub type DmaCpuIntRis = crate::Reg<dma_cpu_int_ris::DmaCpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod dma_cpu_int_ris;
#[doc = "DMA_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpu_int_mis`] module"]
#[doc(alias = "DMA_CPU_INT_MIS")]
pub type DmaCpuIntMis = crate::Reg<dma_cpu_int_mis::DmaCpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod dma_cpu_int_mis;
#[doc = "DMA_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpu_int_iset`] module"]
#[doc(alias = "DMA_CPU_INT_ISET")]
pub type DmaCpuIntIset = crate::Reg<dma_cpu_int_iset::DmaCpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod dma_cpu_int_iset;
#[doc = "DMA_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpu_int_iclr`] module"]
#[doc(alias = "DMA_CPU_INT_ICLR")]
pub type DmaCpuIntIclr = crate::Reg<dma_cpu_int_iclr::DmaCpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod dma_cpu_int_iclr;
