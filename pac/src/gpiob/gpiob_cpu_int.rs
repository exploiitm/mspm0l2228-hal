#[repr(C)]
#[doc = "GPIOB_CPU_INT\\[%s\\]"]
#[doc(alias = "GPIOB_CPU_INT")]
pub struct GpiobCpuInt {
    gpiob_cpu_int_iidx: GpiobCpuIntIidx,
    _reserved1: [u8; 0x04],
    gpiob_cpu_int_imask: GpiobCpuIntImask,
    _reserved2: [u8; 0x04],
    gpiob_cpu_int_ris: GpiobCpuIntRis,
    _reserved3: [u8; 0x04],
    gpiob_cpu_int_mis: GpiobCpuIntMis,
    _reserved4: [u8; 0x04],
    gpiob_cpu_int_iset: GpiobCpuIntIset,
    _reserved5: [u8; 0x04],
    gpiob_cpu_int_iclr: GpiobCpuIntIclr,
}
impl GpiobCpuInt {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpiob_cpu_int_iidx(&self) -> &GpiobCpuIntIidx {
        &self.gpiob_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpiob_cpu_int_imask(&self) -> &GpiobCpuIntImask {
        &self.gpiob_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpiob_cpu_int_ris(&self) -> &GpiobCpuIntRis {
        &self.gpiob_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpiob_cpu_int_mis(&self) -> &GpiobCpuIntMis {
        &self.gpiob_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpiob_cpu_int_iset(&self) -> &GpiobCpuIntIset {
        &self.gpiob_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpiob_cpu_int_iclr(&self) -> &GpiobCpuIntIclr {
        &self.gpiob_cpu_int_iclr
    }
}
#[doc = "GPIOB_CPU_INT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_cpu_int_iidx`] module"]
#[doc(alias = "GPIOB_CPU_INT_IIDX")]
pub type GpiobCpuIntIidx = crate::Reg<gpiob_cpu_int_iidx::GpiobCpuIntIidxSpec>;
#[doc = "Interrupt index"]
pub mod gpiob_cpu_int_iidx;
#[doc = "GPIOB_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_cpu_int_imask`] module"]
#[doc(alias = "GPIOB_CPU_INT_IMASK")]
pub type GpiobCpuIntImask = crate::Reg<gpiob_cpu_int_imask::GpiobCpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpiob_cpu_int_imask;
#[doc = "GPIOB_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_cpu_int_ris`] module"]
#[doc(alias = "GPIOB_CPU_INT_RIS")]
pub type GpiobCpuIntRis = crate::Reg<gpiob_cpu_int_ris::GpiobCpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpiob_cpu_int_ris;
#[doc = "GPIOB_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_cpu_int_mis`] module"]
#[doc(alias = "GPIOB_CPU_INT_MIS")]
pub type GpiobCpuIntMis = crate::Reg<gpiob_cpu_int_mis::GpiobCpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpiob_cpu_int_mis;
#[doc = "GPIOB_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_cpu_int_iset`] module"]
#[doc(alias = "GPIOB_CPU_INT_ISET")]
pub type GpiobCpuIntIset = crate::Reg<gpiob_cpu_int_iset::GpiobCpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod gpiob_cpu_int_iset;
#[doc = "GPIOB_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_cpu_int_iclr`] module"]
#[doc(alias = "GPIOB_CPU_INT_ICLR")]
pub type GpiobCpuIntIclr = crate::Reg<gpiob_cpu_int_iclr::GpiobCpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpiob_cpu_int_iclr;
