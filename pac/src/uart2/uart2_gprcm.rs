#[repr(C)]
#[doc = "UART2_GPRCM\\[%s\\]"]
#[doc(alias = "UART2_GPRCM")]
pub struct Uart2Gprcm {
    uart2_pwren: Uart2Pwren,
    uart2_rstctl: Uart2Rstctl,
    uart2_clkcfg: Uart2Clkcfg,
    _reserved3: [u8; 0x08],
    uart2_gprcm_stat: Uart2GprcmStat,
}
impl Uart2Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn uart2_pwren(&self) -> &Uart2Pwren {
        &self.uart2_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn uart2_rstctl(&self) -> &Uart2Rstctl {
        &self.uart2_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn uart2_clkcfg(&self) -> &Uart2Clkcfg {
        &self.uart2_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn uart2_gprcm_stat(&self) -> &Uart2GprcmStat {
        &self.uart2_gprcm_stat
    }
}
#[doc = "UART2_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_pwren`] module"]
#[doc(alias = "UART2_PWREN")]
pub type Uart2Pwren = crate::Reg<uart2_pwren::Uart2PwrenSpec>;
#[doc = "Power enable"]
pub mod uart2_pwren;
#[doc = "UART2_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_rstctl`] module"]
#[doc(alias = "UART2_RSTCTL")]
pub type Uart2Rstctl = crate::Reg<uart2_rstctl::Uart2RstctlSpec>;
#[doc = "Reset Control"]
pub mod uart2_rstctl;
#[doc = "UART2_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_clkcfg`] module"]
#[doc(alias = "UART2_CLKCFG")]
pub type Uart2Clkcfg = crate::Reg<uart2_clkcfg::Uart2ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod uart2_clkcfg;
#[doc = "UART2_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_gprcm_stat`] module"]
#[doc(alias = "UART2_GPRCM_STAT")]
pub type Uart2GprcmStat = crate::Reg<uart2_gprcm_stat::Uart2GprcmStatSpec>;
#[doc = "Status Register"]
pub mod uart2_gprcm_stat;
