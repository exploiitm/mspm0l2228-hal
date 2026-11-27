#[repr(C)]
#[doc = "SPI0_CPU_INT\\[%s\\]"]
#[doc(alias = "SPI0_CPU_INT")]
pub struct Spi0CpuInt {
    spi0_cpu_int_iidx: Spi0CpuIntIidx,
    _reserved1: [u8; 0x04],
    spi0_cpu_int_imask: Spi0CpuIntImask,
    _reserved2: [u8; 0x04],
    spi0_cpu_int_ris: Spi0CpuIntRis,
    _reserved3: [u8; 0x04],
    spi0_cpu_int_mis: Spi0CpuIntMis,
    _reserved4: [u8; 0x04],
    spi0_cpu_int_iset: Spi0CpuIntIset,
    _reserved5: [u8; 0x04],
    spi0_cpu_int_iclr: Spi0CpuIntIclr,
}
impl Spi0CpuInt {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn spi0_cpu_int_iidx(&self) -> &Spi0CpuIntIidx {
        &self.spi0_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn spi0_cpu_int_imask(&self) -> &Spi0CpuIntImask {
        &self.spi0_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn spi0_cpu_int_ris(&self) -> &Spi0CpuIntRis {
        &self.spi0_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn spi0_cpu_int_mis(&self) -> &Spi0CpuIntMis {
        &self.spi0_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn spi0_cpu_int_iset(&self) -> &Spi0CpuIntIset {
        &self.spi0_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn spi0_cpu_int_iclr(&self) -> &Spi0CpuIntIclr {
        &self.spi0_cpu_int_iclr
    }
}
#[doc = "SPI0_CPU_INT_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_cpu_int_iidx`] module"]
#[doc(alias = "SPI0_CPU_INT_IIDX")]
pub type Spi0CpuIntIidx = crate::Reg<spi0_cpu_int_iidx::Spi0CpuIntIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod spi0_cpu_int_iidx;
#[doc = "SPI0_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_cpu_int_imask`] module"]
#[doc(alias = "SPI0_CPU_INT_IMASK")]
pub type Spi0CpuIntImask = crate::Reg<spi0_cpu_int_imask::Spi0CpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod spi0_cpu_int_imask;
#[doc = "SPI0_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_cpu_int_ris`] module"]
#[doc(alias = "SPI0_CPU_INT_RIS")]
pub type Spi0CpuIntRis = crate::Reg<spi0_cpu_int_ris::Spi0CpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod spi0_cpu_int_ris;
#[doc = "SPI0_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_cpu_int_mis`] module"]
#[doc(alias = "SPI0_CPU_INT_MIS")]
pub type Spi0CpuIntMis = crate::Reg<spi0_cpu_int_mis::Spi0CpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod spi0_cpu_int_mis;
#[doc = "SPI0_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_cpu_int_iset`] module"]
#[doc(alias = "SPI0_CPU_INT_ISET")]
pub type Spi0CpuIntIset = crate::Reg<spi0_cpu_int_iset::Spi0CpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod spi0_cpu_int_iset;
#[doc = "SPI0_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_cpu_int_iclr`] module"]
#[doc(alias = "SPI0_CPU_INT_ICLR")]
pub type Spi0CpuIntIclr = crate::Reg<spi0_cpu_int_iclr::Spi0CpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod spi0_cpu_int_iclr;
