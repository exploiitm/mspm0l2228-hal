#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    i2c0_gprcm: [I2c0Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    i2c0_clkdiv: I2c0Clkdiv,
    i2c0_clksel: I2c0Clksel,
    _reserved3: [u8; 0x10],
    i2c0_pdbgctl: I2c0Pdbgctl,
    _reserved4: [u8; 0x04],
    i2c0_cpu_int: [I2c0CpuInt; 1],
    _reserved5: [u8; 0x04],
    i2c0_dma_trig1: [I2c0DmaTrig1; 1],
    _reserved6: [u8; 0x04],
    i2c0_dma_trig0: [I2c0DmaTrig0; 1],
    _reserved7: [u8; 0x34],
    i2c0_evt_mode: I2c0EvtMode,
    i2c0_intctl: I2c0Intctl,
    _reserved9: [u8; 0x14],
    i2c0_desc: I2c0Desc,
    _reserved10: [u8; 0x0100],
    i2c0_gfctl: I2c0Gfctl,
    i2c0_timeout_ctl: I2c0TimeoutCtl,
    i2c0_timeout_cnt: I2c0TimeoutCnt,
    _reserved13: [u8; 0x04],
    i2c0_controller: [I2c0Controller; 1],
    _reserved14: [u8; 0x08],
    i2c0_target: [I2c0Target; 1],
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - I2C0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c0_gprcm(&self, n: usize) -> &I2c0Gprcm {
        &self.i2c0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - I2C0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn i2c0_gprcm_iter(&self) -> impl Iterator<Item = &I2c0Gprcm> {
        self.i2c0_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn i2c0_clkdiv(&self) -> &I2c0Clkdiv {
        &self.i2c0_clkdiv
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn i2c0_clksel(&self) -> &I2c0Clksel {
        &self.i2c0_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn i2c0_pdbgctl(&self) -> &I2c0Pdbgctl {
        &self.i2c0_pdbgctl
    }
    #[doc = "0x1020..0x104c - I2C0_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c0_cpu_int(&self, n: usize) -> &I2c0CpuInt {
        &self.i2c0_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - I2C0_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn i2c0_cpu_int_iter(&self) -> impl Iterator<Item = &I2c0CpuInt> {
        self.i2c0_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - I2C0_DMA_TRIG1\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c0_dma_trig1(&self, n: usize) -> &I2c0DmaTrig1 {
        &self.i2c0_dma_trig1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - I2C0_DMA_TRIG1\\[%s\\]"]
    #[inline(always)]
    pub fn i2c0_dma_trig1_iter(&self) -> impl Iterator<Item = &I2c0DmaTrig1> {
        self.i2c0_dma_trig1.iter()
    }
    #[doc = "0x1080..0x10ac - I2C0_DMA_TRIG0\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c0_dma_trig0(&self, n: usize) -> &I2c0DmaTrig0 {
        &self.i2c0_dma_trig0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - I2C0_DMA_TRIG0\\[%s\\]"]
    #[inline(always)]
    pub fn i2c0_dma_trig0_iter(&self) -> impl Iterator<Item = &I2c0DmaTrig0> {
        self.i2c0_dma_trig0.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn i2c0_evt_mode(&self) -> &I2c0EvtMode {
        &self.i2c0_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn i2c0_intctl(&self) -> &I2c0Intctl {
        &self.i2c0_intctl
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn i2c0_desc(&self) -> &I2c0Desc {
        &self.i2c0_desc
    }
    #[doc = "0x1200 - I2C Glitch Filter Control"]
    #[inline(always)]
    pub const fn i2c0_gfctl(&self) -> &I2c0Gfctl {
        &self.i2c0_gfctl
    }
    #[doc = "0x1204 - I2C Timeout Count Control Register"]
    #[inline(always)]
    pub const fn i2c0_timeout_ctl(&self) -> &I2c0TimeoutCtl {
        &self.i2c0_timeout_ctl
    }
    #[doc = "0x1208 - I2C Timeout Count Register"]
    #[inline(always)]
    pub const fn i2c0_timeout_cnt(&self) -> &I2c0TimeoutCnt {
        &self.i2c0_timeout_cnt
    }
    #[doc = "0x1210..0x1248 - I2C0_CONTROLLER\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c0_controller(&self, n: usize) -> &I2c0Controller {
        &self.i2c0_controller[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1210..0x1248 - I2C0_CONTROLLER\\[%s\\]"]
    #[inline(always)]
    pub fn i2c0_controller_iter(&self) -> impl Iterator<Item = &I2c0Controller> {
        self.i2c0_controller.iter()
    }
    #[doc = "0x1250..0x127c - I2C0_TARGET\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c0_target(&self, n: usize) -> &I2c0Target {
        &self.i2c0_target[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1250..0x127c - I2C0_TARGET\\[%s\\]"]
    #[inline(always)]
    pub fn i2c0_target_iter(&self) -> impl Iterator<Item = &I2c0Target> {
        self.i2c0_target.iter()
    }
}
#[doc = "I2C0_GPRCM\\[%s\\]"]
pub use self::i2c0_gprcm::I2c0Gprcm;
#[doc = r"Cluster"]
#[doc = "I2C0_GPRCM\\[%s\\]"]
pub mod i2c0_gprcm;
#[doc = "I2C0_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_clkdiv`] module"]
#[doc(alias = "I2C0_CLKDIV")]
pub type I2c0Clkdiv = crate::Reg<i2c0_clkdiv::I2c0ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod i2c0_clkdiv;
#[doc = "I2C0_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_clksel`] module"]
#[doc(alias = "I2C0_CLKSEL")]
pub type I2c0Clksel = crate::Reg<i2c0_clksel::I2c0ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod i2c0_clksel;
#[doc = "I2C0_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_pdbgctl`] module"]
#[doc(alias = "I2C0_PDBGCTL")]
pub type I2c0Pdbgctl = crate::Reg<i2c0_pdbgctl::I2c0PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod i2c0_pdbgctl;
#[doc = "I2C0_CPU_INT\\[%s\\]"]
pub use self::i2c0_cpu_int::I2c0CpuInt;
#[doc = r"Cluster"]
#[doc = "I2C0_CPU_INT\\[%s\\]"]
pub mod i2c0_cpu_int;
#[doc = "I2C0_DMA_TRIG1\\[%s\\]"]
pub use self::i2c0_dma_trig1::I2c0DmaTrig1;
#[doc = r"Cluster"]
#[doc = "I2C0_DMA_TRIG1\\[%s\\]"]
pub mod i2c0_dma_trig1;
#[doc = "I2C0_DMA_TRIG0\\[%s\\]"]
pub use self::i2c0_dma_trig0::I2c0DmaTrig0;
#[doc = r"Cluster"]
#[doc = "I2C0_DMA_TRIG0\\[%s\\]"]
pub mod i2c0_dma_trig0;
#[doc = "I2C0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_evt_mode`] module"]
#[doc(alias = "I2C0_EVT_MODE")]
pub type I2c0EvtMode = crate::Reg<i2c0_evt_mode::I2c0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod i2c0_evt_mode;
#[doc = "I2C0_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_intctl`] module"]
#[doc(alias = "I2C0_INTCTL")]
pub type I2c0Intctl = crate::Reg<i2c0_intctl::I2c0IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod i2c0_intctl;
#[doc = "I2C0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_desc`] module"]
#[doc(alias = "I2C0_DESC")]
pub type I2c0Desc = crate::Reg<i2c0_desc::I2c0DescSpec>;
#[doc = "Module Description"]
pub mod i2c0_desc;
#[doc = "I2C0_GFCTL (rw) register accessor: I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_gfctl`] module"]
#[doc(alias = "I2C0_GFCTL")]
pub type I2c0Gfctl = crate::Reg<i2c0_gfctl::I2c0GfctlSpec>;
#[doc = "I2C Glitch Filter Control"]
pub mod i2c0_gfctl;
#[doc = "I2C0_TIMEOUT_CTL (rw) register accessor: I2C Timeout Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_timeout_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_timeout_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_timeout_ctl`] module"]
#[doc(alias = "I2C0_TIMEOUT_CTL")]
pub type I2c0TimeoutCtl = crate::Reg<i2c0_timeout_ctl::I2c0TimeoutCtlSpec>;
#[doc = "I2C Timeout Count Control Register"]
pub mod i2c0_timeout_ctl;
#[doc = "I2C0_TIMEOUT_CNT (r) register accessor: I2C Timeout Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_timeout_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_timeout_cnt`] module"]
#[doc(alias = "I2C0_TIMEOUT_CNT")]
pub type I2c0TimeoutCnt = crate::Reg<i2c0_timeout_cnt::I2c0TimeoutCntSpec>;
#[doc = "I2C Timeout Count Register"]
pub mod i2c0_timeout_cnt;
#[doc = "I2C0_CONTROLLER\\[%s\\]"]
pub use self::i2c0_controller::I2c0Controller;
#[doc = r"Cluster"]
#[doc = "I2C0_CONTROLLER\\[%s\\]"]
pub mod i2c0_controller;
#[doc = "I2C0_TARGET\\[%s\\]"]
pub use self::i2c0_target::I2c0Target;
#[doc = r"Cluster"]
#[doc = "I2C0_TARGET\\[%s\\]"]
pub mod i2c0_target;
