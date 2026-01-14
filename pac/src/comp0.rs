#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    comp0_fsub_0: Comp0Fsub0,
    comp0_fsub_1: Comp0Fsub1,
    _reserved2: [u8; 0x3c],
    comp0_fpub_1: Comp0Fpub1,
    _reserved3: [u8; 0x03b8],
    comp0_gprcm: [Comp0Gprcm; 1],
    _reserved4: [u8; 0x0808],
    comp0_cpu_int: [Comp0CpuInt; 1],
    _reserved5: [u8; 0x04],
    comp0_gen_event: [Comp0GenEvent; 1],
    _reserved6: [u8; 0x64],
    comp0_evt_mode: Comp0EvtMode,
    _reserved7: [u8; 0x18],
    comp0_desc: Comp0Desc,
    comp0_ctl0: Comp0Ctl0,
    comp0_ctl1: Comp0Ctl1,
    comp0_ctl2: Comp0Ctl2,
    comp0_ctl3: Comp0Ctl3,
    _reserved12: [u8; 0x10],
    comp0_stat: Comp0Stat,
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Port 0"]
    #[inline(always)]
    pub const fn comp0_fsub_0(&self) -> &Comp0Fsub0 {
        &self.comp0_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn comp0_fsub_1(&self) -> &Comp0Fsub1 {
        &self.comp0_fsub_1
    }
    #[doc = "0x444 - Publisher port 1"]
    #[inline(always)]
    pub const fn comp0_fpub_1(&self) -> &Comp0Fpub1 {
        &self.comp0_fpub_1
    }
    #[doc = "0x800..0x818 - COMP0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn comp0_gprcm(&self, n: usize) -> &Comp0Gprcm {
        &self.comp0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - COMP0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn comp0_gprcm_iter(&self) -> impl Iterator<Item = &Comp0Gprcm> {
        self.comp0_gprcm.iter()
    }
    #[doc = "0x1020..0x104c - COMP0_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn comp0_cpu_int(&self, n: usize) -> &Comp0CpuInt {
        &self.comp0_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - COMP0_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn comp0_cpu_int_iter(&self) -> impl Iterator<Item = &Comp0CpuInt> {
        self.comp0_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - COMP0_GEN_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn comp0_gen_event(&self, n: usize) -> &Comp0GenEvent {
        &self.comp0_gen_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - COMP0_GEN_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn comp0_gen_event_iter(&self) -> impl Iterator<Item = &Comp0GenEvent> {
        self.comp0_gen_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn comp0_evt_mode(&self) -> &Comp0EvtMode {
        &self.comp0_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn comp0_desc(&self) -> &Comp0Desc {
        &self.comp0_desc
    }
    #[doc = "0x1100 - Control 0"]
    #[inline(always)]
    pub const fn comp0_ctl0(&self) -> &Comp0Ctl0 {
        &self.comp0_ctl0
    }
    #[doc = "0x1104 - Control 1"]
    #[inline(always)]
    pub const fn comp0_ctl1(&self) -> &Comp0Ctl1 {
        &self.comp0_ctl1
    }
    #[doc = "0x1108 - Control 2"]
    #[inline(always)]
    pub const fn comp0_ctl2(&self) -> &Comp0Ctl2 {
        &self.comp0_ctl2
    }
    #[doc = "0x110c - Control 3"]
    #[inline(always)]
    pub const fn comp0_ctl3(&self) -> &Comp0Ctl3 {
        &self.comp0_ctl3
    }
    #[doc = "0x1120 - Status"]
    #[inline(always)]
    pub const fn comp0_stat(&self) -> &Comp0Stat {
        &self.comp0_stat
    }
}
#[doc = "COMP0_FSUB_0 (rw) register accessor: Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_fsub_0`] module"]
#[doc(alias = "COMP0_FSUB_0")]
pub type Comp0Fsub0 = crate::Reg<comp0_fsub_0::Comp0Fsub0Spec>;
#[doc = "Subscriber Port 0"]
pub mod comp0_fsub_0;
#[doc = "COMP0_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_fsub_1`] module"]
#[doc(alias = "COMP0_FSUB_1")]
pub type Comp0Fsub1 = crate::Reg<comp0_fsub_1::Comp0Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod comp0_fsub_1;
#[doc = "COMP0_FPUB_1 (rw) register accessor: Publisher port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_fpub_1`] module"]
#[doc(alias = "COMP0_FPUB_1")]
pub type Comp0Fpub1 = crate::Reg<comp0_fpub_1::Comp0Fpub1Spec>;
#[doc = "Publisher port 1"]
pub mod comp0_fpub_1;
#[doc = "COMP0_GPRCM\\[%s\\]"]
pub use self::comp0_gprcm::Comp0Gprcm;
#[doc = r"Cluster"]
#[doc = "COMP0_GPRCM\\[%s\\]"]
pub mod comp0_gprcm;
#[doc = "COMP0_CPU_INT\\[%s\\]"]
pub use self::comp0_cpu_int::Comp0CpuInt;
#[doc = r"Cluster"]
#[doc = "COMP0_CPU_INT\\[%s\\]"]
pub mod comp0_cpu_int;
#[doc = "COMP0_GEN_EVENT\\[%s\\]"]
pub use self::comp0_gen_event::Comp0GenEvent;
#[doc = r"Cluster"]
#[doc = "COMP0_GEN_EVENT\\[%s\\]"]
pub mod comp0_gen_event;
#[doc = "COMP0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_evt_mode`] module"]
#[doc(alias = "COMP0_EVT_MODE")]
pub type Comp0EvtMode = crate::Reg<comp0_evt_mode::Comp0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod comp0_evt_mode;
#[doc = "COMP0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_desc`] module"]
#[doc(alias = "COMP0_DESC")]
pub type Comp0Desc = crate::Reg<comp0_desc::Comp0DescSpec>;
#[doc = "Module Description"]
pub mod comp0_desc;
#[doc = "COMP0_CTL0 (rw) register accessor: Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_ctl0`] module"]
#[doc(alias = "COMP0_CTL0")]
pub type Comp0Ctl0 = crate::Reg<comp0_ctl0::Comp0Ctl0Spec>;
#[doc = "Control 0"]
pub mod comp0_ctl0;
#[doc = "COMP0_CTL1 (rw) register accessor: Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_ctl1`] module"]
#[doc(alias = "COMP0_CTL1")]
pub type Comp0Ctl1 = crate::Reg<comp0_ctl1::Comp0Ctl1Spec>;
#[doc = "Control 1"]
pub mod comp0_ctl1;
#[doc = "COMP0_CTL2 (rw) register accessor: Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_ctl2`] module"]
#[doc(alias = "COMP0_CTL2")]
pub type Comp0Ctl2 = crate::Reg<comp0_ctl2::Comp0Ctl2Spec>;
#[doc = "Control 2"]
pub mod comp0_ctl2;
#[doc = "COMP0_CTL3 (rw) register accessor: Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_ctl3`] module"]
#[doc(alias = "COMP0_CTL3")]
pub type Comp0Ctl3 = crate::Reg<comp0_ctl3::Comp0Ctl3Spec>;
#[doc = "Control 3"]
pub mod comp0_ctl3;
#[doc = "COMP0_STAT (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_stat`] module"]
#[doc(alias = "COMP0_STAT")]
pub type Comp0Stat = crate::Reg<comp0_stat::Comp0StatSpec>;
#[doc = "Status"]
pub mod comp0_stat;
