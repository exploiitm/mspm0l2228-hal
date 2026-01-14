#[repr(C)]
#[doc = "UART3_GPRCM\\[%s\\]"]
#[doc(alias = "UART3_GPRCM")]
pub struct Uart3Gprcm {
    uart3_pwren: Uart3Pwren,
    uart3_rstctl: Uart3Rstctl,
    uart3_clkcfg: Uart3Clkcfg,
    _reserved3: [u8; 0x08],
    uart3_gprcm_stat: Uart3GprcmStat,
}
impl Uart3Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn uart3_pwren(&self) -> &Uart3Pwren {
        &self.uart3_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn uart3_rstctl(&self) -> &Uart3Rstctl {
        &self.uart3_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn uart3_clkcfg(&self) -> &Uart3Clkcfg {
        &self.uart3_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn uart3_gprcm_stat(&self) -> &Uart3GprcmStat {
        &self.uart3_gprcm_stat
    }
}
#[doc = "UART3_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_pwren`] module"]
#[doc(alias = "UART3_PWREN")]
pub type Uart3Pwren = crate::Reg<uart3_pwren::Uart3PwrenSpec>;
#[doc = "Power enable"]
pub mod uart3_pwren;
#[doc = "UART3_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_rstctl`] module"]
#[doc(alias = "UART3_RSTCTL")]
pub type Uart3Rstctl = crate::Reg<uart3_rstctl::Uart3RstctlSpec>;
#[doc = "Reset Control"]
pub mod uart3_rstctl;
#[doc = "UART3_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_clkcfg`] module"]
#[doc(alias = "UART3_CLKCFG")]
pub type Uart3Clkcfg = crate::Reg<uart3_clkcfg::Uart3ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod uart3_clkcfg;
#[doc = "UART3_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_gprcm_stat`] module"]
#[doc(alias = "UART3_GPRCM_STAT")]
pub type Uart3GprcmStat = crate::Reg<uart3_gprcm_stat::Uart3GprcmStatSpec>;
#[doc = "Status Register"]
pub mod uart3_gprcm_stat;
