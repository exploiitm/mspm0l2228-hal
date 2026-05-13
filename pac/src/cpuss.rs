#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10e0],
    cpuss_evt_mode: CpussEvtMode,
    _reserved1: [u8; 0x18],
    cpuss_desc: CpussDesc,
    cpuss_int_group0: [CpussIntGroup0; 1],
    _reserved3: [u8; 0x04],
    cpuss_int_group1: [CpussIntGroup1; 1],
    _reserved4: [u8; 0x01a4],
    cpuss_ctl: CpussCtl,
}
impl RegisterBlock {
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn cpuss_evt_mode(&self) -> &CpussEvtMode {
        &self.cpuss_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn cpuss_desc(&self) -> &CpussDesc {
        &self.cpuss_desc
    }
    #[doc = "0x1100..0x112c - CPUSS_INT_GROUP0\\[%s\\]"]
    #[inline(always)]
    pub const fn cpuss_int_group0(&self, n: usize) -> &CpussIntGroup0 {
        &self.cpuss_int_group0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x112c - CPUSS_INT_GROUP0\\[%s\\]"]
    #[inline(always)]
    pub fn cpuss_int_group0_iter(&self) -> impl Iterator<Item = &CpussIntGroup0> {
        self.cpuss_int_group0.iter()
    }
    #[doc = "0x1130..0x115c - CPUSS_INT_GROUP1\\[%s\\]"]
    #[inline(always)]
    pub const fn cpuss_int_group1(&self, n: usize) -> &CpussIntGroup1 {
        &self.cpuss_int_group1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1130..0x115c - CPUSS_INT_GROUP1\\[%s\\]"]
    #[inline(always)]
    pub fn cpuss_int_group1_iter(&self) -> impl Iterator<Item = &CpussIntGroup1> {
        self.cpuss_int_group1.iter()
    }
    #[doc = "0x1300 - Prefetch/Cache control"]
    #[inline(always)]
    pub const fn cpuss_ctl(&self) -> &CpussCtl {
        &self.cpuss_ctl
    }
}
#[doc = "CPUSS_EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_evt_mode`] module"]
#[doc(alias = "CPUSS_EVT_MODE")]
pub type CpussEvtMode = crate::Reg<cpuss_evt_mode::CpussEvtModeSpec>;
#[doc = "Event Mode"]
pub mod cpuss_evt_mode;
#[doc = "CPUSS_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_desc`] module"]
#[doc(alias = "CPUSS_DESC")]
pub type CpussDesc = crate::Reg<cpuss_desc::CpussDescSpec>;
#[doc = "Module Description"]
pub mod cpuss_desc;
#[doc = "CPUSS_INT_GROUP0\\[%s\\]"]
pub use self::cpuss_int_group0::CpussIntGroup0;
#[doc = r"Cluster"]
#[doc = "CPUSS_INT_GROUP0\\[%s\\]"]
pub mod cpuss_int_group0;
#[doc = "CPUSS_INT_GROUP1\\[%s\\]"]
pub use self::cpuss_int_group1::CpussIntGroup1;
#[doc = r"Cluster"]
#[doc = "CPUSS_INT_GROUP1\\[%s\\]"]
pub mod cpuss_int_group1;
#[doc = "CPUSS_CTL (rw) register accessor: Prefetch/Cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuss_ctl`] module"]
#[doc(alias = "CPUSS_CTL")]
pub type CpussCtl = crate::Reg<cpuss_ctl::CpussCtlSpec>;
#[doc = "Prefetch/Cache control"]
pub mod cpuss_ctl;
