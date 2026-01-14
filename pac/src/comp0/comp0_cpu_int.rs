#[repr(C)]
#[doc = "COMP0_CPU_INT\\[%s\\]"]
#[doc(alias = "COMP0_CPU_INT")]
pub struct Comp0CpuInt {
    comp0_cpu_int_iidx: Comp0CpuIntIidx,
    _reserved1: [u8; 0x04],
    comp0_cpu_int_imask: Comp0CpuIntImask,
    _reserved2: [u8; 0x04],
    comp0_cpu_int_ris: Comp0CpuIntRis,
    _reserved3: [u8; 0x04],
    comp0_cpu_int_mis: Comp0CpuIntMis,
    _reserved4: [u8; 0x04],
    comp0_cpu_int_iset: Comp0CpuIntIset,
    _reserved5: [u8; 0x04],
    comp0_cpu_int_iclr: Comp0CpuIntIclr,
}
impl Comp0CpuInt {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn comp0_cpu_int_iidx(&self) -> &Comp0CpuIntIidx {
        &self.comp0_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn comp0_cpu_int_imask(&self) -> &Comp0CpuIntImask {
        &self.comp0_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn comp0_cpu_int_ris(&self) -> &Comp0CpuIntRis {
        &self.comp0_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn comp0_cpu_int_mis(&self) -> &Comp0CpuIntMis {
        &self.comp0_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn comp0_cpu_int_iset(&self) -> &Comp0CpuIntIset {
        &self.comp0_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn comp0_cpu_int_iclr(&self) -> &Comp0CpuIntIclr {
        &self.comp0_cpu_int_iclr
    }
}
#[doc = "COMP0_CPU_INT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_cpu_int_iidx`] module"]
#[doc(alias = "COMP0_CPU_INT_IIDX")]
pub type Comp0CpuIntIidx = crate::Reg<comp0_cpu_int_iidx::Comp0CpuIntIidxSpec>;
#[doc = "Interrupt index"]
pub mod comp0_cpu_int_iidx;
#[doc = "COMP0_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_cpu_int_imask`] module"]
#[doc(alias = "COMP0_CPU_INT_IMASK")]
pub type Comp0CpuIntImask = crate::Reg<comp0_cpu_int_imask::Comp0CpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod comp0_cpu_int_imask;
#[doc = "COMP0_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_cpu_int_ris`] module"]
#[doc(alias = "COMP0_CPU_INT_RIS")]
pub type Comp0CpuIntRis = crate::Reg<comp0_cpu_int_ris::Comp0CpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod comp0_cpu_int_ris;
#[doc = "COMP0_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_cpu_int_mis`] module"]
#[doc(alias = "COMP0_CPU_INT_MIS")]
pub type Comp0CpuIntMis = crate::Reg<comp0_cpu_int_mis::Comp0CpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod comp0_cpu_int_mis;
#[doc = "COMP0_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_cpu_int_iset`] module"]
#[doc(alias = "COMP0_CPU_INT_ISET")]
pub type Comp0CpuIntIset = crate::Reg<comp0_cpu_int_iset::Comp0CpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod comp0_cpu_int_iset;
#[doc = "COMP0_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_cpu_int_iclr`] module"]
#[doc(alias = "COMP0_CPU_INT_ICLR")]
pub type Comp0CpuIntIclr = crate::Reg<comp0_cpu_int_iclr::Comp0CpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod comp0_cpu_int_iclr;
