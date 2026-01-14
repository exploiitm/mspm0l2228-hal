#[repr(C)]
#[doc = "UART1_GPRCM\\[%s\\]"]
#[doc(alias = "UART1_GPRCM")]
pub struct Uart1Gprcm {
    uart1_pwren: Uart1Pwren,
    uart1_rstctl: Uart1Rstctl,
    uart1_clkcfg: Uart1Clkcfg,
    _reserved3: [u8; 0x08],
    uart1_gprcm_stat: Uart1GprcmStat,
}
impl Uart1Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn uart1_pwren(&self) -> &Uart1Pwren {
        &self.uart1_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn uart1_rstctl(&self) -> &Uart1Rstctl {
        &self.uart1_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn uart1_clkcfg(&self) -> &Uart1Clkcfg {
        &self.uart1_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn uart1_gprcm_stat(&self) -> &Uart1GprcmStat {
        &self.uart1_gprcm_stat
    }
}
#[doc = "UART1_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_pwren`] module"]
#[doc(alias = "UART1_PWREN")]
pub type Uart1Pwren = crate::Reg<uart1_pwren::Uart1PwrenSpec>;
#[doc = "Power enable"]
pub mod uart1_pwren;
#[doc = "UART1_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_rstctl`] module"]
#[doc(alias = "UART1_RSTCTL")]
pub type Uart1Rstctl = crate::Reg<uart1_rstctl::Uart1RstctlSpec>;
#[doc = "Reset Control"]
pub mod uart1_rstctl;
#[doc = "UART1_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_clkcfg`] module"]
#[doc(alias = "UART1_CLKCFG")]
pub type Uart1Clkcfg = crate::Reg<uart1_clkcfg::Uart1ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod uart1_clkcfg;
#[doc = "UART1_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_gprcm_stat`] module"]
#[doc(alias = "UART1_GPRCM_STAT")]
pub type Uart1GprcmStat = crate::Reg<uart1_gprcm_stat::Uart1GprcmStatSpec>;
#[doc = "Status Register"]
pub mod uart1_gprcm_stat;
