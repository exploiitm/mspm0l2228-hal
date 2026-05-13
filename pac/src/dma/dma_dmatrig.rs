#[repr(C)]
#[doc = "DMA_DMATRIG\\[%s\\]"]
#[doc(alias = "DMA_DMATRIG")]
pub struct DmaDmatrig {
    dma_dmatctl: DmaDmatctl,
}
impl DmaDmatrig {
    #[doc = "0x00 - DMA Trigger Select"]
    #[inline(always)]
    pub const fn dma_dmatctl(&self) -> &DmaDmatctl {
        &self.dma_dmatctl
    }
}
#[doc = "DMA_DMATCTL (rw) register accessor: DMA Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmatctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmatctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dmatctl`] module"]
#[doc(alias = "DMA_DMATCTL")]
pub type DmaDmatctl = crate::Reg<dma_dmatctl::DmaDmatctlSpec>;
#[doc = "DMA Trigger Select"]
pub mod dma_dmatctl;
