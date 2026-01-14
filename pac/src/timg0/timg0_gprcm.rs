#[repr(C)]
#[doc = "TIMG0_GPRCM\\[%s\\]"]
#[doc(alias = "TIMG0_GPRCM")]
pub struct Timg0Gprcm {
    timg0_pwren: Timg0Pwren,
    timg0_rstctl: Timg0Rstctl,
    _reserved2: [u8; 0x0c],
    timg0_stat: Timg0Stat,
}
impl Timg0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn timg0_pwren(&self) -> &Timg0Pwren {
        &self.timg0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn timg0_rstctl(&self) -> &Timg0Rstctl {
        &self.timg0_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn timg0_stat(&self) -> &Timg0Stat {
        &self.timg0_stat
    }
}
#[doc = "TIMG0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_pwren`] module"]
#[doc(alias = "TIMG0_PWREN")]
pub type Timg0Pwren = crate::Reg<timg0_pwren::Timg0PwrenSpec>;
#[doc = "Power enable"]
pub mod timg0_pwren;
#[doc = "TIMG0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_rstctl`] module"]
#[doc(alias = "TIMG0_RSTCTL")]
pub type Timg0Rstctl = crate::Reg<timg0_rstctl::Timg0RstctlSpec>;
#[doc = "Reset Control"]
pub mod timg0_rstctl;
#[doc = "TIMG0_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_stat`] module"]
#[doc(alias = "TIMG0_STAT")]
pub type Timg0Stat = crate::Reg<timg0_stat::Timg0StatSpec>;
#[doc = "Status Register"]
pub mod timg0_stat;
