#[repr(C)]
#[doc = "TIMA0_GPRCM\\[%s\\]"]
#[doc(alias = "TIMA0_GPRCM")]
pub struct Tima0Gprcm {
    tima0_pwren: Tima0Pwren,
    tima0_rstctl: Tima0Rstctl,
    _reserved2: [u8; 0x0c],
    tima0_stat: Tima0Stat,
}
impl Tima0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn tima0_pwren(&self) -> &Tima0Pwren {
        &self.tima0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn tima0_rstctl(&self) -> &Tima0Rstctl {
        &self.tima0_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn tima0_stat(&self) -> &Tima0Stat {
        &self.tima0_stat
    }
}
#[doc = "TIMA0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_pwren`] module"]
#[doc(alias = "TIMA0_PWREN")]
pub type Tima0Pwren = crate::Reg<tima0_pwren::Tima0PwrenSpec>;
#[doc = "Power enable"]
pub mod tima0_pwren;
#[doc = "TIMA0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_rstctl`] module"]
#[doc(alias = "TIMA0_RSTCTL")]
pub type Tima0Rstctl = crate::Reg<tima0_rstctl::Tima0RstctlSpec>;
#[doc = "Reset Control"]
pub mod tima0_rstctl;
#[doc = "TIMA0_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_stat`] module"]
#[doc(alias = "TIMA0_STAT")]
pub type Tima0Stat = crate::Reg<tima0_stat::Tima0StatSpec>;
#[doc = "Status Register"]
pub mod tima0_stat;
