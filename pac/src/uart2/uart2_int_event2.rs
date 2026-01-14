#[repr(C)]
#[doc = "UART2_INT_EVENT2\\[%s\\]"]
#[doc(alias = "UART2_INT_EVENT2")]
pub struct Uart2IntEvent2 {
    uart2_int_event2_iidx: Uart2IntEvent2Iidx,
    _reserved1: [u8; 0x04],
    uart2_int_event2_imask: Uart2IntEvent2Imask,
    _reserved2: [u8; 0x04],
    uart2_int_event2_ris: Uart2IntEvent2Ris,
    _reserved3: [u8; 0x04],
    uart2_int_event2_mis: Uart2IntEvent2Mis,
    _reserved4: [u8; 0x04],
    uart2_int_event2_iset: Uart2IntEvent2Iset,
    _reserved5: [u8; 0x04],
    uart2_int_event2_iclr: Uart2IntEvent2Iclr,
}
impl Uart2IntEvent2 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn uart2_int_event2_iidx(&self) -> &Uart2IntEvent2Iidx {
        &self.uart2_int_event2_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn uart2_int_event2_imask(&self) -> &Uart2IntEvent2Imask {
        &self.uart2_int_event2_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn uart2_int_event2_ris(&self) -> &Uart2IntEvent2Ris {
        &self.uart2_int_event2_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn uart2_int_event2_mis(&self) -> &Uart2IntEvent2Mis {
        &self.uart2_int_event2_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn uart2_int_event2_iset(&self) -> &Uart2IntEvent2Iset {
        &self.uart2_int_event2_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn uart2_int_event2_iclr(&self) -> &Uart2IntEvent2Iclr {
        &self.uart2_int_event2_iclr
    }
}
#[doc = "UART2_INT_EVENT2_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event2_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event2_iidx`] module"]
#[doc(alias = "UART2_INT_EVENT2_IIDX")]
pub type Uart2IntEvent2Iidx = crate::Reg<uart2_int_event2_iidx::Uart2IntEvent2IidxSpec>;
#[doc = "Interrupt index"]
pub mod uart2_int_event2_iidx;
#[doc = "UART2_INT_EVENT2_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event2_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event2_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event2_imask`] module"]
#[doc(alias = "UART2_INT_EVENT2_IMASK")]
pub type Uart2IntEvent2Imask = crate::Reg<uart2_int_event2_imask::Uart2IntEvent2ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod uart2_int_event2_imask;
#[doc = "UART2_INT_EVENT2_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event2_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event2_ris`] module"]
#[doc(alias = "UART2_INT_EVENT2_RIS")]
pub type Uart2IntEvent2Ris = crate::Reg<uart2_int_event2_ris::Uart2IntEvent2RisSpec>;
#[doc = "Raw interrupt status"]
pub mod uart2_int_event2_ris;
#[doc = "UART2_INT_EVENT2_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event2_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event2_mis`] module"]
#[doc(alias = "UART2_INT_EVENT2_MIS")]
pub type Uart2IntEvent2Mis = crate::Reg<uart2_int_event2_mis::Uart2IntEvent2MisSpec>;
#[doc = "Masked interrupt status"]
pub mod uart2_int_event2_mis;
#[doc = "UART2_INT_EVENT2_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event2_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event2_iset`] module"]
#[doc(alias = "UART2_INT_EVENT2_ISET")]
pub type Uart2IntEvent2Iset = crate::Reg<uart2_int_event2_iset::Uart2IntEvent2IsetSpec>;
#[doc = "Interrupt set"]
pub mod uart2_int_event2_iset;
#[doc = "UART2_INT_EVENT2_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event2_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_event2_iclr`] module"]
#[doc(alias = "UART2_INT_EVENT2_ICLR")]
pub type Uart2IntEvent2Iclr = crate::Reg<uart2_int_event2_iclr::Uart2IntEvent2IclrSpec>;
#[doc = "Interrupt clear"]
pub mod uart2_int_event2_iclr;
