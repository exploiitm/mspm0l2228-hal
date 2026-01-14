#[repr(C)]
#[doc = "GPIOA_CPU_INT\\[%s\\]"]
#[doc(alias = "GPIOA_CPU_INT")]
pub struct GpioaCpuInt {
    gpioa_cpu_int_iidx: GpioaCpuIntIidx,
    _reserved1: [u8; 0x04],
    gpioa_cpu_int_imask: GpioaCpuIntImask,
    _reserved2: [u8; 0x04],
    gpioa_cpu_int_ris: GpioaCpuIntRis,
    _reserved3: [u8; 0x04],
    gpioa_cpu_int_mis: GpioaCpuIntMis,
    _reserved4: [u8; 0x04],
    gpioa_cpu_int_iset: GpioaCpuIntIset,
    _reserved5: [u8; 0x04],
    gpioa_cpu_int_iclr: GpioaCpuIntIclr,
}
impl GpioaCpuInt {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn gpioa_cpu_int_iidx(&self) -> &GpioaCpuIntIidx {
        &self.gpioa_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn gpioa_cpu_int_imask(&self) -> &GpioaCpuIntImask {
        &self.gpioa_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn gpioa_cpu_int_ris(&self) -> &GpioaCpuIntRis {
        &self.gpioa_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn gpioa_cpu_int_mis(&self) -> &GpioaCpuIntMis {
        &self.gpioa_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn gpioa_cpu_int_iset(&self) -> &GpioaCpuIntIset {
        &self.gpioa_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn gpioa_cpu_int_iclr(&self) -> &GpioaCpuIntIclr {
        &self.gpioa_cpu_int_iclr
    }
}
#[doc = "GPIOA_CPU_INT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_cpu_int_iidx`] module"]
#[doc(alias = "GPIOA_CPU_INT_IIDX")]
pub type GpioaCpuIntIidx = crate::Reg<gpioa_cpu_int_iidx::GpioaCpuIntIidxSpec>;
#[doc = "Interrupt index"]
pub mod gpioa_cpu_int_iidx;
#[doc = "GPIOA_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_cpu_int_imask`] module"]
#[doc(alias = "GPIOA_CPU_INT_IMASK")]
pub type GpioaCpuIntImask = crate::Reg<gpioa_cpu_int_imask::GpioaCpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod gpioa_cpu_int_imask;
#[doc = "GPIOA_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_cpu_int_ris`] module"]
#[doc(alias = "GPIOA_CPU_INT_RIS")]
pub type GpioaCpuIntRis = crate::Reg<gpioa_cpu_int_ris::GpioaCpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod gpioa_cpu_int_ris;
#[doc = "GPIOA_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_cpu_int_mis`] module"]
#[doc(alias = "GPIOA_CPU_INT_MIS")]
pub type GpioaCpuIntMis = crate::Reg<gpioa_cpu_int_mis::GpioaCpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod gpioa_cpu_int_mis;
#[doc = "GPIOA_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_cpu_int_iset`] module"]
#[doc(alias = "GPIOA_CPU_INT_ISET")]
pub type GpioaCpuIntIset = crate::Reg<gpioa_cpu_int_iset::GpioaCpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod gpioa_cpu_int_iset;
#[doc = "GPIOA_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_cpu_int_iclr`] module"]
#[doc(alias = "GPIOA_CPU_INT_ICLR")]
pub type GpioaCpuIntIclr = crate::Reg<gpioa_cpu_int_iclr::GpioaCpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod gpioa_cpu_int_iclr;
