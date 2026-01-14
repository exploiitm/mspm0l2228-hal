#[repr(C)]
#[doc = "TIMG0_COMMONREGS\\[%s\\]"]
#[doc(alias = "TIMG0_COMMONREGS")]
pub struct Timg0Commonregs {
    timg0_ccpd: Timg0Ccpd,
    timg0_odis: Timg0Odis,
    timg0_cclkctl: Timg0Cclkctl,
    timg0_cps: Timg0Cps,
    timg0_cpsv: Timg0Cpsv,
    timg0_cttrigctl: Timg0Cttrigctl,
    _reserved6: [u8; 0x04],
    timg0_cttrig: Timg0Cttrig,
}
impl Timg0Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn timg0_ccpd(&self) -> &Timg0Ccpd {
        &self.timg0_ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn timg0_odis(&self) -> &Timg0Odis {
        &self.timg0_odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn timg0_cclkctl(&self) -> &Timg0Cclkctl {
        &self.timg0_cclkctl
    }
    #[doc = "0x0c - Clock Prescale Register"]
    #[inline(always)]
    pub const fn timg0_cps(&self) -> &Timg0Cps {
        &self.timg0_cps
    }
    #[doc = "0x10 - Clock prescale count status register"]
    #[inline(always)]
    pub const fn timg0_cpsv(&self) -> &Timg0Cpsv {
        &self.timg0_cpsv
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn timg0_cttrigctl(&self) -> &Timg0Cttrigctl {
        &self.timg0_cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn timg0_cttrig(&self) -> &Timg0Cttrig {
        &self.timg0_cttrig
    }
}
#[doc = "TIMG0_CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ccpd`] module"]
#[doc(alias = "TIMG0_CCPD")]
pub type Timg0Ccpd = crate::Reg<timg0_ccpd::Timg0CcpdSpec>;
#[doc = "CCP Direction"]
pub mod timg0_ccpd;
#[doc = "TIMG0_ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_odis`] module"]
#[doc(alias = "TIMG0_ODIS")]
pub type Timg0Odis = crate::Reg<timg0_odis::Timg0OdisSpec>;
#[doc = "Output Disable"]
pub mod timg0_odis;
#[doc = "TIMG0_CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_cclkctl`] module"]
#[doc(alias = "TIMG0_CCLKCTL")]
pub type Timg0Cclkctl = crate::Reg<timg0_cclkctl::Timg0CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod timg0_cclkctl;
#[doc = "TIMG0_CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_cps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_cps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_cps`] module"]
#[doc(alias = "TIMG0_CPS")]
pub type Timg0Cps = crate::Reg<timg0_cps::Timg0CpsSpec>;
#[doc = "Clock Prescale Register"]
pub mod timg0_cps;
#[doc = "TIMG0_CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_cpsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_cpsv`] module"]
#[doc(alias = "TIMG0_CPSV")]
pub type Timg0Cpsv = crate::Reg<timg0_cpsv::Timg0CpsvSpec>;
#[doc = "Clock prescale count status register"]
pub mod timg0_cpsv;
#[doc = "TIMG0_CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_cttrigctl`] module"]
#[doc(alias = "TIMG0_CTTRIGCTL")]
pub type Timg0Cttrigctl = crate::Reg<timg0_cttrigctl::Timg0CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod timg0_cttrigctl;
#[doc = "TIMG0_CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_cttrig`] module"]
#[doc(alias = "TIMG0_CTTRIG")]
pub type Timg0Cttrig = crate::Reg<timg0_cttrig::Timg0CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod timg0_cttrig;
