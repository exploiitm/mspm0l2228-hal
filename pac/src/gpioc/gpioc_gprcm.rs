#[repr(C)]
#[doc = "GPIOC_GPRCM\\[%s\\]"]
#[doc(alias = "GPIOC_GPRCM")]
pub struct GpiocGprcm {
    gpioc_pwren: GpiocPwren,
    gpioc_rstctl: GpiocRstctl,
    _reserved2: [u8; 0x0c],
    gpioc_stat: GpiocStat,
}
impl GpiocGprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn gpioc_pwren(&self) -> &GpiocPwren {
        &self.gpioc_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn gpioc_rstctl(&self) -> &GpiocRstctl {
        &self.gpioc_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn gpioc_stat(&self) -> &GpiocStat {
        &self.gpioc_stat
    }
}
#[doc = "GPIOC_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_pwren`] module"]
#[doc(alias = "GPIOC_PWREN")]
pub type GpiocPwren = crate::Reg<gpioc_pwren::GpiocPwrenSpec>;
#[doc = "Power enable"]
pub mod gpioc_pwren;
#[doc = "GPIOC_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_rstctl`] module"]
#[doc(alias = "GPIOC_RSTCTL")]
pub type GpiocRstctl = crate::Reg<gpioc_rstctl::GpiocRstctlSpec>;
#[doc = "Reset Control"]
pub mod gpioc_rstctl;
#[doc = "GPIOC_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_stat`] module"]
#[doc(alias = "GPIOC_STAT")]
pub type GpiocStat = crate::Reg<gpioc_stat::GpiocStatSpec>;
#[doc = "Status Register"]
pub mod gpioc_stat;
