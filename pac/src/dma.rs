#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    dma_fsub_0: DmaFsub0,
    dma_fsub_1: DmaFsub1,
    _reserved2: [u8; 0x3c],
    dma_fpub_1: DmaFpub1,
    _reserved3: [u8; 0x0bd0],
    dma_pdbgctl: DmaPdbgctl,
    _reserved4: [u8; 0x04],
    dma_cpu_int: [DmaCpuInt; 1],
    _reserved5: [u8; 0x04],
    dma_gen_event: [DmaGenEvent; 1],
    _reserved6: [u8; 0x64],
    dma_evt_mode: DmaEvtMode,
    _reserved7: [u8; 0x18],
    dma_desc: DmaDesc,
    dma_dmaprio: DmaDmaprio,
    _reserved9: [u8; 0x0c],
    dma_dmatrig: [DmaDmatrig; 7],
    _reserved10: [u8; 0xd4],
    dma_dmachan: [DmaDmachan; 7],
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Port 0"]
    #[inline(always)]
    pub const fn dma_fsub_0(&self) -> &DmaFsub0 {
        &self.dma_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn dma_fsub_1(&self) -> &DmaFsub1 {
        &self.dma_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn dma_fpub_1(&self) -> &DmaFpub1 {
        &self.dma_fpub_1
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn dma_pdbgctl(&self) -> &DmaPdbgctl {
        &self.dma_pdbgctl
    }
    #[doc = "0x1020..0x104c - DMA_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn dma_cpu_int(&self, n: usize) -> &DmaCpuInt {
        &self.dma_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - DMA_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn dma_cpu_int_iter(&self) -> impl Iterator<Item = &DmaCpuInt> {
        self.dma_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - DMA_GEN_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn dma_gen_event(&self, n: usize) -> &DmaGenEvent {
        &self.dma_gen_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - DMA_GEN_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn dma_gen_event_iter(&self) -> impl Iterator<Item = &DmaGenEvent> {
        self.dma_gen_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn dma_evt_mode(&self) -> &DmaEvtMode {
        &self.dma_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn dma_desc(&self) -> &DmaDesc {
        &self.dma_desc
    }
    #[doc = "0x1100 - DMA Channel Priority Control"]
    #[inline(always)]
    pub const fn dma_dmaprio(&self) -> &DmaDmaprio {
        &self.dma_dmaprio
    }
    #[doc = "0x1110..0x112c - DMA_DMATRIG\\[%s\\]"]
    #[inline(always)]
    pub const fn dma_dmatrig(&self, n: usize) -> &DmaDmatrig {
        &self.dma_dmatrig[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1110..0x112c - DMA_DMATRIG\\[%s\\]"]
    #[inline(always)]
    pub fn dma_dmatrig_iter(&self) -> impl Iterator<Item = &DmaDmatrig> {
        self.dma_dmatrig.iter()
    }
    #[doc = "0x1200..0x1270 - DMA_DMACHAN\\[%s\\]"]
    #[inline(always)]
    pub const fn dma_dmachan(&self, n: usize) -> &DmaDmachan {
        &self.dma_dmachan[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1200..0x1270 - DMA_DMACHAN\\[%s\\]"]
    #[inline(always)]
    pub fn dma_dmachan_iter(&self) -> impl Iterator<Item = &DmaDmachan> {
        self.dma_dmachan.iter()
    }
}
#[doc = "DMA_FSUB_0 (rw) register accessor: Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_fsub_0`] module"]
#[doc(alias = "DMA_FSUB_0")]
pub type DmaFsub0 = crate::Reg<dma_fsub_0::DmaFsub0Spec>;
#[doc = "Subscriber Port 0"]
pub mod dma_fsub_0;
#[doc = "DMA_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_fsub_1`] module"]
#[doc(alias = "DMA_FSUB_1")]
pub type DmaFsub1 = crate::Reg<dma_fsub_1::DmaFsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod dma_fsub_1;
#[doc = "DMA_FPUB_1 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_fpub_1`] module"]
#[doc(alias = "DMA_FPUB_1")]
pub type DmaFpub1 = crate::Reg<dma_fpub_1::DmaFpub1Spec>;
#[doc = "Publisher Port 0"]
pub mod dma_fpub_1;
#[doc = "DMA_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_pdbgctl`] module"]
#[doc(alias = "DMA_PDBGCTL")]
pub type DmaPdbgctl = crate::Reg<dma_pdbgctl::DmaPdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod dma_pdbgctl;
#[doc = "DMA_CPU_INT\\[%s\\]"]
pub use self::dma_cpu_int::DmaCpuInt;
#[doc = r"Cluster"]
#[doc = "DMA_CPU_INT\\[%s\\]"]
pub mod dma_cpu_int;
#[doc = "DMA_GEN_EVENT\\[%s\\]"]
pub use self::dma_gen_event::DmaGenEvent;
#[doc = r"Cluster"]
#[doc = "DMA_GEN_EVENT\\[%s\\]"]
pub mod dma_gen_event;
#[doc = "DMA_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_evt_mode`] module"]
#[doc(alias = "DMA_EVT_MODE")]
pub type DmaEvtMode = crate::Reg<dma_evt_mode::DmaEvtModeSpec>;
#[doc = "Event Mode"]
pub mod dma_evt_mode;
#[doc = "DMA_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_desc`] module"]
#[doc(alias = "DMA_DESC")]
pub type DmaDesc = crate::Reg<dma_desc::DmaDescSpec>;
#[doc = "Module Description"]
pub mod dma_desc;
#[doc = "DMA_DMAPRIO (rw) register accessor: DMA Channel Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmaprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmaprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dmaprio`] module"]
#[doc(alias = "DMA_DMAPRIO")]
pub type DmaDmaprio = crate::Reg<dma_dmaprio::DmaDmaprioSpec>;
#[doc = "DMA Channel Priority Control"]
pub mod dma_dmaprio;
#[doc = "DMA_DMATRIG\\[%s\\]"]
pub use self::dma_dmatrig::DmaDmatrig;
#[doc = r"Cluster"]
#[doc = "DMA_DMATRIG\\[%s\\]"]
pub mod dma_dmatrig;
#[doc = "DMA_DMACHAN\\[%s\\]"]
pub use self::dma_dmachan::DmaDmachan;
#[doc = r"Cluster"]
#[doc = "DMA_DMACHAN\\[%s\\]"]
pub mod dma_dmachan;
