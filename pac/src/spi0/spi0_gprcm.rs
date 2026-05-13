#[repr(C)]
#[doc = "SPI0_GPRCM\\[%s\\]"]
#[doc(alias = "SPI0_GPRCM")]
pub struct Spi0Gprcm {
    spi0_pwren: Spi0Pwren,
    spi0_rstctl: Spi0Rstctl,
    spi0_clkcfg: Spi0Clkcfg,
    _reserved3: [u8; 0x08],
    spi0_gprcm_stat: Spi0GprcmStat,
}
impl Spi0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn spi0_pwren(&self) -> &Spi0Pwren {
        &self.spi0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn spi0_rstctl(&self) -> &Spi0Rstctl {
        &self.spi0_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn spi0_clkcfg(&self) -> &Spi0Clkcfg {
        &self.spi0_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn spi0_gprcm_stat(&self) -> &Spi0GprcmStat {
        &self.spi0_gprcm_stat
    }
}
#[doc = "SPI0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_pwren`] module"]
#[doc(alias = "SPI0_PWREN")]
pub type Spi0Pwren = crate::Reg<spi0_pwren::Spi0PwrenSpec>;
#[doc = "Power enable"]
pub mod spi0_pwren;
#[doc = "SPI0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_rstctl`] module"]
#[doc(alias = "SPI0_RSTCTL")]
pub type Spi0Rstctl = crate::Reg<spi0_rstctl::Spi0RstctlSpec>;
#[doc = "Reset Control"]
pub mod spi0_rstctl;
#[doc = "SPI0_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_clkcfg`] module"]
#[doc(alias = "SPI0_CLKCFG")]
pub type Spi0Clkcfg = crate::Reg<spi0_clkcfg::Spi0ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod spi0_clkcfg;
#[doc = "SPI0_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_gprcm_stat`] module"]
#[doc(alias = "SPI0_GPRCM_STAT")]
pub type Spi0GprcmStat = crate::Reg<spi0_gprcm_stat::Spi0GprcmStatSpec>;
#[doc = "Status Register"]
pub mod spi0_gprcm_stat;
