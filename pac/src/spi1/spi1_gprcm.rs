#[repr(C)]
#[doc = "SPI1_GPRCM\\[%s\\]"]
#[doc(alias = "SPI1_GPRCM")]
pub struct Spi1Gprcm {
    spi1_pwren: Spi1Pwren,
    spi1_rstctl: Spi1Rstctl,
    spi1_clkcfg: Spi1Clkcfg,
    _reserved3: [u8; 0x08],
    spi1_gprcm_stat: Spi1GprcmStat,
}
impl Spi1Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn spi1_pwren(&self) -> &Spi1Pwren {
        &self.spi1_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn spi1_rstctl(&self) -> &Spi1Rstctl {
        &self.spi1_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn spi1_clkcfg(&self) -> &Spi1Clkcfg {
        &self.spi1_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn spi1_gprcm_stat(&self) -> &Spi1GprcmStat {
        &self.spi1_gprcm_stat
    }
}
#[doc = "SPI1_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_pwren`] module"]
#[doc(alias = "SPI1_PWREN")]
pub type Spi1Pwren = crate::Reg<spi1_pwren::Spi1PwrenSpec>;
#[doc = "Power enable"]
pub mod spi1_pwren;
#[doc = "SPI1_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_rstctl`] module"]
#[doc(alias = "SPI1_RSTCTL")]
pub type Spi1Rstctl = crate::Reg<spi1_rstctl::Spi1RstctlSpec>;
#[doc = "Reset Control"]
pub mod spi1_rstctl;
#[doc = "SPI1_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_clkcfg`] module"]
#[doc(alias = "SPI1_CLKCFG")]
pub type Spi1Clkcfg = crate::Reg<spi1_clkcfg::Spi1ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod spi1_clkcfg;
#[doc = "SPI1_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_gprcm_stat`] module"]
#[doc(alias = "SPI1_GPRCM_STAT")]
pub type Spi1GprcmStat = crate::Reg<spi1_gprcm_stat::Spi1GprcmStatSpec>;
#[doc = "Status Register"]
pub mod spi1_gprcm_stat;
