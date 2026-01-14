#[repr(C)]
#[doc = "CPUSS_INT_GROUP0\\[%s\\]"]
#[doc(alias = "CPUSS_INT_GROUP0")]
pub struct CpussIntGroup0 {
    cpuss_int_group0_iidx: CpussIntGroup0Iidx,
    _reserved1: [u8; 0x04],
    cpuss_int_group0_imask: CpussIntGroup0Imask,
    _reserved2: [u8; 0x04],
    cpuss_int_group0_ris: CpussIntGroup0Ris,
    _reserved3: [u8; 0x04],
    cpuss_int_group0_mis: CpussIntGroup0Mis,
    _reserved4: [u8; 0x04],
    cpuss_int_group0_iset: CpussIntGroup0Iset,
    _reserved5: [u8; 0x04],
    cpuss_int_group0_iclr: CpussIntGroup0Iclr,
}
impl CpussIntGroup0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn cpuss_int_group0_iidx(&self) -> &CpussIntGroup0Iidx {
        &self.cpuss_int_group0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn cpuss_int_group0_imask(&self) -> &CpussIntGroup0Imask {
        &self.cpuss_int_group0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn cpuss_int_group0_ris(&self) -> &CpussIntGroup0Ris {
        &self.cpuss_int_group0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn cpuss_int_group0_mis(&self) -> &CpussIntGroup0Mis {
        &self.cpuss_int_group0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn cpuss_int_group0_iset(&self) -> &CpussIntGroup0Iset {
        &self.cpuss_int_group0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn cpuss_int_group0_iclr(&self) -> &CpussIntGroup0Iclr {
        &self.cpuss_int_group0_iclr
    }
}
#[doc = "CPUSS_INT_GROUP0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group0_iidx`] module"]
#[doc(alias = "CPUSS_INT_GROUP0_IIDX")]
pub type CpussIntGroup0Iidx = crate::Reg<cpuss_int_group0_iidx::CpussIntGroup0IidxSpec>;
#[doc = "Interrupt index"]
pub mod cpuss_int_group0_iidx;
#[doc = "CPUSS_INT_GROUP0_IMASK (r) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group0_imask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group0_imask`] module"]
#[doc(alias = "CPUSS_INT_GROUP0_IMASK")]
pub type CpussIntGroup0Imask = crate::Reg<cpuss_int_group0_imask::CpussIntGroup0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod cpuss_int_group0_imask;
#[doc = "CPUSS_INT_GROUP0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group0_ris`] module"]
#[doc(alias = "CPUSS_INT_GROUP0_RIS")]
pub type CpussIntGroup0Ris = crate::Reg<cpuss_int_group0_ris::CpussIntGroup0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod cpuss_int_group0_ris;
#[doc = "CPUSS_INT_GROUP0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group0_mis`] module"]
#[doc(alias = "CPUSS_INT_GROUP0_MIS")]
pub type CpussIntGroup0Mis = crate::Reg<cpuss_int_group0_mis::CpussIntGroup0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod cpuss_int_group0_mis;
#[doc = "CPUSS_INT_GROUP0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_int_group0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group0_iset`] module"]
#[doc(alias = "CPUSS_INT_GROUP0_ISET")]
pub type CpussIntGroup0Iset = crate::Reg<cpuss_int_group0_iset::CpussIntGroup0IsetSpec>;
#[doc = "Interrupt set"]
pub mod cpuss_int_group0_iset;
#[doc = "CPUSS_INT_GROUP0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_int_group0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group0_iclr`] module"]
#[doc(alias = "CPUSS_INT_GROUP0_ICLR")]
pub type CpussIntGroup0Iclr = crate::Reg<cpuss_int_group0_iclr::CpussIntGroup0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod cpuss_int_group0_iclr;
