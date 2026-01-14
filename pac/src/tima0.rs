#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    tima0_fsub_0: Tima0Fsub0,
    tima0_fsub_1: Tima0Fsub1,
    _reserved2: [u8; 0x3c],
    tima0_fpub_0: Tima0Fpub0,
    tima0_fpub_1: Tima0Fpub1,
    _reserved4: [u8; 0x03b4],
    tima0_gprcm: [Tima0Gprcm; 1],
    _reserved5: [u8; 0x07e8],
    tima0_clkdiv: Tima0Clkdiv,
    _reserved6: [u8; 0x04],
    tima0_clksel: Tima0Clksel,
    _reserved7: [u8; 0x0c],
    tima0_pdbgctl: Tima0Pdbgctl,
    _reserved8: [u8; 0x04],
    tima0_int_event: [Tima0IntEvent; 3],
    _reserved9: [u8; 0x3c],
    tima0_evt_mode: Tima0EvtMode,
    _reserved10: [u8; 0x18],
    tima0_desc: Tima0Desc,
    tima0_commonregs: [Tima0Commonregs; 1],
    _reserved12: [u8; 0x06d8],
    tima0_counterregs: [Tima0Counterregs; 1],
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn tima0_fsub_0(&self) -> &Tima0Fsub0 {
        &self.tima0_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn tima0_fsub_1(&self) -> &Tima0Fsub1 {
        &self.tima0_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn tima0_fpub_0(&self) -> &Tima0Fpub0 {
        &self.tima0_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn tima0_fpub_1(&self) -> &Tima0Fpub1 {
        &self.tima0_fpub_1
    }
    #[doc = "0x800..0x818 - TIMA0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn tima0_gprcm(&self, n: usize) -> &Tima0Gprcm {
        &self.tima0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TIMA0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn tima0_gprcm_iter(&self) -> impl Iterator<Item = &Tima0Gprcm> {
        self.tima0_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn tima0_clkdiv(&self) -> &Tima0Clkdiv {
        &self.tima0_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn tima0_clksel(&self) -> &Tima0Clksel {
        &self.tima0_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn tima0_pdbgctl(&self) -> &Tima0Pdbgctl {
        &self.tima0_pdbgctl
    }
    #[doc = "0x1020..0x10a4 - TIMA0_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn tima0_int_event(&self, n: usize) -> &Tima0IntEvent {
        &self.tima0_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x10a4 - TIMA0_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn tima0_int_event_iter(&self) -> impl Iterator<Item = &Tima0IntEvent> {
        self.tima0_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn tima0_evt_mode(&self) -> &Tima0EvtMode {
        &self.tima0_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn tima0_desc(&self) -> &Tima0Desc {
        &self.tima0_desc
    }
    #[doc = "0x1100..0x1128 - TIMA0_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn tima0_commonregs(&self, n: usize) -> &Tima0Commonregs {
        &self.tima0_commonregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1128 - TIMA0_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub fn tima0_commonregs_iter(&self) -> impl Iterator<Item = &Tima0Commonregs> {
        self.tima0_commonregs.iter()
    }
    #[doc = "0x1800..0x18d8 - TIMA0_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn tima0_counterregs(&self, n: usize) -> &Tima0Counterregs {
        &self.tima0_counterregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x18d8 - TIMA0_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub fn tima0_counterregs_iter(&self) -> impl Iterator<Item = &Tima0Counterregs> {
        self.tima0_counterregs.iter()
    }
}
#[doc = "TIMA0_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fsub_0`] module"]
#[doc(alias = "TIMA0_FSUB_0")]
pub type Tima0Fsub0 = crate::Reg<tima0_fsub_0::Tima0Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod tima0_fsub_0;
#[doc = "TIMA0_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fsub_1`] module"]
#[doc(alias = "TIMA0_FSUB_1")]
pub type Tima0Fsub1 = crate::Reg<tima0_fsub_1::Tima0Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod tima0_fsub_1;
#[doc = "TIMA0_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fpub_0`] module"]
#[doc(alias = "TIMA0_FPUB_0")]
pub type Tima0Fpub0 = crate::Reg<tima0_fpub_0::Tima0Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod tima0_fpub_0;
#[doc = "TIMA0_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_fpub_1`] module"]
#[doc(alias = "TIMA0_FPUB_1")]
pub type Tima0Fpub1 = crate::Reg<tima0_fpub_1::Tima0Fpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod tima0_fpub_1;
#[doc = "TIMA0_GPRCM\\[%s\\]"]
pub use self::tima0_gprcm::Tima0Gprcm;
#[doc = r"Cluster"]
#[doc = "TIMA0_GPRCM\\[%s\\]"]
pub mod tima0_gprcm;
#[doc = "TIMA0_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_clkdiv`] module"]
#[doc(alias = "TIMA0_CLKDIV")]
pub type Tima0Clkdiv = crate::Reg<tima0_clkdiv::Tima0ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod tima0_clkdiv;
#[doc = "TIMA0_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_clksel`] module"]
#[doc(alias = "TIMA0_CLKSEL")]
pub type Tima0Clksel = crate::Reg<tima0_clksel::Tima0ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod tima0_clksel;
#[doc = "TIMA0_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_pdbgctl`] module"]
#[doc(alias = "TIMA0_PDBGCTL")]
pub type Tima0Pdbgctl = crate::Reg<tima0_pdbgctl::Tima0PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod tima0_pdbgctl;
#[doc = "TIMA0_INT_EVENT\\[%s\\]"]
pub use self::tima0_int_event::Tima0IntEvent;
#[doc = r"Cluster"]
#[doc = "TIMA0_INT_EVENT\\[%s\\]"]
pub mod tima0_int_event;
#[doc = "TIMA0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_evt_mode`] module"]
#[doc(alias = "TIMA0_EVT_MODE")]
pub type Tima0EvtMode = crate::Reg<tima0_evt_mode::Tima0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod tima0_evt_mode;
#[doc = "TIMA0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tima0_desc`] module"]
#[doc(alias = "TIMA0_DESC")]
pub type Tima0Desc = crate::Reg<tima0_desc::Tima0DescSpec>;
#[doc = "Module Description"]
pub mod tima0_desc;
#[doc = "TIMA0_COMMONREGS\\[%s\\]"]
pub use self::tima0_commonregs::Tima0Commonregs;
#[doc = r"Cluster"]
#[doc = "TIMA0_COMMONREGS\\[%s\\]"]
pub mod tima0_commonregs;
#[doc = "TIMA0_COUNTERREGS\\[%s\\]"]
pub use self::tima0_counterregs::Tima0Counterregs;
#[doc = r"Cluster"]
#[doc = "TIMA0_COUNTERREGS\\[%s\\]"]
pub mod tima0_counterregs;
