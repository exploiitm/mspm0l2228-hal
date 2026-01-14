#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    timg4_fsub_0: Timg4Fsub0,
    timg4_fsub_1: Timg4Fsub1,
    _reserved2: [u8; 0x3c],
    timg4_fpub_0: Timg4Fpub0,
    timg4_fpub_1: Timg4Fpub1,
    _reserved4: [u8; 0x03b4],
    timg4_gprcm: [Timg4Gprcm; 1],
    _reserved5: [u8; 0x07e8],
    timg4_clkdiv: Timg4Clkdiv,
    _reserved6: [u8; 0x04],
    timg4_clksel: Timg4Clksel,
    _reserved7: [u8; 0x0c],
    timg4_pdbgctl: Timg4Pdbgctl,
    _reserved8: [u8; 0x04],
    timg4_int_event: [Timg4IntEvent; 3],
    _reserved9: [u8; 0x3c],
    timg4_evt_mode: Timg4EvtMode,
    _reserved10: [u8; 0x18],
    timg4_desc: Timg4Desc,
    timg4_commonregs: [Timg4Commonregs; 1],
    _reserved12: [u8; 0x06d8],
    timg4_counterregs: [Timg4Counterregs; 1],
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn timg4_fsub_0(&self) -> &Timg4Fsub0 {
        &self.timg4_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn timg4_fsub_1(&self) -> &Timg4Fsub1 {
        &self.timg4_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn timg4_fpub_0(&self) -> &Timg4Fpub0 {
        &self.timg4_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn timg4_fpub_1(&self) -> &Timg4Fpub1 {
        &self.timg4_fpub_1
    }
    #[doc = "0x800..0x818 - TIMG4_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn timg4_gprcm(&self, n: usize) -> &Timg4Gprcm {
        &self.timg4_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TIMG4_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn timg4_gprcm_iter(&self) -> impl Iterator<Item = &Timg4Gprcm> {
        self.timg4_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn timg4_clkdiv(&self) -> &Timg4Clkdiv {
        &self.timg4_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn timg4_clksel(&self) -> &Timg4Clksel {
        &self.timg4_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn timg4_pdbgctl(&self) -> &Timg4Pdbgctl {
        &self.timg4_pdbgctl
    }
    #[doc = "0x1020..0x10a4 - TIMG4_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn timg4_int_event(&self, n: usize) -> &Timg4IntEvent {
        &self.timg4_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x10a4 - TIMG4_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn timg4_int_event_iter(&self) -> impl Iterator<Item = &Timg4IntEvent> {
        self.timg4_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn timg4_evt_mode(&self) -> &Timg4EvtMode {
        &self.timg4_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn timg4_desc(&self) -> &Timg4Desc {
        &self.timg4_desc
    }
    #[doc = "0x1100..0x1128 - TIMG4_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg4_commonregs(&self, n: usize) -> &Timg4Commonregs {
        &self.timg4_commonregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1128 - TIMG4_COMMONREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg4_commonregs_iter(&self) -> impl Iterator<Item = &Timg4Commonregs> {
        self.timg4_commonregs.iter()
    }
    #[doc = "0x1800..0x18b4 - TIMG4_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub const fn timg4_counterregs(&self, n: usize) -> &Timg4Counterregs {
        &self.timg4_counterregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x18b4 - TIMG4_COUNTERREGS\\[%s\\]"]
    #[inline(always)]
    pub fn timg4_counterregs_iter(&self) -> impl Iterator<Item = &Timg4Counterregs> {
        self.timg4_counterregs.iter()
    }
}
#[doc = "TIMG4_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_fsub_0`] module"]
#[doc(alias = "TIMG4_FSUB_0")]
pub type Timg4Fsub0 = crate::Reg<timg4_fsub_0::Timg4Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod timg4_fsub_0;
#[doc = "TIMG4_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_fsub_1`] module"]
#[doc(alias = "TIMG4_FSUB_1")]
pub type Timg4Fsub1 = crate::Reg<timg4_fsub_1::Timg4Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod timg4_fsub_1;
#[doc = "TIMG4_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_fpub_0`] module"]
#[doc(alias = "TIMG4_FPUB_0")]
pub type Timg4Fpub0 = crate::Reg<timg4_fpub_0::Timg4Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod timg4_fpub_0;
#[doc = "TIMG4_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_fpub_1`] module"]
#[doc(alias = "TIMG4_FPUB_1")]
pub type Timg4Fpub1 = crate::Reg<timg4_fpub_1::Timg4Fpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod timg4_fpub_1;
#[doc = "TIMG4_GPRCM\\[%s\\]"]
pub use self::timg4_gprcm::Timg4Gprcm;
#[doc = r"Cluster"]
#[doc = "TIMG4_GPRCM\\[%s\\]"]
pub mod timg4_gprcm;
#[doc = "TIMG4_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_clkdiv`] module"]
#[doc(alias = "TIMG4_CLKDIV")]
pub type Timg4Clkdiv = crate::Reg<timg4_clkdiv::Timg4ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod timg4_clkdiv;
#[doc = "TIMG4_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_clksel`] module"]
#[doc(alias = "TIMG4_CLKSEL")]
pub type Timg4Clksel = crate::Reg<timg4_clksel::Timg4ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod timg4_clksel;
#[doc = "TIMG4_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_pdbgctl`] module"]
#[doc(alias = "TIMG4_PDBGCTL")]
pub type Timg4Pdbgctl = crate::Reg<timg4_pdbgctl::Timg4PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod timg4_pdbgctl;
#[doc = "TIMG4_INT_EVENT\\[%s\\]"]
pub use self::timg4_int_event::Timg4IntEvent;
#[doc = r"Cluster"]
#[doc = "TIMG4_INT_EVENT\\[%s\\]"]
pub mod timg4_int_event;
#[doc = "TIMG4_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_evt_mode`] module"]
#[doc(alias = "TIMG4_EVT_MODE")]
pub type Timg4EvtMode = crate::Reg<timg4_evt_mode::Timg4EvtModeSpec>;
#[doc = "Event Mode"]
pub mod timg4_evt_mode;
#[doc = "TIMG4_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg4_desc`] module"]
#[doc(alias = "TIMG4_DESC")]
pub type Timg4Desc = crate::Reg<timg4_desc::Timg4DescSpec>;
#[doc = "Module Description"]
pub mod timg4_desc;
#[doc = "TIMG4_COMMONREGS\\[%s\\]"]
pub use self::timg4_commonregs::Timg4Commonregs;
#[doc = r"Cluster"]
#[doc = "TIMG4_COMMONREGS\\[%s\\]"]
pub mod timg4_commonregs;
#[doc = "TIMG4_COUNTERREGS\\[%s\\]"]
pub use self::timg4_counterregs::Timg4Counterregs;
#[doc = r"Cluster"]
#[doc = "TIMG4_COUNTERREGS\\[%s\\]"]
pub mod timg4_counterregs;
