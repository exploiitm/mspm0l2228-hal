#[repr(C)]
#[doc = "LFSS_GEN_EVENT\\[%s\\]"]
#[doc(alias = "LFSS_GEN_EVENT")]
pub struct LfssGenEvent {
    lfss_gen_event_iidx: LfssGenEventIidx,
    _reserved1: [u8; 0x04],
    lfss_gen_event_imask: LfssGenEventImask,
    _reserved2: [u8; 0x04],
    lfss_gen_event_ris: LfssGenEventRis,
    _reserved3: [u8; 0x04],
    lfss_gen_event_mis: LfssGenEventMis,
    _reserved4: [u8; 0x04],
    lfss_gen_event_iset: LfssGenEventIset,
    _reserved5: [u8; 0x04],
    lfss_gen_event_iclr: LfssGenEventIclr,
}
impl LfssGenEvent {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn lfss_gen_event_iidx(&self) -> &LfssGenEventIidx {
        &self.lfss_gen_event_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn lfss_gen_event_imask(&self) -> &LfssGenEventImask {
        &self.lfss_gen_event_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn lfss_gen_event_ris(&self) -> &LfssGenEventRis {
        &self.lfss_gen_event_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn lfss_gen_event_mis(&self) -> &LfssGenEventMis {
        &self.lfss_gen_event_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn lfss_gen_event_iset(&self) -> &LfssGenEventIset {
        &self.lfss_gen_event_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn lfss_gen_event_iclr(&self) -> &LfssGenEventIclr {
        &self.lfss_gen_event_iclr
    }
}
#[doc = "LFSS_GEN_EVENT_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_gen_event_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_gen_event_iidx`] module"]
#[doc(alias = "LFSS_GEN_EVENT_IIDX")]
pub type LfssGenEventIidx = crate::Reg<lfss_gen_event_iidx::LfssGenEventIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod lfss_gen_event_iidx;
#[doc = "LFSS_GEN_EVENT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_gen_event_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_gen_event_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_gen_event_imask`] module"]
#[doc(alias = "LFSS_GEN_EVENT_IMASK")]
pub type LfssGenEventImask = crate::Reg<lfss_gen_event_imask::LfssGenEventImaskSpec>;
#[doc = "Interrupt mask"]
pub mod lfss_gen_event_imask;
#[doc = "LFSS_GEN_EVENT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_gen_event_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_gen_event_ris`] module"]
#[doc(alias = "LFSS_GEN_EVENT_RIS")]
pub type LfssGenEventRis = crate::Reg<lfss_gen_event_ris::LfssGenEventRisSpec>;
#[doc = "Raw interrupt status"]
pub mod lfss_gen_event_ris;
#[doc = "LFSS_GEN_EVENT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_gen_event_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_gen_event_mis`] module"]
#[doc(alias = "LFSS_GEN_EVENT_MIS")]
pub type LfssGenEventMis = crate::Reg<lfss_gen_event_mis::LfssGenEventMisSpec>;
#[doc = "Masked interrupt status"]
pub mod lfss_gen_event_mis;
#[doc = "LFSS_GEN_EVENT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_gen_event_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_gen_event_iset`] module"]
#[doc(alias = "LFSS_GEN_EVENT_ISET")]
pub type LfssGenEventIset = crate::Reg<lfss_gen_event_iset::LfssGenEventIsetSpec>;
#[doc = "Interrupt set"]
pub mod lfss_gen_event_iset;
#[doc = "LFSS_GEN_EVENT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_gen_event_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_gen_event_iclr`] module"]
#[doc(alias = "LFSS_GEN_EVENT_ICLR")]
pub type LfssGenEventIclr = crate::Reg<lfss_gen_event_iclr::LfssGenEventIclrSpec>;
#[doc = "Interrupt clear"]
pub mod lfss_gen_event_iclr;
