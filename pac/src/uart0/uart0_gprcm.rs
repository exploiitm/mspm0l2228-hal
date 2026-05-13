#[repr(C)]
#[doc = "UART0_GPRCM\\[%s\\]"]
#[doc(alias = "UART0_GPRCM")]
pub struct Uart0Gprcm {
    uart0_pwren: Uart0Pwren,
    uart0_rstctl: Uart0Rstctl,
    uart0_clkcfg: Uart0Clkcfg,
    _reserved3: [u8; 0x08],
    uart0_gprcm_stat: Uart0GprcmStat,
}
impl Uart0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn uart0_pwren(&self) -> &Uart0Pwren {
        &self.uart0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn uart0_rstctl(&self) -> &Uart0Rstctl {
        &self.uart0_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn uart0_clkcfg(&self) -> &Uart0Clkcfg {
        &self.uart0_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn uart0_gprcm_stat(&self) -> &Uart0GprcmStat {
        &self.uart0_gprcm_stat
    }
}
#[doc = "UART0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_pwren`] module"]
#[doc(alias = "UART0_PWREN")]
pub type Uart0Pwren = crate::Reg<uart0_pwren::Uart0PwrenSpec>;
#[doc = "Power enable"]
pub mod uart0_pwren;
#[doc = "UART0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_rstctl`] module"]
#[doc(alias = "UART0_RSTCTL")]
pub type Uart0Rstctl = crate::Reg<uart0_rstctl::Uart0RstctlSpec>;
#[doc = "Reset Control"]
pub mod uart0_rstctl;
#[doc = "UART0_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_clkcfg`] module"]
#[doc(alias = "UART0_CLKCFG")]
pub type Uart0Clkcfg = crate::Reg<uart0_clkcfg::Uart0ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod uart0_clkcfg;
#[doc = "UART0_GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_gprcm_stat`] module"]
#[doc(alias = "UART0_GPRCM_STAT")]
pub type Uart0GprcmStat = crate::Reg<uart0_gprcm_stat::Uart0GprcmStatSpec>;
#[doc = "Status Register"]
pub mod uart0_gprcm_stat;
