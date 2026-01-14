#[repr(C)]
#[doc = "TIMG8_COMMONREGS\\[%s\\]"]
#[doc(alias = "TIMG8_COMMONREGS")]
pub struct Timg8Commonregs {
    timg8_ccpd: Timg8Ccpd,
    timg8_odis: Timg8Odis,
    timg8_cclkctl: Timg8Cclkctl,
    timg8_cps: Timg8Cps,
    timg8_cpsv: Timg8Cpsv,
    timg8_cttrigctl: Timg8Cttrigctl,
    _reserved6: [u8; 0x04],
    timg8_cttrig: Timg8Cttrig,
}
impl Timg8Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn timg8_ccpd(&self) -> &Timg8Ccpd {
        &self.timg8_ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn timg8_odis(&self) -> &Timg8Odis {
        &self.timg8_odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn timg8_cclkctl(&self) -> &Timg8Cclkctl {
        &self.timg8_cclkctl
    }
    #[doc = "0x0c - Clock Prescale Register"]
    #[inline(always)]
    pub const fn timg8_cps(&self) -> &Timg8Cps {
        &self.timg8_cps
    }
    #[doc = "0x10 - Clock prescale count status register"]
    #[inline(always)]
    pub const fn timg8_cpsv(&self) -> &Timg8Cpsv {
        &self.timg8_cpsv
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn timg8_cttrigctl(&self) -> &Timg8Cttrigctl {
        &self.timg8_cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn timg8_cttrig(&self) -> &Timg8Cttrig {
        &self.timg8_cttrig
    }
}
#[doc = "TIMG8_CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ccpd`] module"]
#[doc(alias = "TIMG8_CCPD")]
pub type Timg8Ccpd = crate::Reg<timg8_ccpd::Timg8CcpdSpec>;
#[doc = "CCP Direction"]
pub mod timg8_ccpd;
#[doc = "TIMG8_ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_odis`] module"]
#[doc(alias = "TIMG8_ODIS")]
pub type Timg8Odis = crate::Reg<timg8_odis::Timg8OdisSpec>;
#[doc = "Output Disable"]
pub mod timg8_odis;
#[doc = "TIMG8_CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_cclkctl`] module"]
#[doc(alias = "TIMG8_CCLKCTL")]
pub type Timg8Cclkctl = crate::Reg<timg8_cclkctl::Timg8CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod timg8_cclkctl;
#[doc = "TIMG8_CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_cps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_cps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_cps`] module"]
#[doc(alias = "TIMG8_CPS")]
pub type Timg8Cps = crate::Reg<timg8_cps::Timg8CpsSpec>;
#[doc = "Clock Prescale Register"]
pub mod timg8_cps;
#[doc = "TIMG8_CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_cpsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_cpsv`] module"]
#[doc(alias = "TIMG8_CPSV")]
pub type Timg8Cpsv = crate::Reg<timg8_cpsv::Timg8CpsvSpec>;
#[doc = "Clock prescale count status register"]
pub mod timg8_cpsv;
#[doc = "TIMG8_CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_cttrigctl`] module"]
#[doc(alias = "TIMG8_CTTRIGCTL")]
pub type Timg8Cttrigctl = crate::Reg<timg8_cttrigctl::Timg8CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod timg8_cttrigctl;
#[doc = "TIMG8_CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_cttrig`] module"]
#[doc(alias = "TIMG8_CTTRIG")]
pub type Timg8Cttrig = crate::Reg<timg8_cttrig::Timg8CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod timg8_cttrig;
