#[repr(C)]
#[doc = "LFSS_CPU_INT\\[%s\\]"]
#[doc(alias = "LFSS_CPU_INT")]
pub struct LfssCpuInt {
    lfss_cpu_int_iidx: LfssCpuIntIidx,
    _reserved1: [u8; 0x04],
    lfss_cpu_int_imask: LfssCpuIntImask,
    _reserved2: [u8; 0x04],
    lfss_cpu_int_ris: LfssCpuIntRis,
    _reserved3: [u8; 0x04],
    lfss_cpu_int_mis: LfssCpuIntMis,
    _reserved4: [u8; 0x04],
    lfss_cpu_int_iset: LfssCpuIntIset,
    _reserved5: [u8; 0x04],
    lfss_cpu_int_iclr: LfssCpuIntIclr,
}
impl LfssCpuInt {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn lfss_cpu_int_iidx(&self) -> &LfssCpuIntIidx {
        &self.lfss_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn lfss_cpu_int_imask(&self) -> &LfssCpuIntImask {
        &self.lfss_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn lfss_cpu_int_ris(&self) -> &LfssCpuIntRis {
        &self.lfss_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn lfss_cpu_int_mis(&self) -> &LfssCpuIntMis {
        &self.lfss_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn lfss_cpu_int_iset(&self) -> &LfssCpuIntIset {
        &self.lfss_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn lfss_cpu_int_iclr(&self) -> &LfssCpuIntIclr {
        &self.lfss_cpu_int_iclr
    }
}
#[doc = "LFSS_CPU_INT_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cpu_int_iidx`] module"]
#[doc(alias = "LFSS_CPU_INT_IIDX")]
pub type LfssCpuIntIidx = crate::Reg<lfss_cpu_int_iidx::LfssCpuIntIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod lfss_cpu_int_iidx;
#[doc = "LFSS_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cpu_int_imask`] module"]
#[doc(alias = "LFSS_CPU_INT_IMASK")]
pub type LfssCpuIntImask = crate::Reg<lfss_cpu_int_imask::LfssCpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod lfss_cpu_int_imask;
#[doc = "LFSS_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cpu_int_ris`] module"]
#[doc(alias = "LFSS_CPU_INT_RIS")]
pub type LfssCpuIntRis = crate::Reg<lfss_cpu_int_ris::LfssCpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod lfss_cpu_int_ris;
#[doc = "LFSS_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cpu_int_mis`] module"]
#[doc(alias = "LFSS_CPU_INT_MIS")]
pub type LfssCpuIntMis = crate::Reg<lfss_cpu_int_mis::LfssCpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod lfss_cpu_int_mis;
#[doc = "LFSS_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cpu_int_iset`] module"]
#[doc(alias = "LFSS_CPU_INT_ISET")]
pub type LfssCpuIntIset = crate::Reg<lfss_cpu_int_iset::LfssCpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod lfss_cpu_int_iset;
#[doc = "LFSS_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cpu_int_iclr`] module"]
#[doc(alias = "LFSS_CPU_INT_ICLR")]
pub type LfssCpuIntIclr = crate::Reg<lfss_cpu_int_iclr::LfssCpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod lfss_cpu_int_iclr;
