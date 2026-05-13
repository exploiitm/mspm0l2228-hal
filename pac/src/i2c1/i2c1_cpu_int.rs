#[repr(C)]
#[doc = "I2C1_CPU_INT\\[%s\\]"]
#[doc(alias = "I2C1_CPU_INT")]
pub struct I2c1CpuInt {
    i2c1_cpu_int_iidx: I2c1CpuIntIidx,
    _reserved1: [u8; 0x04],
    i2c1_cpu_int_imask: I2c1CpuIntImask,
    _reserved2: [u8; 0x04],
    i2c1_cpu_int_ris: I2c1CpuIntRis,
    _reserved3: [u8; 0x04],
    i2c1_cpu_int_mis: I2c1CpuIntMis,
    _reserved4: [u8; 0x04],
    i2c1_cpu_int_iset: I2c1CpuIntIset,
    _reserved5: [u8; 0x04],
    i2c1_cpu_int_iclr: I2c1CpuIntIclr,
}
impl I2c1CpuInt {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn i2c1_cpu_int_iidx(&self) -> &I2c1CpuIntIidx {
        &self.i2c1_cpu_int_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn i2c1_cpu_int_imask(&self) -> &I2c1CpuIntImask {
        &self.i2c1_cpu_int_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn i2c1_cpu_int_ris(&self) -> &I2c1CpuIntRis {
        &self.i2c1_cpu_int_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn i2c1_cpu_int_mis(&self) -> &I2c1CpuIntMis {
        &self.i2c1_cpu_int_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn i2c1_cpu_int_iset(&self) -> &I2c1CpuIntIset {
        &self.i2c1_cpu_int_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn i2c1_cpu_int_iclr(&self) -> &I2c1CpuIntIclr {
        &self.i2c1_cpu_int_iclr
    }
}
#[doc = "I2C1_CPU_INT_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cpu_int_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cpu_int_iidx`] module"]
#[doc(alias = "I2C1_CPU_INT_IIDX")]
pub type I2c1CpuIntIidx = crate::Reg<i2c1_cpu_int_iidx::I2c1CpuIntIidxSpec>;
#[doc = "Interrupt index"]
pub mod i2c1_cpu_int_iidx;
#[doc = "I2C1_CPU_INT_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cpu_int_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_cpu_int_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cpu_int_imask`] module"]
#[doc(alias = "I2C1_CPU_INT_IMASK")]
pub type I2c1CpuIntImask = crate::Reg<i2c1_cpu_int_imask::I2c1CpuIntImaskSpec>;
#[doc = "Interrupt mask"]
pub mod i2c1_cpu_int_imask;
#[doc = "I2C1_CPU_INT_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cpu_int_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cpu_int_ris`] module"]
#[doc(alias = "I2C1_CPU_INT_RIS")]
pub type I2c1CpuIntRis = crate::Reg<i2c1_cpu_int_ris::I2c1CpuIntRisSpec>;
#[doc = "Raw interrupt status"]
pub mod i2c1_cpu_int_ris;
#[doc = "I2C1_CPU_INT_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cpu_int_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cpu_int_mis`] module"]
#[doc(alias = "I2C1_CPU_INT_MIS")]
pub type I2c1CpuIntMis = crate::Reg<i2c1_cpu_int_mis::I2c1CpuIntMisSpec>;
#[doc = "Masked interrupt status"]
pub mod i2c1_cpu_int_mis;
#[doc = "I2C1_CPU_INT_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_cpu_int_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cpu_int_iset`] module"]
#[doc(alias = "I2C1_CPU_INT_ISET")]
pub type I2c1CpuIntIset = crate::Reg<i2c1_cpu_int_iset::I2c1CpuIntIsetSpec>;
#[doc = "Interrupt set"]
pub mod i2c1_cpu_int_iset;
#[doc = "I2C1_CPU_INT_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_cpu_int_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cpu_int_iclr`] module"]
#[doc(alias = "I2C1_CPU_INT_ICLR")]
pub type I2c1CpuIntIclr = crate::Reg<i2c1_cpu_int_iclr::I2c1CpuIntIclrSpec>;
#[doc = "Interrupt clear"]
pub mod i2c1_cpu_int_iclr;
