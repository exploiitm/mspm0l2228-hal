#[repr(C)]
#[doc = "TIMG4_INT_EVENT\\[%s\\]"]
#[doc(alias = "TIMG4_INT_EVENT")]
pub struct Timg4IntEvent {
    timg4_iidx: Timg4Iidx,
    _reserved1: [u8; 0x04],
    timg4_imask: Timg4Imask,
    _reserved2: [u8; 0x04],
    timg4_ris: Timg4Ris,
    _reserved3: [u8; 0x04],
    timg4_mis: Timg4Mis,
    _reserved4: [u8; 0x04],
    timg4_iset: Timg4Iset,
    _reserved5: [u8; 0x04],
    timg4_iclr: Timg4Iclr,
}
impl Timg4IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn timg4_iidx(&self) -> &Timg4Iidx {
        &self.timg4_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn timg4_imask(&self) -> &Timg4Imask {
        &self.timg4_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn timg4_ris(&self) -> &Timg4Ris {
        &self.timg4_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn timg4_mis(&self) -> &Timg4Mis {
        &self.timg4_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn timg4_iset(&self) -> &Timg4Iset {
        &self.timg4_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn timg4_iclr(&self) -> &Timg4Iclr {
        &self.timg4_iclr
    }
}
#[doc = "TIMG4_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_iidx`] module"]
#[doc(alias = "TIMG4_IIDX")]
pub type Timg4Iidx = crate::Reg<timg4_iidx::Timg4IidxSpec>;
#[doc = "Interrupt index"]
pub mod timg4_iidx;
#[doc = "TIMG4_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_imask`] module"]
#[doc(alias = "TIMG4_IMASK")]
pub type Timg4Imask = crate::Reg<timg4_imask::Timg4ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod timg4_imask;
#[doc = "TIMG4_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ris`] module"]
#[doc(alias = "TIMG4_RIS")]
pub type Timg4Ris = crate::Reg<timg4_ris::Timg4RisSpec>;
#[doc = "Raw interrupt status"]
pub mod timg4_ris;
#[doc = "TIMG4_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_mis`] module"]
#[doc(alias = "TIMG4_MIS")]
pub type Timg4Mis = crate::Reg<timg4_mis::Timg4MisSpec>;
#[doc = "Masked interrupt status"]
pub mod timg4_mis;
#[doc = "TIMG4_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_iset`] module"]
#[doc(alias = "TIMG4_ISET")]
pub type Timg4Iset = crate::Reg<timg4_iset::Timg4IsetSpec>;
#[doc = "Interrupt set"]
pub mod timg4_iset;
#[doc = "TIMG4_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_iclr`] module"]
#[doc(alias = "TIMG4_ICLR")]
pub type Timg4Iclr = crate::Reg<timg4_iclr::Timg4IclrSpec>;
#[doc = "Interrupt clear"]
pub mod timg4_iclr;
