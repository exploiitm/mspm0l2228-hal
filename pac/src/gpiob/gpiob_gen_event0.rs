#[repr(C)]
#[doc = "GPIOB_GEN_EVENT0\\[%s\\]"]
#[doc(alias = "GPIOB_GEN_EVENT0")]
pub struct GpiobGenEvent0 {
    gpiob_gen_event0_iidx: GpiobGenEvent0Iidx,
    _reserved1: [u8; 0x04],
    gpiob_gen_event0_imask: GpiobGenEvent0Imask,
    _reserved2: [u8; 0x04],
    gpiob_gen_event0_ris: GpiobGenEvent0Ris,
    _reserved3: [u8; 0x04],
    gpiob_gen_event0_mis: GpiobGenEvent0Mis,
    _reserved4: [u8; 0x04],
    gpiob_gen_event0_iset: GpiobGenEvent0Iset,
    _reserved5: [u8; 0x04],
    gpiob_gen_event0_iclr: GpiobGenEvent0Iclr,
}
impl GpiobGenEvent0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpiob_gen_event0_iidx(&self) -> &GpiobGenEvent0Iidx {
        &self.gpiob_gen_event0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpiob_gen_event0_imask(&self) -> &GpiobGenEvent0Imask {
        &self.gpiob_gen_event0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpiob_gen_event0_ris(&self) -> &GpiobGenEvent0Ris {
        &self.gpiob_gen_event0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpiob_gen_event0_mis(&self) -> &GpiobGenEvent0Mis {
        &self.gpiob_gen_event0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpiob_gen_event0_iset(&self) -> &GpiobGenEvent0Iset {
        &self.gpiob_gen_event0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpiob_gen_event0_iclr(&self) -> &GpiobGenEvent0Iclr {
        &self.gpiob_gen_event0_iclr
    }
}
#[doc = "GPIOB_GEN_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event0_iidx`] module"]
#[doc(alias = "GPIOB_GEN_EVENT0_IIDX")]
pub type GpiobGenEvent0Iidx = crate::Reg<gpiob_gen_event0_iidx::GpiobGenEvent0IidxSpec>;
#[doc = "Interrupt index"]
pub mod gpiob_gen_event0_iidx;
#[doc = "GPIOB_GEN_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event0_imask`] module"]
#[doc(alias = "GPIOB_GEN_EVENT0_IMASK")]
pub type GpiobGenEvent0Imask = crate::Reg<gpiob_gen_event0_imask::GpiobGenEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpiob_gen_event0_imask;
#[doc = "GPIOB_GEN_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event0_ris`] module"]
#[doc(alias = "GPIOB_GEN_EVENT0_RIS")]
pub type GpiobGenEvent0Ris = crate::Reg<gpiob_gen_event0_ris::GpiobGenEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpiob_gen_event0_ris;
#[doc = "GPIOB_GEN_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event0_mis`] module"]
#[doc(alias = "GPIOB_GEN_EVENT0_MIS")]
pub type GpiobGenEvent0Mis = crate::Reg<gpiob_gen_event0_mis::GpiobGenEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpiob_gen_event0_mis;
#[doc = "GPIOB_GEN_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event0_iset`] module"]
#[doc(alias = "GPIOB_GEN_EVENT0_ISET")]
pub type GpiobGenEvent0Iset = crate::Reg<gpiob_gen_event0_iset::GpiobGenEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod gpiob_gen_event0_iset;
#[doc = "GPIOB_GEN_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_gen_event0_iclr`] module"]
#[doc(alias = "GPIOB_GEN_EVENT0_ICLR")]
pub type GpiobGenEvent0Iclr = crate::Reg<gpiob_gen_event0_iclr::GpiobGenEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpiob_gen_event0_iclr;
