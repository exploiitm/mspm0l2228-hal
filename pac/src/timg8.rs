#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    timg8_fsub_0: Timg8Fsub0,
    timg8_fsub_1: Timg8Fsub1,
    _reserved2: [u8; 0x3c],
    timg8_fpub_0: Timg8Fpub0,
    timg8_fpub_1: Timg8Fpub1,
    _reserved4: [u8; 0x03b4],
    timg8_gprcm: [Timg8Gprcm; 1],
    _reserved5: [u8; 0x07e8],
    timg8_clkdiv: Timg8Clkdiv,
    _reserved6: [u8; 0x04],
    timg8_clksel: Timg8Clksel,
    _reserved7: [u8; 0x0c],
    timg8_pdbgctl: Timg8Pdbgctl,
    _reserved8: [u8; 0x04],
    timg8_int_event: [Timg8IntEvent; 3],
    _reserved9: [u8; 0x3c],
    timg8_evt_mode: Timg8EvtMode,
    _reserved10: [u8; 0x18],
    timg8_desc: Timg8Desc,
    timg8_commonregs: [Timg8Commonregs; 1],
    _reserved12: [u8; 0x06e0],
    timg8_counterregs: [Timg8Counterregs; 1],
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn timg8_fsub_0(&self) -> &Timg8Fsub0 {
        &self.timg8_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn timg8_fsub_1(&self) -> &Timg8Fsub1 {
        &self.timg8_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn timg8_fpub_0(&self) -> &Timg8Fpub0 {
        &self.timg8_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn timg8_fpub_1(&self) -> &Timg8Fpub1 {
        &self.timg8_fpub_1
    }
    #[doc = "0x800..0x818 - TIMG8_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn timg8_gprcm(&self, n: usize) -> &Timg8Gprcm {
        &self.timg8_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TIMG8_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn timg8_gprcm_iter(&self) -> impl Iterator<Item = &Timg8Gprcm> {
        self.timg8_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn timg8_clkdiv(&self) -> &Timg8Clkdiv {
        &self.timg8_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn timg8_clksel(&self) -> &Timg8Clksel {
        &self.timg8_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn timg8_pdbgctl(&self) -> &Timg8Pdbgctl {
        &self.timg8_pdbgctl
    }
    #[doc = "0x1020..0x10a4 - TIMG8_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn timg8_int_event(&self, n: usize) -> &Timg8IntEvent {
        &self.timg8_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x10a4 - TIMG8_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn timg8_int_event_iter(&self) -> impl Iterator<Item = &Timg8IntEvent> {
        self.timg8_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn timg8_evt_mode(&self) -> &Timg8EvtMode {
        &self.timg8_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn timg8_desc(&self) -> &Timg8Desc {
        &self.timg8_desc
    }
    #[doc = "0x1100..0x1120 - TIMG8_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg8_commonregs(&self, n: usize) -> &Timg8Commonregs {
        &self.timg8_commonregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1120 - TIMG8_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg8_commonregs_iter(&self) -> impl Iterator<Item = &Timg8Commonregs> {
        self.timg8_commonregs.iter()
    }
    #[doc = "0x1800..0x18c0 - TIMG8_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg8_counterregs(&self, n: usize) -> &Timg8Counterregs {
        &self.timg8_counterregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x18c0 - TIMG8_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg8_counterregs_iter(&self) -> impl Iterator<Item = &Timg8Counterregs> {
        self.timg8_counterregs.iter()
    }
}
#[doc = "TIMG8_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_fsub_0`] module"]
#[doc(alias = "TIMG8_FSUB_0")]
pub type Timg8Fsub0 = crate::Reg<timg8_fsub_0::Timg8Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod timg8_fsub_0;
#[doc = "TIMG8_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_fsub_1`] module"]
#[doc(alias = "TIMG8_FSUB_1")]
pub type Timg8Fsub1 = crate::Reg<timg8_fsub_1::Timg8Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod timg8_fsub_1;
#[doc = "TIMG8_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_fpub_0`] module"]
#[doc(alias = "TIMG8_FPUB_0")]
pub type Timg8Fpub0 = crate::Reg<timg8_fpub_0::Timg8Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod timg8_fpub_0;
#[doc = "TIMG8_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_fpub_1`] module"]
#[doc(alias = "TIMG8_FPUB_1")]
pub type Timg8Fpub1 = crate::Reg<timg8_fpub_1::Timg8Fpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod timg8_fpub_1;
#[doc = "TIMG8_GPRCM\\[%s\\]"]
pub use self::timg8_gprcm::Timg8Gprcm;
#[doc = r"Cluster"]
#[doc = "TIMG8_GPRCM\\[%s\\]"]
pub mod timg8_gprcm;
#[doc = "TIMG8_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_clkdiv`] module"]
#[doc(alias = "TIMG8_CLKDIV")]
pub type Timg8Clkdiv = crate::Reg<timg8_clkdiv::Timg8ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod timg8_clkdiv;
#[doc = "TIMG8_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_clksel`] module"]
#[doc(alias = "TIMG8_CLKSEL")]
pub type Timg8Clksel = crate::Reg<timg8_clksel::Timg8ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod timg8_clksel;
#[doc = "TIMG8_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_pdbgctl`] module"]
#[doc(alias = "TIMG8_PDBGCTL")]
pub type Timg8Pdbgctl = crate::Reg<timg8_pdbgctl::Timg8PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod timg8_pdbgctl;
#[doc = "TIMG8_INT_EVENT\\[%s\\]"]
pub use self::timg8_int_event::Timg8IntEvent;
#[doc = r"Cluster"]
#[doc = "TIMG8_INT_EVENT\\[%s\\]"]
pub mod timg8_int_event;
#[doc = "TIMG8_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_evt_mode`] module"]
#[doc(alias = "TIMG8_EVT_MODE")]
pub type Timg8EvtMode = crate::Reg<timg8_evt_mode::Timg8EvtModeSpec>;
#[doc = "Event Mode"]
pub mod timg8_evt_mode;
#[doc = "TIMG8_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg8_desc`] module"]
#[doc(alias = "TIMG8_DESC")]
pub type Timg8Desc = crate::Reg<timg8_desc::Timg8DescSpec>;
#[doc = "Module Description"]
pub mod timg8_desc;
#[doc = "TIMG8_COMMONREGS\\[%s\\]"]
pub use self::timg8_commonregs::Timg8Commonregs;
#[doc = r"Cluster"]
#[doc = "TIMG8_COMMONREGS\\[%s\\]"]
pub mod timg8_commonregs;
#[doc = "TIMG8_COUNTERREGS\\[%s\\]"]
pub use self::timg8_counterregs::Timg8Counterregs;
#[doc = r"Cluster"]
#[doc = "TIMG8_COUNTERREGS\\[%s\\]"]
pub mod timg8_counterregs;
