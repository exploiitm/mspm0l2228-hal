#[repr(C)]
#[doc = "TIMG4_COUNTERREGS\\[%s\\]"]
#[doc(alias = "TIMG4_COUNTERREGS")]
pub struct Timg4Counterregs {
    timg4_ctr: Timg4Ctr,
    timg4_ctrctl: Timg4Ctrctl,
    timg4_load: Timg4Load,
    _reserved3: [u8; 0x04],
    timg4_cc_01: [Timg4Cc01; 2],
    _reserved4: [u8; 0x18],
    timg4_ccctl_01: [Timg4Ccctl01; 2],
    _reserved5: [u8; 0x18],
    timg4_octl_01: [Timg4Octl01; 2],
    _reserved6: [u8; 0x18],
    timg4_ccact_01: [Timg4Ccact01; 2],
    _reserved7: [u8; 0x08],
    timg4_ifctl_01: [Timg4Ifctl01; 2],
    _reserved8: [u8; 0x28],
    timg4_tsel: Timg4Tsel,
}
impl Timg4Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn timg4_ctr(&self) -> &Timg4Ctr {
        &self.timg4_ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn timg4_ctrctl(&self) -> &Timg4Ctrctl {
        &self.timg4_ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn timg4_load(&self) -> &Timg4Load {
        &self.timg4_load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn timg4_cc_01(&self, n: usize) -> &Timg4Cc01 {
        &self.timg4_cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn timg4_cc_01_iter(&self) -> impl Iterator<Item = &Timg4Cc01> {
        self.timg4_cc_01.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn timg4_ccctl_01(&self, n: usize) -> &Timg4Ccctl01 {
        &self.timg4_ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn timg4_ccctl_01_iter(&self) -> impl Iterator<Item = &Timg4Ccctl01> {
        self.timg4_ccctl_01.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn timg4_octl_01(&self, n: usize) -> &Timg4Octl01 {
        &self.timg4_octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn timg4_octl_01_iter(&self) -> impl Iterator<Item = &Timg4Octl01> {
        self.timg4_octl_01.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn timg4_ccact_01(&self, n: usize) -> &Timg4Ccact01 {
        &self.timg4_ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn timg4_ccact_01_iter(&self) -> impl Iterator<Item = &Timg4Ccact01> {
        self.timg4_ccact_01.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn timg4_ifctl_01(&self, n: usize) -> &Timg4Ifctl01 {
        &self.timg4_ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub fn timg4_ifctl_01_iter(&self) -> impl Iterator<Item = &Timg4Ifctl01> {
        self.timg4_ifctl_01.iter()
    }
    #[doc = "0xb0 - Trigger Select"]
    #[inline(always)]
    pub const fn timg4_tsel(&self) -> &Timg4Tsel {
        &self.timg4_tsel
    }
}
#[doc = "TIMG4_CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ctr`] module"]
#[doc(alias = "TIMG4_CTR")]
pub type Timg4Ctr = crate::Reg<timg4_ctr::Timg4CtrSpec>;
#[doc = "Counter Register"]
pub mod timg4_ctr;
#[doc = "TIMG4_CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ctrctl`] module"]
#[doc(alias = "TIMG4_CTRCTL")]
pub type Timg4Ctrctl = crate::Reg<timg4_ctrctl::Timg4CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod timg4_ctrctl;
#[doc = "TIMG4_LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_load`] module"]
#[doc(alias = "TIMG4_LOAD")]
pub type Timg4Load = crate::Reg<timg4_load::Timg4LoadSpec>;
#[doc = "Load Register"]
pub mod timg4_load;
#[doc = "TIMG4_CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_cc_01`] module"]
#[doc(alias = "TIMG4_CC_01")]
pub type Timg4Cc01 = crate::Reg<timg4_cc_01::Timg4Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod timg4_cc_01;
#[doc = "TIMG4_CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ccctl_01`] module"]
#[doc(alias = "TIMG4_CCCTL_01")]
pub type Timg4Ccctl01 = crate::Reg<timg4_ccctl_01::Timg4Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod timg4_ccctl_01;
#[doc = "TIMG4_OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_octl_01`] module"]
#[doc(alias = "TIMG4_OCTL_01")]
pub type Timg4Octl01 = crate::Reg<timg4_octl_01::Timg4Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod timg4_octl_01;
#[doc = "TIMG4_CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ccact_01`] module"]
#[doc(alias = "TIMG4_CCACT_01")]
pub type Timg4Ccact01 = crate::Reg<timg4_ccact_01::Timg4Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod timg4_ccact_01;
#[doc = "TIMG4_IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_ifctl_01`] module"]
#[doc(alias = "TIMG4_IFCTL_01")]
pub type Timg4Ifctl01 = crate::Reg<timg4_ifctl_01::Timg4Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod timg4_ifctl_01;
#[doc = "TIMG4_TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_tsel`] module"]
#[doc(alias = "TIMG4_TSEL")]
pub type Timg4Tsel = crate::Reg<timg4_tsel::Timg4TselSpec>;
#[doc = "Trigger Select"]
pub mod timg4_tsel;
