#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    wwdt0_gprcm: [Wwdt0Gprcm; 1],
    _reserved1: [u8; 0x0800],
    wwdt0_pdbgctl: Wwdt0Pdbgctl,
    _reserved2: [u8; 0x04],
    wwdt0_int_event: [Wwdt0IntEvent; 1],
    _reserved3: [u8; 0x94],
    wwdt0_evt_mode: Wwdt0EvtMode,
    _reserved4: [u8; 0x18],
    wwdt0_desc: Wwdt0Desc,
    wwdt0_wwdtctl0: Wwdt0Wwdtctl0,
    wwdt0_wwdtctl1: Wwdt0Wwdtctl1,
    wwdt0_wwdtcntrst: Wwdt0Wwdtcntrst,
    wwdt0_wwdtstat: Wwdt0Wwdtstat,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - WWDT0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn wwdt0_gprcm(&self, n: usize) -> &Wwdt0Gprcm {
        &self.wwdt0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - WWDT0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn wwdt0_gprcm_iter(&self) -> impl Iterator<Item = &Wwdt0Gprcm> {
        self.wwdt0_gprcm.iter()
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn wwdt0_pdbgctl(&self) -> &Wwdt0Pdbgctl {
        &self.wwdt0_pdbgctl
    }
    #[doc = "0x1020..0x104c - WWDT0_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn wwdt0_int_event(&self, n: usize) -> &Wwdt0IntEvent {
        &self.wwdt0_int_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - WWDT0_INT_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn wwdt0_int_event_iter(&self) -> impl Iterator<Item = &Wwdt0IntEvent> {
        self.wwdt0_int_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn wwdt0_evt_mode(&self) -> &Wwdt0EvtMode {
        &self.wwdt0_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn wwdt0_desc(&self) -> &Wwdt0Desc {
        &self.wwdt0_desc
    }
    #[doc = "0x1100 - Window Watchdog Timer Control Register 0"]
    #[inline(always)]
    pub const fn wwdt0_wwdtctl0(&self) -> &Wwdt0Wwdtctl0 {
        &self.wwdt0_wwdtctl0
    }
    #[doc = "0x1104 - Window Watchdog Timer Control Register 0"]
    #[inline(always)]
    pub const fn wwdt0_wwdtctl1(&self) -> &Wwdt0Wwdtctl1 {
        &self.wwdt0_wwdtctl1
    }
    #[doc = "0x1108 - Window Watchdog Timer Counter Reset Register"]
    #[inline(always)]
    pub const fn wwdt0_wwdtcntrst(&self) -> &Wwdt0Wwdtcntrst {
        &self.wwdt0_wwdtcntrst
    }
    #[doc = "0x110c - Window Watchdog Timer Status Register"]
    #[inline(always)]
    pub const fn wwdt0_wwdtstat(&self) -> &Wwdt0Wwdtstat {
        &self.wwdt0_wwdtstat
    }
}
#[doc = "WWDT0_GPRCM\\[%s\\]"]
pub use self::wwdt0_gprcm::Wwdt0Gprcm;
#[doc = r"Cluster"]
#[doc = "WWDT0_GPRCM\\[%s\\]"]
pub mod wwdt0_gprcm;
#[doc = "WWDT0_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_pdbgctl`] module"]
#[doc(alias = "WWDT0_PDBGCTL")]
pub type Wwdt0Pdbgctl = crate::Reg<wwdt0_pdbgctl::Wwdt0PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod wwdt0_pdbgctl;
#[doc = "WWDT0_INT_EVENT\\[%s\\]"]
pub use self::wwdt0_int_event::Wwdt0IntEvent;
#[doc = r"Cluster"]
#[doc = "WWDT0_INT_EVENT\\[%s\\]"]
pub mod wwdt0_int_event;
#[doc = "WWDT0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_evt_mode`] module"]
#[doc(alias = "WWDT0_EVT_MODE")]
pub type Wwdt0EvtMode = crate::Reg<wwdt0_evt_mode::Wwdt0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod wwdt0_evt_mode;
#[doc = "WWDT0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_desc`] module"]
#[doc(alias = "WWDT0_DESC")]
pub type Wwdt0Desc = crate::Reg<wwdt0_desc::Wwdt0DescSpec>;
#[doc = "Module Description"]
pub mod wwdt0_desc;
#[doc = "WWDT0_WWDTCTL0 (rw) register accessor: Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_wwdtctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_wwdtctl0`] module"]
#[doc(alias = "WWDT0_WWDTCTL0")]
pub type Wwdt0Wwdtctl0 = crate::Reg<wwdt0_wwdtctl0::Wwdt0Wwdtctl0Spec>;
#[doc = "Window Watchdog Timer Control Register 0"]
pub mod wwdt0_wwdtctl0;
#[doc = "WWDT0_WWDTCTL1 (rw) register accessor: Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_wwdtctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_wwdtctl1`] module"]
#[doc(alias = "WWDT0_WWDTCTL1")]
pub type Wwdt0Wwdtctl1 = crate::Reg<wwdt0_wwdtctl1::Wwdt0Wwdtctl1Spec>;
#[doc = "Window Watchdog Timer Control Register 0"]
pub mod wwdt0_wwdtctl1;
#[doc = "WWDT0_WWDTCNTRST (rw) register accessor: Window Watchdog Timer Counter Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtcntrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_wwdtcntrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_wwdtcntrst`] module"]
#[doc(alias = "WWDT0_WWDTCNTRST")]
pub type Wwdt0Wwdtcntrst = crate::Reg<wwdt0_wwdtcntrst::Wwdt0WwdtcntrstSpec>;
#[doc = "Window Watchdog Timer Counter Reset Register"]
pub mod wwdt0_wwdtcntrst;
#[doc = "WWDT0_WWDTSTAT (r) register accessor: Window Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdt0_wwdtstat`] module"]
#[doc(alias = "WWDT0_WWDTSTAT")]
pub type Wwdt0Wwdtstat = crate::Reg<wwdt0_wwdtstat::Wwdt0WwdtstatSpec>;
#[doc = "Window Watchdog Timer Status Register"]
pub mod wwdt0_wwdtstat;
