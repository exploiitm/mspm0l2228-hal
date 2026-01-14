#[repr(C)]
#[doc = "COMP0_GPRCM\\[%s\\]"]
#[doc(alias = "COMP0_GPRCM")]
pub struct Comp0Gprcm {
    comp0_pwren: Comp0Pwren,
    comp0_rstctl: Comp0Rstctl,
    comp0_clkcfg: Comp0Clkcfg,
    _reserved3: [u8; 0x08],
    comp0_gprcm_stat: Comp0GprcmStat,
}
impl Comp0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn comp0_pwren(&self) -> &Comp0Pwren {
        &self.comp0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn comp0_rstctl(&self) -> &Comp0Rstctl {
        &self.comp0_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn comp0_clkcfg(&self) -> &Comp0Clkcfg {
        &self.comp0_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn comp0_gprcm_stat(&self) -> &Comp0GprcmStat {
        &self.comp0_gprcm_stat
    }
}
#[doc = "COMP0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_pwren`] module"]
#[doc(alias = "COMP0_PWREN")]
pub type Comp0Pwren = crate::Reg<comp0_pwren::Comp0PwrenSpec>;
#[doc = "Power enable"]
pub mod comp0_pwren;
#[doc = "COMP0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_rstctl`] module"]
#[doc(alias = "COMP0_RSTCTL")]
pub type Comp0Rstctl = crate::Reg<comp0_rstctl::Comp0RstctlSpec>;
#[doc = "Reset Control"]
pub mod comp0_rstctl;
#[doc = "COMP0_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_clkcfg`] module"]
#[doc(alias = "COMP0_CLKCFG")]
pub type Comp0Clkcfg = crate::Reg<comp0_clkcfg::Comp0ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod comp0_clkcfg;
#[doc = "COMP0_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_gprcm_stat`] module"]
#[doc(alias = "COMP0_GPRCM_STAT")]
pub type Comp0GprcmStat = crate::Reg<comp0_gprcm_stat::Comp0GprcmStatSpec>;
#[doc = "Status Register"]
pub mod comp0_gprcm_stat;
