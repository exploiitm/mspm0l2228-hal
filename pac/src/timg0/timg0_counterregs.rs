#[repr(C)]
#[doc = "TIMG0_COUNTERREGS\\[%s\\]"]
#[doc(alias = "TIMG0_COUNTERREGS")]
pub struct Timg0Counterregs {
    timg0_ctr: Timg0Ctr,
    timg0_ctrctl: Timg0Ctrctl,
    timg0_load: Timg0Load,
    _reserved3: [u8; 0x04],
    timg0_cc_01: [Timg0Cc01; 2],
    _reserved4: [u8; 0x18],
    timg0_ccctl_01: [Timg0Ccctl01; 2],
    _reserved5: [u8; 0x18],
    timg0_octl_01: [Timg0Octl01; 2],
    _reserved6: [u8; 0x18],
    timg0_ccact_01: [Timg0Ccact01; 2],
    _reserved7: [u8; 0x08],
    timg0_ifctl_01: [Timg0Ifctl01; 2],
    _reserved8: [u8; 0x28],
    timg0_tsel: Timg0Tsel,
}
impl Timg0Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn timg0_ctr(&self) -> &Timg0Ctr {
        &self.timg0_ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn timg0_ctrctl(&self) -> &Timg0Ctrctl {
        &self.timg0_ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn timg0_load(&self) -> &Timg0Load {
        &self.timg0_load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn timg0_cc_01(&self, n: usize) -> &Timg0Cc01 {
        &self.timg0_cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn timg0_cc_01_iter(&self) -> impl Iterator<Item = &Timg0Cc01> {
        self.timg0_cc_01.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn timg0_ccctl_01(&self, n: usize) -> &Timg0Ccctl01 {
        &self.timg0_ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn timg0_ccctl_01_iter(&self) -> impl Iterator<Item = &Timg0Ccctl01> {
        self.timg0_ccctl_01.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn timg0_octl_01(&self, n: usize) -> &Timg0Octl01 {
        &self.timg0_octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn timg0_octl_01_iter(&self) -> impl Iterator<Item = &Timg0Octl01> {
        self.timg0_octl_01.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn timg0_ccact_01(&self, n: usize) -> &Timg0Ccact01 {
        &self.timg0_ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn timg0_ccact_01_iter(&self) -> impl Iterator<Item = &Timg0Ccact01> {
        self.timg0_ccact_01.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn timg0_ifctl_01(&self, n: usize) -> &Timg0Ifctl01 {
        &self.timg0_ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub fn timg0_ifctl_01_iter(&self) -> impl Iterator<Item = &Timg0Ifctl01> {
        self.timg0_ifctl_01.iter()
    }
    #[doc = "0xb0 - Trigger Select"]
    #[inline(always)]
    pub const fn timg0_tsel(&self) -> &Timg0Tsel {
        &self.timg0_tsel
    }
}
#[doc = "TIMG0_CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ctr`] module"]
#[doc(alias = "TIMG0_CTR")]
pub type Timg0Ctr = crate::Reg<timg0_ctr::Timg0CtrSpec>;
#[doc = "Counter Register"]
pub mod timg0_ctr;
#[doc = "TIMG0_CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ctrctl`] module"]
#[doc(alias = "TIMG0_CTRCTL")]
pub type Timg0Ctrctl = crate::Reg<timg0_ctrctl::Timg0CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod timg0_ctrctl;
#[doc = "TIMG0_LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_load`] module"]
#[doc(alias = "TIMG0_LOAD")]
pub type Timg0Load = crate::Reg<timg0_load::Timg0LoadSpec>;
#[doc = "Load Register"]
pub mod timg0_load;
#[doc = "TIMG0_CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_cc_01`] module"]
#[doc(alias = "TIMG0_CC_01")]
pub type Timg0Cc01 = crate::Reg<timg0_cc_01::Timg0Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod timg0_cc_01;
#[doc = "TIMG0_CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ccctl_01`] module"]
#[doc(alias = "TIMG0_CCCTL_01")]
pub type Timg0Ccctl01 = crate::Reg<timg0_ccctl_01::Timg0Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod timg0_ccctl_01;
#[doc = "TIMG0_OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_octl_01`] module"]
#[doc(alias = "TIMG0_OCTL_01")]
pub type Timg0Octl01 = crate::Reg<timg0_octl_01::Timg0Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod timg0_octl_01;
#[doc = "TIMG0_CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ccact_01`] module"]
#[doc(alias = "TIMG0_CCACT_01")]
pub type Timg0Ccact01 = crate::Reg<timg0_ccact_01::Timg0Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod timg0_ccact_01;
#[doc = "TIMG0_IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_ifctl_01`] module"]
#[doc(alias = "TIMG0_IFCTL_01")]
pub type Timg0Ifctl01 = crate::Reg<timg0_ifctl_01::Timg0Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod timg0_ifctl_01;
#[doc = "TIMG0_TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_tsel`] module"]
#[doc(alias = "TIMG0_TSEL")]
pub type Timg0Tsel = crate::Reg<timg0_tsel::Timg0TselSpec>;
#[doc = "Trigger Select"]
pub mod timg0_tsel;
