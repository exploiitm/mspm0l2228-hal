#[repr(C)]
#[doc = "DMA_DMACHAN\\[%s\\]"]
#[doc(alias = "DMA_DMACHAN")]
pub struct DmaDmachan {
    dma_dmactl: DmaDmactl,
    dma_dmasa: DmaDmasa,
    dma_dmada: DmaDmada,
    dma_dmasz: DmaDmasz,
}
impl DmaDmachan {
    #[doc = "0x00 - DMA Channel Control"]
    #[inline(always)]
    pub const fn dma_dmactl(&self) -> &DmaDmactl {
        &self.dma_dmactl
    }
    #[doc = "0x04 - DMA Channel Source Address"]
    #[inline(always)]
    pub const fn dma_dmasa(&self) -> &DmaDmasa {
        &self.dma_dmasa
    }
    #[doc = "0x08 - DMA Channel Destination Address"]
    #[inline(always)]
    pub const fn dma_dmada(&self) -> &DmaDmada {
        &self.dma_dmada
    }
    #[doc = "0x0c - DMA Channel Size"]
    #[inline(always)]
    pub const fn dma_dmasz(&self) -> &DmaDmasz {
        &self.dma_dmasz
    }
}
#[doc = "DMA_DMACTL (rw) register accessor: DMA Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dmactl`] module"]
#[doc(alias = "DMA_DMACTL")]
pub type DmaDmactl = crate::Reg<dma_dmactl::DmaDmactlSpec>;
#[doc = "DMA Channel Control"]
pub mod dma_dmactl;
#[doc = "DMA_DMASA (rw) register accessor: DMA Channel Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmasa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmasa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dmasa`] module"]
#[doc(alias = "DMA_DMASA")]
pub type DmaDmasa = crate::Reg<dma_dmasa::DmaDmasaSpec>;
#[doc = "DMA Channel Source Address"]
pub mod dma_dmasa;
#[doc = "DMA_DMADA (rw) register accessor: DMA Channel Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmada::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmada::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dmada`] module"]
#[doc(alias = "DMA_DMADA")]
pub type DmaDmada = crate::Reg<dma_dmada::DmaDmadaSpec>;
#[doc = "DMA Channel Destination Address"]
pub mod dma_dmada;
#[doc = "DMA_DMASZ (rw) register accessor: DMA Channel Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmasz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmasz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dmasz`] module"]
#[doc(alias = "DMA_DMASZ")]
pub type DmaDmasz = crate::Reg<dma_dmasz::DmaDmaszSpec>;
#[doc = "DMA Channel Size"]
pub mod dma_dmasz;
