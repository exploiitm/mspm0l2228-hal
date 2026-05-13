#[repr(C)]
#[doc = "I2C2_GPRCM\\[%s\\]"]
#[doc(alias = "I2C2_GPRCM")]
pub struct I2c2Gprcm {
    i2c2_pwren: I2c2Pwren,
    i2c2_rstctl: I2c2Rstctl,
    i2c2_clkcfg: I2c2Clkcfg,
    _reserved3: [u8; 0x08],
    i2c2_stat: I2c2Stat,
}
impl I2c2Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn i2c2_pwren(&self) -> &I2c2Pwren {
        &self.i2c2_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn i2c2_rstctl(&self) -> &I2c2Rstctl {
        &self.i2c2_rstctl
    }
    #[doc = "0x08 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn i2c2_clkcfg(&self) -> &I2c2Clkcfg {
        &self.i2c2_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn i2c2_stat(&self) -> &I2c2Stat {
        &self.i2c2_stat
    }
}
#[doc = "I2C2_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_pwren`] module"]
#[doc(alias = "I2C2_PWREN")]
pub type I2c2Pwren = crate::Reg<i2c2_pwren::I2c2PwrenSpec>;
#[doc = "Power enable"]
pub mod i2c2_pwren;
#[doc = "I2C2_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_rstctl`] module"]
#[doc(alias = "I2C2_RSTCTL")]
pub type I2c2Rstctl = crate::Reg<i2c2_rstctl::I2c2RstctlSpec>;
#[doc = "Reset Control"]
pub mod i2c2_rstctl;
#[doc = "I2C2_CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_clkcfg`] module"]
#[doc(alias = "I2C2_CLKCFG")]
pub type I2c2Clkcfg = crate::Reg<i2c2_clkcfg::I2c2ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod i2c2_clkcfg;
#[doc = "I2C2_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_stat`] module"]
#[doc(alias = "I2C2_STAT")]
pub type I2c2Stat = crate::Reg<i2c2_stat::I2c2StatSpec>;
#[doc = "Status Register"]
pub mod i2c2_stat;
