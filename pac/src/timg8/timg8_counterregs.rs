#[repr(C)]
#[doc = "TIMG8_COUNTERREGS\\[%s\\]"]
#[doc(alias = "TIMG8_COUNTERREGS")]
pub struct Timg8Counterregs {
    timg8_ctr: Timg8Ctr,
    timg8_ctrctl: Timg8Ctrctl,
    timg8_load: Timg8Load,
    _reserved3: [u8; 0x04],
    timg8_cc_01: [Timg8Cc01; 2],
    _reserved4: [u8; 0x18],
    timg8_ccctl_01: [Timg8Ccctl01; 2],
    _reserved5: [u8; 0x18],
    timg8_octl_01: [Timg8Octl01; 2],
    _reserved6: [u8; 0x18],
    timg8_ccact_01: [Timg8Ccact01; 2],
    _reserved7: [u8; 0x08],
    timg8_ifctl_01: [Timg8Ifctl01; 2],
    _reserved8: [u8; 0x28],
    timg8_tsel: Timg8Tsel,
    _reserved9: [u8; 0x08],
    timg8_qdir: Timg8Qdir,
}
impl Timg8Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn timg8_ctr(&self) -> &Timg8Ctr {
        &self.timg8_ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn timg8_ctrctl(&self) -> &Timg8Ctrctl {
        &self.timg8_ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn timg8_load(&self) -> &Timg8Load {
        &self.timg8_load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn timg8_cc_01(&self, n: usize) -> &Timg8Cc01 {
        &self.timg8_cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn timg8_cc_01_iter(&self) -> impl Iterator<Item = &Timg8Cc01> {
        self.timg8_cc_01.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn timg8_ccctl_01(&self, n: usize) -> &Timg8Ccctl01 {
        &self.timg8_ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn timg8_ccctl_01_iter(&self) -> impl Iterator<Item = &Timg8Ccctl01> {
        self.timg8_ccctl_01.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn timg8_octl_01(&self, n: usize) -> &Timg8Octl01 {
        &self.timg8_octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn timg8_octl_01_iter(&self) -> impl Iterator<Item = &Timg8Octl01> {
        self.timg8_octl_01.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn timg8_ccact_01(&self, n: usize) -> &Timg8Ccact01 {
        &self.timg8_ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn timg8_ccact_01_iter(&self) -> impl Iterator<Item = &Timg8Ccact01> {
        self.timg8_ccact_01.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn timg8_ifctl_01(&self, n: usize) -> &Timg8Ifctl01 {
        &self.timg8_ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub fn timg8_ifctl_01_iter(&self) -> impl Iterator<Item = &Timg8Ifctl01> {
        self.timg8_ifctl_01.iter()
    }
    #[doc = "0xb0 - Trigger Select"]
    #[inline(always)]
    pub const fn timg8_tsel(&self) -> &Timg8Tsel {
        &self.timg8_tsel
    }
    #[doc = "0xbc - Count Direction Register"]
    #[inline(always)]
    pub const fn timg8_qdir(&self) -> &Timg8Qdir {
        &self.timg8_qdir
    }
}
#[doc = "TIMG8_CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ctr`] module"]
#[doc(alias = "TIMG8_CTR")]
pub type Timg8Ctr = crate::Reg<timg8_ctr::Timg8CtrSpec>;
#[doc = "Counter Register"]
pub mod timg8_ctr;
#[doc = "TIMG8_CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ctrctl`] module"]
#[doc(alias = "TIMG8_CTRCTL")]
pub type Timg8Ctrctl = crate::Reg<timg8_ctrctl::Timg8CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod timg8_ctrctl;
#[doc = "TIMG8_LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_load`] module"]
#[doc(alias = "TIMG8_LOAD")]
pub type Timg8Load = crate::Reg<timg8_load::Timg8LoadSpec>;
#[doc = "Load Register"]
pub mod timg8_load;
#[doc = "TIMG8_CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_cc_01`] module"]
#[doc(alias = "TIMG8_CC_01")]
pub type Timg8Cc01 = crate::Reg<timg8_cc_01::Timg8Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod timg8_cc_01;
#[doc = "TIMG8_CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ccctl_01`] module"]
#[doc(alias = "TIMG8_CCCTL_01")]
pub type Timg8Ccctl01 = crate::Reg<timg8_ccctl_01::Timg8Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod timg8_ccctl_01;
#[doc = "TIMG8_OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_octl_01`] module"]
#[doc(alias = "TIMG8_OCTL_01")]
pub type Timg8Octl01 = crate::Reg<timg8_octl_01::Timg8Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod timg8_octl_01;
#[doc = "TIMG8_CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ccact_01`] module"]
#[doc(alias = "TIMG8_CCACT_01")]
pub type Timg8Ccact01 = crate::Reg<timg8_ccact_01::Timg8Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod timg8_ccact_01;
#[doc = "TIMG8_IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_ifctl_01`] module"]
#[doc(alias = "TIMG8_IFCTL_01")]
pub type Timg8Ifctl01 = crate::Reg<timg8_ifctl_01::Timg8Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod timg8_ifctl_01;
#[doc = "TIMG8_TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_tsel`] module"]
#[doc(alias = "TIMG8_TSEL")]
pub type Timg8Tsel = crate::Reg<timg8_tsel::Timg8TselSpec>;
#[doc = "Trigger Select"]
pub mod timg8_tsel;
#[doc = "TIMG8_QDIR (r) register accessor: Count Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_qdir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_qdir`] module"]
#[doc(alias = "TIMG8_QDIR")]
pub type Timg8Qdir = crate::Reg<timg8_qdir::Timg8QdirSpec>;
#[doc = "Count Direction Register"]
pub mod timg8_qdir;
