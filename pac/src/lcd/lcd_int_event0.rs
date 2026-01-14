#[repr(C)]
#[doc = "LCD_INT_EVENT0\\[%s\\]"]
#[doc(alias = "LCD_INT_EVENT0")]
pub struct LcdIntEvent0 {
    lcd_iidx: LcdIidx,
    _reserved1: [u8; 0x04],
    lcd_imask: LcdImask,
    _reserved2: [u8; 0x04],
    lcd_ris: LcdRis,
    _reserved3: [u8; 0x04],
    lcd_mis: LcdMis,
    _reserved4: [u8; 0x04],
    lcd_iset: LcdIset,
    _reserved5: [u8; 0x04],
    lcd_iclr: LcdIclr,
}
impl LcdIntEvent0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn lcd_iidx(&self) -> &LcdIidx {
        &self.lcd_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn lcd_imask(&self) -> &LcdImask {
        &self.lcd_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn lcd_ris(&self) -> &LcdRis {
        &self.lcd_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn lcd_mis(&self) -> &LcdMis {
        &self.lcd_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn lcd_iset(&self) -> &LcdIset {
        &self.lcd_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn lcd_iclr(&self) -> &LcdIclr {
        &self.lcd_iclr
    }
}
#[doc = "LCD_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_iidx`] module"]
#[doc(alias = "LCD_IIDX")]
pub type LcdIidx = crate::Reg<lcd_iidx::LcdIidxSpec>;
#[doc = "Interrupt index"]
pub mod lcd_iidx;
#[doc = "LCD_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_imask`] module"]
#[doc(alias = "LCD_IMASK")]
pub type LcdImask = crate::Reg<lcd_imask::LcdImaskSpec>;
#[doc = "Interrupt mask"]
pub mod lcd_imask;
#[doc = "LCD_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ris`] module"]
#[doc(alias = "LCD_RIS")]
pub type LcdRis = crate::Reg<lcd_ris::LcdRisSpec>;
#[doc = "Raw interrupt status"]
pub mod lcd_ris;
#[doc = "LCD_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_mis`] module"]
#[doc(alias = "LCD_MIS")]
pub type LcdMis = crate::Reg<lcd_mis::LcdMisSpec>;
#[doc = "Masked interrupt status"]
pub mod lcd_mis;
#[doc = "LCD_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_iset`] module"]
#[doc(alias = "LCD_ISET")]
pub type LcdIset = crate::Reg<lcd_iset::LcdIsetSpec>;
#[doc = "Interrupt set"]
pub mod lcd_iset;
#[doc = "LCD_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_iclr`] module"]
#[doc(alias = "LCD_ICLR")]
pub type LcdIclr = crate::Reg<lcd_iclr::LcdIclrSpec>;
#[doc = "Interrupt clear"]
pub mod lcd_iclr;
