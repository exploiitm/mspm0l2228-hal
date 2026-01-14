#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    timg5_fsub_0: Timg5Fsub0,
    timg5_fsub_1: Timg5Fsub1,
    _reserved2: [u8; 0x3c],
    timg5_fpub_0: Timg5Fpub0,
    timg5_fpub_1: Timg5Fpub1,
    _reserved4: [u8; 0x03b4],
    timg5_gprcm: [Timg5Gprcm; 1],
    _reserved5: [u8; 0x07e8],
    timg5_clkdiv: Timg5Clkdiv,
    _reserved6: [u8; 0x04],
    timg5_clksel: Timg5Clksel,
    _reserved7: [u8; 0x0c],
    timg5_pdbgctl: Timg5Pdbgctl,
    _reserved8: [u8; 0x04],
    timg5_int_event: [Timg5IntEvent; 3],
    _reserved9: [u8; 0x3c],
    timg5_evt_mode: Timg5EvtMode,
    _reserved10: [u8; 0x18],
    timg5_desc: Timg5Desc,
    timg5_commonregs: [Timg5Commonregs; 1],
    _reserved12: [u8; 0x06d8],
    timg5_counterregs: [Timg5Counterregs; 1],
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn timg5_fsub_0(&self) -> &Timg5Fsub0 {
        &self.timg5_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn timg5_fsub_1(&self) -> &Timg5Fsub1 {
        &self.timg5_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn timg5_fpub_0(&self) -> &Timg5Fpub0 {
        &self.timg5_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn timg5_fpub_1(&self) -> &Timg5Fpub1 {
        &self.timg5_fpub_1
    }
    #[doc = "0x800..0x818 - TIMG5_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn timg5_gprcm(&self, n: usize) -> &Timg5Gprcm {
        &self.timg5_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TIMG5_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn timg5_gprcm_iter(&self) -> impl Iterator<Item = &Timg5Gprcm> {
        self.timg5_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn timg5_clkdiv(&self) -> &Timg5Clkdiv {
        &self.timg5_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn timg5_clksel(&self) -> &Timg5Clksel {
        &self.timg5_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn timg5_pdbgctl(&self) -> &Timg5Pdbgctl {
        &self.timg5_pdbgctl
    }
    #[doc = "0x1020..0x10a4 - TIMG5_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn timg5_int_event(&self, n: usize) -> &Timg5IntEvent {
        &self.timg5_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x10a4 - TIMG5_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn timg5_int_event_iter(&self) -> impl Iterator<Item = &Timg5IntEvent> {
        self.timg5_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn timg5_evt_mode(&self) -> &Timg5EvtMode {
        &self.timg5_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn timg5_desc(&self) -> &Timg5Desc {
        &self.timg5_desc
    }
    #[doc = "0x1100..0x1128 - TIMG5_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg5_commonregs(&self, n: usize) -> &Timg5Commonregs {
        &self.timg5_commonregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1128 - TIMG5_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg5_commonregs_iter(&self) -> impl Iterator<Item = &Timg5Commonregs> {
        self.timg5_commonregs.iter()
    }
    #[doc = "0x1800..0x18b4 - TIMG5_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg5_counterregs(&self, n: usize) -> &Timg5Counterregs {
        &self.timg5_counterregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x18b4 - TIMG5_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg5_counterregs_iter(&self) -> impl Iterator<Item = &Timg5Counterregs> {
        self.timg5_counterregs.iter()
    }
}
#[doc = "TIMG5_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_fsub_0`] module"]
#[doc(alias = "TIMG5_FSUB_0")]
pub type Timg5Fsub0 = crate::Reg<timg5_fsub_0::Timg5Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod timg5_fsub_0;
#[doc = "TIMG5_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_fsub_1`] module"]
#[doc(alias = "TIMG5_FSUB_1")]
pub type Timg5Fsub1 = crate::Reg<timg5_fsub_1::Timg5Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod timg5_fsub_1;
#[doc = "TIMG5_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_fpub_0`] module"]
#[doc(alias = "TIMG5_FPUB_0")]
pub type Timg5Fpub0 = crate::Reg<timg5_fpub_0::Timg5Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod timg5_fpub_0;
#[doc = "TIMG5_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_fpub_1`] module"]
#[doc(alias = "TIMG5_FPUB_1")]
pub type Timg5Fpub1 = crate::Reg<timg5_fpub_1::Timg5Fpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod timg5_fpub_1;
#[doc = "TIMG5_GPRCM\\[%s\\]"]
pub use self::timg5_gprcm::Timg5Gprcm;
#[doc = r"Cluster"]
#[doc = "TIMG5_GPRCM\\[%s\\]"]
pub mod timg5_gprcm;
#[doc = "TIMG5_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_clkdiv`] module"]
#[doc(alias = "TIMG5_CLKDIV")]
pub type Timg5Clkdiv = crate::Reg<timg5_clkdiv::Timg5ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod timg5_clkdiv;
#[doc = "TIMG5_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_clksel`] module"]
#[doc(alias = "TIMG5_CLKSEL")]
pub type Timg5Clksel = crate::Reg<timg5_clksel::Timg5ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod timg5_clksel;
#[doc = "TIMG5_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_pdbgctl`] module"]
#[doc(alias = "TIMG5_PDBGCTL")]
pub type Timg5Pdbgctl = crate::Reg<timg5_pdbgctl::Timg5PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod timg5_pdbgctl;
#[doc = "TIMG5_INT_EVENT\\[%s\\]"]
pub use self::timg5_int_event::Timg5IntEvent;
#[doc = r"Cluster"]
#[doc = "TIMG5_INT_EVENT\\[%s\\]"]
pub mod timg5_int_event;
#[doc = "TIMG5_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_evt_mode`] module"]
#[doc(alias = "TIMG5_EVT_MODE")]
pub type Timg5EvtMode = crate::Reg<timg5_evt_mode::Timg5EvtModeSpec>;
#[doc = "Event Mode"]
pub mod timg5_evt_mode;
#[doc = "TIMG5_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg5_desc`] module"]
#[doc(alias = "TIMG5_DESC")]
pub type Timg5Desc = crate::Reg<timg5_desc::Timg5DescSpec>;
#[doc = "Module Description"]
pub mod timg5_desc;
#[doc = "TIMG5_COMMONREGS\\[%s\\]"]
pub use self::timg5_commonregs::Timg5Commonregs;
#[doc = r"Cluster"]
#[doc = "TIMG5_COMMONREGS\\[%s\\]"]
pub mod timg5_commonregs;
#[doc = "TIMG5_COUNTERREGS\\[%s\\]"]
pub use self::timg5_counterregs::Timg5Counterregs;
#[doc = r"Cluster"]
#[doc = "TIMG5_COUNTERREGS\\[%s\\]"]
pub mod timg5_counterregs;
