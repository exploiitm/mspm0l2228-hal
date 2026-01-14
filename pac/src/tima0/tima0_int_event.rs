#[repr(C)]
#[doc = "TIMA0_INT_EVENT\\[%s\\]"]
#[doc(alias = "TIMA0_INT_EVENT")]
pub struct Tima0IntEvent {
    tima0_iidx: Tima0Iidx,
    _reserved1: [u8; 0x04],
    tima0_imask: Tima0Imask,
    _reserved2: [u8; 0x04],
    tima0_ris: Tima0Ris,
    _reserved3: [u8; 0x04],
    tima0_mis: Tima0Mis,
    _reserved4: [u8; 0x04],
    tima0_iset: Tima0Iset,
    _reserved5: [u8; 0x04],
    tima0_iclr: Tima0Iclr,
}
impl Tima0IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn tima0_iidx(&self) -> &Tima0Iidx {
        &self.tima0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn tima0_imask(&self) -> &Tima0Imask {
        &self.tima0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn tima0_ris(&self) -> &Tima0Ris {
        &self.tima0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn tima0_mis(&self) -> &Tima0Mis {
        &self.tima0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn tima0_iset(&self) -> &Tima0Iset {
        &self.tima0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn tima0_iclr(&self) -> &Tima0Iclr {
        &self.tima0_iclr
    }
}
#[doc = "TIMA0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_iidx`] module"]
#[doc(alias = "TIMA0_IIDX")]
pub type Tima0Iidx = crate::Reg<tima0_iidx::Tima0IidxSpec>;
#[doc = "Interrupt index"]
pub mod tima0_iidx;
#[doc = "TIMA0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_imask`] module"]
#[doc(alias = "TIMA0_IMASK")]
pub type Tima0Imask = crate::Reg<tima0_imask::Tima0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod tima0_imask;
#[doc = "TIMA0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ris`] module"]
#[doc(alias = "TIMA0_RIS")]
pub type Tima0Ris = crate::Reg<tima0_ris::Tima0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod tima0_ris;
#[doc = "TIMA0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_mis`] module"]
#[doc(alias = "TIMA0_MIS")]
pub type Tima0Mis = crate::Reg<tima0_mis::Tima0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod tima0_mis;
#[doc = "TIMA0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_iset`] module"]
#[doc(alias = "TIMA0_ISET")]
pub type Tima0Iset = crate::Reg<tima0_iset::Tima0IsetSpec>;
#[doc = "Interrupt set"]
pub mod tima0_iset;
#[doc = "TIMA0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_iclr`] module"]
#[doc(alias = "TIMA0_ICLR")]
pub type Tima0Iclr = crate::Reg<tima0_iclr::Tima0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod tima0_iclr;
