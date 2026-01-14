#[repr(C)]
#[doc = "TIMG0_INT_EVENT\\[%s\\]"]
#[doc(alias = "TIMG0_INT_EVENT")]
pub struct Timg0IntEvent {
    timg0_iidx: Timg0Iidx,
    _reserved1: [u8; 0x04],
    timg0_imask: Timg0Imask,
    _reserved2: [u8; 0x04],
    timg0_ris: Timg0Ris,
    _reserved3: [u8; 0x04],
    timg0_mis: Timg0Mis,
    _reserved4: [u8; 0x04],
    timg0_iset: Timg0Iset,
    _reserved5: [u8; 0x04],
    timg0_iclr: Timg0Iclr,
}
impl Timg0IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn timg0_iidx(&self) -> &Timg0Iidx {
        &self.timg0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn timg0_imask(&self) -> &Timg0Imask {
        &self.timg0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn timg0_ris(&self) -> &Timg0Ris {
        &self.timg0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn timg0_mis(&self) -> &Timg0Mis {
        &self.timg0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn timg0_iset(&self) -> &Timg0Iset {
        &self.timg0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn timg0_iclr(&self) -> &Timg0Iclr {
        &self.timg0_iclr
    }
}
#[doc = "TIMG0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_iidx`] module"]
#[doc(alias = "TIMG0_IIDX")]
pub type Timg0Iidx = crate::Reg<timg0_iidx::Timg0IidxSpec>;
#[doc = "Interrupt index"]
pub mod timg0_iidx;
#[doc = "TIMG0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_imask`] module"]
#[doc(alias = "TIMG0_IMASK")]
pub type Timg0Imask = crate::Reg<timg0_imask::Timg0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod timg0_imask;
#[doc = "TIMG0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ris`] module"]
#[doc(alias = "TIMG0_RIS")]
pub type Timg0Ris = crate::Reg<timg0_ris::Timg0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod timg0_ris;
#[doc = "TIMG0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_mis`] module"]
#[doc(alias = "TIMG0_MIS")]
pub type Timg0Mis = crate::Reg<timg0_mis::Timg0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod timg0_mis;
#[doc = "TIMG0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_iset`] module"]
#[doc(alias = "TIMG0_ISET")]
pub type Timg0Iset = crate::Reg<timg0_iset::Timg0IsetSpec>;
#[doc = "Interrupt set"]
pub mod timg0_iset;
#[doc = "TIMG0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_iclr`] module"]
#[doc(alias = "TIMG0_ICLR")]
pub type Timg0Iclr = crate::Reg<timg0_iclr::Timg0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod timg0_iclr;
