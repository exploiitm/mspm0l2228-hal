#[repr(C)]
#[doc = "TIMG8_GPRCM\\[%s\\]"]
#[doc(alias = "TIMG8_GPRCM")]
pub struct Timg8Gprcm {
    timg8_pwren: Timg8Pwren,
    timg8_rstctl: Timg8Rstctl,
    _reserved2: [u8; 0x0c],
    timg8_stat: Timg8Stat,
}
impl Timg8Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn timg8_pwren(&self) -> &Timg8Pwren {
        &self.timg8_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn timg8_rstctl(&self) -> &Timg8Rstctl {
        &self.timg8_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn timg8_stat(&self) -> &Timg8Stat {
        &self.timg8_stat
    }
}
#[doc = "TIMG8_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_pwren`] module"]
#[doc(alias = "TIMG8_PWREN")]
pub type Timg8Pwren = crate::Reg<timg8_pwren::Timg8PwrenSpec>;
#[doc = "Power enable"]
pub mod timg8_pwren;
#[doc = "TIMG8_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_rstctl`] module"]
#[doc(alias = "TIMG8_RSTCTL")]
pub type Timg8Rstctl = crate::Reg<timg8_rstctl::Timg8RstctlSpec>;
#[doc = "Reset Control"]
pub mod timg8_rstctl;
#[doc = "TIMG8_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_stat`] module"]
#[doc(alias = "TIMG8_STAT")]
pub type Timg8Stat = crate::Reg<timg8_stat::Timg8StatSpec>;
#[doc = "Status Register"]
pub mod timg8_stat;
