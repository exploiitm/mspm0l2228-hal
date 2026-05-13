#[repr(C)]
#[doc = "GPIOC_GEN_EVENT0\\[%s\\]"]
#[doc(alias = "GPIOC_GEN_EVENT0")]
pub struct GpiocGenEvent0 {
    gpioc_gen_event0_iidx: GpiocGenEvent0Iidx,
    _reserved1: [u8; 0x04],
    gpioc_gen_event0_imask: GpiocGenEvent0Imask,
    _reserved2: [u8; 0x04],
    gpioc_gen_event0_ris: GpiocGenEvent0Ris,
    _reserved3: [u8; 0x04],
    gpioc_gen_event0_mis: GpiocGenEvent0Mis,
    _reserved4: [u8; 0x04],
    gpioc_gen_event0_iset: GpiocGenEvent0Iset,
    _reserved5: [u8; 0x04],
    gpioc_gen_event0_iclr: GpiocGenEvent0Iclr,
}
impl GpiocGenEvent0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpioc_gen_event0_iidx(&self) -> &GpiocGenEvent0Iidx {
        &self.gpioc_gen_event0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpioc_gen_event0_imask(&self) -> &GpiocGenEvent0Imask {
        &self.gpioc_gen_event0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpioc_gen_event0_ris(&self) -> &GpiocGenEvent0Ris {
        &self.gpioc_gen_event0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpioc_gen_event0_mis(&self) -> &GpiocGenEvent0Mis {
        &self.gpioc_gen_event0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpioc_gen_event0_iset(&self) -> &GpiocGenEvent0Iset {
        &self.gpioc_gen_event0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpioc_gen_event0_iclr(&self) -> &GpiocGenEvent0Iclr {
        &self.gpioc_gen_event0_iclr
    }
}
#[doc = "GPIOC_GEN_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event0_iidx`] module"]
#[doc(alias = "GPIOC_GEN_EVENT0_IIDX")]
pub type GpiocGenEvent0Iidx = crate::Reg<gpioc_gen_event0_iidx::GpiocGenEvent0IidxSpec>;
#[doc = "Interrupt index"]
pub mod gpioc_gen_event0_iidx;
#[doc = "GPIOC_GEN_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event0_imask`] module"]
#[doc(alias = "GPIOC_GEN_EVENT0_IMASK")]
pub type GpiocGenEvent0Imask = crate::Reg<gpioc_gen_event0_imask::GpiocGenEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpioc_gen_event0_imask;
#[doc = "GPIOC_GEN_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event0_ris`] module"]
#[doc(alias = "GPIOC_GEN_EVENT0_RIS")]
pub type GpiocGenEvent0Ris = crate::Reg<gpioc_gen_event0_ris::GpiocGenEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpioc_gen_event0_ris;
#[doc = "GPIOC_GEN_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_gen_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event0_mis`] module"]
#[doc(alias = "GPIOC_GEN_EVENT0_MIS")]
pub type GpiocGenEvent0Mis = crate::Reg<gpioc_gen_event0_mis::GpiocGenEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpioc_gen_event0_mis;
#[doc = "GPIOC_GEN_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event0_iset`] module"]
#[doc(alias = "GPIOC_GEN_EVENT0_ISET")]
pub type GpiocGenEvent0Iset = crate::Reg<gpioc_gen_event0_iset::GpiocGenEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod gpioc_gen_event0_iset;
#[doc = "GPIOC_GEN_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_gen_event0_iclr`] module"]
#[doc(alias = "GPIOC_GEN_EVENT0_ICLR")]
pub type GpiocGenEvent0Iclr = crate::Reg<gpioc_gen_event0_iclr::GpiocGenEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpioc_gen_event0_iclr;
