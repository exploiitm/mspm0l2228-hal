#[repr(C)]
#[doc = "GPIOB_GPRCM\\[%s\\]"]
#[doc(alias = "GPIOB_GPRCM")]
pub struct GpiobGprcm {
    gpiob_pwren: GpiobPwren,
    gpiob_rstctl: GpiobRstctl,
    _reserved2: [u8; 0x0c],
    gpiob_stat: GpiobStat,
}
impl GpiobGprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn gpiob_pwren(&self) -> &GpiobPwren {
        &self.gpiob_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn gpiob_rstctl(&self) -> &GpiobRstctl {
        &self.gpiob_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn gpiob_stat(&self) -> &GpiobStat {
        &self.gpiob_stat
    }
}
#[doc = "GPIOB_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_pwren`] module"]
#[doc(alias = "GPIOB_PWREN")]
pub type GpiobPwren = crate::Reg<gpiob_pwren::GpiobPwrenSpec>;
#[doc = "Power enable"]
pub mod gpiob_pwren;
#[doc = "GPIOB_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_rstctl`] module"]
#[doc(alias = "GPIOB_RSTCTL")]
pub type GpiobRstctl = crate::Reg<gpiob_rstctl::GpiobRstctlSpec>;
#[doc = "Reset Control"]
pub mod gpiob_rstctl;
#[doc = "GPIOB_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_stat`] module"]
#[doc(alias = "GPIOB_STAT")]
pub type GpiobStat = crate::Reg<gpiob_stat::GpiobStatSpec>;
#[doc = "Status Register"]
pub mod gpiob_stat;
