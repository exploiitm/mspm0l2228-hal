#[repr(C)]
#[doc = "TIMG8_INT_EVENT\\[%s\\]"]
#[doc(alias = "TIMG8_INT_EVENT")]
pub struct Timg8IntEvent {
    timg8_iidx: Timg8Iidx,
    _reserved1: [u8; 0x04],
    timg8_imask: Timg8Imask,
    _reserved2: [u8; 0x04],
    timg8_ris: Timg8Ris,
    _reserved3: [u8; 0x04],
    timg8_mis: Timg8Mis,
    _reserved4: [u8; 0x04],
    timg8_iset: Timg8Iset,
    _reserved5: [u8; 0x04],
    timg8_iclr: Timg8Iclr,
}
impl Timg8IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn timg8_iidx(&self) -> &Timg8Iidx {
        &self.timg8_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn timg8_imask(&self) -> &Timg8Imask {
        &self.timg8_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn timg8_ris(&self) -> &Timg8Ris {
        &self.timg8_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn timg8_mis(&self) -> &Timg8Mis {
        &self.timg8_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn timg8_iset(&self) -> &Timg8Iset {
        &self.timg8_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn timg8_iclr(&self) -> &Timg8Iclr {
        &self.timg8_iclr
    }
}
#[doc = "TIMG8_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_iidx`] module"]
#[doc(alias = "TIMG8_IIDX")]
pub type Timg8Iidx = crate::Reg<timg8_iidx::Timg8IidxSpec>;
#[doc = "Interrupt index"]
pub mod timg8_iidx;
#[doc = "TIMG8_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_imask`] module"]
#[doc(alias = "TIMG8_IMASK")]
pub type Timg8Imask = crate::Reg<timg8_imask::Timg8ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod timg8_imask;
#[doc = "TIMG8_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ris`] module"]
#[doc(alias = "TIMG8_RIS")]
pub type Timg8Ris = crate::Reg<timg8_ris::Timg8RisSpec>;
#[doc = "Raw interrupt status"]
pub mod timg8_ris;
#[doc = "TIMG8_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_mis`] module"]
#[doc(alias = "TIMG8_MIS")]
pub type Timg8Mis = crate::Reg<timg8_mis::Timg8MisSpec>;
#[doc = "Masked interrupt status"]
pub mod timg8_mis;
#[doc = "TIMG8_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_iset`] module"]
#[doc(alias = "TIMG8_ISET")]
pub type Timg8Iset = crate::Reg<timg8_iset::Timg8IsetSpec>;
#[doc = "Interrupt set"]
pub mod timg8_iset;
#[doc = "TIMG8_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_iclr`] module"]
#[doc(alias = "TIMG8_ICLR")]
pub type Timg8Iclr = crate::Reg<timg8_iclr::Timg8IclrSpec>;
#[doc = "Interrupt clear"]
pub mod timg8_iclr;
