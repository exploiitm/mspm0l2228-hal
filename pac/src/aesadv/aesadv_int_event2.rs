#[repr(C)]
#[doc = "AESADV_INT_EVENT2\\[%s\\]"]
#[doc(alias = "AESADV_INT_EVENT2")]
pub struct AesadvIntEvent2 {
    aesadv_int_event2_iidx: AesadvIntEvent2Iidx,
    _reserved1: [u8; 0x04],
    aesadv_int_event2_imask: AesadvIntEvent2Imask,
    _reserved2: [u8; 0x04],
    aesadv_int_event2_ris: AesadvIntEvent2Ris,
    _reserved3: [u8; 0x04],
    aesadv_int_event2_mis: AesadvIntEvent2Mis,
    _reserved4: [u8; 0x04],
    aesadv_int_event2_iset: AesadvIntEvent2Iset,
    _reserved5: [u8; 0x04],
    aesadv_int_event2_iclr: AesadvIntEvent2Iclr,
}
impl AesadvIntEvent2 {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn aesadv_int_event2_iidx(&self) -> &AesadvIntEvent2Iidx {
        &self.aesadv_int_event2_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn aesadv_int_event2_imask(&self) -> &AesadvIntEvent2Imask {
        &self.aesadv_int_event2_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn aesadv_int_event2_ris(&self) -> &AesadvIntEvent2Ris {
        &self.aesadv_int_event2_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn aesadv_int_event2_mis(&self) -> &AesadvIntEvent2Mis {
        &self.aesadv_int_event2_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn aesadv_int_event2_iset(&self) -> &AesadvIntEvent2Iset {
        &self.aesadv_int_event2_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn aesadv_int_event2_iclr(&self) -> &AesadvIntEvent2Iclr {
        &self.aesadv_int_event2_iclr
    }
}
#[doc = "AESADV_INT_EVENT2_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event2_iidx`] module"]
#[doc(alias = "AESADV_INT_EVENT2_IIDX")]
pub type AesadvIntEvent2Iidx = crate::Reg<aesadv_int_event2_iidx::AesadvIntEvent2IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod aesadv_int_event2_iidx;
#[doc = "AESADV_INT_EVENT2_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event2_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event2_imask`] module"]
#[doc(alias = "AESADV_INT_EVENT2_IMASK")]
pub type AesadvIntEvent2Imask = crate::Reg<aesadv_int_event2_imask::AesadvIntEvent2ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod aesadv_int_event2_imask;
#[doc = "AESADV_INT_EVENT2_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event2_ris`] module"]
#[doc(alias = "AESADV_INT_EVENT2_RIS")]
pub type AesadvIntEvent2Ris = crate::Reg<aesadv_int_event2_ris::AesadvIntEvent2RisSpec>;
#[doc = "Raw interrupt status"]
pub mod aesadv_int_event2_ris;
#[doc = "AESADV_INT_EVENT2_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event2_mis`] module"]
#[doc(alias = "AESADV_INT_EVENT2_MIS")]
pub type AesadvIntEvent2Mis = crate::Reg<aesadv_int_event2_mis::AesadvIntEvent2MisSpec>;
#[doc = "Masked interrupt status"]
pub mod aesadv_int_event2_mis;
#[doc = "AESADV_INT_EVENT2_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event2_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event2_iset`] module"]
#[doc(alias = "AESADV_INT_EVENT2_ISET")]
pub type AesadvIntEvent2Iset = crate::Reg<aesadv_int_event2_iset::AesadvIntEvent2IsetSpec>;
#[doc = "Interrupt set"]
pub mod aesadv_int_event2_iset;
#[doc = "AESADV_INT_EVENT2_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event2_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_int_event2_iclr`] module"]
#[doc(alias = "AESADV_INT_EVENT2_ICLR")]
pub type AesadvIntEvent2Iclr = crate::Reg<aesadv_int_event2_iclr::AesadvIntEvent2IclrSpec>;
#[doc = "Interrupt clear"]
pub mod aesadv_int_event2_iclr;
