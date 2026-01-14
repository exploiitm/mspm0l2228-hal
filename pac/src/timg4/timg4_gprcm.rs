#[repr(C)]
#[doc = "TIMG4_GPRCM\\[%s\\]"]
#[doc(alias = "TIMG4_GPRCM")]
pub struct Timg4Gprcm {
    timg4_pwren: Timg4Pwren,
    timg4_rstctl: Timg4Rstctl,
    _reserved2: [u8; 0x0c],
    timg4_stat: Timg4Stat,
}
impl Timg4Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn timg4_pwren(&self) -> &Timg4Pwren {
        &self.timg4_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn timg4_rstctl(&self) -> &Timg4Rstctl {
        &self.timg4_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn timg4_stat(&self) -> &Timg4Stat {
        &self.timg4_stat
    }
}
#[doc = "TIMG4_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_pwren`] module"]
#[doc(alias = "TIMG4_PWREN")]
pub type Timg4Pwren = crate::Reg<timg4_pwren::Timg4PwrenSpec>;
#[doc = "Power enable"]
pub mod timg4_pwren;
#[doc = "TIMG4_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_rstctl`] module"]
#[doc(alias = "TIMG4_RSTCTL")]
pub type Timg4Rstctl = crate::Reg<timg4_rstctl::Timg4RstctlSpec>;
#[doc = "Reset Control"]
pub mod timg4_rstctl;
#[doc = "TIMG4_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_stat`] module"]
#[doc(alias = "TIMG4_STAT")]
pub type Timg4Stat = crate::Reg<timg4_stat::Timg4StatSpec>;
#[doc = "Status Register"]
pub mod timg4_stat;
