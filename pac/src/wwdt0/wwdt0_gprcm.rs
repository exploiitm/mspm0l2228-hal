#[repr(C)]
#[doc = "WWDT0_GPRCM\\[%s\\]"]
#[doc(alias = "WWDT0_GPRCM")]
pub struct Wwdt0Gprcm {
    wwdt0_pwren: Wwdt0Pwren,
    wwdt0_rstctl: Wwdt0Rstctl,
    _reserved2: [u8; 0x0c],
    wwdt0_stat: Wwdt0Stat,
}
impl Wwdt0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn wwdt0_pwren(&self) -> &Wwdt0Pwren {
        &self.wwdt0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn wwdt0_rstctl(&self) -> &Wwdt0Rstctl {
        &self.wwdt0_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn wwdt0_stat(&self) -> &Wwdt0Stat {
        &self.wwdt0_stat
    }
}
#[doc = "WWDT0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_pwren`] module"]
#[doc(alias = "WWDT0_PWREN")]
pub type Wwdt0Pwren = crate::Reg<wwdt0_pwren::Wwdt0PwrenSpec>;
#[doc = "Power enable"]
pub mod wwdt0_pwren;
#[doc = "WWDT0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_rstctl`] module"]
#[doc(alias = "WWDT0_RSTCTL")]
pub type Wwdt0Rstctl = crate::Reg<wwdt0_rstctl::Wwdt0RstctlSpec>;
#[doc = "Reset Control"]
pub mod wwdt0_rstctl;
#[doc = "WWDT0_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_stat`] module"]
#[doc(alias = "WWDT0_STAT")]
pub type Wwdt0Stat = crate::Reg<wwdt0_stat::Wwdt0StatSpec>;
#[doc = "Status Register"]
pub mod wwdt0_stat;
