#[repr(C)]
#[doc = "UART2_INT_EVENT0\\[%s\\]"]
#[doc(alias = "UART2_INT_EVENT0")]
pub struct Uart2IntEvent0 {
    uart2_int_event0_iidx: Uart2IntEvent0Iidx,
    _reserved1: [u8; 0x04],
    uart2_int_event0_imask: Uart2IntEvent0Imask,
    _reserved2: [u8; 0x04],
    uart2_int_event0_ris: Uart2IntEvent0Ris,
    _reserved3: [u8; 0x04],
    uart2_int_event0_mis: Uart2IntEvent0Mis,
    _reserved4: [u8; 0x04],
    uart2_int_event0_iset: Uart2IntEvent0Iset,
    _reserved5: [u8; 0x04],
    uart2_int_event0_iclr: Uart2IntEvent0Iclr,
}
impl Uart2IntEvent0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn uart2_int_event0_iidx(&self) -> &Uart2IntEvent0Iidx {
        &self.uart2_int_event0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn uart2_int_event0_imask(&self) -> &Uart2IntEvent0Imask {
        &self.uart2_int_event0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn uart2_int_event0_ris(&self) -> &Uart2IntEvent0Ris {
        &self.uart2_int_event0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn uart2_int_event0_mis(&self) -> &Uart2IntEvent0Mis {
        &self.uart2_int_event0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn uart2_int_event0_iset(&self) -> &Uart2IntEvent0Iset {
        &self.uart2_int_event0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn uart2_int_event0_iclr(&self) -> &Uart2IntEvent0Iclr {
        &self.uart2_int_event0_iclr
    }
}
#[doc = "UART2_INT_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event0_iidx`] module"]
#[doc(alias = "UART2_INT_EVENT0_IIDX")]
pub type Uart2IntEvent0Iidx = crate::Reg<uart2_int_event0_iidx::Uart2IntEvent0IidxSpec>;
#[doc = "Interrupt index"]
pub mod uart2_int_event0_iidx;
#[doc = "UART2_INT_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event0_imask`] module"]
#[doc(alias = "UART2_INT_EVENT0_IMASK")]
pub type Uart2IntEvent0Imask = crate::Reg<uart2_int_event0_imask::Uart2IntEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod uart2_int_event0_imask;
#[doc = "UART2_INT_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event0_ris`] module"]
#[doc(alias = "UART2_INT_EVENT0_RIS")]
pub type Uart2IntEvent0Ris = crate::Reg<uart2_int_event0_ris::Uart2IntEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod uart2_int_event0_ris;
#[doc = "UART2_INT_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event0_mis`] module"]
#[doc(alias = "UART2_INT_EVENT0_MIS")]
pub type Uart2IntEvent0Mis = crate::Reg<uart2_int_event0_mis::Uart2IntEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod uart2_int_event0_mis;
#[doc = "UART2_INT_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event0_iset`] module"]
#[doc(alias = "UART2_INT_EVENT0_ISET")]
pub type Uart2IntEvent0Iset = crate::Reg<uart2_int_event0_iset::Uart2IntEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod uart2_int_event0_iset;
#[doc = "UART2_INT_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event0_iclr`] module"]
#[doc(alias = "UART2_INT_EVENT0_ICLR")]
pub type Uart2IntEvent0Iclr = crate::Reg<uart2_int_event0_iclr::Uart2IntEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod uart2_int_event0_iclr;
