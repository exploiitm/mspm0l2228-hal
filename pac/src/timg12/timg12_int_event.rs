#[repr(C)]
#[doc = "TIMG12_INT_EVENT\\[%s\\]"]
#[doc(alias = "TIMG12_INT_EVENT")]
pub struct Timg12IntEvent {
    timg12_iidx: Timg12Iidx,
    _reserved1: [u8; 0x04],
    timg12_imask: Timg12Imask,
    _reserved2: [u8; 0x04],
    timg12_ris: Timg12Ris,
    _reserved3: [u8; 0x04],
    timg12_mis: Timg12Mis,
    _reserved4: [u8; 0x04],
    timg12_iset: Timg12Iset,
    _reserved5: [u8; 0x04],
    timg12_iclr: Timg12Iclr,
}
impl Timg12IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn timg12_iidx(&self) -> &Timg12Iidx {
        &self.timg12_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn timg12_imask(&self) -> &Timg12Imask {
        &self.timg12_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn timg12_ris(&self) -> &Timg12Ris {
        &self.timg12_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn timg12_mis(&self) -> &Timg12Mis {
        &self.timg12_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn timg12_iset(&self) -> &Timg12Iset {
        &self.timg12_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn timg12_iclr(&self) -> &Timg12Iclr {
        &self.timg12_iclr
    }
}
#[doc = "TIMG12_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_iidx`] module"]
#[doc(alias = "TIMG12_IIDX")]
pub type Timg12Iidx = crate::Reg<timg12_iidx::Timg12IidxSpec>;
#[doc = "Interrupt index"]
pub mod timg12_iidx;
#[doc = "TIMG12_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_imask`] module"]
#[doc(alias = "TIMG12_IMASK")]
pub type Timg12Imask = crate::Reg<timg12_imask::Timg12ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod timg12_imask;
#[doc = "TIMG12_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ris`] module"]
#[doc(alias = "TIMG12_RIS")]
pub type Timg12Ris = crate::Reg<timg12_ris::Timg12RisSpec>;
#[doc = "Raw interrupt status"]
pub mod timg12_ris;
#[doc = "TIMG12_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_mis`] module"]
#[doc(alias = "TIMG12_MIS")]
pub type Timg12Mis = crate::Reg<timg12_mis::Timg12MisSpec>;
#[doc = "Masked interrupt status"]
pub mod timg12_mis;
#[doc = "TIMG12_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_iset`] module"]
#[doc(alias = "TIMG12_ISET")]
pub type Timg12Iset = crate::Reg<timg12_iset::Timg12IsetSpec>;
#[doc = "Interrupt set"]
pub mod timg12_iset;
#[doc = "TIMG12_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_iclr`] module"]
#[doc(alias = "TIMG12_ICLR")]
pub type Timg12Iclr = crate::Reg<timg12_iclr::Timg12IclrSpec>;
#[doc = "Interrupt clear"]
pub mod timg12_iclr;
