#[repr(C)]
#[doc = "TIMG12_COUNTERREGS\\[%s\\]"]
#[doc(alias = "TIMG12_COUNTERREGS")]
pub struct Timg12Counterregs {
    timg12_ctr: Timg12Ctr,
    timg12_ctrctl: Timg12Ctrctl,
    timg12_load: Timg12Load,
    _reserved3: [u8; 0x04],
    timg12_cc_01: [Timg12Cc01; 2],
    _reserved4: [u8; 0x18],
    timg12_ccctl_01: [Timg12Ccctl01; 2],
    _reserved5: [u8; 0x18],
    timg12_octl_01: [Timg12Octl01; 2],
    _reserved6: [u8; 0x18],
    timg12_ccact_01: [Timg12Ccact01; 2],
    _reserved7: [u8; 0x08],
    timg12_ifctl_01: [Timg12Ifctl01; 2],
    _reserved8: [u8; 0x28],
    timg12_tsel: Timg12Tsel,
}
impl Timg12Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn timg12_ctr(&self) -> &Timg12Ctr {
        &self.timg12_ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn timg12_ctrctl(&self) -> &Timg12Ctrctl {
        &self.timg12_ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn timg12_load(&self) -> &Timg12Load {
        &self.timg12_load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn timg12_cc_01(&self, n: usize) -> &Timg12Cc01 {
        &self.timg12_cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn timg12_cc_01_iter(&self) -> impl Iterator<Item = &Timg12Cc01> {
        self.timg12_cc_01.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn timg12_ccctl_01(&self, n: usize) -> &Timg12Ccctl01 {
        &self.timg12_ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn timg12_ccctl_01_iter(&self) -> impl Iterator<Item = &Timg12Ccctl01> {
        self.timg12_ccctl_01.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn timg12_octl_01(&self, n: usize) -> &Timg12Octl01 {
        &self.timg12_octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn timg12_octl_01_iter(&self) -> impl Iterator<Item = &Timg12Octl01> {
        self.timg12_octl_01.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn timg12_ccact_01(&self, n: usize) -> &Timg12Ccact01 {
        &self.timg12_ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn timg12_ccact_01_iter(&self) -> impl Iterator<Item = &Timg12Ccact01> {
        self.timg12_ccact_01.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn timg12_ifctl_01(&self, n: usize) -> &Timg12Ifctl01 {
        &self.timg12_ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub fn timg12_ifctl_01_iter(&self) -> impl Iterator<Item = &Timg12Ifctl01> {
        self.timg12_ifctl_01.iter()
    }
    #[doc = "0xb0 - Trigger Select"]
    #[inline(always)]
    pub const fn timg12_tsel(&self) -> &Timg12Tsel {
        &self.timg12_tsel
    }
}
#[doc = "TIMG12_CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ctr`] module"]
#[doc(alias = "TIMG12_CTR")]
pub type Timg12Ctr = crate::Reg<timg12_ctr::Timg12CtrSpec>;
#[doc = "Counter Register"]
pub mod timg12_ctr;
#[doc = "TIMG12_CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ctrctl`] module"]
#[doc(alias = "TIMG12_CTRCTL")]
pub type Timg12Ctrctl = crate::Reg<timg12_ctrctl::Timg12CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod timg12_ctrctl;
#[doc = "TIMG12_LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_load`] module"]
#[doc(alias = "TIMG12_LOAD")]
pub type Timg12Load = crate::Reg<timg12_load::Timg12LoadSpec>;
#[doc = "Load Register"]
pub mod timg12_load;
#[doc = "TIMG12_CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_cc_01`] module"]
#[doc(alias = "TIMG12_CC_01")]
pub type Timg12Cc01 = crate::Reg<timg12_cc_01::Timg12Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod timg12_cc_01;
#[doc = "TIMG12_CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ccctl_01`] module"]
#[doc(alias = "TIMG12_CCCTL_01")]
pub type Timg12Ccctl01 = crate::Reg<timg12_ccctl_01::Timg12Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod timg12_ccctl_01;
#[doc = "TIMG12_OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_octl_01`] module"]
#[doc(alias = "TIMG12_OCTL_01")]
pub type Timg12Octl01 = crate::Reg<timg12_octl_01::Timg12Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod timg12_octl_01;
#[doc = "TIMG12_CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ccact_01`] module"]
#[doc(alias = "TIMG12_CCACT_01")]
pub type Timg12Ccact01 = crate::Reg<timg12_ccact_01::Timg12Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod timg12_ccact_01;
#[doc = "TIMG12_IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_ifctl_01`] module"]
#[doc(alias = "TIMG12_IFCTL_01")]
pub type Timg12Ifctl01 = crate::Reg<timg12_ifctl_01::Timg12Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod timg12_ifctl_01;
#[doc = "TIMG12_TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_tsel`] module"]
#[doc(alias = "TIMG12_TSEL")]
pub type Timg12Tsel = crate::Reg<timg12_tsel::Timg12TselSpec>;
#[doc = "Trigger Select"]
pub mod timg12_tsel;
