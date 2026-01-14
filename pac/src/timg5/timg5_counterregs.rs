#[repr(C)]
#[doc = "TIMG5_COUNTERREGS\\[%s\\]"]
#[doc(alias = "TIMG5_COUNTERREGS")]
pub struct Timg5Counterregs {
    timg5_ctr: Timg5Ctr,
    timg5_ctrctl: Timg5Ctrctl,
    timg5_load: Timg5Load,
    _reserved3: [u8; 0x04],
    timg5_cc_01: [Timg5Cc01; 2],
    _reserved4: [u8; 0x18],
    timg5_ccctl_01: [Timg5Ccctl01; 2],
    _reserved5: [u8; 0x18],
    timg5_octl_01: [Timg5Octl01; 2],
    _reserved6: [u8; 0x18],
    timg5_ccact_01: [Timg5Ccact01; 2],
    _reserved7: [u8; 0x08],
    timg5_ifctl_01: [Timg5Ifctl01; 2],
    _reserved8: [u8; 0x28],
    timg5_tsel: Timg5Tsel,
}
impl Timg5Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn timg5_ctr(&self) -> &Timg5Ctr {
        &self.timg5_ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn timg5_ctrctl(&self) -> &Timg5Ctrctl {
        &self.timg5_ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn timg5_load(&self) -> &Timg5Load {
        &self.timg5_load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn timg5_cc_01(&self, n: usize) -> &Timg5Cc01 {
        &self.timg5_cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn timg5_cc_01_iter(&self) -> impl Iterator<Item = &Timg5Cc01> {
        self.timg5_cc_01.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn timg5_ccctl_01(&self, n: usize) -> &Timg5Ccctl01 {
        &self.timg5_ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn timg5_ccctl_01_iter(&self) -> impl Iterator<Item = &Timg5Ccctl01> {
        self.timg5_ccctl_01.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn timg5_octl_01(&self, n: usize) -> &Timg5Octl01 {
        &self.timg5_octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn timg5_octl_01_iter(&self) -> impl Iterator<Item = &Timg5Octl01> {
        self.timg5_octl_01.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn timg5_ccact_01(&self, n: usize) -> &Timg5Ccact01 {
        &self.timg5_ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn timg5_ccact_01_iter(&self) -> impl Iterator<Item = &Timg5Ccact01> {
        self.timg5_ccact_01.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn timg5_ifctl_01(&self, n: usize) -> &Timg5Ifctl01 {
        &self.timg5_ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub fn timg5_ifctl_01_iter(&self) -> impl Iterator<Item = &Timg5Ifctl01> {
        self.timg5_ifctl_01.iter()
    }
    #[doc = "0xb0 - Trigger Select"]
    #[inline(always)]
    pub const fn timg5_tsel(&self) -> &Timg5Tsel {
        &self.timg5_tsel
    }
}
#[doc = "TIMG5_CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ctr`] module"]
#[doc(alias = "TIMG5_CTR")]
pub type Timg5Ctr = crate::Reg<timg5_ctr::Timg5CtrSpec>;
#[doc = "Counter Register"]
pub mod timg5_ctr;
#[doc = "TIMG5_CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ctrctl`] module"]
#[doc(alias = "TIMG5_CTRCTL")]
pub type Timg5Ctrctl = crate::Reg<timg5_ctrctl::Timg5CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod timg5_ctrctl;
#[doc = "TIMG5_LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_load`] module"]
#[doc(alias = "TIMG5_LOAD")]
pub type Timg5Load = crate::Reg<timg5_load::Timg5LoadSpec>;
#[doc = "Load Register"]
pub mod timg5_load;
#[doc = "TIMG5_CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_cc_01`] module"]
#[doc(alias = "TIMG5_CC_01")]
pub type Timg5Cc01 = crate::Reg<timg5_cc_01::Timg5Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod timg5_cc_01;
#[doc = "TIMG5_CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ccctl_01`] module"]
#[doc(alias = "TIMG5_CCCTL_01")]
pub type Timg5Ccctl01 = crate::Reg<timg5_ccctl_01::Timg5Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod timg5_ccctl_01;
#[doc = "TIMG5_OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_octl_01`] module"]
#[doc(alias = "TIMG5_OCTL_01")]
pub type Timg5Octl01 = crate::Reg<timg5_octl_01::Timg5Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod timg5_octl_01;
#[doc = "TIMG5_CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ccact_01`] module"]
#[doc(alias = "TIMG5_CCACT_01")]
pub type Timg5Ccact01 = crate::Reg<timg5_ccact_01::Timg5Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod timg5_ccact_01;
#[doc = "TIMG5_IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_ifctl_01`] module"]
#[doc(alias = "TIMG5_IFCTL_01")]
pub type Timg5Ifctl01 = crate::Reg<timg5_ifctl_01::Timg5Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod timg5_ifctl_01;
#[doc = "TIMG5_TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_tsel`] module"]
#[doc(alias = "TIMG5_TSEL")]
pub type Timg5Tsel = crate::Reg<timg5_tsel::Timg5TselSpec>;
#[doc = "Trigger Select"]
pub mod timg5_tsel;
