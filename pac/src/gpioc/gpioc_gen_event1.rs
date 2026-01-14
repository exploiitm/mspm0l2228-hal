#[repr(C)]
#[doc = "GPIOC_GEN_EVENT1\\[%s\\]"]
#[doc(alias = "GPIOC_GEN_EVENT1")]
pub struct GpiocGenEvent1 {
    gpioc_gen_event1_iidx: GpiocGenEvent1Iidx,
    _reserved1: [u8; 0x04],
    gpioc_gen_event1_imask: GpiocGenEvent1Imask,
    _reserved2: [u8; 0x04],
    gpioc_gen_event1_ris: GpiocGenEvent1Ris,
    _reserved3: [u8; 0x04],
    gpioc_gen_event1_mis: GpiocGenEvent1Mis,
    _reserved4: [u8; 0x04],
    gpioc_gen_event1_iset: GpiocGenEvent1Iset,
    _reserved5: [u8; 0x04],
    gpioc_gen_event1_iclr: GpiocGenEvent1Iclr,
}
impl GpiocGenEvent1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpioc_gen_event1_iidx(&self) -> &GpiocGenEvent1Iidx {
        &self.gpioc_gen_event1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpioc_gen_event1_imask(&self) -> &GpiocGenEvent1Imask {
        &self.gpioc_gen_event1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpioc_gen_event1_ris(&self) -> &GpiocGenEvent1Ris {
        &self.gpioc_gen_event1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpioc_gen_event1_mis(&self) -> &GpiocGenEvent1Mis {
        &self.gpioc_gen_event1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpioc_gen_event1_iset(&self) -> &GpiocGenEvent1Iset {
        &self.gpioc_gen_event1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpioc_gen_event1_iclr(&self) -> &GpiocGenEvent1Iclr {
        &self.gpioc_gen_event1_iclr
    }
}
#[doc = "GPIOC_GEN_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event1_iidx`] module"]
#[doc(alias = "GPIOC_GEN_EVENT1_IIDX")]
pub type GpiocGenEvent1Iidx = crate::Reg<gpioc_gen_event1_iidx::GpiocGenEvent1IidxSpec>;
#[doc = "Interrupt index"]
pub mod gpioc_gen_event1_iidx;
#[doc = "GPIOC_GEN_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event1_imask`] module"]
#[doc(alias = "GPIOC_GEN_EVENT1_IMASK")]
pub type GpiocGenEvent1Imask = crate::Reg<gpioc_gen_event1_imask::GpiocGenEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpioc_gen_event1_imask;
#[doc = "GPIOC_GEN_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event1_ris`] module"]
#[doc(alias = "GPIOC_GEN_EVENT1_RIS")]
pub type GpiocGenEvent1Ris = crate::Reg<gpioc_gen_event1_ris::GpiocGenEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpioc_gen_event1_ris;
#[doc = "GPIOC_GEN_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event1_mis`] module"]
#[doc(alias = "GPIOC_GEN_EVENT1_MIS")]
pub type GpiocGenEvent1Mis = crate::Reg<gpioc_gen_event1_mis::GpiocGenEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpioc_gen_event1_mis;
#[doc = "GPIOC_GEN_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event1_iset`] module"]
#[doc(alias = "GPIOC_GEN_EVENT1_ISET")]
pub type GpiocGenEvent1Iset = crate::Reg<gpioc_gen_event1_iset::GpiocGenEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod gpioc_gen_event1_iset;
#[doc = "GPIOC_GEN_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event1_iclr`] module"]
#[doc(alias = "GPIOC_GEN_EVENT1_ICLR")]
pub type GpiocGenEvent1Iclr = crate::Reg<gpioc_gen_event1_iclr::GpiocGenEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpioc_gen_event1_iclr;
