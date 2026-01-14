#[repr(C)]
#[doc = "SPI1_CPU_INT\\[%s\\]"]
#[doc(alias = "SPI1_CPU_INT")]
pub struct Spi1CpuInt {
    spi1_cpu_int_iidx: Spi1CpuIntIidx,
    _reserved1: [u8; 0x04],
    spi1_cpu_int_imask: Spi1CpuIntImask,
    _reserved2: [u8; 0x04],
    spi1_cpu_int_ris: Spi1CpuIntRis,
    _reserved3: [u8; 0x04],
    spi1_cpu_int_mis: Spi1CpuIntMis,
    _reserved4: [u8; 0x04],
    spi1_cpu_int_iset: Spi1CpuIntIset,
    _reserved5: [u8; 0x04],
    spi1_cpu_int_iclr: Spi1CpuIntIclr,
}
impl Spi1CpuInt {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn spi1_cpu_int_iidx(&self) -> &Spi1CpuIntIidx {
        &self.spi1_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn spi1_cpu_int_imask(&self) -> &Spi1CpuIntImask {
        &self.spi1_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn spi1_cpu_int_ris(&self) -> &Spi1CpuIntRis {
        &self.spi1_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn spi1_cpu_int_mis(&self) -> &Spi1CpuIntMis {
        &self.spi1_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn spi1_cpu_int_iset(&self) -> &Spi1CpuIntIset {
        &self.spi1_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn spi1_cpu_int_iclr(&self) -> &Spi1CpuIntIclr {
        &self.spi1_cpu_int_iclr
    }
}
#[doc = "SPI1_CPU_INT_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_cpu_int_iidx`] module"]
#[doc(alias = "SPI1_CPU_INT_IIDX")]
pub type Spi1CpuIntIidx = crate::Reg<spi1_cpu_int_iidx::Spi1CpuIntIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod spi1_cpu_int_iidx;
#[doc = "SPI1_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_cpu_int_imask`] module"]
#[doc(alias = "SPI1_CPU_INT_IMASK")]
pub type Spi1CpuIntImask = crate::Reg<spi1_cpu_int_imask::Spi1CpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod spi1_cpu_int_imask;
#[doc = "SPI1_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_cpu_int_ris`] module"]
#[doc(alias = "SPI1_CPU_INT_RIS")]
pub type Spi1CpuIntRis = crate::Reg<spi1_cpu_int_ris::Spi1CpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod spi1_cpu_int_ris;
#[doc = "SPI1_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_cpu_int_mis`] module"]
#[doc(alias = "SPI1_CPU_INT_MIS")]
pub type Spi1CpuIntMis = crate::Reg<spi1_cpu_int_mis::Spi1CpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod spi1_cpu_int_mis;
#[doc = "SPI1_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_cpu_int_iset`] module"]
#[doc(alias = "SPI1_CPU_INT_ISET")]
pub type Spi1CpuIntIset = crate::Reg<spi1_cpu_int_iset::Spi1CpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod spi1_cpu_int_iset;
#[doc = "SPI1_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_cpu_int_iclr`] module"]
#[doc(alias = "SPI1_CPU_INT_ICLR")]
pub type Spi1CpuIntIclr = crate::Reg<spi1_cpu_int_iclr::Spi1CpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod spi1_cpu_int_iclr;
