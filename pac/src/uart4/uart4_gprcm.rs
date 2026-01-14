#[repr(C)]
#[doc = "UART4_GPRCM\\[%s\\]"]
#[doc(alias = "UART4_GPRCM")]
pub struct Uart4Gprcm {
    uart4_pwren: Uart4Pwren,
    uart4_rstctl: Uart4Rstctl,
    uart4_clkcfg: Uart4Clkcfg,
    _reserved3: [u8; 0x08],
    uart4_gprcm_stat: Uart4GprcmStat,
}
impl Uart4Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn uart4_pwren(&self) -> &Uart4Pwren {
        &self.uart4_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn uart4_rstctl(&self) -> &Uart4Rstctl {
        &self.uart4_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn uart4_clkcfg(&self) -> &Uart4Clkcfg {
        &self.uart4_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn uart4_gprcm_stat(&self) -> &Uart4GprcmStat {
        &self.uart4_gprcm_stat
    }
}
#[doc = "UART4_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_pwren`] module"]
#[doc(alias = "UART4_PWREN")]
pub type Uart4Pwren = crate::Reg<uart4_pwren::Uart4PwrenSpec>;
#[doc = "Power enable"]
pub mod uart4_pwren;
#[doc = "UART4_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_rstctl`] module"]
#[doc(alias = "UART4_RSTCTL")]
pub type Uart4Rstctl = crate::Reg<uart4_rstctl::Uart4RstctlSpec>;
#[doc = "Reset Control"]
pub mod uart4_rstctl;
#[doc = "UART4_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_clkcfg`] module"]
#[doc(alias = "UART4_CLKCFG")]
pub type Uart4Clkcfg = crate::Reg<uart4_clkcfg::Uart4ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod uart4_clkcfg;
#[doc = "UART4_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_gprcm_stat`] module"]
#[doc(alias = "UART4_GPRCM_STAT")]
pub type Uart4GprcmStat = crate::Reg<uart4_gprcm_stat::Uart4GprcmStatSpec>;
#[doc = "Status Register"]
pub mod uart4_gprcm_stat;
