#[repr(C)]
#[doc = "UART3_INT_EVENT1\\[%s\\]"]
#[doc(alias = "UART3_INT_EVENT1")]
pub struct Uart3IntEvent1 {
    uart3_int_event1_iidx: Uart3IntEvent1Iidx,
    _reserved1: [u8; 0x04],
    uart3_int_event1_imask: Uart3IntEvent1Imask,
    _reserved2: [u8; 0x04],
    uart3_int_event1_ris: Uart3IntEvent1Ris,
    _reserved3: [u8; 0x04],
    uart3_int_event1_mis: Uart3IntEvent1Mis,
    _reserved4: [u8; 0x04],
    uart3_int_event1_iset: Uart3IntEvent1Iset,
    _reserved5: [u8; 0x04],
    uart3_int_event1_iclr: Uart3IntEvent1Iclr,
}
impl Uart3IntEvent1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn uart3_int_event1_iidx(&self) -> &Uart3IntEvent1Iidx {
        &self.uart3_int_event1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn uart3_int_event1_imask(&self) -> &Uart3IntEvent1Imask {
        &self.uart3_int_event1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn uart3_int_event1_ris(&self) -> &Uart3IntEvent1Ris {
        &self.uart3_int_event1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn uart3_int_event1_mis(&self) -> &Uart3IntEvent1Mis {
        &self.uart3_int_event1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn uart3_int_event1_iset(&self) -> &Uart3IntEvent1Iset {
        &self.uart3_int_event1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn uart3_int_event1_iclr(&self) -> &Uart3IntEvent1Iclr {
        &self.uart3_int_event1_iclr
    }
}
#[doc = "UART3_INT_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_int_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_event1_iidx`] module"]
#[doc(alias = "UART3_INT_EVENT1_IIDX")]
pub type Uart3IntEvent1Iidx = crate::Reg<uart3_int_event1_iidx::Uart3IntEvent1IidxSpec>;
#[doc = "Interrupt index"]
pub mod uart3_int_event1_iidx;
#[doc = "UART3_INT_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_int_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_int_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_event1_imask`] module"]
#[doc(alias = "UART3_INT_EVENT1_IMASK")]
pub type Uart3IntEvent1Imask = crate::Reg<uart3_int_event1_imask::Uart3IntEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod uart3_int_event1_imask;
#[doc = "UART3_INT_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_int_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_event1_ris`] module"]
#[doc(alias = "UART3_INT_EVENT1_RIS")]
pub type Uart3IntEvent1Ris = crate::Reg<uart3_int_event1_ris::Uart3IntEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod uart3_int_event1_ris;
#[doc = "UART3_INT_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_int_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_event1_mis`] module"]
#[doc(alias = "UART3_INT_EVENT1_MIS")]
pub type Uart3IntEvent1Mis = crate::Reg<uart3_int_event1_mis::Uart3IntEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod uart3_int_event1_mis;
#[doc = "UART3_INT_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_int_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_event1_iset`] module"]
#[doc(alias = "UART3_INT_EVENT1_ISET")]
pub type Uart3IntEvent1Iset = crate::Reg<uart3_int_event1_iset::Uart3IntEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod uart3_int_event1_iset;
#[doc = "UART3_INT_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_int_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_event1_iclr`] module"]
#[doc(alias = "UART3_INT_EVENT1_ICLR")]
pub type Uart3IntEvent1Iclr = crate::Reg<uart3_int_event1_iclr::Uart3IntEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod uart3_int_event1_iclr;
