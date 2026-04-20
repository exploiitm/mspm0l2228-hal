#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    i2c2_gprcm: [I2c2Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    i2c2_clkdiv: I2c2Clkdiv,
    i2c2_clksel: I2c2Clksel,
    _reserved3: [u8; 0x10],
    i2c2_pdbgctl: I2c2Pdbgctl,
    _reserved4: [u8; 0x04],
    i2c2_cpu_int: [I2c2CpuInt; 1],
    _reserved5: [u8; 0x04],
    i2c2_dma_trig1: [I2c2DmaTrig1; 1],
    _reserved6: [u8; 0x04],
    i2c2_dma_trig0: [I2c2DmaTrig0; 1],
    _reserved7: [u8; 0x34],
    i2c2_evt_mode: I2c2EvtMode,
    i2c2_intctl: I2c2Intctl,
    _reserved9: [u8; 0x14],
    i2c2_desc: I2c2Desc,
    _reserved10: [u8; 0x0100],
    i2c2_gfctl: I2c2Gfctl,
    i2c2_timeout_ctl: I2c2TimeoutCtl,
    i2c2_timeout_cnt: I2c2TimeoutCnt,
    _reserved13: [u8; 0x04],
    i2c2_controller: [I2c2Controller; 1],
    _reserved14: [u8; 0x08],
    i2c2_target: [I2c2Target; 1],
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - I2C2_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c2_gprcm(&self, n: usize) -> &I2c2Gprcm {
        &self.i2c2_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - I2C2_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn i2c2_gprcm_iter(&self) -> impl Iterator<Item = &I2c2Gprcm> {
        self.i2c2_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn i2c2_clkdiv(&self) -> &I2c2Clkdiv {
        &self.i2c2_clkdiv
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn i2c2_clksel(&self) -> &I2c2Clksel {
        &self.i2c2_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn i2c2_pdbgctl(&self) -> &I2c2Pdbgctl {
        &self.i2c2_pdbgctl
    }
    #[doc = "0x1020..0x104c - I2C2_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c2_cpu_int(&self, n: usize) -> &I2c2CpuInt {
        &self.i2c2_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - I2C2_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn i2c2_cpu_int_iter(&self) -> impl Iterator<Item = &I2c2CpuInt> {
        self.i2c2_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - I2C2_DMA_TRIG1\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c2_dma_trig1(&self, n: usize) -> &I2c2DmaTrig1 {
        &self.i2c2_dma_trig1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - I2C2_DMA_TRIG1\\[%s\\]"]
    #[inline(always)]
    pub fn i2c2_dma_trig1_iter(&self) -> impl Iterator<Item = &I2c2DmaTrig1> {
        self.i2c2_dma_trig1.iter()
    }
    #[doc = "0x1080..0x10ac - I2C2_DMA_TRIG0\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c2_dma_trig0(&self, n: usize) -> &I2c2DmaTrig0 {
        &self.i2c2_dma_trig0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - I2C2_DMA_TRIG0\\[%s\\]"]
    #[inline(always)]
    pub fn i2c2_dma_trig0_iter(&self) -> impl Iterator<Item = &I2c2DmaTrig0> {
        self.i2c2_dma_trig0.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn i2c2_evt_mode(&self) -> &I2c2EvtMode {
        &self.i2c2_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn i2c2_intctl(&self) -> &I2c2Intctl {
        &self.i2c2_intctl
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn i2c2_desc(&self) -> &I2c2Desc {
        &self.i2c2_desc
    }
    #[doc = "0x1200 - I2C Glitch Filter Control"]
    #[inline(always)]
    pub const fn i2c2_gfctl(&self) -> &I2c2Gfctl {
        &self.i2c2_gfctl
    }
    #[doc = "0x1204 - I2C Timeout Count Control Register"]
    #[inline(always)]
    pub const fn i2c2_timeout_ctl(&self) -> &I2c2TimeoutCtl {
        &self.i2c2_timeout_ctl
    }
    #[doc = "0x1208 - I2C Timeout Count Register"]
    #[inline(always)]
    pub const fn i2c2_timeout_cnt(&self) -> &I2c2TimeoutCnt {
        &self.i2c2_timeout_cnt
    }
    #[doc = "0x1210..0x1248 - I2C2_CONTROLLER\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c2_controller(&self, n: usize) -> &I2c2Controller {
        &self.i2c2_controller[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1210..0x1248 - I2C2_CONTROLLER\\[%s\\]"]
    #[inline(always)]
    pub fn i2c2_controller_iter(&self) -> impl Iterator<Item = &I2c2Controller> {
        self.i2c2_controller.iter()
    }
    #[doc = "0x1250..0x127c - I2C2_TARGET\\[%s\\]"]
    #[inline(always)]
    pub const fn i2c2_target(&self, n: usize) -> &I2c2Target {
        &self.i2c2_target[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1250..0x127c - I2C2_TARGET\\[%s\\]"]
    #[inline(always)]
    pub fn i2c2_target_iter(&self) -> impl Iterator<Item = &I2c2Target> {
        self.i2c2_target.iter()
    }
}
#[doc = "I2C2_GPRCM\\[%s\\]"]
pub use self::i2c2_gprcm::I2c2Gprcm;
#[doc = r"Cluster"]
#[doc = "I2C2_GPRCM\\[%s\\]"]
pub mod i2c2_gprcm;
#[doc = "I2C2_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_clkdiv`] module"]
#[doc(alias = "I2C2_CLKDIV")]
pub type I2c2Clkdiv = crate::Reg<i2c2_clkdiv::I2c2ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod i2c2_clkdiv;
#[doc = "I2C2_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_clksel`] module"]
#[doc(alias = "I2C2_CLKSEL")]
pub type I2c2Clksel = crate::Reg<i2c2_clksel::I2c2ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod i2c2_clksel;
#[doc = "I2C2_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_pdbgctl`] module"]
#[doc(alias = "I2C2_PDBGCTL")]
pub type I2c2Pdbgctl = crate::Reg<i2c2_pdbgctl::I2c2PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod i2c2_pdbgctl;
#[doc = "I2C2_CPU_INT\\[%s\\]"]
pub use self::i2c2_cpu_int::I2c2CpuInt;
#[doc = r"Cluster"]
#[doc = "I2C2_CPU_INT\\[%s\\]"]
pub mod i2c2_cpu_int;
#[doc = "I2C2_DMA_TRIG1\\[%s\\]"]
pub use self::i2c2_dma_trig1::I2c2DmaTrig1;
#[doc = r"Cluster"]
#[doc = "I2C2_DMA_TRIG1\\[%s\\]"]
pub mod i2c2_dma_trig1;
#[doc = "I2C2_DMA_TRIG0\\[%s\\]"]
pub use self::i2c2_dma_trig0::I2c2DmaTrig0;
#[doc = r"Cluster"]
#[doc = "I2C2_DMA_TRIG0\\[%s\\]"]
pub mod i2c2_dma_trig0;
#[doc = "I2C2_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_evt_mode`] module"]
#[doc(alias = "I2C2_EVT_MODE")]
pub type I2c2EvtMode = crate::Reg<i2c2_evt_mode::I2c2EvtModeSpec>;
#[doc = "Event Mode"]
pub mod i2c2_evt_mode;
#[doc = "I2C2_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_intctl`] module"]
#[doc(alias = "I2C2_INTCTL")]
pub type I2c2Intctl = crate::Reg<i2c2_intctl::I2c2IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod i2c2_intctl;
#[doc = "I2C2_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_desc`] module"]
#[doc(alias = "I2C2_DESC")]
pub type I2c2Desc = crate::Reg<i2c2_desc::I2c2DescSpec>;
#[doc = "Module Description"]
pub mod i2c2_desc;
#[doc = "I2C2_GFCTL (rw) register accessor: I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_gfctl`] module"]
#[doc(alias = "I2C2_GFCTL")]
pub type I2c2Gfctl = crate::Reg<i2c2_gfctl::I2c2GfctlSpec>;
#[doc = "I2C Glitch Filter Control"]
pub mod i2c2_gfctl;
#[doc = "I2C2_TIMEOUT_CTL (rw) register accessor: I2C Timeout Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_timeout_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_timeout_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_timeout_ctl`] module"]
#[doc(alias = "I2C2_TIMEOUT_CTL")]
pub type I2c2TimeoutCtl = crate::Reg<i2c2_timeout_ctl::I2c2TimeoutCtlSpec>;
#[doc = "I2C Timeout Count Control Register"]
pub mod i2c2_timeout_ctl;
#[doc = "I2C2_TIMEOUT_CNT (r) register accessor: I2C Timeout Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_timeout_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_timeout_cnt`] module"]
#[doc(alias = "I2C2_TIMEOUT_CNT")]
pub type I2c2TimeoutCnt = crate::Reg<i2c2_timeout_cnt::I2c2TimeoutCntSpec>;
#[doc = "I2C Timeout Count Register"]
pub mod i2c2_timeout_cnt;
#[doc = "I2C2_CONTROLLER\\[%s\\]"]
pub use self::i2c2_controller::I2c2Controller;
#[doc = r"Cluster"]
#[doc = "I2C2_CONTROLLER\\[%s\\]"]
pub mod i2c2_controller;
#[doc = "I2C2_TARGET\\[%s\\]"]
pub use self::i2c2_target::I2c2Target;
#[doc = r"Cluster"]
#[doc = "I2C2_TARGET\\[%s\\]"]
pub mod i2c2_target;
