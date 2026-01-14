#[repr(C)]
#[doc = "VREF_GPRCM\\[%s\\]"]
#[doc(alias = "VREF_GPRCM")]
pub struct VrefGprcm {
    vref_pwren: VrefPwren,
    vref_rstctl: VrefRstctl,
    _reserved2: [u8; 0x0c],
    vref_stat: VrefStat,
}
impl VrefGprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn vref_pwren(&self) -> &VrefPwren {
        &self.vref_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn vref_rstctl(&self) -> &VrefRstctl {
        &self.vref_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn vref_stat(&self) -> &VrefStat {
        &self.vref_stat
    }
}
#[doc = "VREF_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_pwren`] module"]
#[doc(alias = "VREF_PWREN")]
pub type VrefPwren = crate::Reg<vref_pwren::VrefPwrenSpec>;
#[doc = "Power enable"]
pub mod vref_pwren;
#[doc = "VREF_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_rstctl`] module"]
#[doc(alias = "VREF_RSTCTL")]
pub type VrefRstctl = crate::Reg<vref_rstctl::VrefRstctlSpec>;
#[doc = "Reset Control"]
pub mod vref_rstctl;
#[doc = "VREF_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_stat`] module"]
#[doc(alias = "VREF_STAT")]
pub type VrefStat = crate::Reg<vref_stat::VrefStatSpec>;
#[doc = "Status Register"]
pub mod vref_stat;
