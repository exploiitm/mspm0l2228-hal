#[repr(C)]
#[doc = "GPIOA_GEN_EVENT0\\[%s\\]"]
#[doc(alias = "GPIOA_GEN_EVENT0")]
pub struct GpioaGenEvent0 {
    gpioa_gen_event0_iidx: GpioaGenEvent0Iidx,
    _reserved1: [u8; 0x04],
    gpioa_gen_event0_imask: GpioaGenEvent0Imask,
    _reserved2: [u8; 0x04],
    gpioa_gen_event0_ris: GpioaGenEvent0Ris,
    _reserved3: [u8; 0x04],
    gpioa_gen_event0_mis: GpioaGenEvent0Mis,
    _reserved4: [u8; 0x04],
    gpioa_gen_event0_iset: GpioaGenEvent0Iset,
    _reserved5: [u8; 0x04],
    gpioa_gen_event0_iclr: GpioaGenEvent0Iclr,
}
impl GpioaGenEvent0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpioa_gen_event0_iidx(&self) -> &GpioaGenEvent0Iidx {
        &self.gpioa_gen_event0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpioa_gen_event0_imask(&self) -> &GpioaGenEvent0Imask {
        &self.gpioa_gen_event0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpioa_gen_event0_ris(&self) -> &GpioaGenEvent0Ris {
        &self.gpioa_gen_event0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpioa_gen_event0_mis(&self) -> &GpioaGenEvent0Mis {
        &self.gpioa_gen_event0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpioa_gen_event0_iset(&self) -> &GpioaGenEvent0Iset {
        &self.gpioa_gen_event0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpioa_gen_event0_iclr(&self) -> &GpioaGenEvent0Iclr {
        &self.gpioa_gen_event0_iclr
    }
}
#[doc = "GPIOA_GEN_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event0_iidx`] module"]
#[doc(alias = "GPIOA_GEN_EVENT0_IIDX")]
pub type GpioaGenEvent0Iidx = crate::Reg<gpioa_gen_event0_iidx::GpioaGenEvent0IidxSpec>;
#[doc = "Interrupt index"]
pub mod gpioa_gen_event0_iidx;
#[doc = "GPIOA_GEN_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_gen_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event0_imask`] module"]
#[doc(alias = "GPIOA_GEN_EVENT0_IMASK")]
pub type GpioaGenEvent0Imask = crate::Reg<gpioa_gen_event0_imask::GpioaGenEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpioa_gen_event0_imask;
#[doc = "GPIOA_GEN_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event0_ris`] module"]
#[doc(alias = "GPIOA_GEN_EVENT0_RIS")]
pub type GpioaGenEvent0Ris = crate::Reg<gpioa_gen_event0_ris::GpioaGenEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpioa_gen_event0_ris;
#[doc = "GPIOA_GEN_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event0_mis`] module"]
#[doc(alias = "GPIOA_GEN_EVENT0_MIS")]
pub type GpioaGenEvent0Mis = crate::Reg<gpioa_gen_event0_mis::GpioaGenEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpioa_gen_event0_mis;
#[doc = "GPIOA_GEN_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_gen_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event0_iset`] module"]
#[doc(alias = "GPIOA_GEN_EVENT0_ISET")]
pub type GpioaGenEvent0Iset = crate::Reg<gpioa_gen_event0_iset::GpioaGenEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod gpioa_gen_event0_iset;
#[doc = "GPIOA_GEN_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_gen_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event0_iclr`] module"]
#[doc(alias = "GPIOA_GEN_EVENT0_ICLR")]
pub type GpioaGenEvent0Iclr = crate::Reg<gpioa_gen_event0_iclr::GpioaGenEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpioa_gen_event0_iclr;
