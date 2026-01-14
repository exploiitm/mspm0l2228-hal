#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    timg0_fsub_0: Timg0Fsub0,
    timg0_fsub_1: Timg0Fsub1,
    _reserved2: [u8; 0x3c],
    timg0_fpub_0: Timg0Fpub0,
    timg0_fpub_1: Timg0Fpub1,
    _reserved4: [u8; 0x03b4],
    timg0_gprcm: [Timg0Gprcm; 1],
    _reserved5: [u8; 0x07e8],
    timg0_clkdiv: Timg0Clkdiv,
    _reserved6: [u8; 0x04],
    timg0_clksel: Timg0Clksel,
    _reserved7: [u8; 0x0c],
    timg0_pdbgctl: Timg0Pdbgctl,
    _reserved8: [u8; 0x04],
    timg0_int_event: [Timg0IntEvent; 3],
    _reserved9: [u8; 0x3c],
    timg0_evt_mode: Timg0EvtMode,
    _reserved10: [u8; 0x18],
    timg0_desc: Timg0Desc,
    timg0_commonregs: [Timg0Commonregs; 1],
    _reserved12: [u8; 0x06e0],
    timg0_counterregs: [Timg0Counterregs; 1],
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn timg0_fsub_0(&self) -> &Timg0Fsub0 {
        &self.timg0_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn timg0_fsub_1(&self) -> &Timg0Fsub1 {
        &self.timg0_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn timg0_fpub_0(&self) -> &Timg0Fpub0 {
        &self.timg0_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn timg0_fpub_1(&self) -> &Timg0Fpub1 {
        &self.timg0_fpub_1
    }
    #[doc = "0x800..0x818 - TIMG0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn timg0_gprcm(&self, n: usize) -> &Timg0Gprcm {
        &self.timg0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TIMG0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn timg0_gprcm_iter(&self) -> impl Iterator<Item = &Timg0Gprcm> {
        self.timg0_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn timg0_clkdiv(&self) -> &Timg0Clkdiv {
        &self.timg0_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn timg0_clksel(&self) -> &Timg0Clksel {
        &self.timg0_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn timg0_pdbgctl(&self) -> &Timg0Pdbgctl {
        &self.timg0_pdbgctl
    }
    #[doc = "0x1020..0x10a4 - TIMG0_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn timg0_int_event(&self, n: usize) -> &Timg0IntEvent {
        &self.timg0_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x10a4 - TIMG0_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn timg0_int_event_iter(&self) -> impl Iterator<Item = &Timg0IntEvent> {
        self.timg0_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn timg0_evt_mode(&self) -> &Timg0EvtMode {
        &self.timg0_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn timg0_desc(&self) -> &Timg0Desc {
        &self.timg0_desc
    }
    #[doc = "0x1100..0x1120 - TIMG0_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg0_commonregs(&self, n: usize) -> &Timg0Commonregs {
        &self.timg0_commonregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1120 - TIMG0_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg0_commonregs_iter(&self) -> impl Iterator<Item = &Timg0Commonregs> {
        self.timg0_commonregs.iter()
    }
    #[doc = "0x1800..0x18b4 - TIMG0_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg0_counterregs(&self, n: usize) -> &Timg0Counterregs {
        &self.timg0_counterregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x18b4 - TIMG0_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg0_counterregs_iter(&self) -> impl Iterator<Item = &Timg0Counterregs> {
        self.timg0_counterregs.iter()
    }
}
#[doc = "TIMG0_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_fsub_0`] module"]
#[doc(alias = "TIMG0_FSUB_0")]
pub type Timg0Fsub0 = crate::Reg<timg0_fsub_0::Timg0Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod timg0_fsub_0;
#[doc = "TIMG0_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_fsub_1`] module"]
#[doc(alias = "TIMG0_FSUB_1")]
pub type Timg0Fsub1 = crate::Reg<timg0_fsub_1::Timg0Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod timg0_fsub_1;
#[doc = "TIMG0_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_fpub_0`] module"]
#[doc(alias = "TIMG0_FPUB_0")]
pub type Timg0Fpub0 = crate::Reg<timg0_fpub_0::Timg0Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod timg0_fpub_0;
#[doc = "TIMG0_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_fpub_1`] module"]
#[doc(alias = "TIMG0_FPUB_1")]
pub type Timg0Fpub1 = crate::Reg<timg0_fpub_1::Timg0Fpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod timg0_fpub_1;
#[doc = "TIMG0_GPRCM\\[%s\\]"]
pub use self::timg0_gprcm::Timg0Gprcm;
#[doc = r"Cluster"]
#[doc = "TIMG0_GPRCM\\[%s\\]"]
pub mod timg0_gprcm;
#[doc = "TIMG0_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_clkdiv`] module"]
#[doc(alias = "TIMG0_CLKDIV")]
pub type Timg0Clkdiv = crate::Reg<timg0_clkdiv::Timg0ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod timg0_clkdiv;
#[doc = "TIMG0_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_clksel`] module"]
#[doc(alias = "TIMG0_CLKSEL")]
pub type Timg0Clksel = crate::Reg<timg0_clksel::Timg0ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod timg0_clksel;
#[doc = "TIMG0_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_pdbgctl`] module"]
#[doc(alias = "TIMG0_PDBGCTL")]
pub type Timg0Pdbgctl = crate::Reg<timg0_pdbgctl::Timg0PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod timg0_pdbgctl;
#[doc = "TIMG0_INT_EVENT\\[%s\\]"]
pub use self::timg0_int_event::Timg0IntEvent;
#[doc = r"Cluster"]
#[doc = "TIMG0_INT_EVENT\\[%s\\]"]
pub mod timg0_int_event;
#[doc = "TIMG0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_evt_mode`] module"]
#[doc(alias = "TIMG0_EVT_MODE")]
pub type Timg0EvtMode = crate::Reg<timg0_evt_mode::Timg0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod timg0_evt_mode;
#[doc = "TIMG0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg0_desc`] module"]
#[doc(alias = "TIMG0_DESC")]
pub type Timg0Desc = crate::Reg<timg0_desc::Timg0DescSpec>;
#[doc = "Module Description"]
pub mod timg0_desc;
#[doc = "TIMG0_COMMONREGS\\[%s\\]"]
pub use self::timg0_commonregs::Timg0Commonregs;
#[doc = r"Cluster"]
#[doc = "TIMG0_COMMONREGS\\[%s\\]"]
pub mod timg0_commonregs;
#[doc = "TIMG0_COUNTERREGS\\[%s\\]"]
pub use self::timg0_counterregs::Timg0Counterregs;
#[doc = r"Cluster"]
#[doc = "TIMG0_COUNTERREGS\\[%s\\]"]
pub mod timg0_counterregs;
