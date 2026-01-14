#[repr(C)]
#[doc = "CPUSS_INT_GROUP1\\[%s\\]"]
#[doc(alias = "CPUSS_INT_GROUP1")]
pub struct CpussIntGroup1 {
    cpuss_int_group1_iidx: CpussIntGroup1Iidx,
    _reserved1: [u8; 0x04],
    cpuss_int_group1_imask: CpussIntGroup1Imask,
    _reserved2: [u8; 0x04],
    cpuss_int_group1_ris: CpussIntGroup1Ris,
    _reserved3: [u8; 0x04],
    cpuss_int_group1_mis: CpussIntGroup1Mis,
    _reserved4: [u8; 0x04],
    cpuss_int_group1_iset: CpussIntGroup1Iset,
    _reserved5: [u8; 0x04],
    cpuss_int_group1_iclr: CpussIntGroup1Iclr,
}
impl CpussIntGroup1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn cpuss_int_group1_iidx(&self) -> &CpussIntGroup1Iidx {
        &self.cpuss_int_group1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn cpuss_int_group1_imask(&self) -> &CpussIntGroup1Imask {
        &self.cpuss_int_group1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn cpuss_int_group1_ris(&self) -> &CpussIntGroup1Ris {
        &self.cpuss_int_group1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn cpuss_int_group1_mis(&self) -> &CpussIntGroup1Mis {
        &self.cpuss_int_group1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn cpuss_int_group1_iset(&self) -> &CpussIntGroup1Iset {
        &self.cpuss_int_group1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn cpuss_int_group1_iclr(&self) -> &CpussIntGroup1Iclr {
        &self.cpuss_int_group1_iclr
    }
}
#[doc = "CPUSS_INT_GROUP1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group1_iidx`] module"]
#[doc(alias = "CPUSS_INT_GROUP1_IIDX")]
pub type CpussIntGroup1Iidx = crate::Reg<cpuss_int_group1_iidx::CpussIntGroup1IidxSpec>;
#[doc = "Interrupt index"]
pub mod cpuss_int_group1_iidx;
#[doc = "CPUSS_INT_GROUP1_IMASK (r) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_imask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group1_imask`] module"]
#[doc(alias = "CPUSS_INT_GROUP1_IMASK")]
pub type CpussIntGroup1Imask = crate::Reg<cpuss_int_group1_imask::CpussIntGroup1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod cpuss_int_group1_imask;
#[doc = "CPUSS_INT_GROUP1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group1_ris`] module"]
#[doc(alias = "CPUSS_INT_GROUP1_RIS")]
pub type CpussIntGroup1Ris = crate::Reg<cpuss_int_group1_ris::CpussIntGroup1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod cpuss_int_group1_ris;
#[doc = "CPUSS_INT_GROUP1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group1_mis`] module"]
#[doc(alias = "CPUSS_INT_GROUP1_MIS")]
pub type CpussIntGroup1Mis = crate::Reg<cpuss_int_group1_mis::CpussIntGroup1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod cpuss_int_group1_mis;
#[doc = "CPUSS_INT_GROUP1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_int_group1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group1_iset`] module"]
#[doc(alias = "CPUSS_INT_GROUP1_ISET")]
pub type CpussIntGroup1Iset = crate::Reg<cpuss_int_group1_iset::CpussIntGroup1IsetSpec>;
#[doc = "Interrupt set"]
pub mod cpuss_int_group1_iset;
#[doc = "CPUSS_INT_GROUP1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_int_group1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_int_group1_iclr`] module"]
#[doc(alias = "CPUSS_INT_GROUP1_ICLR")]
pub type CpussIntGroup1Iclr = crate::Reg<cpuss_int_group1_iclr::CpussIntGroup1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod cpuss_int_group1_iclr;
