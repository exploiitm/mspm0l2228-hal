#[repr(C)]
#[doc = "TIMG5_INT_EVENT\\[%s\\]"]
#[doc(alias = "TIMG5_INT_EVENT")]
pub struct Timg5IntEvent {
    timg5_iidx: Timg5Iidx,
    _reserved1: [u8; 0x04],
    timg5_imask: Timg5Imask,
    _reserved2: [u8; 0x04],
    timg5_ris: Timg5Ris,
    _reserved3: [u8; 0x04],
    timg5_mis: Timg5Mis,
    _reserved4: [u8; 0x04],
    timg5_iset: Timg5Iset,
    _reserved5: [u8; 0x04],
    timg5_iclr: Timg5Iclr,
}
impl Timg5IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn timg5_iidx(&self) -> &Timg5Iidx {
        &self.timg5_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn timg5_imask(&self) -> &Timg5Imask {
        &self.timg5_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn timg5_ris(&self) -> &Timg5Ris {
        &self.timg5_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn timg5_mis(&self) -> &Timg5Mis {
        &self.timg5_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn timg5_iset(&self) -> &Timg5Iset {
        &self.timg5_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn timg5_iclr(&self) -> &Timg5Iclr {
        &self.timg5_iclr
    }
}
#[doc = "TIMG5_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_iidx`] module"]
#[doc(alias = "TIMG5_IIDX")]
pub type Timg5Iidx = crate::Reg<timg5_iidx::Timg5IidxSpec>;
#[doc = "Interrupt index"]
pub mod timg5_iidx;
#[doc = "TIMG5_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_imask`] module"]
#[doc(alias = "TIMG5_IMASK")]
pub type Timg5Imask = crate::Reg<timg5_imask::Timg5ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod timg5_imask;
#[doc = "TIMG5_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ris`] module"]
#[doc(alias = "TIMG5_RIS")]
pub type Timg5Ris = crate::Reg<timg5_ris::Timg5RisSpec>;
#[doc = "Raw interrupt status"]
pub mod timg5_ris;
#[doc = "TIMG5_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_mis`] module"]
#[doc(alias = "TIMG5_MIS")]
pub type Timg5Mis = crate::Reg<timg5_mis::Timg5MisSpec>;
#[doc = "Masked interrupt status"]
pub mod timg5_mis;
#[doc = "TIMG5_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_iset`] module"]
#[doc(alias = "TIMG5_ISET")]
pub type Timg5Iset = crate::Reg<timg5_iset::Timg5IsetSpec>;
#[doc = "Interrupt set"]
pub mod timg5_iset;
#[doc = "TIMG5_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_iclr`] module"]
#[doc(alias = "TIMG5_ICLR")]
pub type Timg5Iclr = crate::Reg<timg5_iclr::Timg5IclrSpec>;
#[doc = "Interrupt clear"]
pub mod timg5_iclr;
