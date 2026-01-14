#[repr(C)]
#[doc = "GPIOA_GPRCM\\[%s\\]"]
#[doc(alias = "GPIOA_GPRCM")]
pub struct GpioaGprcm {
    gpioa_pwren: GpioaPwren,
    gpioa_rstctl: GpioaRstctl,
    _reserved2: [u8; 0x0c],
    gpioa_stat: GpioaStat,
}
impl GpioaGprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn gpioa_pwren(&self) -> &GpioaPwren {
        &self.gpioa_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn gpioa_rstctl(&self) -> &GpioaRstctl {
        &self.gpioa_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn gpioa_stat(&self) -> &GpioaStat {
        &self.gpioa_stat
    }
}
#[doc = "GPIOA_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_pwren`] module"]
#[doc(alias = "GPIOA_PWREN")]
pub type GpioaPwren = crate::Reg<gpioa_pwren::GpioaPwrenSpec>;
#[doc = "Power enable"]
pub mod gpioa_pwren;
#[doc = "GPIOA_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_rstctl`] module"]
#[doc(alias = "GPIOA_RSTCTL")]
pub type GpioaRstctl = crate::Reg<gpioa_rstctl::GpioaRstctlSpec>;
#[doc = "Reset Control"]
pub mod gpioa_rstctl;
#[doc = "GPIOA_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_stat`] module"]
#[doc(alias = "GPIOA_STAT")]
pub type GpioaStat = crate::Reg<gpioa_stat::GpioaStatSpec>;
#[doc = "Status Register"]
pub mod gpioa_stat;
