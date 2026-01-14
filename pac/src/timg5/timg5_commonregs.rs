#[repr(C)]
#[doc = "TIMG5_COMMONREGS\\[%s\\]"]
#[doc(alias = "TIMG5_COMMONREGS")]
pub struct Timg5Commonregs {
    timg5_ccpd: Timg5Ccpd,
    timg5_odis: Timg5Odis,
    timg5_cclkctl: Timg5Cclkctl,
    timg5_cps: Timg5Cps,
    timg5_cpsv: Timg5Cpsv,
    timg5_cttrigctl: Timg5Cttrigctl,
    _reserved6: [u8; 0x04],
    timg5_cttrig: Timg5Cttrig,
    _reserved7: [u8; 0x04],
    timg5_gctl: Timg5Gctl,
}
impl Timg5Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn timg5_ccpd(&self) -> &Timg5Ccpd {
        &self.timg5_ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn timg5_odis(&self) -> &Timg5Odis {
        &self.timg5_odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn timg5_cclkctl(&self) -> &Timg5Cclkctl {
        &self.timg5_cclkctl
    }
    #[doc = "0x0c - Clock Prescale Register"]
    #[inline(always)]
    pub const fn timg5_cps(&self) -> &Timg5Cps {
        &self.timg5_cps
    }
    #[doc = "0x10 - Clock prescale count status register"]
    #[inline(always)]
    pub const fn timg5_cpsv(&self) -> &Timg5Cpsv {
        &self.timg5_cpsv
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn timg5_cttrigctl(&self) -> &Timg5Cttrigctl {
        &self.timg5_cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn timg5_cttrig(&self) -> &Timg5Cttrig {
        &self.timg5_cttrig
    }
    #[doc = "0x24 - Shadow to active load mask"]
    #[inline(always)]
    pub const fn timg5_gctl(&self) -> &Timg5Gctl {
        &self.timg5_gctl
    }
}
#[doc = "TIMG5_CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ccpd`] module"]
#[doc(alias = "TIMG5_CCPD")]
pub type Timg5Ccpd = crate::Reg<timg5_ccpd::Timg5CcpdSpec>;
#[doc = "CCP Direction"]
pub mod timg5_ccpd;
#[doc = "TIMG5_ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_odis`] module"]
#[doc(alias = "TIMG5_ODIS")]
pub type Timg5Odis = crate::Reg<timg5_odis::Timg5OdisSpec>;
#[doc = "Output Disable"]
pub mod timg5_odis;
#[doc = "TIMG5_CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_cclkctl`] module"]
#[doc(alias = "TIMG5_CCLKCTL")]
pub type Timg5Cclkctl = crate::Reg<timg5_cclkctl::Timg5CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod timg5_cclkctl;
#[doc = "TIMG5_CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_cps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_cps`] module"]
#[doc(alias = "TIMG5_CPS")]
pub type Timg5Cps = crate::Reg<timg5_cps::Timg5CpsSpec>;
#[doc = "Clock Prescale Register"]
pub mod timg5_cps;
#[doc = "TIMG5_CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cpsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_cpsv`] module"]
#[doc(alias = "TIMG5_CPSV")]
pub type Timg5Cpsv = crate::Reg<timg5_cpsv::Timg5CpsvSpec>;
#[doc = "Clock prescale count status register"]
pub mod timg5_cpsv;
#[doc = "TIMG5_CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_cttrigctl`] module"]
#[doc(alias = "TIMG5_CTTRIGCTL")]
pub type Timg5Cttrigctl = crate::Reg<timg5_cttrigctl::Timg5CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod timg5_cttrigctl;
#[doc = "TIMG5_CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_cttrig`] module"]
#[doc(alias = "TIMG5_CTTRIG")]
pub type Timg5Cttrig = crate::Reg<timg5_cttrig::Timg5CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod timg5_cttrig;
#[doc = "TIMG5_GCTL (rw) register accessor: Shadow to active load mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_gctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_gctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_gctl`] module"]
#[doc(alias = "TIMG5_GCTL")]
pub type Timg5Gctl = crate::Reg<timg5_gctl::Timg5GctlSpec>;
#[doc = "Shadow to active load mask"]
pub mod timg5_gctl;
