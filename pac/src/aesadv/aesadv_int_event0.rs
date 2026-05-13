#[repr(C)]
#[doc = "AESADV_INT_EVENT0\\[%s\\]"]
#[doc(alias = "AESADV_INT_EVENT0")]
pub struct AesadvIntEvent0 {
    aesadv_int_event0_iidx: AesadvIntEvent0Iidx,
    _reserved1: [u8; 0x04],
    aesadv_int_event0_imask: AesadvIntEvent0Imask,
    _reserved2: [u8; 0x04],
    aesadv_int_event0_ris: AesadvIntEvent0Ris,
    _reserved3: [u8; 0x04],
    aesadv_int_event0_mis: AesadvIntEvent0Mis,
    _reserved4: [u8; 0x04],
    aesadv_int_event0_iset: AesadvIntEvent0Iset,
    _reserved5: [u8; 0x04],
    aesadv_int_event0_iclr: AesadvIntEvent0Iclr,
}
impl AesadvIntEvent0 {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn aesadv_int_event0_iidx(&self) -> &AesadvIntEvent0Iidx {
        &self.aesadv_int_event0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn aesadv_int_event0_imask(&self) -> &AesadvIntEvent0Imask {
        &self.aesadv_int_event0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn aesadv_int_event0_ris(&self) -> &AesadvIntEvent0Ris {
        &self.aesadv_int_event0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn aesadv_int_event0_mis(&self) -> &AesadvIntEvent0Mis {
        &self.aesadv_int_event0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn aesadv_int_event0_iset(&self) -> &AesadvIntEvent0Iset {
        &self.aesadv_int_event0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn aesadv_int_event0_iclr(&self) -> &AesadvIntEvent0Iclr {
        &self.aesadv_int_event0_iclr
    }
}
#[doc = "AESADV_INT_EVENT0_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event0_iidx`] module"]
#[doc(alias = "AESADV_INT_EVENT0_IIDX")]
pub type AesadvIntEvent0Iidx = crate::Reg<aesadv_int_event0_iidx::AesadvIntEvent0IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod aesadv_int_event0_iidx;
#[doc = "AESADV_INT_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event0_imask`] module"]
#[doc(alias = "AESADV_INT_EVENT0_IMASK")]
pub type AesadvIntEvent0Imask = crate::Reg<aesadv_int_event0_imask::AesadvIntEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod aesadv_int_event0_imask;
#[doc = "AESADV_INT_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event0_ris`] module"]
#[doc(alias = "AESADV_INT_EVENT0_RIS")]
pub type AesadvIntEvent0Ris = crate::Reg<aesadv_int_event0_ris::AesadvIntEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod aesadv_int_event0_ris;
#[doc = "AESADV_INT_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event0_mis`] module"]
#[doc(alias = "AESADV_INT_EVENT0_MIS")]
pub type AesadvIntEvent0Mis = crate::Reg<aesadv_int_event0_mis::AesadvIntEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod aesadv_int_event0_mis;
#[doc = "AESADV_INT_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event0_iset`] module"]
#[doc(alias = "AESADV_INT_EVENT0_ISET")]
pub type AesadvIntEvent0Iset = crate::Reg<aesadv_int_event0_iset::AesadvIntEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod aesadv_int_event0_iset;
#[doc = "AESADV_INT_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event0_iclr`] module"]
#[doc(alias = "AESADV_INT_EVENT0_ICLR")]
pub type AesadvIntEvent0Iclr = crate::Reg<aesadv_int_event0_iclr::AesadvIntEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod aesadv_int_event0_iclr;
