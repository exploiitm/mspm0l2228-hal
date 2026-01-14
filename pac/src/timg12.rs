#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    timg12_fsub_0: Timg12Fsub0,
    timg12_fsub_1: Timg12Fsub1,
    _reserved2: [u8; 0x3c],
    timg12_fpub_0: Timg12Fpub0,
    timg12_fpub_1: Timg12Fpub1,
    _reserved4: [u8; 0x03b4],
    timg12_gprcm: [Timg12Gprcm; 1],
    _reserved5: [u8; 0x07e8],
    timg12_clkdiv: Timg12Clkdiv,
    _reserved6: [u8; 0x04],
    timg12_clksel: Timg12Clksel,
    _reserved7: [u8; 0x0c],
    timg12_pdbgctl: Timg12Pdbgctl,
    _reserved8: [u8; 0x04],
    timg12_int_event: [Timg12IntEvent; 3],
    _reserved9: [u8; 0x3c],
    timg12_evt_mode: Timg12EvtMode,
    _reserved10: [u8; 0x18],
    timg12_desc: Timg12Desc,
    timg12_commonregs: [Timg12Commonregs; 1],
    _reserved12: [u8; 0x06e0],
    timg12_counterregs: [Timg12Counterregs; 1],
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn timg12_fsub_0(&self) -> &Timg12Fsub0 {
        &self.timg12_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn timg12_fsub_1(&self) -> &Timg12Fsub1 {
        &self.timg12_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn timg12_fpub_0(&self) -> &Timg12Fpub0 {
        &self.timg12_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn timg12_fpub_1(&self) -> &Timg12Fpub1 {
        &self.timg12_fpub_1
    }
    #[doc = "0x800..0x818 - TIMG12_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn timg12_gprcm(&self, n: usize) -> &Timg12Gprcm {
        &self.timg12_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TIMG12_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn timg12_gprcm_iter(&self) -> impl Iterator<Item = &Timg12Gprcm> {
        self.timg12_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn timg12_clkdiv(&self) -> &Timg12Clkdiv {
        &self.timg12_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn timg12_clksel(&self) -> &Timg12Clksel {
        &self.timg12_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn timg12_pdbgctl(&self) -> &Timg12Pdbgctl {
        &self.timg12_pdbgctl
    }
    #[doc = "0x1020..0x10a4 - TIMG12_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn timg12_int_event(&self, n: usize) -> &Timg12IntEvent {
        &self.timg12_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x10a4 - TIMG12_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn timg12_int_event_iter(&self) -> impl Iterator<Item = &Timg12IntEvent> {
        self.timg12_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn timg12_evt_mode(&self) -> &Timg12EvtMode {
        &self.timg12_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn timg12_desc(&self) -> &Timg12Desc {
        &self.timg12_desc
    }
    #[doc = "0x1100..0x1120 - TIMG12_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg12_commonregs(&self, n: usize) -> &Timg12Commonregs {
        &self.timg12_commonregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1120 - TIMG12_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg12_commonregs_iter(&self) -> impl Iterator<Item = &Timg12Commonregs> {
        self.timg12_commonregs.iter()
    }
    #[doc = "0x1800..0x18b4 - TIMG12_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg12_counterregs(&self, n: usize) -> &Timg12Counterregs {
        &self.timg12_counterregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x18b4 - TIMG12_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg12_counterregs_iter(&self) -> impl Iterator<Item = &Timg12Counterregs> {
        self.timg12_counterregs.iter()
    }
}
#[doc = "TIMG12_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_fsub_0`] module"]
#[doc(alias = "TIMG12_FSUB_0")]
pub type Timg12Fsub0 = crate::Reg<timg12_fsub_0::Timg12Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod timg12_fsub_0;
#[doc = "TIMG12_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_fsub_1`] module"]
#[doc(alias = "TIMG12_FSUB_1")]
pub type Timg12Fsub1 = crate::Reg<timg12_fsub_1::Timg12Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod timg12_fsub_1;
#[doc = "TIMG12_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_fpub_0`] module"]
#[doc(alias = "TIMG12_FPUB_0")]
pub type Timg12Fpub0 = crate::Reg<timg12_fpub_0::Timg12Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod timg12_fpub_0;
#[doc = "TIMG12_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_fpub_1`] module"]
#[doc(alias = "TIMG12_FPUB_1")]
pub type Timg12Fpub1 = crate::Reg<timg12_fpub_1::Timg12Fpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod timg12_fpub_1;
#[doc = "TIMG12_GPRCM\\[%s\\]"]
pub use self::timg12_gprcm::Timg12Gprcm;
#[doc = r"Cluster"]
#[doc = "TIMG12_GPRCM\\[%s\\]"]
pub mod timg12_gprcm;
#[doc = "TIMG12_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_clkdiv`] module"]
#[doc(alias = "TIMG12_CLKDIV")]
pub type Timg12Clkdiv = crate::Reg<timg12_clkdiv::Timg12ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod timg12_clkdiv;
#[doc = "TIMG12_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_clksel`] module"]
#[doc(alias = "TIMG12_CLKSEL")]
pub type Timg12Clksel = crate::Reg<timg12_clksel::Timg12ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod timg12_clksel;
#[doc = "TIMG12_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_pdbgctl`] module"]
#[doc(alias = "TIMG12_PDBGCTL")]
pub type Timg12Pdbgctl = crate::Reg<timg12_pdbgctl::Timg12PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod timg12_pdbgctl;
#[doc = "TIMG12_INT_EVENT\\[%s\\]"]
pub use self::timg12_int_event::Timg12IntEvent;
#[doc = r"Cluster"]
#[doc = "TIMG12_INT_EVENT\\[%s\\]"]
pub mod timg12_int_event;
#[doc = "TIMG12_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_evt_mode`] module"]
#[doc(alias = "TIMG12_EVT_MODE")]
pub type Timg12EvtMode = crate::Reg<timg12_evt_mode::Timg12EvtModeSpec>;
#[doc = "Event Mode"]
pub mod timg12_evt_mode;
#[doc = "TIMG12_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg12_desc`] module"]
#[doc(alias = "TIMG12_DESC")]
pub type Timg12Desc = crate::Reg<timg12_desc::Timg12DescSpec>;
#[doc = "Module Description"]
pub mod timg12_desc;
#[doc = "TIMG12_COMMONREGS\\[%s\\]"]
pub use self::timg12_commonregs::Timg12Commonregs;
#[doc = r"Cluster"]
#[doc = "TIMG12_COMMONREGS\\[%s\\]"]
pub mod timg12_commonregs;
#[doc = "TIMG12_COUNTERREGS\\[%s\\]"]
pub use self::timg12_counterregs::Timg12Counterregs;
#[doc = r"Cluster"]
#[doc = "TIMG12_COUNTERREGS\\[%s\\]"]
pub mod timg12_counterregs;
