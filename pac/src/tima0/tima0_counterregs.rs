#[repr(C)]
#[doc = "TIMA0_COUNTERREGS\\[%s\\]"]
#[doc(alias = "TIMA0_COUNTERREGS")]
pub struct Tima0Counterregs {
    tima0_ctr: Tima0Ctr,
    tima0_ctrctl: Tima0Ctrctl,
    tima0_load: Tima0Load,
    _reserved3: [u8; 0x04],
    tima0_cc_01: [Tima0Cc01; 2],
    tima0_cc_23: [Tima0Cc23; 2],
    tima0_cc_45: [Tima0Cc45; 2],
    _reserved6: [u8; 0x08],
    tima0_ccctl_01: [Tima0Ccctl01; 2],
    tima0_ccctl_23: [Tima0Ccctl23; 2],
    tima0_ccctl_45: [Tima0Ccctl45; 2],
    _reserved9: [u8; 0x08],
    tima0_octl_01: [Tima0Octl01; 2],
    tima0_octl_23: [Tima0Octl23; 2],
    _reserved11: [u8; 0x10],
    tima0_ccact_01: [Tima0Ccact01; 2],
    tima0_ccact_23: [Tima0Ccact23; 2],
    tima0_ifctl_01: [Tima0Ifctl01; 2],
    tima0_ifctl_23: [Tima0Ifctl23; 2],
    _reserved15: [u8; 0x10],
    tima0_pl: Tima0Pl,
    tima0_dbctl: Tima0Dbctl,
    _reserved17: [u8; 0x08],
    tima0_tsel: Tima0Tsel,
    tima0_rc: Tima0Rc,
    tima0_rcld: Tima0Rcld,
    _reserved20: [u8; 0x14],
    tima0_fctl: Tima0Fctl,
    tima0_fifctl: Tima0Fifctl,
}
impl Tima0Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn tima0_ctr(&self) -> &Tima0Ctr {
        &self.tima0_ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn tima0_ctrctl(&self) -> &Tima0Ctrctl {
        &self.tima0_ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn tima0_load(&self) -> &Tima0Load {
        &self.tima0_load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn tima0_cc_01(&self, n: usize) -> &Tima0Cc01 {
        &self.tima0_cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn tima0_cc_01_iter(&self) -> impl Iterator<Item = &Tima0Cc01> {
        self.tima0_cc_01.iter()
    }
    #[doc = "0x18..0x20 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn tima0_cc_23(&self, n: usize) -> &Tima0Cc23 {
        &self.tima0_cc_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn tima0_cc_23_iter(&self) -> impl Iterator<Item = &Tima0Cc23> {
        self.tima0_cc_23.iter()
    }
    #[doc = "0x20..0x28 - Compare Register 4 to Compare Register 5"]
    #[inline(always)]
    pub const fn tima0_cc_45(&self, n: usize) -> &Tima0Cc45 {
        &self.tima0_cc_45[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - Compare Register 4 to Compare Register 5"]
    #[inline(always)]
    pub fn tima0_cc_45_iter(&self) -> impl Iterator<Item = &Tima0Cc45> {
        self.tima0_cc_45.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn tima0_ccctl_01(&self, n: usize) -> &Tima0Ccctl01 {
        &self.tima0_ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn tima0_ccctl_01_iter(&self) -> impl Iterator<Item = &Tima0Ccctl01> {
        self.tima0_ccctl_01.iter()
    }
    #[doc = "0x38..0x40 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn tima0_ccctl_23(&self, n: usize) -> &Tima0Ccctl23 {
        &self.tima0_ccctl_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x40 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn tima0_ccctl_23_iter(&self) -> impl Iterator<Item = &Tima0Ccctl23> {
        self.tima0_ccctl_23.iter()
    }
    #[doc = "0x40..0x48 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn tima0_ccctl_45(&self, n: usize) -> &Tima0Ccctl45 {
        &self.tima0_ccctl_45[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn tima0_ccctl_45_iter(&self) -> impl Iterator<Item = &Tima0Ccctl45> {
        self.tima0_ccctl_45.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn tima0_octl_01(&self, n: usize) -> &Tima0Octl01 {
        &self.tima0_octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn tima0_octl_01_iter(&self) -> impl Iterator<Item = &Tima0Octl01> {
        self.tima0_octl_01.iter()
    }
    #[doc = "0x58..0x60 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn tima0_octl_23(&self, n: usize) -> &Tima0Octl23 {
        &self.tima0_octl_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x60 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn tima0_octl_23_iter(&self) -> impl Iterator<Item = &Tima0Octl23> {
        self.tima0_octl_23.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn tima0_ccact_01(&self, n: usize) -> &Tima0Ccact01 {
        &self.tima0_ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn tima0_ccact_01_iter(&self) -> impl Iterator<Item = &Tima0Ccact01> {
        self.tima0_ccact_01.iter()
    }
    #[doc = "0x78..0x80 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn tima0_ccact_23(&self, n: usize) -> &Tima0Ccact23 {
        &self.tima0_ccact_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x78..0x80 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn tima0_ccact_23_iter(&self) -> impl Iterator<Item = &Tima0Ccact23> {
        self.tima0_ccact_23.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn tima0_ifctl_01(&self, n: usize) -> &Tima0Ifctl01 {
        &self.tima0_ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register"]
    #[inline(always)]
    pub fn tima0_ifctl_01_iter(&self) -> impl Iterator<Item = &Tima0Ifctl01> {
        self.tima0_ifctl_01.iter()
    }
    #[doc = "0x88..0x90 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn tima0_ifctl_23(&self, n: usize) -> &Tima0Ifctl23 {
        &self.tima0_ifctl_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x90 - Input Filter Control Register"]
    #[inline(always)]
    pub fn tima0_ifctl_23_iter(&self) -> impl Iterator<Item = &Tima0Ifctl23> {
        self.tima0_ifctl_23.iter()
    }
    #[doc = "0xa0 - Counter Register"]
    #[inline(always)]
    pub const fn tima0_pl(&self) -> &Tima0Pl {
        &self.tima0_pl
    }
    #[doc = "0xa4 - Dead Band insertion control register"]
    #[inline(always)]
    pub const fn tima0_dbctl(&self) -> &Tima0Dbctl {
        &self.tima0_dbctl
    }
    #[doc = "0xb0 - Trigger Select"]
    #[inline(always)]
    pub const fn tima0_tsel(&self) -> &Tima0Tsel {
        &self.tima0_tsel
    }
    #[doc = "0xb4 - Repeat counter"]
    #[inline(always)]
    pub const fn tima0_rc(&self) -> &Tima0Rc {
        &self.tima0_rc
    }
    #[doc = "0xb8 - Repeat counter"]
    #[inline(always)]
    pub const fn tima0_rcld(&self) -> &Tima0Rcld {
        &self.tima0_rcld
    }
    #[doc = "0xd0 - Fault Control Register"]
    #[inline(always)]
    pub const fn tima0_fctl(&self) -> &Tima0Fctl {
        &self.tima0_fctl
    }
    #[doc = "0xd4 - Fault input Filter control register"]
    #[inline(always)]
    pub const fn tima0_fifctl(&self) -> &Tima0Fifctl {
        &self.tima0_fifctl
    }
}
#[doc = "TIMA0_CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ctr`] module"]
#[doc(alias = "TIMA0_CTR")]
pub type Tima0Ctr = crate::Reg<tima0_ctr::Tima0CtrSpec>;
#[doc = "Counter Register"]
pub mod tima0_ctr;
#[doc = "TIMA0_CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ctrctl`] module"]
#[doc(alias = "TIMA0_CTRCTL")]
pub type Tima0Ctrctl = crate::Reg<tima0_ctrctl::Tima0CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod tima0_ctrctl;
#[doc = "TIMA0_LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_load`] module"]
#[doc(alias = "TIMA0_LOAD")]
pub type Tima0Load = crate::Reg<tima0_load::Tima0LoadSpec>;
#[doc = "Load Register"]
pub mod tima0_load;
#[doc = "TIMA0_CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cc_01`] module"]
#[doc(alias = "TIMA0_CC_01")]
pub type Tima0Cc01 = crate::Reg<tima0_cc_01::Tima0Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod tima0_cc_01;
#[doc = "TIMA0_CC_23 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cc_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cc_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cc_23`] module"]
#[doc(alias = "TIMA0_CC_23")]
pub type Tima0Cc23 = crate::Reg<tima0_cc_23::Tima0Cc23Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod tima0_cc_23;
#[doc = "TIMA0_CC_45 (rw) register accessor: Compare Register 4 to Compare Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cc_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cc_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_cc_45`] module"]
#[doc(alias = "TIMA0_CC_45")]
pub type Tima0Cc45 = crate::Reg<tima0_cc_45::Tima0Cc45Spec>;
#[doc = "Compare Register 4 to Compare Register 5"]
pub mod tima0_cc_45;
#[doc = "TIMA0_CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ccctl_01`] module"]
#[doc(alias = "TIMA0_CCCTL_01")]
pub type Tima0Ccctl01 = crate::Reg<tima0_ccctl_01::Tima0Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod tima0_ccctl_01;
#[doc = "TIMA0_CCCTL_23 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccctl_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccctl_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ccctl_23`] module"]
#[doc(alias = "TIMA0_CCCTL_23")]
pub type Tima0Ccctl23 = crate::Reg<tima0_ccctl_23::Tima0Ccctl23Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod tima0_ccctl_23;
#[doc = "TIMA0_CCCTL_45 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccctl_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccctl_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ccctl_45`] module"]
#[doc(alias = "TIMA0_CCCTL_45")]
pub type Tima0Ccctl45 = crate::Reg<tima0_ccctl_45::Tima0Ccctl45Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod tima0_ccctl_45;
#[doc = "TIMA0_OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_octl_01`] module"]
#[doc(alias = "TIMA0_OCTL_01")]
pub type Tima0Octl01 = crate::Reg<tima0_octl_01::Tima0Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod tima0_octl_01;
#[doc = "TIMA0_OCTL_23 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_octl_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_octl_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_octl_23`] module"]
#[doc(alias = "TIMA0_OCTL_23")]
pub type Tima0Octl23 = crate::Reg<tima0_octl_23::Tima0Octl23Spec>;
#[doc = "CCP Output Control Registers"]
pub mod tima0_octl_23;
#[doc = "TIMA0_CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ccact_01`] module"]
#[doc(alias = "TIMA0_CCACT_01")]
pub type Tima0Ccact01 = crate::Reg<tima0_ccact_01::Tima0Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod tima0_ccact_01;
#[doc = "TIMA0_CCACT_23 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccact_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccact_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ccact_23`] module"]
#[doc(alias = "TIMA0_CCACT_23")]
pub type Tima0Ccact23 = crate::Reg<tima0_ccact_23::Tima0Ccact23Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod tima0_ccact_23;
#[doc = "TIMA0_IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ifctl_01`] module"]
#[doc(alias = "TIMA0_IFCTL_01")]
pub type Tima0Ifctl01 = crate::Reg<tima0_ifctl_01::Tima0Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod tima0_ifctl_01;
#[doc = "TIMA0_IFCTL_23 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ifctl_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ifctl_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_ifctl_23`] module"]
#[doc(alias = "TIMA0_IFCTL_23")]
pub type Tima0Ifctl23 = crate::Reg<tima0_ifctl_23::Tima0Ifctl23Spec>;
#[doc = "Input Filter Control Register"]
pub mod tima0_ifctl_23;
#[doc = "TIMA0_PL (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_pl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_pl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_pl`] module"]
#[doc(alias = "TIMA0_PL")]
pub type Tima0Pl = crate::Reg<tima0_pl::Tima0PlSpec>;
#[doc = "Counter Register"]
pub mod tima0_pl;
#[doc = "TIMA0_DBCTL (rw) register accessor: Dead Band insertion control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_dbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_dbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_dbctl`] module"]
#[doc(alias = "TIMA0_DBCTL")]
pub type Tima0Dbctl = crate::Reg<tima0_dbctl::Tima0DbctlSpec>;
#[doc = "Dead Band insertion control register"]
pub mod tima0_dbctl;
#[doc = "TIMA0_TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_tsel`] module"]
#[doc(alias = "TIMA0_TSEL")]
pub type Tima0Tsel = crate::Reg<tima0_tsel::Tima0TselSpec>;
#[doc = "Trigger Select"]
pub mod tima0_tsel;
#[doc = "TIMA0_RC (r) register accessor: Repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_rc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_rc`] module"]
#[doc(alias = "TIMA0_RC")]
pub type Tima0Rc = crate::Reg<tima0_rc::Tima0RcSpec>;
#[doc = "Repeat counter"]
pub mod tima0_rc;
#[doc = "TIMA0_RCLD (rw) register accessor: Repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_rcld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_rcld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_rcld`] module"]
#[doc(alias = "TIMA0_RCLD")]
pub type Tima0Rcld = crate::Reg<tima0_rcld::Tima0RcldSpec>;
#[doc = "Repeat counter"]
pub mod tima0_rcld;
#[doc = "TIMA0_FCTL (rw) register accessor: Fault Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fctl`] module"]
#[doc(alias = "TIMA0_FCTL")]
pub type Tima0Fctl = crate::Reg<tima0_fctl::Tima0FctlSpec>;
#[doc = "Fault Control Register"]
pub mod tima0_fctl;
#[doc = "TIMA0_FIFCTL (rw) register accessor: Fault input Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fifctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fifctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fifctl`] module"]
#[doc(alias = "TIMA0_FIFCTL")]
pub type Tima0Fifctl = crate::Reg<tima0_fifctl::Tima0FifctlSpec>;
#[doc = "Fault input Filter control register"]
pub mod tima0_fifctl;
