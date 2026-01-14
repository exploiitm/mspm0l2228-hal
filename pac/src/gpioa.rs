#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    gpioa_fsub_0: GpioaFsub0,
    gpioa_fsub_1: GpioaFsub1,
    _reserved2: [u8; 0x3c],
    gpioa_fpub_0: GpioaFpub0,
    gpioa_fpub_1: GpioaFpub1,
    _reserved4: [u8; 0x03b4],
    gpioa_gprcm: [GpioaGprcm; 1],
    _reserved5: [u8; 0x07f8],
    gpioa_clkovr: GpioaClkovr,
    _reserved6: [u8; 0x04],
    gpioa_pdbgctl: GpioaPdbgctl,
    _reserved7: [u8; 0x04],
    gpioa_cpu_int: [GpioaCpuInt; 1],
    _reserved8: [u8; 0x04],
    gpioa_gen_event0: [GpioaGenEvent0; 1],
    _reserved9: [u8; 0x04],
    gpioa_gen_event1: [GpioaGenEvent1; 1],
    _reserved10: [u8; 0x34],
    gpioa_evt_mode: GpioaEvtMode,
    _reserved11: [u8; 0x18],
    gpioa_desc: GpioaDesc,
    _reserved12: [u8; 0x0100],
    gpioa_dout3_0: GpioaDout3_0,
    gpioa_dout7_4: GpioaDout7_4,
    gpioa_dout11_8: GpioaDout11_8,
    gpioa_dout15_12: GpioaDout15_12,
    gpioa_dout19_16: GpioaDout19_16,
    gpioa_dout23_20: GpioaDout23_20,
    gpioa_dout27_24: GpioaDout27_24,
    gpioa_dout31_28: GpioaDout31_28,
    _reserved20: [u8; 0x60],
    gpioa_dout31_0: GpioaDout31_0,
    _reserved21: [u8; 0x0c],
    gpioa_doutset31_0: GpioaDoutset31_0,
    _reserved22: [u8; 0x0c],
    gpioa_doutclr31_0: GpioaDoutclr31_0,
    _reserved23: [u8; 0x0c],
    gpioa_douttgl31_0: GpioaDouttgl31_0,
    _reserved24: [u8; 0x0c],
    gpioa_doe31_0: GpioaDoe31_0,
    _reserved25: [u8; 0x0c],
    gpioa_doeset31_0: GpioaDoeset31_0,
    _reserved26: [u8; 0x0c],
    gpioa_doeclr31_0: GpioaDoeclr31_0,
    _reserved27: [u8; 0x1c],
    gpioa_din3_0: GpioaDin3_0,
    gpioa_din7_4: GpioaDin7_4,
    gpioa_din11_8: GpioaDin11_8,
    gpioa_din15_12: GpioaDin15_12,
    gpioa_din19_16: GpioaDin19_16,
    gpioa_din23_20: GpioaDin23_20,
    gpioa_din27_24: GpioaDin27_24,
    gpioa_din31_28: GpioaDin31_28,
    _reserved35: [u8; 0x60],
    gpioa_din31_0: GpioaDin31_0,
    _reserved36: [u8; 0x0c],
    gpioa_polarity15_0: GpioaPolarity15_0,
    _reserved37: [u8; 0x0c],
    gpioa_polarity31_16: GpioaPolarity31_16,
    _reserved38: [u8; 0x5c],
    gpioa_ctl: GpioaCtl,
    gpioa_fastwake: GpioaFastwake,
    _reserved40: [u8; 0xf8],
    gpioa_sub0cfg: GpioaSub0cfg,
    _reserved41: [u8; 0x04],
    gpioa_filteren15_0: GpioaFilteren15_0,
    gpioa_filteren31_16: GpioaFilteren31_16,
    gpioa_dmamask: GpioaDmamask,
    _reserved44: [u8; 0x0c],
    gpioa_sub1cfg: GpioaSub1cfg,
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn gpioa_fsub_0(&self) -> &GpioaFsub0 {
        &self.gpioa_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn gpioa_fsub_1(&self) -> &GpioaFsub1 {
        &self.gpioa_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn gpioa_fpub_0(&self) -> &GpioaFpub0 {
        &self.gpioa_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn gpioa_fpub_1(&self) -> &GpioaFpub1 {
        &self.gpioa_fpub_1
    }
    #[doc = "0x800..0x818 - GPIOA_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioa_gprcm(&self, n: usize) -> &GpioaGprcm {
        &self.gpioa_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - GPIOA_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn gpioa_gprcm_iter(&self) -> impl Iterator<Item = &GpioaGprcm> {
        self.gpioa_gprcm.iter()
    }
    #[doc = "0x1010 - Clock Override"]
    #[inline(always)]
    pub const fn gpioa_clkovr(&self) -> &GpioaClkovr {
        &self.gpioa_clkovr
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn gpioa_pdbgctl(&self) -> &GpioaPdbgctl {
        &self.gpioa_pdbgctl
    }
    #[doc = "0x1020..0x104c - GPIOA_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioa_cpu_int(&self, n: usize) -> &GpioaCpuInt {
        &self.gpioa_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - GPIOA_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn gpioa_cpu_int_iter(&self) -> impl Iterator<Item = &GpioaCpuInt> {
        self.gpioa_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - GPIOA_GEN_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioa_gen_event0(&self, n: usize) -> &GpioaGenEvent0 {
        &self.gpioa_gen_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - GPIOA_GEN_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn gpioa_gen_event0_iter(&self) -> impl Iterator<Item = &GpioaGenEvent0> {
        self.gpioa_gen_event0.iter()
    }
    #[doc = "0x1080..0x10ac - GPIOA_GEN_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioa_gen_event1(&self, n: usize) -> &GpioaGenEvent1 {
        &self.gpioa_gen_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - GPIOA_GEN_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn gpioa_gen_event1_iter(&self) -> impl Iterator<Item = &GpioaGenEvent1> {
        self.gpioa_gen_event1.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn gpioa_evt_mode(&self) -> &GpioaEvtMode {
        &self.gpioa_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn gpioa_desc(&self) -> &GpioaDesc {
        &self.gpioa_desc
    }
    #[doc = "0x1200 - Data output 3 to 0"]
    #[inline(always)]
    pub const fn gpioa_dout3_0(&self) -> &GpioaDout3_0 {
        &self.gpioa_dout3_0
    }
    #[doc = "0x1204 - Data output 7 to 4"]
    #[inline(always)]
    pub const fn gpioa_dout7_4(&self) -> &GpioaDout7_4 {
        &self.gpioa_dout7_4
    }
    #[doc = "0x1208 - Data output 11 to 8"]
    #[inline(always)]
    pub const fn gpioa_dout11_8(&self) -> &GpioaDout11_8 {
        &self.gpioa_dout11_8
    }
    #[doc = "0x120c - Data output 15 to 12"]
    #[inline(always)]
    pub const fn gpioa_dout15_12(&self) -> &GpioaDout15_12 {
        &self.gpioa_dout15_12
    }
    #[doc = "0x1210 - Data output 19 to 16"]
    #[inline(always)]
    pub const fn gpioa_dout19_16(&self) -> &GpioaDout19_16 {
        &self.gpioa_dout19_16
    }
    #[doc = "0x1214 - Data output 23 to 20"]
    #[inline(always)]
    pub const fn gpioa_dout23_20(&self) -> &GpioaDout23_20 {
        &self.gpioa_dout23_20
    }
    #[doc = "0x1218 - Data output 27 to 24"]
    #[inline(always)]
    pub const fn gpioa_dout27_24(&self) -> &GpioaDout27_24 {
        &self.gpioa_dout27_24
    }
    #[doc = "0x121c - Data output 31 to 28"]
    #[inline(always)]
    pub const fn gpioa_dout31_28(&self) -> &GpioaDout31_28 {
        &self.gpioa_dout31_28
    }
    #[doc = "0x1280 - Data output 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_dout31_0(&self) -> &GpioaDout31_0 {
        &self.gpioa_dout31_0
    }
    #[doc = "0x1290 - Data output set 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_doutset31_0(&self) -> &GpioaDoutset31_0 {
        &self.gpioa_doutset31_0
    }
    #[doc = "0x12a0 - Data output clear 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_doutclr31_0(&self) -> &GpioaDoutclr31_0 {
        &self.gpioa_doutclr31_0
    }
    #[doc = "0x12b0 - Data output toggle 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_douttgl31_0(&self) -> &GpioaDouttgl31_0 {
        &self.gpioa_douttgl31_0
    }
    #[doc = "0x12c0 - Data output enable 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_doe31_0(&self) -> &GpioaDoe31_0 {
        &self.gpioa_doe31_0
    }
    #[doc = "0x12d0 - Data output enable set 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_doeset31_0(&self) -> &GpioaDoeset31_0 {
        &self.gpioa_doeset31_0
    }
    #[doc = "0x12e0 - Data output enable clear 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_doeclr31_0(&self) -> &GpioaDoeclr31_0 {
        &self.gpioa_doeclr31_0
    }
    #[doc = "0x1300 - Data input 3 to 0"]
    #[inline(always)]
    pub const fn gpioa_din3_0(&self) -> &GpioaDin3_0 {
        &self.gpioa_din3_0
    }
    #[doc = "0x1304 - Data input 7 to 4"]
    #[inline(always)]
    pub const fn gpioa_din7_4(&self) -> &GpioaDin7_4 {
        &self.gpioa_din7_4
    }
    #[doc = "0x1308 - Data input 11 to 8"]
    #[inline(always)]
    pub const fn gpioa_din11_8(&self) -> &GpioaDin11_8 {
        &self.gpioa_din11_8
    }
    #[doc = "0x130c - Data input 15 to 12"]
    #[inline(always)]
    pub const fn gpioa_din15_12(&self) -> &GpioaDin15_12 {
        &self.gpioa_din15_12
    }
    #[doc = "0x1310 - Data input 19 to 16"]
    #[inline(always)]
    pub const fn gpioa_din19_16(&self) -> &GpioaDin19_16 {
        &self.gpioa_din19_16
    }
    #[doc = "0x1314 - Data input 23 to 20"]
    #[inline(always)]
    pub const fn gpioa_din23_20(&self) -> &GpioaDin23_20 {
        &self.gpioa_din23_20
    }
    #[doc = "0x1318 - Data input 27 to 24"]
    #[inline(always)]
    pub const fn gpioa_din27_24(&self) -> &GpioaDin27_24 {
        &self.gpioa_din27_24
    }
    #[doc = "0x131c - Data input 31 to 28"]
    #[inline(always)]
    pub const fn gpioa_din31_28(&self) -> &GpioaDin31_28 {
        &self.gpioa_din31_28
    }
    #[doc = "0x1380 - Data input 31 to 0"]
    #[inline(always)]
    pub const fn gpioa_din31_0(&self) -> &GpioaDin31_0 {
        &self.gpioa_din31_0
    }
    #[doc = "0x1390 - Polarity 15 to 0"]
    #[inline(always)]
    pub const fn gpioa_polarity15_0(&self) -> &GpioaPolarity15_0 {
        &self.gpioa_polarity15_0
    }
    #[doc = "0x13a0 - Polarity 31 to 16"]
    #[inline(always)]
    pub const fn gpioa_polarity31_16(&self) -> &GpioaPolarity31_16 {
        &self.gpioa_polarity31_16
    }
    #[doc = "0x1400 - FAST WAKE GLOBAL EN"]
    #[inline(always)]
    pub const fn gpioa_ctl(&self) -> &GpioaCtl {
        &self.gpioa_ctl
    }
    #[doc = "0x1404 - FAST WAKE ENABLE"]
    #[inline(always)]
    pub const fn gpioa_fastwake(&self) -> &GpioaFastwake {
        &self.gpioa_fastwake
    }
    #[doc = "0x1500 - Subscriber 0 configuration"]
    #[inline(always)]
    pub const fn gpioa_sub0cfg(&self) -> &GpioaSub0cfg {
        &self.gpioa_sub0cfg
    }
    #[doc = "0x1508 - Filter Enable 15 to 0"]
    #[inline(always)]
    pub const fn gpioa_filteren15_0(&self) -> &GpioaFilteren15_0 {
        &self.gpioa_filteren15_0
    }
    #[doc = "0x150c - Filter Enable 31 to 16"]
    #[inline(always)]
    pub const fn gpioa_filteren31_16(&self) -> &GpioaFilteren31_16 {
        &self.gpioa_filteren31_16
    }
    #[doc = "0x1510 - DMA Write MASK"]
    #[inline(always)]
    pub const fn gpioa_dmamask(&self) -> &GpioaDmamask {
        &self.gpioa_dmamask
    }
    #[doc = "0x1520 - Subscriber 1 configuration"]
    #[inline(always)]
    pub const fn gpioa_sub1cfg(&self) -> &GpioaSub1cfg {
        &self.gpioa_sub1cfg
    }
}
#[doc = "GPIOA_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_fsub_0`] module"]
#[doc(alias = "GPIOA_FSUB_0")]
pub type GpioaFsub0 = crate::Reg<gpioa_fsub_0::GpioaFsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod gpioa_fsub_0;
#[doc = "GPIOA_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_fsub_1`] module"]
#[doc(alias = "GPIOA_FSUB_1")]
pub type GpioaFsub1 = crate::Reg<gpioa_fsub_1::GpioaFsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod gpioa_fsub_1;
#[doc = "GPIOA_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_fpub_0`] module"]
#[doc(alias = "GPIOA_FPUB_0")]
pub type GpioaFpub0 = crate::Reg<gpioa_fpub_0::GpioaFpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod gpioa_fpub_0;
#[doc = "GPIOA_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_fpub_1`] module"]
#[doc(alias = "GPIOA_FPUB_1")]
pub type GpioaFpub1 = crate::Reg<gpioa_fpub_1::GpioaFpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod gpioa_fpub_1;
#[doc = "GPIOA_GPRCM\\[%s\\]"]
pub use self::gpioa_gprcm::GpioaGprcm;
#[doc = r"Cluster"]
#[doc = "GPIOA_GPRCM\\[%s\\]"]
pub mod gpioa_gprcm;
#[doc = "GPIOA_CLKOVR (rw) register accessor: Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_clkovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_clkovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_clkovr`] module"]
#[doc(alias = "GPIOA_CLKOVR")]
pub type GpioaClkovr = crate::Reg<gpioa_clkovr::GpioaClkovrSpec>;
#[doc = "Clock Override"]
pub mod gpioa_clkovr;
#[doc = "GPIOA_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_pdbgctl`] module"]
#[doc(alias = "GPIOA_PDBGCTL")]
pub type GpioaPdbgctl = crate::Reg<gpioa_pdbgctl::GpioaPdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod gpioa_pdbgctl;
#[doc = "GPIOA_CPU_INT\\[%s\\]"]
pub use self::gpioa_cpu_int::GpioaCpuInt;
#[doc = r"Cluster"]
#[doc = "GPIOA_CPU_INT\\[%s\\]"]
pub mod gpioa_cpu_int;
#[doc = "GPIOA_GEN_EVENT0\\[%s\\]"]
pub use self::gpioa_gen_event0::GpioaGenEvent0;
#[doc = r"Cluster"]
#[doc = "GPIOA_GEN_EVENT0\\[%s\\]"]
pub mod gpioa_gen_event0;
#[doc = "GPIOA_GEN_EVENT1\\[%s\\]"]
pub use self::gpioa_gen_event1::GpioaGenEvent1;
#[doc = r"Cluster"]
#[doc = "GPIOA_GEN_EVENT1\\[%s\\]"]
pub mod gpioa_gen_event1;
#[doc = "GPIOA_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_evt_mode`] module"]
#[doc(alias = "GPIOA_EVT_MODE")]
pub type GpioaEvtMode = crate::Reg<gpioa_evt_mode::GpioaEvtModeSpec>;
#[doc = "Event Mode"]
pub mod gpioa_evt_mode;
#[doc = "GPIOA_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_desc`] module"]
#[doc(alias = "GPIOA_DESC")]
pub type GpioaDesc = crate::Reg<gpioa_desc::GpioaDescSpec>;
#[doc = "Module Description"]
pub mod gpioa_desc;
#[doc = "GPIOA_DOUT3_0 (w) register accessor: Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout3_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout3_0`] module"]
#[doc(alias = "GPIOA_DOUT3_0")]
pub type GpioaDout3_0 = crate::Reg<gpioa_dout3_0::GpioaDout3_0Spec>;
#[doc = "Data output 3 to 0"]
pub mod gpioa_dout3_0;
#[doc = "GPIOA_DOUT7_4 (w) register accessor: Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout7_4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout7_4`] module"]
#[doc(alias = "GPIOA_DOUT7_4")]
pub type GpioaDout7_4 = crate::Reg<gpioa_dout7_4::GpioaDout7_4Spec>;
#[doc = "Data output 7 to 4"]
pub mod gpioa_dout7_4;
#[doc = "GPIOA_DOUT11_8 (w) register accessor: Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout11_8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout11_8`] module"]
#[doc(alias = "GPIOA_DOUT11_8")]
pub type GpioaDout11_8 = crate::Reg<gpioa_dout11_8::GpioaDout11_8Spec>;
#[doc = "Data output 11 to 8"]
pub mod gpioa_dout11_8;
#[doc = "GPIOA_DOUT15_12 (w) register accessor: Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout15_12::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout15_12`] module"]
#[doc(alias = "GPIOA_DOUT15_12")]
pub type GpioaDout15_12 = crate::Reg<gpioa_dout15_12::GpioaDout15_12Spec>;
#[doc = "Data output 15 to 12"]
pub mod gpioa_dout15_12;
#[doc = "GPIOA_DOUT19_16 (w) register accessor: Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout19_16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout19_16`] module"]
#[doc(alias = "GPIOA_DOUT19_16")]
pub type GpioaDout19_16 = crate::Reg<gpioa_dout19_16::GpioaDout19_16Spec>;
#[doc = "Data output 19 to 16"]
pub mod gpioa_dout19_16;
#[doc = "GPIOA_DOUT23_20 (w) register accessor: Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout23_20::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout23_20`] module"]
#[doc(alias = "GPIOA_DOUT23_20")]
pub type GpioaDout23_20 = crate::Reg<gpioa_dout23_20::GpioaDout23_20Spec>;
#[doc = "Data output 23 to 20"]
pub mod gpioa_dout23_20;
#[doc = "GPIOA_DOUT27_24 (w) register accessor: Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout27_24::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout27_24`] module"]
#[doc(alias = "GPIOA_DOUT27_24")]
pub type GpioaDout27_24 = crate::Reg<gpioa_dout27_24::GpioaDout27_24Spec>;
#[doc = "Data output 27 to 24"]
pub mod gpioa_dout27_24;
#[doc = "GPIOA_DOUT31_28 (w) register accessor: Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout31_28::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout31_28`] module"]
#[doc(alias = "GPIOA_DOUT31_28")]
pub type GpioaDout31_28 = crate::Reg<gpioa_dout31_28::GpioaDout31_28Spec>;
#[doc = "Data output 31 to 28"]
pub mod gpioa_dout31_28;
#[doc = "GPIOA_DOUT31_0 (rw) register accessor: Data output 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_dout31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dout31_0`] module"]
#[doc(alias = "GPIOA_DOUT31_0")]
pub type GpioaDout31_0 = crate::Reg<gpioa_dout31_0::GpioaDout31_0Spec>;
#[doc = "Data output 31 to 0"]
pub mod gpioa_dout31_0;
#[doc = "GPIOA_DOUTSET31_0 (w) register accessor: Data output set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_doutset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_doutset31_0`] module"]
#[doc(alias = "GPIOA_DOUTSET31_0")]
pub type GpioaDoutset31_0 = crate::Reg<gpioa_doutset31_0::GpioaDoutset31_0Spec>;
#[doc = "Data output set 31 to 0"]
pub mod gpioa_doutset31_0;
#[doc = "GPIOA_DOUTCLR31_0 (w) register accessor: Data output clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_doutclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_doutclr31_0`] module"]
#[doc(alias = "GPIOA_DOUTCLR31_0")]
pub type GpioaDoutclr31_0 = crate::Reg<gpioa_doutclr31_0::GpioaDoutclr31_0Spec>;
#[doc = "Data output clear 31 to 0"]
pub mod gpioa_doutclr31_0;
#[doc = "GPIOA_DOUTTGL31_0 (w) register accessor: Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_douttgl31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_douttgl31_0`] module"]
#[doc(alias = "GPIOA_DOUTTGL31_0")]
pub type GpioaDouttgl31_0 = crate::Reg<gpioa_douttgl31_0::GpioaDouttgl31_0Spec>;
#[doc = "Data output toggle 31 to 0"]
pub mod gpioa_douttgl31_0;
#[doc = "GPIOA_DOE31_0 (rw) register accessor: Data output enable 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_doe31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_doe31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_doe31_0`] module"]
#[doc(alias = "GPIOA_DOE31_0")]
pub type GpioaDoe31_0 = crate::Reg<gpioa_doe31_0::GpioaDoe31_0Spec>;
#[doc = "Data output enable 31 to 0"]
pub mod gpioa_doe31_0;
#[doc = "GPIOA_DOESET31_0 (w) register accessor: Data output enable set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_doeset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_doeset31_0`] module"]
#[doc(alias = "GPIOA_DOESET31_0")]
pub type GpioaDoeset31_0 = crate::Reg<gpioa_doeset31_0::GpioaDoeset31_0Spec>;
#[doc = "Data output enable set 31 to 0"]
pub mod gpioa_doeset31_0;
#[doc = "GPIOA_DOECLR31_0 (w) register accessor: Data output enable clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_doeclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_doeclr31_0`] module"]
#[doc(alias = "GPIOA_DOECLR31_0")]
pub type GpioaDoeclr31_0 = crate::Reg<gpioa_doeclr31_0::GpioaDoeclr31_0Spec>;
#[doc = "Data output enable clear 31 to 0"]
pub mod gpioa_doeclr31_0;
#[doc = "GPIOA_DIN3_0 (r) register accessor: Data input 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din3_0`] module"]
#[doc(alias = "GPIOA_DIN3_0")]
pub type GpioaDin3_0 = crate::Reg<gpioa_din3_0::GpioaDin3_0Spec>;
#[doc = "Data input 3 to 0"]
pub mod gpioa_din3_0;
#[doc = "GPIOA_DIN7_4 (r) register accessor: Data input 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din7_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din7_4`] module"]
#[doc(alias = "GPIOA_DIN7_4")]
pub type GpioaDin7_4 = crate::Reg<gpioa_din7_4::GpioaDin7_4Spec>;
#[doc = "Data input 7 to 4"]
pub mod gpioa_din7_4;
#[doc = "GPIOA_DIN11_8 (r) register accessor: Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din11_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din11_8`] module"]
#[doc(alias = "GPIOA_DIN11_8")]
pub type GpioaDin11_8 = crate::Reg<gpioa_din11_8::GpioaDin11_8Spec>;
#[doc = "Data input 11 to 8"]
pub mod gpioa_din11_8;
#[doc = "GPIOA_DIN15_12 (r) register accessor: Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din15_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din15_12`] module"]
#[doc(alias = "GPIOA_DIN15_12")]
pub type GpioaDin15_12 = crate::Reg<gpioa_din15_12::GpioaDin15_12Spec>;
#[doc = "Data input 15 to 12"]
pub mod gpioa_din15_12;
#[doc = "GPIOA_DIN19_16 (r) register accessor: Data input 19 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din19_16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din19_16`] module"]
#[doc(alias = "GPIOA_DIN19_16")]
pub type GpioaDin19_16 = crate::Reg<gpioa_din19_16::GpioaDin19_16Spec>;
#[doc = "Data input 19 to 16"]
pub mod gpioa_din19_16;
#[doc = "GPIOA_DIN23_20 (r) register accessor: Data input 23 to 20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din23_20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din23_20`] module"]
#[doc(alias = "GPIOA_DIN23_20")]
pub type GpioaDin23_20 = crate::Reg<gpioa_din23_20::GpioaDin23_20Spec>;
#[doc = "Data input 23 to 20"]
pub mod gpioa_din23_20;
#[doc = "GPIOA_DIN27_24 (r) register accessor: Data input 27 to 24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din27_24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din27_24`] module"]
#[doc(alias = "GPIOA_DIN27_24")]
pub type GpioaDin27_24 = crate::Reg<gpioa_din27_24::GpioaDin27_24Spec>;
#[doc = "Data input 27 to 24"]
pub mod gpioa_din27_24;
#[doc = "GPIOA_DIN31_28 (r) register accessor: Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din31_28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din31_28`] module"]
#[doc(alias = "GPIOA_DIN31_28")]
pub type GpioaDin31_28 = crate::Reg<gpioa_din31_28::GpioaDin31_28Spec>;
#[doc = "Data input 31 to 28"]
pub mod gpioa_din31_28;
#[doc = "GPIOA_DIN31_0 (r) register accessor: Data input 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din31_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_din31_0`] module"]
#[doc(alias = "GPIOA_DIN31_0")]
pub type GpioaDin31_0 = crate::Reg<gpioa_din31_0::GpioaDin31_0Spec>;
#[doc = "Data input 31 to 0"]
pub mod gpioa_din31_0;
#[doc = "GPIOA_POLARITY15_0 (rw) register accessor: Polarity 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_polarity15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_polarity15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_polarity15_0`] module"]
#[doc(alias = "GPIOA_POLARITY15_0")]
pub type GpioaPolarity15_0 = crate::Reg<gpioa_polarity15_0::GpioaPolarity15_0Spec>;
#[doc = "Polarity 15 to 0"]
pub mod gpioa_polarity15_0;
#[doc = "GPIOA_POLARITY31_16 (rw) register accessor: Polarity 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_polarity31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_polarity31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_polarity31_16`] module"]
#[doc(alias = "GPIOA_POLARITY31_16")]
pub type GpioaPolarity31_16 = crate::Reg<gpioa_polarity31_16::GpioaPolarity31_16Spec>;
#[doc = "Polarity 31 to 16"]
pub mod gpioa_polarity31_16;
#[doc = "GPIOA_CTL (rw) register accessor: FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_ctl`] module"]
#[doc(alias = "GPIOA_CTL")]
pub type GpioaCtl = crate::Reg<gpioa_ctl::GpioaCtlSpec>;
#[doc = "FAST WAKE GLOBAL EN"]
pub mod gpioa_ctl;
#[doc = "GPIOA_FASTWAKE (rw) register accessor: FAST WAKE ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_fastwake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_fastwake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_fastwake`] module"]
#[doc(alias = "GPIOA_FASTWAKE")]
pub type GpioaFastwake = crate::Reg<gpioa_fastwake::GpioaFastwakeSpec>;
#[doc = "FAST WAKE ENABLE"]
pub mod gpioa_fastwake;
#[doc = "GPIOA_SUB0CFG (rw) register accessor: Subscriber 0 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_sub0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_sub0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_sub0cfg`] module"]
#[doc(alias = "GPIOA_SUB0CFG")]
pub type GpioaSub0cfg = crate::Reg<gpioa_sub0cfg::GpioaSub0cfgSpec>;
#[doc = "Subscriber 0 configuration"]
pub mod gpioa_sub0cfg;
#[doc = "GPIOA_FILTEREN15_0 (rw) register accessor: Filter Enable 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_filteren15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_filteren15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_filteren15_0`] module"]
#[doc(alias = "GPIOA_FILTEREN15_0")]
pub type GpioaFilteren15_0 = crate::Reg<gpioa_filteren15_0::GpioaFilteren15_0Spec>;
#[doc = "Filter Enable 15 to 0"]
pub mod gpioa_filteren15_0;
#[doc = "GPIOA_FILTEREN31_16 (rw) register accessor: Filter Enable 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_filteren31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_filteren31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_filteren31_16`] module"]
#[doc(alias = "GPIOA_FILTEREN31_16")]
pub type GpioaFilteren31_16 = crate::Reg<gpioa_filteren31_16::GpioaFilteren31_16Spec>;
#[doc = "Filter Enable 31 to 16"]
pub mod gpioa_filteren31_16;
#[doc = "GPIOA_DMAMASK (rw) register accessor: DMA Write MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_dmamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dmamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_dmamask`] module"]
#[doc(alias = "GPIOA_DMAMASK")]
pub type GpioaDmamask = crate::Reg<gpioa_dmamask::GpioaDmamaskSpec>;
#[doc = "DMA Write MASK"]
pub mod gpioa_dmamask;
#[doc = "GPIOA_SUB1CFG (rw) register accessor: Subscriber 1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_sub1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_sub1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa_sub1cfg`] module"]
#[doc(alias = "GPIOA_SUB1CFG")]
pub type GpioaSub1cfg = crate::Reg<gpioa_sub1cfg::GpioaSub1cfgSpec>;
#[doc = "Subscriber 1 configuration"]
pub mod gpioa_sub1cfg;
