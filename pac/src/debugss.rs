#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    debugss_iidx: DebugssIidx,
    _reserved1: [u8; 0x04],
    debugss_imask: DebugssImask,
    _reserved2: [u8; 0x04],
    debugss_ris: DebugssRis,
    _reserved3: [u8; 0x04],
    debugss_mis: DebugssMis,
    _reserved4: [u8; 0x04],
    debugss_iset: DebugssIset,
    _reserved5: [u8; 0x04],
    debugss_iclr: DebugssIclr,
    _reserved6: [u8; 0x94],
    debugss_evt_mode: DebugssEvtMode,
    _reserved7: [u8; 0x18],
    debugss_desc: DebugssDesc,
    debugss_txd: DebugssTxd,
    debugss_txctl: DebugssTxctl,
    debugss_rxd: DebugssRxd,
    debugss_rxctl: DebugssRxctl,
    _reserved12: [u8; 0xf0],
    debugss_special_auth: DebugssSpecialAuth,
    _reserved13: [u8; 0x0c],
    debugss_app_auth: DebugssAppAuth,
}
impl RegisterBlock {
    #[doc = "0x1020 - Interrupt index"]
    #[inline(always)]
    pub const fn debugss_iidx(&self) -> &DebugssIidx {
        &self.debugss_iidx
    }
    #[doc = "0x1028 - Interrupt mask"]
    #[inline(always)]
    pub const fn debugss_imask(&self) -> &DebugssImask {
        &self.debugss_imask
    }
    #[doc = "0x1030 - Raw interrupt status"]
    #[inline(always)]
    pub const fn debugss_ris(&self) -> &DebugssRis {
        &self.debugss_ris
    }
    #[doc = "0x1038 - Masked interrupt status"]
    #[inline(always)]
    pub const fn debugss_mis(&self) -> &DebugssMis {
        &self.debugss_mis
    }
    #[doc = "0x1040 - Interrupt set"]
    #[inline(always)]
    pub const fn debugss_iset(&self) -> &DebugssIset {
        &self.debugss_iset
    }
    #[doc = "0x1048 - Interrupt clear"]
    #[inline(always)]
    pub const fn debugss_iclr(&self) -> &DebugssIclr {
        &self.debugss_iclr
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn debugss_evt_mode(&self) -> &DebugssEvtMode {
        &self.debugss_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn debugss_desc(&self) -> &DebugssDesc {
        &self.debugss_desc
    }
    #[doc = "0x1100 - Transmit data register"]
    #[inline(always)]
    pub const fn debugss_txd(&self) -> &DebugssTxd {
        &self.debugss_txd
    }
    #[doc = "0x1104 - Transmit control register"]
    #[inline(always)]
    pub const fn debugss_txctl(&self) -> &DebugssTxctl {
        &self.debugss_txctl
    }
    #[doc = "0x1108 - Receive data register"]
    #[inline(always)]
    pub const fn debugss_rxd(&self) -> &DebugssRxd {
        &self.debugss_rxd
    }
    #[doc = "0x110c - Receive control register"]
    #[inline(always)]
    pub const fn debugss_rxctl(&self) -> &DebugssRxctl {
        &self.debugss_rxctl
    }
    #[doc = "0x1200 - Special enable authorization register"]
    #[inline(always)]
    pub const fn debugss_special_auth(&self) -> &DebugssSpecialAuth {
        &self.debugss_special_auth
    }
    #[doc = "0x1210 - Application CPU0 authorization register"]
    #[inline(always)]
    pub const fn debugss_app_auth(&self) -> &DebugssAppAuth {
        &self.debugss_app_auth
    }
}
#[doc = "DEBUGSS_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_iidx`] module"]
#[doc(alias = "DEBUGSS_IIDX")]
pub type DebugssIidx = crate::Reg<debugss_iidx::DebugssIidxSpec>;
#[doc = "Interrupt index"]
pub mod debugss_iidx;
#[doc = "DEBUGSS_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_imask`] module"]
#[doc(alias = "DEBUGSS_IMASK")]
pub type DebugssImask = crate::Reg<debugss_imask::DebugssImaskSpec>;
#[doc = "Interrupt mask"]
pub mod debugss_imask;
#[doc = "DEBUGSS_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_ris`] module"]
#[doc(alias = "DEBUGSS_RIS")]
pub type DebugssRis = crate::Reg<debugss_ris::DebugssRisSpec>;
#[doc = "Raw interrupt status"]
pub mod debugss_ris;
#[doc = "DEBUGSS_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_mis`] module"]
#[doc(alias = "DEBUGSS_MIS")]
pub type DebugssMis = crate::Reg<debugss_mis::DebugssMisSpec>;
#[doc = "Masked interrupt status"]
pub mod debugss_mis;
#[doc = "DEBUGSS_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_iset`] module"]
#[doc(alias = "DEBUGSS_ISET")]
pub type DebugssIset = crate::Reg<debugss_iset::DebugssIsetSpec>;
#[doc = "Interrupt set"]
pub mod debugss_iset;
#[doc = "DEBUGSS_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_iclr`] module"]
#[doc(alias = "DEBUGSS_ICLR")]
pub type DebugssIclr = crate::Reg<debugss_iclr::DebugssIclrSpec>;
#[doc = "Interrupt clear"]
pub mod debugss_iclr;
#[doc = "DEBUGSS_EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_evt_mode`] module"]
#[doc(alias = "DEBUGSS_EVT_MODE")]
pub type DebugssEvtMode = crate::Reg<debugss_evt_mode::DebugssEvtModeSpec>;
#[doc = "Event Mode"]
pub mod debugss_evt_mode;
#[doc = "DEBUGSS_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_desc`] module"]
#[doc(alias = "DEBUGSS_DESC")]
pub type DebugssDesc = crate::Reg<debugss_desc::DebugssDescSpec>;
#[doc = "Module Description"]
pub mod debugss_desc;
#[doc = "DEBUGSS_TXD (r) register accessor: Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_txd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_txd`] module"]
#[doc(alias = "DEBUGSS_TXD")]
pub type DebugssTxd = crate::Reg<debugss_txd::DebugssTxdSpec>;
#[doc = "Transmit data register"]
pub mod debugss_txd;
#[doc = "DEBUGSS_TXCTL (r) register accessor: Transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_txctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_txctl`] module"]
#[doc(alias = "DEBUGSS_TXCTL")]
pub type DebugssTxctl = crate::Reg<debugss_txctl::DebugssTxctlSpec>;
#[doc = "Transmit control register"]
pub mod debugss_txctl;
#[doc = "DEBUGSS_RXD (rw) register accessor: Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_rxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_rxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_rxd`] module"]
#[doc(alias = "DEBUGSS_RXD")]
pub type DebugssRxd = crate::Reg<debugss_rxd::DebugssRxdSpec>;
#[doc = "Receive data register"]
pub mod debugss_rxd;
#[doc = "DEBUGSS_RXCTL (rw) register accessor: Receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_rxctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_rxctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_rxctl`] module"]
#[doc(alias = "DEBUGSS_RXCTL")]
pub type DebugssRxctl = crate::Reg<debugss_rxctl::DebugssRxctlSpec>;
#[doc = "Receive control register"]
pub mod debugss_rxctl;
#[doc = "DEBUGSS_SPECIAL_AUTH (r) register accessor: Special enable authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_special_auth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_special_auth`] module"]
#[doc(alias = "DEBUGSS_SPECIAL_AUTH")]
pub type DebugssSpecialAuth = crate::Reg<debugss_special_auth::DebugssSpecialAuthSpec>;
#[doc = "Special enable authorization register"]
pub mod debugss_special_auth;
#[doc = "DEBUGSS_APP_AUTH (r) register accessor: Application CPU0 authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_app_auth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_app_auth`] module"]
#[doc(alias = "DEBUGSS_APP_AUTH")]
pub type DebugssAppAuth = crate::Reg<debugss_app_auth::DebugssAppAuthSpec>;
#[doc = "Application CPU0 authorization register"]
pub mod debugss_app_auth;
