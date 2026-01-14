#[repr(C)]
#[doc = "AESADV_GPRCM\\[%s\\]"]
#[doc(alias = "AESADV_GPRCM")]
pub struct AesadvGprcm {
    aesadv_pwren: AesadvPwren,
    aesadv_rstctl: AesadvRstctl,
    _reserved2: [u8; 0x0c],
    aesadv_stat: AesadvStat,
}
impl AesadvGprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn aesadv_pwren(&self) -> &AesadvPwren {
        &self.aesadv_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn aesadv_rstctl(&self) -> &AesadvRstctl {
        &self.aesadv_rstctl
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn aesadv_stat(&self) -> &AesadvStat {
        &self.aesadv_stat
    }
}
#[doc = "AESADV_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_pwren`] module"]
#[doc(alias = "AESADV_PWREN")]
pub type AesadvPwren = crate::Reg<aesadv_pwren::AesadvPwrenSpec>;
#[doc = "Power enable"]
pub mod aesadv_pwren;
#[doc = "AESADV_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_rstctl`] module"]
#[doc(alias = "AESADV_RSTCTL")]
pub type AesadvRstctl = crate::Reg<aesadv_rstctl::AesadvRstctlSpec>;
#[doc = "Reset Control"]
pub mod aesadv_rstctl;
#[doc = "AESADV_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_stat`] module"]
#[doc(alias = "AESADV_STAT")]
pub type AesadvStat = crate::Reg<aesadv_stat::AesadvStatSpec>;
#[doc = "Status Register"]
pub mod aesadv_stat;
