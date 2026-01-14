#[repr(C)]
#[doc = "TRNG_GPRCM\\[%s\\]"]
#[doc(alias = "TRNG_GPRCM")]
pub struct TrngGprcm {
    trng_pwren: TrngPwren,
    trng_rstctl: TrngRstctl,
    _reserved2: [u8; 0x0c],
    trng_gprcm_stat: TrngGprcmStat,
}
impl TrngGprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn trng_pwren(&self) -> &TrngPwren {
        &self.trng_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn trng_rstctl(&self) -> &TrngRstctl {
        &self.trng_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn trng_gprcm_stat(&self) -> &TrngGprcmStat {
        &self.trng_gprcm_stat
    }
}
#[doc = "TRNG_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_pwren`] module"]
#[doc(alias = "TRNG_PWREN")]
pub type TrngPwren = crate::Reg<trng_pwren::TrngPwrenSpec>;
#[doc = "Power enable"]
pub mod trng_pwren;
#[doc = "TRNG_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_rstctl`] module"]
#[doc(alias = "TRNG_RSTCTL")]
pub type TrngRstctl = crate::Reg<trng_rstctl::TrngRstctlSpec>;
#[doc = "Reset Control"]
pub mod trng_rstctl;
#[doc = "TRNG_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_gprcm_stat`] module"]
#[doc(alias = "TRNG_GPRCM_STAT")]
pub type TrngGprcmStat = crate::Reg<trng_gprcm_stat::TrngGprcmStatSpec>;
#[doc = "Status Register"]
pub mod trng_gprcm_stat;
