#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    gpiob_fsub_0: GpiobFsub0,
    gpiob_fsub_1: GpiobFsub1,
    _reserved2: [u8; 0x3c],
    gpiob_fpub_0: GpiobFpub0,
    gpiob_fpub_1: GpiobFpub1,
    _reserved4: [u8; 0x03b4],
    gpiob_gprcm: [GpiobGprcm; 1],
    _reserved5: [u8; 0x07f8],
    gpiob_clkovr: GpiobClkovr,
    _reserved6: [u8; 0x04],
    gpiob_pdbgctl: GpiobPdbgctl,
    _reserved7: [u8; 0x04],
    gpiob_cpu_int: [GpiobCpuInt; 1],
    _reserved8: [u8; 0x04],
    gpiob_gen_event0: [GpiobGenEvent0; 1],
    _reserved9: [u8; 0x04],
    gpiob_gen_event1: [GpiobGenEvent1; 1],
    _reserved10: [u8; 0x34],
    gpiob_evt_mode: GpiobEvtMode,
    _reserved11: [u8; 0x18],
    gpiob_desc: GpiobDesc,
    _reserved12: [u8; 0x0100],
    gpiob_dout3_0: GpiobDout3_0,
    gpiob_dout7_4: GpiobDout7_4,
    gpiob_dout11_8: GpiobDout11_8,
    gpiob_dout15_12: GpiobDout15_12,
    gpiob_dout19_16: GpiobDout19_16,
    gpiob_dout23_20: GpiobDout23_20,
    gpiob_dout27_24: GpiobDout27_24,
    gpiob_dout31_28: GpiobDout31_28,
    _reserved20: [u8; 0x60],
    gpiob_dout31_0: GpiobDout31_0,
    _reserved21: [u8; 0x0c],
    gpiob_doutset31_0: GpiobDoutset31_0,
    _reserved22: [u8; 0x0c],
    gpiob_doutclr31_0: GpiobDoutclr31_0,
    _reserved23: [u8; 0x0c],
    gpiob_douttgl31_0: GpiobDouttgl31_0,
    _reserved24: [u8; 0x0c],
    gpiob_doe31_0: GpiobDoe31_0,
    _reserved25: [u8; 0x0c],
    gpiob_doeset31_0: GpiobDoeset31_0,
    _reserved26: [u8; 0x0c],
    gpiob_doeclr31_0: GpiobDoeclr31_0,
    _reserved27: [u8; 0x1c],
    gpiob_din3_0: GpiobDin3_0,
    gpiob_din7_4: GpiobDin7_4,
    gpiob_din11_8: GpiobDin11_8,
    gpiob_din15_12: GpiobDin15_12,
    gpiob_din19_16: GpiobDin19_16,
    gpiob_din23_20: GpiobDin23_20,
    gpiob_din27_24: GpiobDin27_24,
    gpiob_din31_28: GpiobDin31_28,
    _reserved35: [u8; 0x60],
    gpiob_din31_0: GpiobDin31_0,
    _reserved36: [u8; 0x0c],
    gpiob_polarity15_0: GpiobPolarity15_0,
    _reserved37: [u8; 0x0c],
    gpiob_polarity31_16: GpiobPolarity31_16,
    _reserved38: [u8; 0x5c],
    gpiob_ctl: GpiobCtl,
    gpiob_fastwake: GpiobFastwake,
    _reserved40: [u8; 0xf8],
    gpiob_sub0cfg: GpiobSub0cfg,
    _reserved41: [u8; 0x04],
    gpiob_filteren15_0: GpiobFilteren15_0,
    gpiob_filteren31_16: GpiobFilteren31_16,
    gpiob_dmamask: GpiobDmamask,
    _reserved44: [u8; 0x0c],
    gpiob_sub1cfg: GpiobSub1cfg,
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn gpiob_fsub_0(&self) -> &GpiobFsub0 {
        &self.gpiob_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn gpiob_fsub_1(&self) -> &GpiobFsub1 {
        &self.gpiob_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn gpiob_fpub_0(&self) -> &GpiobFpub0 {
        &self.gpiob_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn gpiob_fpub_1(&self) -> &GpiobFpub1 {
        &self.gpiob_fpub_1
    }
    #[doc = "0x800..0x818 - GPIOB_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn gpiob_gprcm(&self, n: usize) -> &GpiobGprcm {
        &self.gpiob_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - GPIOB_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn gpiob_gprcm_iter(&self) -> impl Iterator<Item = &GpiobGprcm> {
        self.gpiob_gprcm.iter()
    }
    #[doc = "0x1010 - Clock Override"]
    #[inline(always)]
    pub const fn gpiob_clkovr(&self) -> &GpiobClkovr {
        &self.gpiob_clkovr
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn gpiob_pdbgctl(&self) -> &GpiobPdbgctl {
        &self.gpiob_pdbgctl
    }
    #[doc = "0x1020..0x104c - GPIOB_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn gpiob_cpu_int(&self, n: usize) -> &GpiobCpuInt {
        &self.gpiob_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - GPIOB_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn gpiob_cpu_int_iter(&self) -> impl Iterator<Item = &GpiobCpuInt> {
        self.gpiob_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - GPIOB_GEN_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn gpiob_gen_event0(&self, n: usize) -> &GpiobGenEvent0 {
        &self.gpiob_gen_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - GPIOB_GEN_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn gpiob_gen_event0_iter(&self) -> impl Iterator<Item = &GpiobGenEvent0> {
        self.gpiob_gen_event0.iter()
    }
    #[doc = "0x1080..0x10ac - GPIOB_GEN_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn gpiob_gen_event1(&self, n: usize) -> &GpiobGenEvent1 {
        &self.gpiob_gen_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - GPIOB_GEN_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn gpiob_gen_event1_iter(&self) -> impl Iterator<Item = &GpiobGenEvent1> {
        self.gpiob_gen_event1.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn gpiob_evt_mode(&self) -> &GpiobEvtMode {
        &self.gpiob_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn gpiob_desc(&self) -> &GpiobDesc {
        &self.gpiob_desc
    }
    #[doc = "0x1200 - Data output 3 to 0"]
    #[inline(always)]
    pub const fn gpiob_dout3_0(&self) -> &GpiobDout3_0 {
        &self.gpiob_dout3_0
    }
    #[doc = "0x1204 - Data output 7 to 4"]
    #[inline(always)]
    pub const fn gpiob_dout7_4(&self) -> &GpiobDout7_4 {
        &self.gpiob_dout7_4
    }
    #[doc = "0x1208 - Data output 11 to 8"]
    #[inline(always)]
    pub const fn gpiob_dout11_8(&self) -> &GpiobDout11_8 {
        &self.gpiob_dout11_8
    }
    #[doc = "0x120c - Data output 15 to 12"]
    #[inline(always)]
    pub const fn gpiob_dout15_12(&self) -> &GpiobDout15_12 {
        &self.gpiob_dout15_12
    }
    #[doc = "0x1210 - Data output 19 to 16"]
    #[inline(always)]
    pub const fn gpiob_dout19_16(&self) -> &GpiobDout19_16 {
        &self.gpiob_dout19_16
    }
    #[doc = "0x1214 - Data output 23 to 20"]
    #[inline(always)]
    pub const fn gpiob_dout23_20(&self) -> &GpiobDout23_20 {
        &self.gpiob_dout23_20
    }
    #[doc = "0x1218 - Data output 27 to 24"]
    #[inline(always)]
    pub const fn gpiob_dout27_24(&self) -> &GpiobDout27_24 {
        &self.gpiob_dout27_24
    }
    #[doc = "0x121c - Data output 31 to 28"]
    #[inline(always)]
    pub const fn gpiob_dout31_28(&self) -> &GpiobDout31_28 {
        &self.gpiob_dout31_28
    }
    #[doc = "0x1280 - Data output 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_dout31_0(&self) -> &GpiobDout31_0 {
        &self.gpiob_dout31_0
    }
    #[doc = "0x1290 - Data output set 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_doutset31_0(&self) -> &GpiobDoutset31_0 {
        &self.gpiob_doutset31_0
    }
    #[doc = "0x12a0 - Data output clear 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_doutclr31_0(&self) -> &GpiobDoutclr31_0 {
        &self.gpiob_doutclr31_0
    }
    #[doc = "0x12b0 - Data output toggle 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_douttgl31_0(&self) -> &GpiobDouttgl31_0 {
        &self.gpiob_douttgl31_0
    }
    #[doc = "0x12c0 - Data output enable 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_doe31_0(&self) -> &GpiobDoe31_0 {
        &self.gpiob_doe31_0
    }
    #[doc = "0x12d0 - Data output enable set 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_doeset31_0(&self) -> &GpiobDoeset31_0 {
        &self.gpiob_doeset31_0
    }
    #[doc = "0x12e0 - Data output enable clear 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_doeclr31_0(&self) -> &GpiobDoeclr31_0 {
        &self.gpiob_doeclr31_0
    }
    #[doc = "0x1300 - Data input 3 to 0"]
    #[inline(always)]
    pub const fn gpiob_din3_0(&self) -> &GpiobDin3_0 {
        &self.gpiob_din3_0
    }
    #[doc = "0x1304 - Data input 7 to 4"]
    #[inline(always)]
    pub const fn gpiob_din7_4(&self) -> &GpiobDin7_4 {
        &self.gpiob_din7_4
    }
    #[doc = "0x1308 - Data input 11 to 8"]
    #[inline(always)]
    pub const fn gpiob_din11_8(&self) -> &GpiobDin11_8 {
        &self.gpiob_din11_8
    }
    #[doc = "0x130c - Data input 15 to 12"]
    #[inline(always)]
    pub const fn gpiob_din15_12(&self) -> &GpiobDin15_12 {
        &self.gpiob_din15_12
    }
    #[doc = "0x1310 - Data input 19 to 16"]
    #[inline(always)]
    pub const fn gpiob_din19_16(&self) -> &GpiobDin19_16 {
        &self.gpiob_din19_16
    }
    #[doc = "0x1314 - Data input 23 to 20"]
    #[inline(always)]
    pub const fn gpiob_din23_20(&self) -> &GpiobDin23_20 {
        &self.gpiob_din23_20
    }
    #[doc = "0x1318 - Data input 27 to 24"]
    #[inline(always)]
    pub const fn gpiob_din27_24(&self) -> &GpiobDin27_24 {
        &self.gpiob_din27_24
    }
    #[doc = "0x131c - Data input 31 to 28"]
    #[inline(always)]
    pub const fn gpiob_din31_28(&self) -> &GpiobDin31_28 {
        &self.gpiob_din31_28
    }
    #[doc = "0x1380 - Data input 31 to 0"]
    #[inline(always)]
    pub const fn gpiob_din31_0(&self) -> &GpiobDin31_0 {
        &self.gpiob_din31_0
    }
    #[doc = "0x1390 - Polarity 15 to 0"]
    #[inline(always)]
    pub const fn gpiob_polarity15_0(&self) -> &GpiobPolarity15_0 {
        &self.gpiob_polarity15_0
    }
    #[doc = "0x13a0 - Polarity 31 to 16"]
    #[inline(always)]
    pub const fn gpiob_polarity31_16(&self) -> &GpiobPolarity31_16 {
        &self.gpiob_polarity31_16
    }
    #[doc = "0x1400 - FAST WAKE GLOBAL EN"]
    #[inline(always)]
    pub const fn gpiob_ctl(&self) -> &GpiobCtl {
        &self.gpiob_ctl
    }
    #[doc = "0x1404 - FAST WAKE ENABLE"]
    #[inline(always)]
    pub const fn gpiob_fastwake(&self) -> &GpiobFastwake {
        &self.gpiob_fastwake
    }
    #[doc = "0x1500 - Subscriber 0 configuration"]
    #[inline(always)]
    pub const fn gpiob_sub0cfg(&self) -> &GpiobSub0cfg {
        &self.gpiob_sub0cfg
    }
    #[doc = "0x1508 - Filter Enable 15 to 0"]
    #[inline(always)]
    pub const fn gpiob_filteren15_0(&self) -> &GpiobFilteren15_0 {
        &self.gpiob_filteren15_0
    }
    #[doc = "0x150c - Filter Enable 31 to 16"]
    #[inline(always)]
    pub const fn gpiob_filteren31_16(&self) -> &GpiobFilteren31_16 {
        &self.gpiob_filteren31_16
    }
    #[doc = "0x1510 - DMA Write MASK"]
    #[inline(always)]
    pub const fn gpiob_dmamask(&self) -> &GpiobDmamask {
        &self.gpiob_dmamask
    }
    #[doc = "0x1520 - Subscriber 1 configuration"]
    #[inline(always)]
    pub const fn gpiob_sub1cfg(&self) -> &GpiobSub1cfg {
        &self.gpiob_sub1cfg
    }
}
#[doc = "GPIOB_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_fsub_0`] module"]
#[doc(alias = "GPIOB_FSUB_0")]
pub type GpiobFsub0 = crate::Reg<gpiob_fsub_0::GpiobFsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod gpiob_fsub_0;
#[doc = "GPIOB_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_fsub_1`] module"]
#[doc(alias = "GPIOB_FSUB_1")]
pub type GpiobFsub1 = crate::Reg<gpiob_fsub_1::GpiobFsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod gpiob_fsub_1;
#[doc = "GPIOB_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_fpub_0`] module"]
#[doc(alias = "GPIOB_FPUB_0")]
pub type GpiobFpub0 = crate::Reg<gpiob_fpub_0::GpiobFpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod gpiob_fpub_0;
#[doc = "GPIOB_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_fpub_1`] module"]
#[doc(alias = "GPIOB_FPUB_1")]
pub type GpiobFpub1 = crate::Reg<gpiob_fpub_1::GpiobFpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod gpiob_fpub_1;
#[doc = "GPIOB_GPRCM\\[%s\\]"]
pub use self::gpiob_gprcm::GpiobGprcm;
#[doc = r"Cluster"]
#[doc = "GPIOB_GPRCM\\[%s\\]"]
pub mod gpiob_gprcm;
#[doc = "GPIOB_CLKOVR (rw) register accessor: Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_clkovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_clkovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_clkovr`] module"]
#[doc(alias = "GPIOB_CLKOVR")]
pub type GpiobClkovr = crate::Reg<gpiob_clkovr::GpiobClkovrSpec>;
#[doc = "Clock Override"]
pub mod gpiob_clkovr;
#[doc = "GPIOB_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_pdbgctl`] module"]
#[doc(alias = "GPIOB_PDBGCTL")]
pub type GpiobPdbgctl = crate::Reg<gpiob_pdbgctl::GpiobPdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod gpiob_pdbgctl;
#[doc = "GPIOB_CPU_INT\\[%s\\]"]
pub use self::gpiob_cpu_int::GpiobCpuInt;
#[doc = r"Cluster"]
#[doc = "GPIOB_CPU_INT\\[%s\\]"]
pub mod gpiob_cpu_int;
#[doc = "GPIOB_GEN_EVENT0\\[%s\\]"]
pub use self::gpiob_gen_event0::GpiobGenEvent0;
#[doc = r"Cluster"]
#[doc = "GPIOB_GEN_EVENT0\\[%s\\]"]
pub mod gpiob_gen_event0;
#[doc = "GPIOB_GEN_EVENT1\\[%s\\]"]
pub use self::gpiob_gen_event1::GpiobGenEvent1;
#[doc = r"Cluster"]
#[doc = "GPIOB_GEN_EVENT1\\[%s\\]"]
pub mod gpiob_gen_event1;
#[doc = "GPIOB_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_evt_mode`] module"]
#[doc(alias = "GPIOB_EVT_MODE")]
pub type GpiobEvtMode = crate::Reg<gpiob_evt_mode::GpiobEvtModeSpec>;
#[doc = "Event Mode"]
pub mod gpiob_evt_mode;
#[doc = "GPIOB_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_desc`] module"]
#[doc(alias = "GPIOB_DESC")]
pub type GpiobDesc = crate::Reg<gpiob_desc::GpiobDescSpec>;
#[doc = "Module Description"]
pub mod gpiob_desc;
#[doc = "GPIOB_DOUT3_0 (w) register accessor: Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout3_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout3_0`] module"]
#[doc(alias = "GPIOB_DOUT3_0")]
pub type GpiobDout3_0 = crate::Reg<gpiob_dout3_0::GpiobDout3_0Spec>;
#[doc = "Data output 3 to 0"]
pub mod gpiob_dout3_0;
#[doc = "GPIOB_DOUT7_4 (w) register accessor: Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout7_4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout7_4`] module"]
#[doc(alias = "GPIOB_DOUT7_4")]
pub type GpiobDout7_4 = crate::Reg<gpiob_dout7_4::GpiobDout7_4Spec>;
#[doc = "Data output 7 to 4"]
pub mod gpiob_dout7_4;
#[doc = "GPIOB_DOUT11_8 (w) register accessor: Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout11_8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout11_8`] module"]
#[doc(alias = "GPIOB_DOUT11_8")]
pub type GpiobDout11_8 = crate::Reg<gpiob_dout11_8::GpiobDout11_8Spec>;
#[doc = "Data output 11 to 8"]
pub mod gpiob_dout11_8;
#[doc = "GPIOB_DOUT15_12 (w) register accessor: Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout15_12::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout15_12`] module"]
#[doc(alias = "GPIOB_DOUT15_12")]
pub type GpiobDout15_12 = crate::Reg<gpiob_dout15_12::GpiobDout15_12Spec>;
#[doc = "Data output 15 to 12"]
pub mod gpiob_dout15_12;
#[doc = "GPIOB_DOUT19_16 (w) register accessor: Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout19_16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout19_16`] module"]
#[doc(alias = "GPIOB_DOUT19_16")]
pub type GpiobDout19_16 = crate::Reg<gpiob_dout19_16::GpiobDout19_16Spec>;
#[doc = "Data output 19 to 16"]
pub mod gpiob_dout19_16;
#[doc = "GPIOB_DOUT23_20 (w) register accessor: Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout23_20::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout23_20`] module"]
#[doc(alias = "GPIOB_DOUT23_20")]
pub type GpiobDout23_20 = crate::Reg<gpiob_dout23_20::GpiobDout23_20Spec>;
#[doc = "Data output 23 to 20"]
pub mod gpiob_dout23_20;
#[doc = "GPIOB_DOUT27_24 (w) register accessor: Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout27_24::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout27_24`] module"]
#[doc(alias = "GPIOB_DOUT27_24")]
pub type GpiobDout27_24 = crate::Reg<gpiob_dout27_24::GpiobDout27_24Spec>;
#[doc = "Data output 27 to 24"]
pub mod gpiob_dout27_24;
#[doc = "GPIOB_DOUT31_28 (w) register accessor: Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout31_28::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout31_28`] module"]
#[doc(alias = "GPIOB_DOUT31_28")]
pub type GpiobDout31_28 = crate::Reg<gpiob_dout31_28::GpiobDout31_28Spec>;
#[doc = "Data output 31 to 28"]
pub mod gpiob_dout31_28;
#[doc = "GPIOB_DOUT31_0 (rw) register accessor: Data output 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_dout31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dout31_0`] module"]
#[doc(alias = "GPIOB_DOUT31_0")]
pub type GpiobDout31_0 = crate::Reg<gpiob_dout31_0::GpiobDout31_0Spec>;
#[doc = "Data output 31 to 0"]
pub mod gpiob_dout31_0;
#[doc = "GPIOB_DOUTSET31_0 (w) register accessor: Data output set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_doutset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_doutset31_0`] module"]
#[doc(alias = "GPIOB_DOUTSET31_0")]
pub type GpiobDoutset31_0 = crate::Reg<gpiob_doutset31_0::GpiobDoutset31_0Spec>;
#[doc = "Data output set 31 to 0"]
pub mod gpiob_doutset31_0;
#[doc = "GPIOB_DOUTCLR31_0 (w) register accessor: Data output clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_doutclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_doutclr31_0`] module"]
#[doc(alias = "GPIOB_DOUTCLR31_0")]
pub type GpiobDoutclr31_0 = crate::Reg<gpiob_doutclr31_0::GpiobDoutclr31_0Spec>;
#[doc = "Data output clear 31 to 0"]
pub mod gpiob_doutclr31_0;
#[doc = "GPIOB_DOUTTGL31_0 (w) register accessor: Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_douttgl31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_douttgl31_0`] module"]
#[doc(alias = "GPIOB_DOUTTGL31_0")]
pub type GpiobDouttgl31_0 = crate::Reg<gpiob_douttgl31_0::GpiobDouttgl31_0Spec>;
#[doc = "Data output toggle 31 to 0"]
pub mod gpiob_douttgl31_0;
#[doc = "GPIOB_DOE31_0 (rw) register accessor: Data output enable 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_doe31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_doe31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_doe31_0`] module"]
#[doc(alias = "GPIOB_DOE31_0")]
pub type GpiobDoe31_0 = crate::Reg<gpiob_doe31_0::GpiobDoe31_0Spec>;
#[doc = "Data output enable 31 to 0"]
pub mod gpiob_doe31_0;
#[doc = "GPIOB_DOESET31_0 (w) register accessor: Data output enable set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_doeset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_doeset31_0`] module"]
#[doc(alias = "GPIOB_DOESET31_0")]
pub type GpiobDoeset31_0 = crate::Reg<gpiob_doeset31_0::GpiobDoeset31_0Spec>;
#[doc = "Data output enable set 31 to 0"]
pub mod gpiob_doeset31_0;
#[doc = "GPIOB_DOECLR31_0 (w) register accessor: Data output enable clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_doeclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_doeclr31_0`] module"]
#[doc(alias = "GPIOB_DOECLR31_0")]
pub type GpiobDoeclr31_0 = crate::Reg<gpiob_doeclr31_0::GpiobDoeclr31_0Spec>;
#[doc = "Data output enable clear 31 to 0"]
pub mod gpiob_doeclr31_0;
#[doc = "GPIOB_DIN3_0 (r) register accessor: Data input 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din3_0`] module"]
#[doc(alias = "GPIOB_DIN3_0")]
pub type GpiobDin3_0 = crate::Reg<gpiob_din3_0::GpiobDin3_0Spec>;
#[doc = "Data input 3 to 0"]
pub mod gpiob_din3_0;
#[doc = "GPIOB_DIN7_4 (r) register accessor: Data input 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din7_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din7_4`] module"]
#[doc(alias = "GPIOB_DIN7_4")]
pub type GpiobDin7_4 = crate::Reg<gpiob_din7_4::GpiobDin7_4Spec>;
#[doc = "Data input 7 to 4"]
pub mod gpiob_din7_4;
#[doc = "GPIOB_DIN11_8 (r) register accessor: Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din11_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din11_8`] module"]
#[doc(alias = "GPIOB_DIN11_8")]
pub type GpiobDin11_8 = crate::Reg<gpiob_din11_8::GpiobDin11_8Spec>;
#[doc = "Data input 11 to 8"]
pub mod gpiob_din11_8;
#[doc = "GPIOB_DIN15_12 (r) register accessor: Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din15_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din15_12`] module"]
#[doc(alias = "GPIOB_DIN15_12")]
pub type GpiobDin15_12 = crate::Reg<gpiob_din15_12::GpiobDin15_12Spec>;
#[doc = "Data input 15 to 12"]
pub mod gpiob_din15_12;
#[doc = "GPIOB_DIN19_16 (r) register accessor: Data input 19 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din19_16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din19_16`] module"]
#[doc(alias = "GPIOB_DIN19_16")]
pub type GpiobDin19_16 = crate::Reg<gpiob_din19_16::GpiobDin19_16Spec>;
#[doc = "Data input 19 to 16"]
pub mod gpiob_din19_16;
#[doc = "GPIOB_DIN23_20 (r) register accessor: Data input 23 to 20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din23_20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din23_20`] module"]
#[doc(alias = "GPIOB_DIN23_20")]
pub type GpiobDin23_20 = crate::Reg<gpiob_din23_20::GpiobDin23_20Spec>;
#[doc = "Data input 23 to 20"]
pub mod gpiob_din23_20;
#[doc = "GPIOB_DIN27_24 (r) register accessor: Data input 27 to 24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din27_24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din27_24`] module"]
#[doc(alias = "GPIOB_DIN27_24")]
pub type GpiobDin27_24 = crate::Reg<gpiob_din27_24::GpiobDin27_24Spec>;
#[doc = "Data input 27 to 24"]
pub mod gpiob_din27_24;
#[doc = "GPIOB_DIN31_28 (r) register accessor: Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din31_28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din31_28`] module"]
#[doc(alias = "GPIOB_DIN31_28")]
pub type GpiobDin31_28 = crate::Reg<gpiob_din31_28::GpiobDin31_28Spec>;
#[doc = "Data input 31 to 28"]
pub mod gpiob_din31_28;
#[doc = "GPIOB_DIN31_0 (r) register accessor: Data input 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_din31_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_din31_0`] module"]
#[doc(alias = "GPIOB_DIN31_0")]
pub type GpiobDin31_0 = crate::Reg<gpiob_din31_0::GpiobDin31_0Spec>;
#[doc = "Data input 31 to 0"]
pub mod gpiob_din31_0;
#[doc = "GPIOB_POLARITY15_0 (rw) register accessor: Polarity 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_polarity15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_polarity15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_polarity15_0`] module"]
#[doc(alias = "GPIOB_POLARITY15_0")]
pub type GpiobPolarity15_0 = crate::Reg<gpiob_polarity15_0::GpiobPolarity15_0Spec>;
#[doc = "Polarity 15 to 0"]
pub mod gpiob_polarity15_0;
#[doc = "GPIOB_POLARITY31_16 (rw) register accessor: Polarity 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_polarity31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_polarity31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_polarity31_16`] module"]
#[doc(alias = "GPIOB_POLARITY31_16")]
pub type GpiobPolarity31_16 = crate::Reg<gpiob_polarity31_16::GpiobPolarity31_16Spec>;
#[doc = "Polarity 31 to 16"]
pub mod gpiob_polarity31_16;
#[doc = "GPIOB_CTL (rw) register accessor: FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_ctl`] module"]
#[doc(alias = "GPIOB_CTL")]
pub type GpiobCtl = crate::Reg<gpiob_ctl::GpiobCtlSpec>;
#[doc = "FAST WAKE GLOBAL EN"]
pub mod gpiob_ctl;
#[doc = "GPIOB_FASTWAKE (rw) register accessor: FAST WAKE ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_fastwake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_fastwake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_fastwake`] module"]
#[doc(alias = "GPIOB_FASTWAKE")]
pub type GpiobFastwake = crate::Reg<gpiob_fastwake::GpiobFastwakeSpec>;
#[doc = "FAST WAKE ENABLE"]
pub mod gpiob_fastwake;
#[doc = "GPIOB_SUB0CFG (rw) register accessor: Subscriber 0 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_sub0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_sub0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_sub0cfg`] module"]
#[doc(alias = "GPIOB_SUB0CFG")]
pub type GpiobSub0cfg = crate::Reg<gpiob_sub0cfg::GpiobSub0cfgSpec>;
#[doc = "Subscriber 0 configuration"]
pub mod gpiob_sub0cfg;
#[doc = "GPIOB_FILTEREN15_0 (rw) register accessor: Filter Enable 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_filteren15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_filteren15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_filteren15_0`] module"]
#[doc(alias = "GPIOB_FILTEREN15_0")]
pub type GpiobFilteren15_0 = crate::Reg<gpiob_filteren15_0::GpiobFilteren15_0Spec>;
#[doc = "Filter Enable 15 to 0"]
pub mod gpiob_filteren15_0;
#[doc = "GPIOB_FILTEREN31_16 (rw) register accessor: Filter Enable 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_filteren31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_filteren31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_filteren31_16`] module"]
#[doc(alias = "GPIOB_FILTEREN31_16")]
pub type GpiobFilteren31_16 = crate::Reg<gpiob_filteren31_16::GpiobFilteren31_16Spec>;
#[doc = "Filter Enable 31 to 16"]
pub mod gpiob_filteren31_16;
#[doc = "GPIOB_DMAMASK (rw) register accessor: DMA Write MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_dmamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dmamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_dmamask`] module"]
#[doc(alias = "GPIOB_DMAMASK")]
pub type GpiobDmamask = crate::Reg<gpiob_dmamask::GpiobDmamaskSpec>;
#[doc = "DMA Write MASK"]
pub mod gpiob_dmamask;
#[doc = "GPIOB_SUB1CFG (rw) register accessor: Subscriber 1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_sub1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_sub1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob_sub1cfg`] module"]
#[doc(alias = "GPIOB_SUB1CFG")]
pub type GpiobSub1cfg = crate::Reg<gpiob_sub1cfg::GpiobSub1cfgSpec>;
#[doc = "Subscriber 1 configuration"]
pub mod gpiob_sub1cfg;
