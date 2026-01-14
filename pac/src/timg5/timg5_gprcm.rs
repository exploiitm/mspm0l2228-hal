#[repr(C)]
#[doc = "TIMG5_GPRCM\\[%s\\]"]
#[doc(alias = "TIMG5_GPRCM")]
pub struct Timg5Gprcm {
    timg5_pwren: Timg5Pwren,
    timg5_rstctl: Timg5Rstctl,
    _reserved2: [u8; 0x0c],
    timg5_stat: Timg5Stat,
}
impl Timg5Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn timg5_pwren(&self) -> &Timg5Pwren {
        &self.timg5_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn timg5_rstctl(&self) -> &Timg5Rstctl {
        &self.timg5_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn timg5_stat(&self) -> &Timg5Stat {
        &self.timg5_stat
    }
}
#[doc = "TIMG5_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_pwren`] module"]
#[doc(alias = "TIMG5_PWREN")]
pub type Timg5Pwren = crate::Reg<timg5_pwren::Timg5PwrenSpec>;
#[doc = "Power enable"]
pub mod timg5_pwren;
#[doc = "TIMG5_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_rstctl`] module"]
#[doc(alias = "TIMG5_RSTCTL")]
pub type Timg5Rstctl = crate::Reg<timg5_rstctl::Timg5RstctlSpec>;
#[doc = "Reset Control"]
pub mod timg5_rstctl;
#[doc = "TIMG5_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_stat`] module"]
#[doc(alias = "TIMG5_STAT")]
pub type Timg5Stat = crate::Reg<timg5_stat::Timg5StatSpec>;
#[doc = "Status Register"]
pub mod timg5_stat;
