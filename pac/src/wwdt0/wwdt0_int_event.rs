#[repr(C)]
#[doc = "WWDT0_INT_EVENT\\[%s\\]"]
#[doc(alias = "WWDT0_INT_EVENT")]
pub struct Wwdt0IntEvent {
    wwdt0_iidx: Wwdt0Iidx,
    _reserved1: [u8; 0x04],
    wwdt0_imask: Wwdt0Imask,
    _reserved2: [u8; 0x04],
    wwdt0_ris: Wwdt0Ris,
    _reserved3: [u8; 0x04],
    wwdt0_mis: Wwdt0Mis,
    _reserved4: [u8; 0x04],
    wwdt0_iset: Wwdt0Iset,
    _reserved5: [u8; 0x04],
    wwdt0_iclr: Wwdt0Iclr,
}
impl Wwdt0IntEvent {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn wwdt0_iidx(&self) -> &Wwdt0Iidx {
        &self.wwdt0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn wwdt0_imask(&self) -> &Wwdt0Imask {
        &self.wwdt0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn wwdt0_ris(&self) -> &Wwdt0Ris {
        &self.wwdt0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn wwdt0_mis(&self) -> &Wwdt0Mis {
        &self.wwdt0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn wwdt0_iset(&self) -> &Wwdt0Iset {
        &self.wwdt0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn wwdt0_iclr(&self) -> &Wwdt0Iclr {
        &self.wwdt0_iclr
    }
}
#[doc = "WWDT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_iidx`] module"]
#[doc(alias = "WWDT0_IIDX")]
pub type Wwdt0Iidx = crate::Reg<wwdt0_iidx::Wwdt0IidxSpec>;
#[doc = "Interrupt index"]
pub mod wwdt0_iidx;
#[doc = "WWDT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_imask`] module"]
#[doc(alias = "WWDT0_IMASK")]
pub type Wwdt0Imask = crate::Reg<wwdt0_imask::Wwdt0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod wwdt0_imask;
#[doc = "WWDT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_ris`] module"]
#[doc(alias = "WWDT0_RIS")]
pub type Wwdt0Ris = crate::Reg<wwdt0_ris::Wwdt0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod wwdt0_ris;
#[doc = "WWDT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_mis`] module"]
#[doc(alias = "WWDT0_MIS")]
pub type Wwdt0Mis = crate::Reg<wwdt0_mis::Wwdt0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod wwdt0_mis;
#[doc = "WWDT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_iset`] module"]
#[doc(alias = "WWDT0_ISET")]
pub type Wwdt0Iset = crate::Reg<wwdt0_iset::Wwdt0IsetSpec>;
#[doc = "Interrupt set"]
pub mod wwdt0_iset;
#[doc = "WWDT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_iclr`] module"]
#[doc(alias = "WWDT0_ICLR")]
pub type Wwdt0Iclr = crate::Reg<wwdt0_iclr::Wwdt0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod wwdt0_iclr;
