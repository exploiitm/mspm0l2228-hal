#[repr(C)]
#[doc = "CRCP0_GPRCM\\[%s\\]"]
#[doc(alias = "CRCP0_GPRCM")]
pub struct Crcp0Gprcm {
    crcp0_pwren: Crcp0Pwren,
    crcp0_rstctl: Crcp0Rstctl,
    _reserved2: [u8; 0x0c],
    crcp0_stat: Crcp0Stat,
}
impl Crcp0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn crcp0_pwren(&self) -> &Crcp0Pwren {
        &self.crcp0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn crcp0_rstctl(&self) -> &Crcp0Rstctl {
        &self.crcp0_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn crcp0_stat(&self) -> &Crcp0Stat {
        &self.crcp0_stat
    }
}
#[doc = "CRCP0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_pwren`] module"]
#[doc(alias = "CRCP0_PWREN")]
pub type Crcp0Pwren = crate::Reg<crcp0_pwren::Crcp0PwrenSpec>;
#[doc = "Power enable"]
pub mod crcp0_pwren;
#[doc = "CRCP0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_rstctl`] module"]
#[doc(alias = "CRCP0_RSTCTL")]
pub type Crcp0Rstctl = crate::Reg<crcp0_rstctl::Crcp0RstctlSpec>;
#[doc = "Reset Control"]
pub mod crcp0_rstctl;
#[doc = "CRCP0_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_stat`] module"]
#[doc(alias = "CRCP0_STAT")]
pub type Crcp0Stat = crate::Reg<crcp0_stat::Crcp0StatSpec>;
#[doc = "Status Register"]
pub mod crcp0_stat;
