#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    lcd_pwren: LcdPwren,
    lcd_rstctl: LcdRstctl,
    _reserved2: [u8; 0x0c],
    lcd_stat: LcdStat,
    _reserved3: [u8; 0x0808],
    lcd_int_event0: [LcdIntEvent0; 1],
    _reserved4: [u8; 0x94],
    lcd_evt_mode: LcdEvtMode,
    _reserved5: [u8; 0x1c],
    lcd_lcdctl0: LcdLcdctl0,
    _reserved6: [u8; 0x04],
    lcd_lcdblkctl: LcdLcdblkctl,
    lcd_lcdmemctl: LcdLcdmemctl,
    lcd_lcdvctl: LcdLcdvctl,
    lcd_lcdpctl0: LcdLcdpctl0,
    lcd_lcdpctl1: LcdLcdpctl1,
    lcd_lcdpctl2: LcdLcdpctl2,
    lcd_lcdpctl3: LcdLcdpctl3,
    _reserved13: [u8; 0x04],
    lcd_lcdcssel0: LcdLcdcssel0,
    lcd_lcdcssel1: LcdLcdcssel1,
    lcd_lcdcssel2: LcdLcdcssel2,
    lcd_lcdcssel3: LcdLcdcssel3,
    _reserved17: [u8; 0x08],
    lcd_lcdm: [LcdLcdm; 64],
    lcd_lcdbm: [LcdLcdbm; 32],
}
impl RegisterBlock {
    #[doc = "0x800 - Power enable"]
    #[inline(always)]
    pub const fn lcd_pwren(&self) -> &LcdPwren {
        &self.lcd_pwren
    }
    #[doc = "0x804 - Reset Control"]
    #[inline(always)]
    pub const fn lcd_rstctl(&self) -> &LcdRstctl {
        &self.lcd_rstctl
    }
    #[doc = "0x814 - Status Register"]
    #[inline(always)]
    pub const fn lcd_stat(&self) -> &LcdStat {
        &self.lcd_stat
    }
    #[doc = "0x1020..0x104c - LCD_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn lcd_int_event0(&self, n: usize) -> &LcdIntEvent0 {
        &self.lcd_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - LCD_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn lcd_int_event0_iter(&self) -> impl Iterator<Item = &LcdIntEvent0> {
        self.lcd_int_event0.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn lcd_evt_mode(&self) -> &LcdEvtMode {
        &self.lcd_evt_mode
    }
    #[doc = "0x1100 - LCD control register 0"]
    #[inline(always)]
    pub const fn lcd_lcdctl0(&self) -> &LcdLcdctl0 {
        &self.lcd_lcdctl0
    }
    #[doc = "0x1108 - LCD blicking control register"]
    #[inline(always)]
    pub const fn lcd_lcdblkctl(&self) -> &LcdLcdblkctl {
        &self.lcd_lcdblkctl
    }
    #[doc = "0x110c - LCD memory control LCD memory control register"]
    #[inline(always)]
    pub const fn lcd_lcdmemctl(&self) -> &LcdLcdmemctl {
        &self.lcd_lcdmemctl
    }
    #[doc = "0x1110 - LCD voltage control register"]
    #[inline(always)]
    pub const fn lcd_lcdvctl(&self) -> &LcdLcdvctl {
        &self.lcd_lcdvctl
    }
    #[doc = "0x1114 - LCD port control register 0"]
    #[inline(always)]
    pub const fn lcd_lcdpctl0(&self) -> &LcdLcdpctl0 {
        &self.lcd_lcdpctl0
    }
    #[doc = "0x1118 - LCD port control register 1"]
    #[inline(always)]
    pub const fn lcd_lcdpctl1(&self) -> &LcdLcdpctl1 {
        &self.lcd_lcdpctl1
    }
    #[doc = "0x111c - LCD port control register 2"]
    #[inline(always)]
    pub const fn lcd_lcdpctl2(&self) -> &LcdLcdpctl2 {
        &self.lcd_lcdpctl2
    }
    #[doc = "0x1120 - LCD port control register 3"]
    #[inline(always)]
    pub const fn lcd_lcdpctl3(&self) -> &LcdLcdpctl3 {
        &self.lcd_lcdpctl3
    }
    #[doc = "0x1128 - LCD common segment select register 0"]
    #[inline(always)]
    pub const fn lcd_lcdcssel0(&self) -> &LcdLcdcssel0 {
        &self.lcd_lcdcssel0
    }
    #[doc = "0x112c - LCD common segment select register 1"]
    #[inline(always)]
    pub const fn lcd_lcdcssel1(&self) -> &LcdLcdcssel1 {
        &self.lcd_lcdcssel1
    }
    #[doc = "0x1130 - LCD common segment select register 2"]
    #[inline(always)]
    pub const fn lcd_lcdcssel2(&self) -> &LcdLcdcssel2 {
        &self.lcd_lcdcssel2
    }
    #[doc = "0x1134 - LCD common segment select register 3"]
    #[inline(always)]
    pub const fn lcd_lcdcssel3(&self) -> &LcdLcdcssel3 {
        &self.lcd_lcdcssel3
    }
    #[doc = "0x1140..0x1180 - LCD memory index register"]
    #[inline(always)]
    pub const fn lcd_lcdm(&self, n: usize) -> &LcdLcdm {
        &self.lcd_lcdm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1140..0x1180 - LCD memory index register"]
    #[inline(always)]
    pub fn lcd_lcdm_iter(&self) -> impl Iterator<Item = &LcdLcdm> {
        self.lcd_lcdm.iter()
    }
    #[doc = "0x1180..0x11a0 - LCD blinking memory index register"]
    #[inline(always)]
    pub const fn lcd_lcdbm(&self, n: usize) -> &LcdLcdbm {
        &self.lcd_lcdbm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1180..0x11a0 - LCD blinking memory index register"]
    #[inline(always)]
    pub fn lcd_lcdbm_iter(&self) -> impl Iterator<Item = &LcdLcdbm> {
        self.lcd_lcdbm.iter()
    }
}
#[doc = "LCD_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_pwren`] module"]
#[doc(alias = "LCD_PWREN")]
pub type LcdPwren = crate::Reg<lcd_pwren::LcdPwrenSpec>;
#[doc = "Power enable"]
pub mod lcd_pwren;
#[doc = "LCD_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_rstctl`] module"]
#[doc(alias = "LCD_RSTCTL")]
pub type LcdRstctl = crate::Reg<lcd_rstctl::LcdRstctlSpec>;
#[doc = "Reset Control"]
pub mod lcd_rstctl;
#[doc = "LCD_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_stat`] module"]
#[doc(alias = "LCD_STAT")]
pub type LcdStat = crate::Reg<lcd_stat::LcdStatSpec>;
#[doc = "Status Register"]
pub mod lcd_stat;
#[doc = "LCD_INT_EVENT0\\[%s\\]"]
pub use self::lcd_int_event0::LcdIntEvent0;
#[doc = r"Cluster"]
#[doc = "LCD_INT_EVENT0\\[%s\\]"]
pub mod lcd_int_event0;
#[doc = "LCD_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_evt_mode`] module"]
#[doc(alias = "LCD_EVT_MODE")]
pub type LcdEvtMode = crate::Reg<lcd_evt_mode::LcdEvtModeSpec>;
#[doc = "Event Mode"]
pub mod lcd_evt_mode;
#[doc = "LCD_LCDCTL0 (rw) register accessor: LCD control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdctl0`] module"]
#[doc(alias = "LCD_LCDCTL0")]
pub type LcdLcdctl0 = crate::Reg<lcd_lcdctl0::LcdLcdctl0Spec>;
#[doc = "LCD control register 0"]
pub mod lcd_lcdctl0;
#[doc = "LCD_LCDBLKCTL (rw) register accessor: LCD blicking control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdblkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdblkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdblkctl`] module"]
#[doc(alias = "LCD_LCDBLKCTL")]
pub type LcdLcdblkctl = crate::Reg<lcd_lcdblkctl::LcdLcdblkctlSpec>;
#[doc = "LCD blicking control register"]
pub mod lcd_lcdblkctl;
#[doc = "LCD_LCDMEMCTL (rw) register accessor: LCD memory control LCD memory control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdmemctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdmemctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdmemctl`] module"]
#[doc(alias = "LCD_LCDMEMCTL")]
pub type LcdLcdmemctl = crate::Reg<lcd_lcdmemctl::LcdLcdmemctlSpec>;
#[doc = "LCD memory control LCD memory control register"]
pub mod lcd_lcdmemctl;
#[doc = "LCD_LCDVCTL (rw) register accessor: LCD voltage control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdvctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdvctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdvctl`] module"]
#[doc(alias = "LCD_LCDVCTL")]
pub type LcdLcdvctl = crate::Reg<lcd_lcdvctl::LcdLcdvctlSpec>;
#[doc = "LCD voltage control register"]
pub mod lcd_lcdvctl;
#[doc = "LCD_LCDPCTL0 (rw) register accessor: LCD port control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdpctl0`] module"]
#[doc(alias = "LCD_LCDPCTL0")]
pub type LcdLcdpctl0 = crate::Reg<lcd_lcdpctl0::LcdLcdpctl0Spec>;
#[doc = "LCD port control register 0"]
pub mod lcd_lcdpctl0;
#[doc = "LCD_LCDPCTL1 (rw) register accessor: LCD port control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdpctl1`] module"]
#[doc(alias = "LCD_LCDPCTL1")]
pub type LcdLcdpctl1 = crate::Reg<lcd_lcdpctl1::LcdLcdpctl1Spec>;
#[doc = "LCD port control register 1"]
pub mod lcd_lcdpctl1;
#[doc = "LCD_LCDPCTL2 (rw) register accessor: LCD port control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdpctl2`] module"]
#[doc(alias = "LCD_LCDPCTL2")]
pub type LcdLcdpctl2 = crate::Reg<lcd_lcdpctl2::LcdLcdpctl2Spec>;
#[doc = "LCD port control register 2"]
pub mod lcd_lcdpctl2;
#[doc = "LCD_LCDPCTL3 (rw) register accessor: LCD port control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdpctl3`] module"]
#[doc(alias = "LCD_LCDPCTL3")]
pub type LcdLcdpctl3 = crate::Reg<lcd_lcdpctl3::LcdLcdpctl3Spec>;
#[doc = "LCD port control register 3"]
pub mod lcd_lcdpctl3;
#[doc = "LCD_LCDCSSEL0 (rw) register accessor: LCD common segment select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdcssel0`] module"]
#[doc(alias = "LCD_LCDCSSEL0")]
pub type LcdLcdcssel0 = crate::Reg<lcd_lcdcssel0::LcdLcdcssel0Spec>;
#[doc = "LCD common segment select register 0"]
pub mod lcd_lcdcssel0;
#[doc = "LCD_LCDCSSEL1 (rw) register accessor: LCD common segment select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdcssel1`] module"]
#[doc(alias = "LCD_LCDCSSEL1")]
pub type LcdLcdcssel1 = crate::Reg<lcd_lcdcssel1::LcdLcdcssel1Spec>;
#[doc = "LCD common segment select register 1"]
pub mod lcd_lcdcssel1;
#[doc = "LCD_LCDCSSEL2 (rw) register accessor: LCD common segment select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdcssel2`] module"]
#[doc(alias = "LCD_LCDCSSEL2")]
pub type LcdLcdcssel2 = crate::Reg<lcd_lcdcssel2::LcdLcdcssel2Spec>;
#[doc = "LCD common segment select register 2"]
pub mod lcd_lcdcssel2;
#[doc = "LCD_LCDCSSEL3 (rw) register accessor: LCD common segment select register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdcssel3`] module"]
#[doc(alias = "LCD_LCDCSSEL3")]
pub type LcdLcdcssel3 = crate::Reg<lcd_lcdcssel3::LcdLcdcssel3Spec>;
#[doc = "LCD common segment select register 3"]
pub mod lcd_lcdcssel3;
#[doc = "LCD_LCDM (rw) register accessor: LCD memory index register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdm`] module"]
#[doc(alias = "LCD_LCDM")]
pub type LcdLcdm = crate::Reg<lcd_lcdm::LcdLcdmSpec>;
#[doc = "LCD memory index register"]
pub mod lcd_lcdm;
#[doc = "LCD_LCDBM (rw) register accessor: LCD blinking memory index register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdbm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdbm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_lcdbm`] module"]
#[doc(alias = "LCD_LCDBM")]
pub type LcdLcdbm = crate::Reg<lcd_lcdbm::LcdLcdbmSpec>;
#[doc = "LCD blinking memory index register"]
pub mod lcd_lcdbm;
