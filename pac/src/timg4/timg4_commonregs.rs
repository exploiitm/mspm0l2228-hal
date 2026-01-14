#[repr(C)]
#[doc = "TIMG4_COMMONREGS\\[%s\\]"]
#[doc(alias = "TIMG4_COMMONREGS")]
pub struct Timg4Commonregs {
    timg4_ccpd: Timg4Ccpd,
    timg4_odis: Timg4Odis,
    timg4_cclkctl: Timg4Cclkctl,
    timg4_cps: Timg4Cps,
    timg4_cpsv: Timg4Cpsv,
    timg4_cttrigctl: Timg4Cttrigctl,
    _reserved6: [u8; 0x04],
    timg4_cttrig: Timg4Cttrig,
    _reserved7: [u8; 0x04],
    timg4_gctl: Timg4Gctl,
}
impl Timg4Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn timg4_ccpd(&self) -> &Timg4Ccpd {
        &self.timg4_ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn timg4_odis(&self) -> &Timg4Odis {
        &self.timg4_odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn timg4_cclkctl(&self) -> &Timg4Cclkctl {
        &self.timg4_cclkctl
    }
    #[doc = "0x0c - Clock Prescale Register"]
    #[inline(always)]
    pub const fn timg4_cps(&self) -> &Timg4Cps {
        &self.timg4_cps
    }
    #[doc = "0x10 - Clock prescale count status register"]
    #[inline(always)]
    pub const fn timg4_cpsv(&self) -> &Timg4Cpsv {
        &self.timg4_cpsv
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn timg4_cttrigctl(&self) -> &Timg4Cttrigctl {
        &self.timg4_cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn timg4_cttrig(&self) -> &Timg4Cttrig {
        &self.timg4_cttrig
    }
    #[doc = "0x24 - Shadow to active load mask"]
    #[inline(always)]
    pub const fn timg4_gctl(&self) -> &Timg4Gctl {
        &self.timg4_gctl
    }
}
#[doc = "TIMG4_CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ccpd`] module"]
#[doc(alias = "TIMG4_CCPD")]
pub type Timg4Ccpd = crate::Reg<timg4_ccpd::Timg4CcpdSpec>;
#[doc = "CCP Direction"]
pub mod timg4_ccpd;
#[doc = "TIMG4_ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_odis`] module"]
#[doc(alias = "TIMG4_ODIS")]
pub type Timg4Odis = crate::Reg<timg4_odis::Timg4OdisSpec>;
#[doc = "Output Disable"]
pub mod timg4_odis;
#[doc = "TIMG4_CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_cclkctl`] module"]
#[doc(alias = "TIMG4_CCLKCTL")]
pub type Timg4Cclkctl = crate::Reg<timg4_cclkctl::Timg4CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod timg4_cclkctl;
#[doc = "TIMG4_CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_cps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_cps`] module"]
#[doc(alias = "TIMG4_CPS")]
pub type Timg4Cps = crate::Reg<timg4_cps::Timg4CpsSpec>;
#[doc = "Clock Prescale Register"]
pub mod timg4_cps;
#[doc = "TIMG4_CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_cpsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_cpsv`] module"]
#[doc(alias = "TIMG4_CPSV")]
pub type Timg4Cpsv = crate::Reg<timg4_cpsv::Timg4CpsvSpec>;
#[doc = "Clock prescale count status register"]
pub mod timg4_cpsv;
#[doc = "TIMG4_CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_cttrigctl`] module"]
#[doc(alias = "TIMG4_CTTRIGCTL")]
pub type Timg4Cttrigctl = crate::Reg<timg4_cttrigctl::Timg4CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod timg4_cttrigctl;
#[doc = "TIMG4_CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_cttrig`] module"]
#[doc(alias = "TIMG4_CTTRIG")]
pub type Timg4Cttrig = crate::Reg<timg4_cttrig::Timg4CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod timg4_cttrig;
#[doc = "TIMG4_GCTL (rw) register accessor: Shadow to active load mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_gctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_gctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_gctl`] module"]
#[doc(alias = "TIMG4_GCTL")]
pub type Timg4Gctl = crate::Reg<timg4_gctl::Timg4GctlSpec>;
#[doc = "Shadow to active load mask"]
pub mod timg4_gctl;
