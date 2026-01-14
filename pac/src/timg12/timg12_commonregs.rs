#[repr(C)]
#[doc = "TIMG12_COMMONREGS\\[%s\\]"]
#[doc(alias = "TIMG12_COMMONREGS")]
pub struct Timg12Commonregs {
    timg12_ccpd: Timg12Ccpd,
    timg12_odis: Timg12Odis,
    timg12_cclkctl: Timg12Cclkctl,
    _reserved3: [u8; 0x08],
    timg12_cttrigctl: Timg12Cttrigctl,
    _reserved4: [u8; 0x04],
    timg12_cttrig: Timg12Cttrig,
}
impl Timg12Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn timg12_ccpd(&self) -> &Timg12Ccpd {
        &self.timg12_ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn timg12_odis(&self) -> &Timg12Odis {
        &self.timg12_odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn timg12_cclkctl(&self) -> &Timg12Cclkctl {
        &self.timg12_cclkctl
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn timg12_cttrigctl(&self) -> &Timg12Cttrigctl {
        &self.timg12_cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn timg12_cttrig(&self) -> &Timg12Cttrig {
        &self.timg12_cttrig
    }
}
#[doc = "TIMG12_CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ccpd`] module"]
#[doc(alias = "TIMG12_CCPD")]
pub type Timg12Ccpd = crate::Reg<timg12_ccpd::Timg12CcpdSpec>;
#[doc = "CCP Direction"]
pub mod timg12_ccpd;
#[doc = "TIMG12_ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_odis`] module"]
#[doc(alias = "TIMG12_ODIS")]
pub type Timg12Odis = crate::Reg<timg12_odis::Timg12OdisSpec>;
#[doc = "Output Disable"]
pub mod timg12_odis;
#[doc = "TIMG12_CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_cclkctl`] module"]
#[doc(alias = "TIMG12_CCLKCTL")]
pub type Timg12Cclkctl = crate::Reg<timg12_cclkctl::Timg12CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod timg12_cclkctl;
#[doc = "TIMG12_CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_cttrigctl`] module"]
#[doc(alias = "TIMG12_CTTRIGCTL")]
pub type Timg12Cttrigctl = crate::Reg<timg12_cttrigctl::Timg12CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod timg12_cttrigctl;
#[doc = "TIMG12_CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_cttrig`] module"]
#[doc(alias = "TIMG12_CTTRIG")]
pub type Timg12Cttrig = crate::Reg<timg12_cttrig::Timg12CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod timg12_cttrig;
