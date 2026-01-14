#[repr(C)]
#[doc = "GPIOB_GEN_EVENT1\\[%s\\]"]
#[doc(alias = "GPIOB_GEN_EVENT1")]
pub struct GpiobGenEvent1 {
    gpiob_gen_event1_iidx: GpiobGenEvent1Iidx,
    _reserved1: [u8; 0x04],
    gpiob_gen_event1_imask: GpiobGenEvent1Imask,
    _reserved2: [u8; 0x04],
    gpiob_gen_event1_ris: GpiobGenEvent1Ris,
    _reserved3: [u8; 0x04],
    gpiob_gen_event1_mis: GpiobGenEvent1Mis,
    _reserved4: [u8; 0x04],
    gpiob_gen_event1_iset: GpiobGenEvent1Iset,
    _reserved5: [u8; 0x04],
    gpiob_gen_event1_iclr: GpiobGenEvent1Iclr,
}
impl GpiobGenEvent1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpiob_gen_event1_iidx(&self) -> &GpiobGenEvent1Iidx {
        &self.gpiob_gen_event1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpiob_gen_event1_imask(&self) -> &GpiobGenEvent1Imask {
        &self.gpiob_gen_event1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpiob_gen_event1_ris(&self) -> &GpiobGenEvent1Ris {
        &self.gpiob_gen_event1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpiob_gen_event1_mis(&self) -> &GpiobGenEvent1Mis {
        &self.gpiob_gen_event1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpiob_gen_event1_iset(&self) -> &GpiobGenEvent1Iset {
        &self.gpiob_gen_event1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpiob_gen_event1_iclr(&self) -> &GpiobGenEvent1Iclr {
        &self.gpiob_gen_event1_iclr
    }
}
#[doc = "GPIOB_GEN_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event1_iidx`] module"]
#[doc(alias = "GPIOB_GEN_EVENT1_IIDX")]
pub type GpiobGenEvent1Iidx = crate::Reg<gpiob_gen_event1_iidx::GpiobGenEvent1IidxSpec>;
#[doc = "Interrupt index"]
pub mod gpiob_gen_event1_iidx;
#[doc = "GPIOB_GEN_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event1_imask`] module"]
#[doc(alias = "GPIOB_GEN_EVENT1_IMASK")]
pub type GpiobGenEvent1Imask = crate::Reg<gpiob_gen_event1_imask::GpiobGenEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpiob_gen_event1_imask;
#[doc = "GPIOB_GEN_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event1_ris`] module"]
#[doc(alias = "GPIOB_GEN_EVENT1_RIS")]
pub type GpiobGenEvent1Ris = crate::Reg<gpiob_gen_event1_ris::GpiobGenEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpiob_gen_event1_ris;
#[doc = "GPIOB_GEN_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event1_mis`] module"]
#[doc(alias = "GPIOB_GEN_EVENT1_MIS")]
pub type GpiobGenEvent1Mis = crate::Reg<gpiob_gen_event1_mis::GpiobGenEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpiob_gen_event1_mis;
#[doc = "GPIOB_GEN_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event1_iset`] module"]
#[doc(alias = "GPIOB_GEN_EVENT1_ISET")]
pub type GpiobGenEvent1Iset = crate::Reg<gpiob_gen_event1_iset::GpiobGenEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod gpiob_gen_event1_iset;
#[doc = "GPIOB_GEN_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event1_iclr`] module"]
#[doc(alias = "GPIOB_GEN_EVENT1_ICLR")]
pub type GpiobGenEvent1Iclr = crate::Reg<gpiob_gen_event1_iclr::GpiobGenEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpiob_gen_event1_iclr;
