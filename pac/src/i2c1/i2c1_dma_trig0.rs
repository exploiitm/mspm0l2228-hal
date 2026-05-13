#[repr(C)]
#[doc = "I2C1_DMA_TRIG0\\[%s\\]"]
#[doc(alias = "I2C1_DMA_TRIG0")]
pub struct I2c1DmaTrig0 {
    i2c1_dma_trig0_iidx: I2c1DmaTrig0Iidx,
    _reserved1: [u8; 0x04],
    i2c1_dma_trig0_imask: I2c1DmaTrig0Imask,
    _reserved2: [u8; 0x04],
    i2c1_dma_trig0_ris: I2c1DmaTrig0Ris,
    _reserved3: [u8; 0x04],
    i2c1_dma_trig0_mis: I2c1DmaTrig0Mis,
    _reserved4: [u8; 0x04],
    i2c1_dma_trig0_iset: I2c1DmaTrig0Iset,
    _reserved5: [u8; 0x04],
    i2c1_dma_trig0_iclr: I2c1DmaTrig0Iclr,
}
impl I2c1DmaTrig0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn i2c1_dma_trig0_iidx(&self) -> &I2c1DmaTrig0Iidx {
        &self.i2c1_dma_trig0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn i2c1_dma_trig0_imask(&self) -> &I2c1DmaTrig0Imask {
        &self.i2c1_dma_trig0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn i2c1_dma_trig0_ris(&self) -> &I2c1DmaTrig0Ris {
        &self.i2c1_dma_trig0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn i2c1_dma_trig0_mis(&self) -> &I2c1DmaTrig0Mis {
        &self.i2c1_dma_trig0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn i2c1_dma_trig0_iset(&self) -> &I2c1DmaTrig0Iset {
        &self.i2c1_dma_trig0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn i2c1_dma_trig0_iclr(&self) -> &I2c1DmaTrig0Iclr {
        &self.i2c1_dma_trig0_iclr
    }
}
#[doc = "I2C1_DMA_TRIG0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_dma_trig0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_dma_trig0_iidx`] module"]
#[doc(alias = "I2C1_DMA_TRIG0_IIDX")]
pub type I2c1DmaTrig0Iidx = crate::Reg<i2c1_dma_trig0_iidx::I2c1DmaTrig0IidxSpec>;
#[doc = "Interrupt index"]
pub mod i2c1_dma_trig0_iidx;
#[doc = "I2C1_DMA_TRIG0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_dma_trig0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_dma_trig0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_dma_trig0_imask`] module"]
#[doc(alias = "I2C1_DMA_TRIG0_IMASK")]
pub type I2c1DmaTrig0Imask = crate::Reg<i2c1_dma_trig0_imask::I2c1DmaTrig0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod i2c1_dma_trig0_imask;
#[doc = "I2C1_DMA_TRIG0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_dma_trig0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_dma_trig0_ris`] module"]
#[doc(alias = "I2C1_DMA_TRIG0_RIS")]
pub type I2c1DmaTrig0Ris = crate::Reg<i2c1_dma_trig0_ris::I2c1DmaTrig0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod i2c1_dma_trig0_ris;
#[doc = "I2C1_DMA_TRIG0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_dma_trig0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_dma_trig0_mis`] module"]
#[doc(alias = "I2C1_DMA_TRIG0_MIS")]
pub type I2c1DmaTrig0Mis = crate::Reg<i2c1_dma_trig0_mis::I2c1DmaTrig0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod i2c1_dma_trig0_mis;
#[doc = "I2C1_DMA_TRIG0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_dma_trig0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_dma_trig0_iset`] module"]
#[doc(alias = "I2C1_DMA_TRIG0_ISET")]
pub type I2c1DmaTrig0Iset = crate::Reg<i2c1_dma_trig0_iset::I2c1DmaTrig0IsetSpec>;
#[doc = "Interrupt set"]
pub mod i2c1_dma_trig0_iset;
#[doc = "I2C1_DMA_TRIG0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_dma_trig0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_dma_trig0_iclr`] module"]
#[doc(alias = "I2C1_DMA_TRIG0_ICLR")]
pub type I2c1DmaTrig0Iclr = crate::Reg<i2c1_dma_trig0_iclr::I2c1DmaTrig0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod i2c1_dma_trig0_iclr;
