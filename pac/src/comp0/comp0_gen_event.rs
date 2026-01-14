#[repr(C)]
#[doc = "COMP0_GEN_EVENT\\[%s\\]"]
#[doc(alias = "COMP0_GEN_EVENT")]
pub struct Comp0GenEvent {
    comp0_gen_event_iidx: Comp0GenEventIidx,
    _reserved1: [u8; 0x04],
    comp0_gen_event_imask: Comp0GenEventImask,
    _reserved2: [u8; 0x04],
    comp0_gen_event_ris: Comp0GenEventRis,
    _reserved3: [u8; 0x04],
    comp0_gen_event_mis: Comp0GenEventMis,
    _reserved4: [u8; 0x04],
    comp0_gen_event_iset: Comp0GenEventIset,
    _reserved5: [u8; 0x04],
    comp0_gen_event_iclr: Comp0GenEventIclr,
}
impl Comp0GenEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn comp0_gen_event_iidx(&self) -> &Comp0GenEventIidx {
        &self.comp0_gen_event_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn comp0_gen_event_imask(&self) -> &Comp0GenEventImask {
        &self.comp0_gen_event_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn comp0_gen_event_ris(&self) -> &Comp0GenEventRis {
        &self.comp0_gen_event_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn comp0_gen_event_mis(&self) -> &Comp0GenEventMis {
        &self.comp0_gen_event_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn comp0_gen_event_iset(&self) -> &Comp0GenEventIset {
        &self.comp0_gen_event_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn comp0_gen_event_iclr(&self) -> &Comp0GenEventIclr {
        &self.comp0_gen_event_iclr
    }
}
#[doc = "COMP0_GEN_EVENT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_gen_event_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gen_event_iidx`] module"]
#[doc(alias = "COMP0_GEN_EVENT_IIDX")]
pub type Comp0GenEventIidx = crate::Reg<comp0_gen_event_iidx::Comp0GenEventIidxSpec>;
#[doc = "Interrupt index"]
pub mod comp0_gen_event_iidx;
#[doc = "COMP0_GEN_EVENT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_gen_event_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_gen_event_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gen_event_imask`] module"]
#[doc(alias = "COMP0_GEN_EVENT_IMASK")]
pub type Comp0GenEventImask = crate::Reg<comp0_gen_event_imask::Comp0GenEventImaskSpec>;
#[doc = "Interrupt mask"]
pub mod comp0_gen_event_imask;
#[doc = "COMP0_GEN_EVENT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_gen_event_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gen_event_ris`] module"]
#[doc(alias = "COMP0_GEN_EVENT_RIS")]
pub type Comp0GenEventRis = crate::Reg<comp0_gen_event_ris::Comp0GenEventRisSpec>;
#[doc = "Raw interrupt status"]
pub mod comp0_gen_event_ris;
#[doc = "COMP0_GEN_EVENT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_gen_event_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gen_event_mis`] module"]
#[doc(alias = "COMP0_GEN_EVENT_MIS")]
pub type Comp0GenEventMis = crate::Reg<comp0_gen_event_mis::Comp0GenEventMisSpec>;
#[doc = "Masked interrupt status"]
pub mod comp0_gen_event_mis;
#[doc = "COMP0_GEN_EVENT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_gen_event_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gen_event_iset`] module"]
#[doc(alias = "COMP0_GEN_EVENT_ISET")]
pub type Comp0GenEventIset = crate::Reg<comp0_gen_event_iset::Comp0GenEventIsetSpec>;
#[doc = "Interrupt set"]
pub mod comp0_gen_event_iset;
#[doc = "COMP0_GEN_EVENT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_gen_event_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gen_event_iclr`] module"]
#[doc(alias = "COMP0_GEN_EVENT_ICLR")]
pub type Comp0GenEventIclr = crate::Reg<comp0_gen_event_iclr::Comp0GenEventIclrSpec>;
#[doc = "Interrupt clear"]
pub mod comp0_gen_event_iclr;
