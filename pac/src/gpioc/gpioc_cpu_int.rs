#[repr(C)]
#[doc = "GPIOC_CPU_INT\\[%s\\]"]
#[doc(alias = "GPIOC_CPU_INT")]
pub struct GpiocCpuInt {
    gpioc_cpu_int_iidx: GpiocCpuIntIidx,
    _reserved1: [u8; 0x04],
    gpioc_cpu_int_imask: GpiocCpuIntImask,
    _reserved2: [u8; 0x04],
    gpioc_cpu_int_ris: GpiocCpuIntRis,
    _reserved3: [u8; 0x04],
    gpioc_cpu_int_mis: GpiocCpuIntMis,
    _reserved4: [u8; 0x04],
    gpioc_cpu_int_iset: GpiocCpuIntIset,
    _reserved5: [u8; 0x04],
    gpioc_cpu_int_iclr: GpiocCpuIntIclr,
}
impl GpiocCpuInt {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpioc_cpu_int_iidx(&self) -> &GpiocCpuIntIidx {
        &self.gpioc_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpioc_cpu_int_imask(&self) -> &GpiocCpuIntImask {
        &self.gpioc_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpioc_cpu_int_ris(&self) -> &GpiocCpuIntRis {
        &self.gpioc_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpioc_cpu_int_mis(&self) -> &GpiocCpuIntMis {
        &self.gpioc_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpioc_cpu_int_iset(&self) -> &GpiocCpuIntIset {
        &self.gpioc_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpioc_cpu_int_iclr(&self) -> &GpiocCpuIntIclr {
        &self.gpioc_cpu_int_iclr
    }
}
#[doc = "GPIOC_CPU_INT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_cpu_int_iidx`] module"]
#[doc(alias = "GPIOC_CPU_INT_IIDX")]
pub type GpiocCpuIntIidx = crate::Reg<gpioc_cpu_int_iidx::GpiocCpuIntIidxSpec>;
#[doc = "Interrupt index"]
pub mod gpioc_cpu_int_iidx;
#[doc = "GPIOC_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_cpu_int_imask`] module"]
#[doc(alias = "GPIOC_CPU_INT_IMASK")]
pub type GpiocCpuIntImask = crate::Reg<gpioc_cpu_int_imask::GpiocCpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpioc_cpu_int_imask;
#[doc = "GPIOC_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_cpu_int_ris`] module"]
#[doc(alias = "GPIOC_CPU_INT_RIS")]
pub type GpiocCpuIntRis = crate::Reg<gpioc_cpu_int_ris::GpiocCpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpioc_cpu_int_ris;
#[doc = "GPIOC_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_cpu_int_mis`] module"]
#[doc(alias = "GPIOC_CPU_INT_MIS")]
pub type GpiocCpuIntMis = crate::Reg<gpioc_cpu_int_mis::GpiocCpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpioc_cpu_int_mis;
#[doc = "GPIOC_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_cpu_int_iset`] module"]
#[doc(alias = "GPIOC_CPU_INT_ISET")]
pub type GpiocCpuIntIset = crate::Reg<gpioc_cpu_int_iset::GpiocCpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod gpioc_cpu_int_iset;
#[doc = "GPIOC_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_cpu_int_iclr`] module"]
#[doc(alias = "GPIOC_CPU_INT_ICLR")]
pub type GpiocCpuIntIclr = crate::Reg<gpioc_cpu_int_iclr::GpiocCpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpioc_cpu_int_iclr;
