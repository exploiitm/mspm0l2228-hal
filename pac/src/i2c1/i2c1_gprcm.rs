#[repr(C)]
#[doc = "I2C1_GPRCM\\[%s\\]"]
#[doc(alias = "I2C1_GPRCM")]
pub struct I2c1Gprcm {
    i2c1_pwren: I2c1Pwren,
    i2c1_rstctl: I2c1Rstctl,
    i2c1_clkcfg: I2c1Clkcfg,
    _reserved3: [u8; 0x08],
    i2c1_stat: I2c1Stat,
}
impl I2c1Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn i2c1_pwren(&self) -> &I2c1Pwren {
        &self.i2c1_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn i2c1_rstctl(&self) -> &I2c1Rstctl {
        &self.i2c1_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn i2c1_clkcfg(&self) -> &I2c1Clkcfg {
        &self.i2c1_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn i2c1_stat(&self) -> &I2c1Stat {
        &self.i2c1_stat
    }
}
#[doc = "I2C1_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_pwren`] module"]
#[doc(alias = "I2C1_PWREN")]
pub type I2c1Pwren = crate::Reg<i2c1_pwren::I2c1PwrenSpec>;
#[doc = "Power enable"]
pub mod i2c1_pwren;
#[doc = "I2C1_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_rstctl`] module"]
#[doc(alias = "I2C1_RSTCTL")]
pub type I2c1Rstctl = crate::Reg<i2c1_rstctl::I2c1RstctlSpec>;
#[doc = "Reset Control"]
pub mod i2c1_rstctl;
#[doc = "I2C1_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_clkcfg`] module"]
#[doc(alias = "I2C1_CLKCFG")]
pub type I2c1Clkcfg = crate::Reg<i2c1_clkcfg::I2c1ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod i2c1_clkcfg;
#[doc = "I2C1_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_stat`] module"]
#[doc(alias = "I2C1_STAT")]
pub type I2c1Stat = crate::Reg<i2c1_stat::I2c1StatSpec>;
#[doc = "Status Register"]
pub mod i2c1_stat;
