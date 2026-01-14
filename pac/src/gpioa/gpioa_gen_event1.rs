#[repr(C)]
#[doc = "GPIOA_GEN_EVENT1\\[%s\\]"]
#[doc(alias = "GPIOA_GEN_EVENT1")]
pub struct GpioaGenEvent1 {
    gpioa_gen_event1_iidx: GpioaGenEvent1Iidx,
    _reserved1: [u8; 0x04],
    gpioa_gen_event1_imask: GpioaGenEvent1Imask,
    _reserved2: [u8; 0x04],
    gpioa_gen_event1_ris: GpioaGenEvent1Ris,
    _reserved3: [u8; 0x04],
    gpioa_gen_event1_mis: GpioaGenEvent1Mis,
    _reserved4: [u8; 0x04],
    gpioa_gen_event1_iset: GpioaGenEvent1Iset,
    _reserved5: [u8; 0x04],
    gpioa_gen_event1_iclr: GpioaGenEvent1Iclr,
}
impl GpioaGenEvent1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpioa_gen_event1_iidx(&self) -> &GpioaGenEvent1Iidx {
        &self.gpioa_gen_event1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpioa_gen_event1_imask(&self) -> &GpioaGenEvent1Imask {
        &self.gpioa_gen_event1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpioa_gen_event1_ris(&self) -> &GpioaGenEvent1Ris {
        &self.gpioa_gen_event1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpioa_gen_event1_mis(&self) -> &GpioaGenEvent1Mis {
        &self.gpioa_gen_event1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpioa_gen_event1_iset(&self) -> &GpioaGenEvent1Iset {
        &self.gpioa_gen_event1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpioa_gen_event1_iclr(&self) -> &GpioaGenEvent1Iclr {
        &self.gpioa_gen_event1_iclr
    }
}
#[doc = "GPIOA_GEN_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event1_iidx`] module"]
#[doc(alias = "GPIOA_GEN_EVENT1_IIDX")]
pub type GpioaGenEvent1Iidx = crate::Reg<gpioa_gen_event1_iidx::GpioaGenEvent1IidxSpec>;
#[doc = "Interrupt index"]
pub mod gpioa_gen_event1_iidx;
#[doc = "GPIOA_GEN_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_gen_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event1_imask`] module"]
#[doc(alias = "GPIOA_GEN_EVENT1_IMASK")]
pub type GpioaGenEvent1Imask = crate::Reg<gpioa_gen_event1_imask::GpioaGenEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpioa_gen_event1_imask;
#[doc = "GPIOA_GEN_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event1_ris`] module"]
#[doc(alias = "GPIOA_GEN_EVENT1_RIS")]
pub type GpioaGenEvent1Ris = crate::Reg<gpioa_gen_event1_ris::GpioaGenEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpioa_gen_event1_ris;
#[doc = "GPIOA_GEN_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_gen_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event1_mis`] module"]
#[doc(alias = "GPIOA_GEN_EVENT1_MIS")]
pub type GpioaGenEvent1Mis = crate::Reg<gpioa_gen_event1_mis::GpioaGenEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpioa_gen_event1_mis;
#[doc = "GPIOA_GEN_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_gen_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event1_iset`] module"]
#[doc(alias = "GPIOA_GEN_EVENT1_ISET")]
pub type GpioaGenEvent1Iset = crate::Reg<gpioa_gen_event1_iset::GpioaGenEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod gpioa_gen_event1_iset;
#[doc = "GPIOA_GEN_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_gen_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_gen_event1_iclr`] module"]
#[doc(alias = "GPIOA_GEN_EVENT1_ICLR")]
pub type GpioaGenEvent1Iclr = crate::Reg<gpioa_gen_event1_iclr::GpioaGenEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpioa_gen_event1_iclr;
