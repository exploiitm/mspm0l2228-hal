#[repr(C)]
#[doc = "TIMA0_COMMONREGS\\[%s\\]"]
#[doc(alias = "TIMA0_COMMONREGS")]
pub struct Tima0Commonregs {
    tima0_ccpd: Tima0Ccpd,
    tima0_odis: Tima0Odis,
    tima0_cclkctl: Tima0Cclkctl,
    tima0_cps: Tima0Cps,
    tima0_cpsv: Tima0Cpsv,
    tima0_cttrigctl: Tima0Cttrigctl,
    _reserved6: [u8; 0x04],
    tima0_cttrig: Tima0Cttrig,
    tima0_fsctl: Tima0Fsctl,
    tima0_gctl: Tima0Gctl,
}
impl Tima0Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn tima0_ccpd(&self) -> &Tima0Ccpd {
        &self.tima0_ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn tima0_odis(&self) -> &Tima0Odis {
        &self.tima0_odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn tima0_cclkctl(&self) -> &Tima0Cclkctl {
        &self.tima0_cclkctl
    }
    #[doc = "0x0c - Clock Prescale Register"]
    #[inline(always)]
    pub const fn tima0_cps(&self) -> &Tima0Cps {
        &self.tima0_cps
    }
    #[doc = "0x10 - Clock prescale count status register"]
    #[inline(always)]
    pub const fn tima0_cpsv(&self) -> &Tima0Cpsv {
        &self.tima0_cpsv
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn tima0_cttrigctl(&self) -> &Tima0Cttrigctl {
        &self.tima0_cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn tima0_cttrig(&self) -> &Tima0Cttrig {
        &self.tima0_cttrig
    }
    #[doc = "0x20 - Fault Source Control"]
    #[inline(always)]
    pub const fn tima0_fsctl(&self) -> &Tima0Fsctl {
        &self.tima0_fsctl
    }
    #[doc = "0x24 - Shadow to active load mask"]
    #[inline(always)]
    pub const fn tima0_gctl(&self) -> &Tima0Gctl {
        &self.tima0_gctl
    }
}
#[doc = "TIMA0_CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ccpd`] module"]
#[doc(alias = "TIMA0_CCPD")]
pub type Tima0Ccpd = crate::Reg<tima0_ccpd::Tima0CcpdSpec>;
#[doc = "CCP Direction"]
pub mod tima0_ccpd;
#[doc = "TIMA0_ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_odis`] module"]
#[doc(alias = "TIMA0_ODIS")]
pub type Tima0Odis = crate::Reg<tima0_odis::Tima0OdisSpec>;
#[doc = "Output Disable"]
pub mod tima0_odis;
#[doc = "TIMA0_CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cclkctl`] module"]
#[doc(alias = "TIMA0_CCLKCTL")]
pub type Tima0Cclkctl = crate::Reg<tima0_cclkctl::Tima0CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod tima0_cclkctl;
#[doc = "TIMA0_CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cps`] module"]
#[doc(alias = "TIMA0_CPS")]
pub type Tima0Cps = crate::Reg<tima0_cps::Tima0CpsSpec>;
#[doc = "Clock Prescale Register"]
pub mod tima0_cps;
#[doc = "TIMA0_CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cpsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cpsv`] module"]
#[doc(alias = "TIMA0_CPSV")]
pub type Tima0Cpsv = crate::Reg<tima0_cpsv::Tima0CpsvSpec>;
#[doc = "Clock prescale count status register"]
pub mod tima0_cpsv;
#[doc = "TIMA0_CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cttrigctl`] module"]
#[doc(alias = "TIMA0_CTTRIGCTL")]
pub type Tima0Cttrigctl = crate::Reg<tima0_cttrigctl::Tima0CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod tima0_cttrigctl;
#[doc = "TIMA0_CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cttrig`] module"]
#[doc(alias = "TIMA0_CTTRIG")]
pub type Tima0Cttrig = crate::Reg<tima0_cttrig::Tima0CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod tima0_cttrig;
#[doc = "TIMA0_FSCTL (rw) register accessor: Fault Source Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fsctl`] module"]
#[doc(alias = "TIMA0_FSCTL")]
pub type Tima0Fsctl = crate::Reg<tima0_fsctl::Tima0FsctlSpec>;
#[doc = "Fault Source Control"]
pub mod tima0_fsctl;
#[doc = "TIMA0_GCTL (rw) register accessor: Shadow to active load mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_gctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_gctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_gctl`] module"]
#[doc(alias = "TIMA0_GCTL")]
pub type Tima0Gctl = crate::Reg<tima0_gctl::Tima0GctlSpec>;
#[doc = "Shadow to active load mask"]
pub mod tima0_gctl;
