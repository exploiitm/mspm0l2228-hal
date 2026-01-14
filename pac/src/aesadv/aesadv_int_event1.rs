#[repr(C)]
#[doc = "AESADV_INT_EVENT1\\[%s\\]"]
#[doc(alias = "AESADV_INT_EVENT1")]
pub struct AesadvIntEvent1 {
    aesadv_int_event1_iidx: AesadvIntEvent1Iidx,
    _reserved1: [u8; 0x04],
    aesadv_int_event1_imask: AesadvIntEvent1Imask,
    _reserved2: [u8; 0x04],
    aesadv_int_event1_ris: AesadvIntEvent1Ris,
    _reserved3: [u8; 0x04],
    aesadv_int_event1_mis: AesadvIntEvent1Mis,
    _reserved4: [u8; 0x04],
    aesadv_int_event1_iset: AesadvIntEvent1Iset,
    _reserved5: [u8; 0x04],
    aesadv_int_event1_iclr: AesadvIntEvent1Iclr,
}
impl AesadvIntEvent1 {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn aesadv_int_event1_iidx(&self) -> &AesadvIntEvent1Iidx {
        &self.aesadv_int_event1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn aesadv_int_event1_imask(&self) -> &AesadvIntEvent1Imask {
        &self.aesadv_int_event1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn aesadv_int_event1_ris(&self) -> &AesadvIntEvent1Ris {
        &self.aesadv_int_event1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn aesadv_int_event1_mis(&self) -> &AesadvIntEvent1Mis {
        &self.aesadv_int_event1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn aesadv_int_event1_iset(&self) -> &AesadvIntEvent1Iset {
        &self.aesadv_int_event1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn aesadv_int_event1_iclr(&self) -> &AesadvIntEvent1Iclr {
        &self.aesadv_int_event1_iclr
    }
}
#[doc = "AESADV_INT_EVENT1_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event1_iidx`] module"]
#[doc(alias = "AESADV_INT_EVENT1_IIDX")]
pub type AesadvIntEvent1Iidx = crate::Reg<aesadv_int_event1_iidx::AesadvIntEvent1IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod aesadv_int_event1_iidx;
#[doc = "AESADV_INT_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event1_imask`] module"]
#[doc(alias = "AESADV_INT_EVENT1_IMASK")]
pub type AesadvIntEvent1Imask = crate::Reg<aesadv_int_event1_imask::AesadvIntEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod aesadv_int_event1_imask;
#[doc = "AESADV_INT_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event1_ris`] module"]
#[doc(alias = "AESADV_INT_EVENT1_RIS")]
pub type AesadvIntEvent1Ris = crate::Reg<aesadv_int_event1_ris::AesadvIntEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod aesadv_int_event1_ris;
#[doc = "AESADV_INT_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event1_mis`] module"]
#[doc(alias = "AESADV_INT_EVENT1_MIS")]
pub type AesadvIntEvent1Mis = crate::Reg<aesadv_int_event1_mis::AesadvIntEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod aesadv_int_event1_mis;
#[doc = "AESADV_INT_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event1_iset`] module"]
#[doc(alias = "AESADV_INT_EVENT1_ISET")]
pub type AesadvIntEvent1Iset = crate::Reg<aesadv_int_event1_iset::AesadvIntEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod aesadv_int_event1_iset;
#[doc = "AESADV_INT_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event1_iclr`] module"]
#[doc(alias = "AESADV_INT_EVENT1_ICLR")]
pub type AesadvIntEvent1Iclr = crate::Reg<aesadv_int_event1_iclr::AesadvIntEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod aesadv_int_event1_iclr;
