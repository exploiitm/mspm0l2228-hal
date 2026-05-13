#[repr(C)]
#[doc = "I2C0_DMA_TRIG1\\[%s\\]"]
#[doc(alias = "I2C0_DMA_TRIG1")]
pub struct I2c0DmaTrig1 {
    i2c0_dma_trig1_iidx: I2c0DmaTrig1Iidx,
    _reserved1: [u8; 0x04],
    i2c0_dma_trig1_imask: I2c0DmaTrig1Imask,
    _reserved2: [u8; 0x04],
    i2c0_dma_trig1_ris: I2c0DmaTrig1Ris,
    _reserved3: [u8; 0x04],
    i2c0_dma_trig1_mis: I2c0DmaTrig1Mis,
    _reserved4: [u8; 0x04],
    i2c0_dma_trig1_iset: I2c0DmaTrig1Iset,
    _reserved5: [u8; 0x04],
    i2c0_dma_trig1_iclr: I2c0DmaTrig1Iclr,
}
impl I2c0DmaTrig1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1_iidx(&self) -> &I2c0DmaTrig1Iidx {
        &self.i2c0_dma_trig1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1_imask(&self) -> &I2c0DmaTrig1Imask {
        &self.i2c0_dma_trig1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1_ris(&self) -> &I2c0DmaTrig1Ris {
        &self.i2c0_dma_trig1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1_mis(&self) -> &I2c0DmaTrig1Mis {
        &self.i2c0_dma_trig1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1_iset(&self) -> &I2c0DmaTrig1Iset {
        &self.i2c0_dma_trig1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1_iclr(&self) -> &I2c0DmaTrig1Iclr {
        &self.i2c0_dma_trig1_iclr
    }
}
#[doc = "I2C0_DMA_TRIG1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_dma_trig1_iidx`] module"]
#[doc(alias = "I2C0_DMA_TRIG1_IIDX")]
pub type I2c0DmaTrig1Iidx = crate::Reg<i2c0_dma_trig1_iidx::I2c0DmaTrig1IidxSpec>;
#[doc = "Interrupt index"]
pub mod i2c0_dma_trig1_iidx;
#[doc = "I2C0_DMA_TRIG1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_dma_trig1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_dma_trig1_imask`] module"]
#[doc(alias = "I2C0_DMA_TRIG1_IMASK")]
pub type I2c0DmaTrig1Imask = crate::Reg<i2c0_dma_trig1_imask::I2c0DmaTrig1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod i2c0_dma_trig1_imask;
#[doc = "I2C0_DMA_TRIG1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_dma_trig1_ris`] module"]
#[doc(alias = "I2C0_DMA_TRIG1_RIS")]
pub type I2c0DmaTrig1Ris = crate::Reg<i2c0_dma_trig1_ris::I2c0DmaTrig1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod i2c0_dma_trig1_ris;
#[doc = "I2C0_DMA_TRIG1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_dma_trig1_mis`] module"]
#[doc(alias = "I2C0_DMA_TRIG1_MIS")]
pub type I2c0DmaTrig1Mis = crate::Reg<i2c0_dma_trig1_mis::I2c0DmaTrig1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod i2c0_dma_trig1_mis;
#[doc = "I2C0_DMA_TRIG1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_dma_trig1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_dma_trig1_iset`] module"]
#[doc(alias = "I2C0_DMA_TRIG1_ISET")]
pub type I2c0DmaTrig1Iset = crate::Reg<i2c0_dma_trig1_iset::I2c0DmaTrig1IsetSpec>;
#[doc = "Interrupt set"]
pub mod i2c0_dma_trig1_iset;
#[doc = "I2C0_DMA_TRIG1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_dma_trig1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_dma_trig1_iclr`] module"]
#[doc(alias = "I2C0_DMA_TRIG1_ICLR")]
pub type I2c0DmaTrig1Iclr = crate::Reg<i2c0_dma_trig1_iclr::I2c0DmaTrig1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod i2c0_dma_trig1_iclr;
