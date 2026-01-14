#[repr(C)]
#[doc = "TIMG12_GPRCM\\[%s\\]"]
#[doc(alias = "TIMG12_GPRCM")]
pub struct Timg12Gprcm {
    timg12_pwren: Timg12Pwren,
    timg12_rstctl: Timg12Rstctl,
    _reserved2: [u8; 0x0c],
    timg12_stat: Timg12Stat,
}
impl Timg12Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn timg12_pwren(&self) -> &Timg12Pwren {
        &self.timg12_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn timg12_rstctl(&self) -> &Timg12Rstctl {
        &self.timg12_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn timg12_stat(&self) -> &Timg12Stat {
        &self.timg12_stat
    }
}
#[doc = "TIMG12_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_pwren`] module"]
#[doc(alias = "TIMG12_PWREN")]
pub type Timg12Pwren = crate::Reg<timg12_pwren::Timg12PwrenSpec>;
#[doc = "Power enable"]
pub mod timg12_pwren;
#[doc = "TIMG12_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_rstctl`] module"]
#[doc(alias = "TIMG12_RSTCTL")]
pub type Timg12Rstctl = crate::Reg<timg12_rstctl::Timg12RstctlSpec>;
#[doc = "Reset Control"]
pub mod timg12_rstctl;
#[doc = "TIMG12_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_stat`] module"]
#[doc(alias = "TIMG12_STAT")]
pub type Timg12Stat = crate::Reg<timg12_stat::Timg12StatSpec>;
#[doc = "Status Register"]
pub mod timg12_stat;
