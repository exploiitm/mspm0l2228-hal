#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    gpioc_fsub_0: GpiocFsub0,
    gpioc_fsub_1: GpiocFsub1,
    _reserved2: [u8; 0x3c],
    gpioc_fpub_0: GpiocFpub0,
    gpioc_fpub_1: GpiocFpub1,
    _reserved4: [u8; 0x03b4],
    gpioc_gprcm: [GpiocGprcm; 1],
    _reserved5: [u8; 0x07f8],
    gpioc_clkovr: GpiocClkovr,
    _reserved6: [u8; 0x04],
    gpioc_pdbgctl: GpiocPdbgctl,
    _reserved7: [u8; 0x04],
    gpioc_cpu_int: [GpiocCpuInt; 1],
    _reserved8: [u8; 0x04],
    gpioc_gen_event0: [GpiocGenEvent0; 1],
    _reserved9: [u8; 0x04],
    gpioc_gen_event1: [GpiocGenEvent1; 1],
    _reserved10: [u8; 0x34],
    gpioc_evt_mode: GpiocEvtMode,
    _reserved11: [u8; 0x18],
    gpioc_desc: GpiocDesc,
    _reserved12: [u8; 0x0100],
    gpioc_dout3_0: GpiocDout3_0,
    gpioc_dout7_4: GpiocDout7_4,
    gpioc_dout11_8: GpiocDout11_8,
    gpioc_dout15_12: GpiocDout15_12,
    gpioc_dout19_16: GpiocDout19_16,
    gpioc_dout23_20: GpiocDout23_20,
    gpioc_dout27_24: GpiocDout27_24,
    gpioc_dout31_28: GpiocDout31_28,
    _reserved20: [u8; 0x60],
    gpioc_dout31_0: GpiocDout31_0,
    _reserved21: [u8; 0x0c],
    gpioc_doutset31_0: GpiocDoutset31_0,
    _reserved22: [u8; 0x0c],
    gpioc_doutclr31_0: GpiocDoutclr31_0,
    _reserved23: [u8; 0x0c],
    gpioc_douttgl31_0: GpiocDouttgl31_0,
    _reserved24: [u8; 0x0c],
    gpioc_doe31_0: GpiocDoe31_0,
    _reserved25: [u8; 0x0c],
    gpioc_doeset31_0: GpiocDoeset31_0,
    _reserved26: [u8; 0x0c],
    gpioc_doeclr31_0: GpiocDoeclr31_0,
    _reserved27: [u8; 0x1c],
    gpioc_din3_0: GpiocDin3_0,
    gpioc_din7_4: GpiocDin7_4,
    gpioc_din11_8: GpiocDin11_8,
    gpioc_din15_12: GpiocDin15_12,
    gpioc_din19_16: GpiocDin19_16,
    gpioc_din23_20: GpiocDin23_20,
    gpioc_din27_24: GpiocDin27_24,
    gpioc_din31_28: GpiocDin31_28,
    _reserved35: [u8; 0x60],
    gpioc_din31_0: GpiocDin31_0,
    _reserved36: [u8; 0x0c],
    gpioc_polarity15_0: GpiocPolarity15_0,
    _reserved37: [u8; 0x0c],
    gpioc_polarity31_16: GpiocPolarity31_16,
    _reserved38: [u8; 0x5c],
    gpioc_ctl: GpiocCtl,
    gpioc_fastwake: GpiocFastwake,
    _reserved40: [u8; 0xf8],
    gpioc_sub0cfg: GpiocSub0cfg,
    _reserved41: [u8; 0x04],
    gpioc_filteren15_0: GpiocFilteren15_0,
    gpioc_filteren31_16: GpiocFilteren31_16,
    gpioc_dmamask: GpiocDmamask,
    _reserved44: [u8; 0x0c],
    gpioc_sub1cfg: GpiocSub1cfg,
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn gpioc_fsub_0(&self) -> &GpiocFsub0 {
        &self.gpioc_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn gpioc_fsub_1(&self) -> &GpiocFsub1 {
        &self.gpioc_fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn gpioc_fpub_0(&self) -> &GpiocFpub0 {
        &self.gpioc_fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
    #[inline(always)]
    pub const fn gpioc_fpub_1(&self) -> &GpiocFpub1 {
        &self.gpioc_fpub_1
    }
    #[doc = "0x800..0x818 - GPIOC_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioc_gprcm(&self, n: usize) -> &GpiocGprcm {
        &self.gpioc_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - GPIOC_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn gpioc_gprcm_iter(&self) -> impl Iterator<Item = &GpiocGprcm> {
        self.gpioc_gprcm.iter()
    }
    #[doc = "0x1010 - Clock Override"]
    #[inline(always)]
    pub const fn gpioc_clkovr(&self) -> &GpiocClkovr {
        &self.gpioc_clkovr
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn gpioc_pdbgctl(&self) -> &GpiocPdbgctl {
        &self.gpioc_pdbgctl
    }
    #[doc = "0x1020..0x104c - GPIOC_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioc_cpu_int(&self, n: usize) -> &GpiocCpuInt {
        &self.gpioc_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - GPIOC_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn gpioc_cpu_int_iter(&self) -> impl Iterator<Item = &GpiocCpuInt> {
        self.gpioc_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - GPIOC_GEN_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioc_gen_event0(&self, n: usize) -> &GpiocGenEvent0 {
        &self.gpioc_gen_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - GPIOC_GEN_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn gpioc_gen_event0_iter(&self) -> impl Iterator<Item = &GpiocGenEvent0> {
        self.gpioc_gen_event0.iter()
    }
    #[doc = "0x1080..0x10ac - GPIOC_GEN_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn gpioc_gen_event1(&self, n: usize) -> &GpiocGenEvent1 {
        &self.gpioc_gen_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - GPIOC_GEN_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn gpioc_gen_event1_iter(&self) -> impl Iterator<Item = &GpiocGenEvent1> {
        self.gpioc_gen_event1.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn gpioc_evt_mode(&self) -> &GpiocEvtMode {
        &self.gpioc_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn gpioc_desc(&self) -> &GpiocDesc {
        &self.gpioc_desc
    }
    #[doc = "0x1200 - Data output 3 to 0"]
    #[inline(always)]
    pub const fn gpioc_dout3_0(&self) -> &GpiocDout3_0 {
        &self.gpioc_dout3_0
    }
    #[doc = "0x1204 - Data output 7 to 4"]
    #[inline(always)]
    pub const fn gpioc_dout7_4(&self) -> &GpiocDout7_4 {
        &self.gpioc_dout7_4
    }
    #[doc = "0x1208 - Data output 11 to 8"]
    #[inline(always)]
    pub const fn gpioc_dout11_8(&self) -> &GpiocDout11_8 {
        &self.gpioc_dout11_8
    }
    #[doc = "0x120c - Data output 15 to 12"]
    #[inline(always)]
    pub const fn gpioc_dout15_12(&self) -> &GpiocDout15_12 {
        &self.gpioc_dout15_12
    }
    #[doc = "0x1210 - Data output 19 to 16"]
    #[inline(always)]
    pub const fn gpioc_dout19_16(&self) -> &GpiocDout19_16 {
        &self.gpioc_dout19_16
    }
    #[doc = "0x1214 - Data output 23 to 20"]
    #[inline(always)]
    pub const fn gpioc_dout23_20(&self) -> &GpiocDout23_20 {
        &self.gpioc_dout23_20
    }
    #[doc = "0x1218 - Data output 27 to 24"]
    #[inline(always)]
    pub const fn gpioc_dout27_24(&self) -> &GpiocDout27_24 {
        &self.gpioc_dout27_24
    }
    #[doc = "0x121c - Data output 31 to 28"]
    #[inline(always)]
    pub const fn gpioc_dout31_28(&self) -> &GpiocDout31_28 {
        &self.gpioc_dout31_28
    }
    #[doc = "0x1280 - Data output 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_dout31_0(&self) -> &GpiocDout31_0 {
        &self.gpioc_dout31_0
    }
    #[doc = "0x1290 - Data output set 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_doutset31_0(&self) -> &GpiocDoutset31_0 {
        &self.gpioc_doutset31_0
    }
    #[doc = "0x12a0 - Data output clear 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_doutclr31_0(&self) -> &GpiocDoutclr31_0 {
        &self.gpioc_doutclr31_0
    }
    #[doc = "0x12b0 - Data output toggle 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_douttgl31_0(&self) -> &GpiocDouttgl31_0 {
        &self.gpioc_douttgl31_0
    }
    #[doc = "0x12c0 - Data output enable 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_doe31_0(&self) -> &GpiocDoe31_0 {
        &self.gpioc_doe31_0
    }
    #[doc = "0x12d0 - Data output enable set 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_doeset31_0(&self) -> &GpiocDoeset31_0 {
        &self.gpioc_doeset31_0
    }
    #[doc = "0x12e0 - Data output enable clear 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_doeclr31_0(&self) -> &GpiocDoeclr31_0 {
        &self.gpioc_doeclr31_0
    }
    #[doc = "0x1300 - Data input 3 to 0"]
    #[inline(always)]
    pub const fn gpioc_din3_0(&self) -> &GpiocDin3_0 {
        &self.gpioc_din3_0
    }
    #[doc = "0x1304 - Data input 7 to 4"]
    #[inline(always)]
    pub const fn gpioc_din7_4(&self) -> &GpiocDin7_4 {
        &self.gpioc_din7_4
    }
    #[doc = "0x1308 - Data input 11 to 8"]
    #[inline(always)]
    pub const fn gpioc_din11_8(&self) -> &GpiocDin11_8 {
        &self.gpioc_din11_8
    }
    #[doc = "0x130c - Data input 15 to 12"]
    #[inline(always)]
    pub const fn gpioc_din15_12(&self) -> &GpiocDin15_12 {
        &self.gpioc_din15_12
    }
    #[doc = "0x1310 - Data input 19 to 16"]
    #[inline(always)]
    pub const fn gpioc_din19_16(&self) -> &GpiocDin19_16 {
        &self.gpioc_din19_16
    }
    #[doc = "0x1314 - Data input 23 to 20"]
    #[inline(always)]
    pub const fn gpioc_din23_20(&self) -> &GpiocDin23_20 {
        &self.gpioc_din23_20
    }
    #[doc = "0x1318 - Data input 27 to 24"]
    #[inline(always)]
    pub const fn gpioc_din27_24(&self) -> &GpiocDin27_24 {
        &self.gpioc_din27_24
    }
    #[doc = "0x131c - Data input 31 to 28"]
    #[inline(always)]
    pub const fn gpioc_din31_28(&self) -> &GpiocDin31_28 {
        &self.gpioc_din31_28
    }
    #[doc = "0x1380 - Data input 31 to 0"]
    #[inline(always)]
    pub const fn gpioc_din31_0(&self) -> &GpiocDin31_0 {
        &self.gpioc_din31_0
    }
    #[doc = "0x1390 - Polarity 15 to 0"]
    #[inline(always)]
    pub const fn gpioc_polarity15_0(&self) -> &GpiocPolarity15_0 {
        &self.gpioc_polarity15_0
    }
    #[doc = "0x13a0 - Polarity 31 to 16"]
    #[inline(always)]
    pub const fn gpioc_polarity31_16(&self) -> &GpiocPolarity31_16 {
        &self.gpioc_polarity31_16
    }
    #[doc = "0x1400 - FAST WAKE GLOBAL EN"]
    #[inline(always)]
    pub const fn gpioc_ctl(&self) -> &GpiocCtl {
        &self.gpioc_ctl
    }
    #[doc = "0x1404 - FAST WAKE ENABLE"]
    #[inline(always)]
    pub const fn gpioc_fastwake(&self) -> &GpiocFastwake {
        &self.gpioc_fastwake
    }
    #[doc = "0x1500 - Subscriber 0 configuration"]
    #[inline(always)]
    pub const fn gpioc_sub0cfg(&self) -> &GpiocSub0cfg {
        &self.gpioc_sub0cfg
    }
    #[doc = "0x1508 - Filter Enable 15 to 0"]
    #[inline(always)]
    pub const fn gpioc_filteren15_0(&self) -> &GpiocFilteren15_0 {
        &self.gpioc_filteren15_0
    }
    #[doc = "0x150c - Filter Enable 31 to 16"]
    #[inline(always)]
    pub const fn gpioc_filteren31_16(&self) -> &GpiocFilteren31_16 {
        &self.gpioc_filteren31_16
    }
    #[doc = "0x1510 - DMA Write MASK"]
    #[inline(always)]
    pub const fn gpioc_dmamask(&self) -> &GpiocDmamask {
        &self.gpioc_dmamask
    }
    #[doc = "0x1520 - Subscriber 1 configuration"]
    #[inline(always)]
    pub const fn gpioc_sub1cfg(&self) -> &GpiocSub1cfg {
        &self.gpioc_sub1cfg
    }
}
#[doc = "GPIOC_FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_fsub_0`] module"]
#[doc(alias = "GPIOC_FSUB_0")]
pub type GpiocFsub0 = crate::Reg<gpioc_fsub_0::GpiocFsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod gpioc_fsub_0;
#[doc = "GPIOC_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_fsub_1`] module"]
#[doc(alias = "GPIOC_FSUB_1")]
pub type GpiocFsub1 = crate::Reg<gpioc_fsub_1::GpiocFsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod gpioc_fsub_1;
#[doc = "GPIOC_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_fpub_0`] module"]
#[doc(alias = "GPIOC_FPUB_0")]
pub type GpiocFpub0 = crate::Reg<gpioc_fpub_0::GpiocFpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod gpioc_fpub_0;
#[doc = "GPIOC_FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_fpub_1`] module"]
#[doc(alias = "GPIOC_FPUB_1")]
pub type GpiocFpub1 = crate::Reg<gpioc_fpub_1::GpiocFpub1Spec>;
#[doc = "Publisher Port 1"]
pub mod gpioc_fpub_1;
#[doc = "GPIOC_GPRCM\\[%s\\]"]
pub use self::gpioc_gprcm::GpiocGprcm;
#[doc = r"Cluster"]
#[doc = "GPIOC_GPRCM\\[%s\\]"]
pub mod gpioc_gprcm;
#[doc = "GPIOC_CLKOVR (rw) register accessor: Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_clkovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_clkovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_clkovr`] module"]
#[doc(alias = "GPIOC_CLKOVR")]
pub type GpiocClkovr = crate::Reg<gpioc_clkovr::GpiocClkovrSpec>;
#[doc = "Clock Override"]
pub mod gpioc_clkovr;
#[doc = "GPIOC_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_pdbgctl`] module"]
#[doc(alias = "GPIOC_PDBGCTL")]
pub type GpiocPdbgctl = crate::Reg<gpioc_pdbgctl::GpiocPdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod gpioc_pdbgctl;
#[doc = "GPIOC_CPU_INT\\[%s\\]"]
pub use self::gpioc_cpu_int::GpiocCpuInt;
#[doc = r"Cluster"]
#[doc = "GPIOC_CPU_INT\\[%s\\]"]
pub mod gpioc_cpu_int;
#[doc = "GPIOC_GEN_EVENT0\\[%s\\]"]
pub use self::gpioc_gen_event0::GpiocGenEvent0;
#[doc = r"Cluster"]
#[doc = "GPIOC_GEN_EVENT0\\[%s\\]"]
pub mod gpioc_gen_event0;
#[doc = "GPIOC_GEN_EVENT1\\[%s\\]"]
pub use self::gpioc_gen_event1::GpiocGenEvent1;
#[doc = r"Cluster"]
#[doc = "GPIOC_GEN_EVENT1\\[%s\\]"]
pub mod gpioc_gen_event1;
#[doc = "GPIOC_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_evt_mode`] module"]
#[doc(alias = "GPIOC_EVT_MODE")]
pub type GpiocEvtMode = crate::Reg<gpioc_evt_mode::GpiocEvtModeSpec>;
#[doc = "Event Mode"]
pub mod gpioc_evt_mode;
#[doc = "GPIOC_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_desc`] module"]
#[doc(alias = "GPIOC_DESC")]
pub type GpiocDesc = crate::Reg<gpioc_desc::GpiocDescSpec>;
#[doc = "Module Description"]
pub mod gpioc_desc;
#[doc = "GPIOC_DOUT3_0 (w) register accessor: Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout3_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout3_0`] module"]
#[doc(alias = "GPIOC_DOUT3_0")]
pub type GpiocDout3_0 = crate::Reg<gpioc_dout3_0::GpiocDout3_0Spec>;
#[doc = "Data output 3 to 0"]
pub mod gpioc_dout3_0;
#[doc = "GPIOC_DOUT7_4 (w) register accessor: Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout7_4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout7_4`] module"]
#[doc(alias = "GPIOC_DOUT7_4")]
pub type GpiocDout7_4 = crate::Reg<gpioc_dout7_4::GpiocDout7_4Spec>;
#[doc = "Data output 7 to 4"]
pub mod gpioc_dout7_4;
#[doc = "GPIOC_DOUT11_8 (w) register accessor: Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout11_8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout11_8`] module"]
#[doc(alias = "GPIOC_DOUT11_8")]
pub type GpiocDout11_8 = crate::Reg<gpioc_dout11_8::GpiocDout11_8Spec>;
#[doc = "Data output 11 to 8"]
pub mod gpioc_dout11_8;
#[doc = "GPIOC_DOUT15_12 (w) register accessor: Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout15_12::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout15_12`] module"]
#[doc(alias = "GPIOC_DOUT15_12")]
pub type GpiocDout15_12 = crate::Reg<gpioc_dout15_12::GpiocDout15_12Spec>;
#[doc = "Data output 15 to 12"]
pub mod gpioc_dout15_12;
#[doc = "GPIOC_DOUT19_16 (w) register accessor: Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout19_16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout19_16`] module"]
#[doc(alias = "GPIOC_DOUT19_16")]
pub type GpiocDout19_16 = crate::Reg<gpioc_dout19_16::GpiocDout19_16Spec>;
#[doc = "Data output 19 to 16"]
pub mod gpioc_dout19_16;
#[doc = "GPIOC_DOUT23_20 (w) register accessor: Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout23_20::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout23_20`] module"]
#[doc(alias = "GPIOC_DOUT23_20")]
pub type GpiocDout23_20 = crate::Reg<gpioc_dout23_20::GpiocDout23_20Spec>;
#[doc = "Data output 23 to 20"]
pub mod gpioc_dout23_20;
#[doc = "GPIOC_DOUT27_24 (w) register accessor: Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout27_24::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout27_24`] module"]
#[doc(alias = "GPIOC_DOUT27_24")]
pub type GpiocDout27_24 = crate::Reg<gpioc_dout27_24::GpiocDout27_24Spec>;
#[doc = "Data output 27 to 24"]
pub mod gpioc_dout27_24;
#[doc = "GPIOC_DOUT31_28 (w) register accessor: Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout31_28::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout31_28`] module"]
#[doc(alias = "GPIOC_DOUT31_28")]
pub type GpiocDout31_28 = crate::Reg<gpioc_dout31_28::GpiocDout31_28Spec>;
#[doc = "Data output 31 to 28"]
pub mod gpioc_dout31_28;
#[doc = "GPIOC_DOUT31_0 (rw) register accessor: Data output 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_dout31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dout31_0`] module"]
#[doc(alias = "GPIOC_DOUT31_0")]
pub type GpiocDout31_0 = crate::Reg<gpioc_dout31_0::GpiocDout31_0Spec>;
#[doc = "Data output 31 to 0"]
pub mod gpioc_dout31_0;
#[doc = "GPIOC_DOUTSET31_0 (w) register accessor: Data output set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_doutset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_doutset31_0`] module"]
#[doc(alias = "GPIOC_DOUTSET31_0")]
pub type GpiocDoutset31_0 = crate::Reg<gpioc_doutset31_0::GpiocDoutset31_0Spec>;
#[doc = "Data output set 31 to 0"]
pub mod gpioc_doutset31_0;
#[doc = "GPIOC_DOUTCLR31_0 (w) register accessor: Data output clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_doutclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_doutclr31_0`] module"]
#[doc(alias = "GPIOC_DOUTCLR31_0")]
pub type GpiocDoutclr31_0 = crate::Reg<gpioc_doutclr31_0::GpiocDoutclr31_0Spec>;
#[doc = "Data output clear 31 to 0"]
pub mod gpioc_doutclr31_0;
#[doc = "GPIOC_DOUTTGL31_0 (w) register accessor: Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_douttgl31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_douttgl31_0`] module"]
#[doc(alias = "GPIOC_DOUTTGL31_0")]
pub type GpiocDouttgl31_0 = crate::Reg<gpioc_douttgl31_0::GpiocDouttgl31_0Spec>;
#[doc = "Data output toggle 31 to 0"]
pub mod gpioc_douttgl31_0;
#[doc = "GPIOC_DOE31_0 (rw) register accessor: Data output enable 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_doe31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_doe31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_doe31_0`] module"]
#[doc(alias = "GPIOC_DOE31_0")]
pub type GpiocDoe31_0 = crate::Reg<gpioc_doe31_0::GpiocDoe31_0Spec>;
#[doc = "Data output enable 31 to 0"]
pub mod gpioc_doe31_0;
#[doc = "GPIOC_DOESET31_0 (w) register accessor: Data output enable set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_doeset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_doeset31_0`] module"]
#[doc(alias = "GPIOC_DOESET31_0")]
pub type GpiocDoeset31_0 = crate::Reg<gpioc_doeset31_0::GpiocDoeset31_0Spec>;
#[doc = "Data output enable set 31 to 0"]
pub mod gpioc_doeset31_0;
#[doc = "GPIOC_DOECLR31_0 (w) register accessor: Data output enable clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_doeclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_doeclr31_0`] module"]
#[doc(alias = "GPIOC_DOECLR31_0")]
pub type GpiocDoeclr31_0 = crate::Reg<gpioc_doeclr31_0::GpiocDoeclr31_0Spec>;
#[doc = "Data output enable clear 31 to 0"]
pub mod gpioc_doeclr31_0;
#[doc = "GPIOC_DIN3_0 (r) register accessor: Data input 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din3_0`] module"]
#[doc(alias = "GPIOC_DIN3_0")]
pub type GpiocDin3_0 = crate::Reg<gpioc_din3_0::GpiocDin3_0Spec>;
#[doc = "Data input 3 to 0"]
pub mod gpioc_din3_0;
#[doc = "GPIOC_DIN7_4 (r) register accessor: Data input 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din7_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din7_4`] module"]
#[doc(alias = "GPIOC_DIN7_4")]
pub type GpiocDin7_4 = crate::Reg<gpioc_din7_4::GpiocDin7_4Spec>;
#[doc = "Data input 7 to 4"]
pub mod gpioc_din7_4;
#[doc = "GPIOC_DIN11_8 (r) register accessor: Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din11_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din11_8`] module"]
#[doc(alias = "GPIOC_DIN11_8")]
pub type GpiocDin11_8 = crate::Reg<gpioc_din11_8::GpiocDin11_8Spec>;
#[doc = "Data input 11 to 8"]
pub mod gpioc_din11_8;
#[doc = "GPIOC_DIN15_12 (r) register accessor: Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din15_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din15_12`] module"]
#[doc(alias = "GPIOC_DIN15_12")]
pub type GpiocDin15_12 = crate::Reg<gpioc_din15_12::GpiocDin15_12Spec>;
#[doc = "Data input 15 to 12"]
pub mod gpioc_din15_12;
#[doc = "GPIOC_DIN19_16 (r) register accessor: Data input 19 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din19_16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din19_16`] module"]
#[doc(alias = "GPIOC_DIN19_16")]
pub type GpiocDin19_16 = crate::Reg<gpioc_din19_16::GpiocDin19_16Spec>;
#[doc = "Data input 19 to 16"]
pub mod gpioc_din19_16;
#[doc = "GPIOC_DIN23_20 (r) register accessor: Data input 23 to 20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din23_20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din23_20`] module"]
#[doc(alias = "GPIOC_DIN23_20")]
pub type GpiocDin23_20 = crate::Reg<gpioc_din23_20::GpiocDin23_20Spec>;
#[doc = "Data input 23 to 20"]
pub mod gpioc_din23_20;
#[doc = "GPIOC_DIN27_24 (r) register accessor: Data input 27 to 24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din27_24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din27_24`] module"]
#[doc(alias = "GPIOC_DIN27_24")]
pub type GpiocDin27_24 = crate::Reg<gpioc_din27_24::GpiocDin27_24Spec>;
#[doc = "Data input 27 to 24"]
pub mod gpioc_din27_24;
#[doc = "GPIOC_DIN31_28 (r) register accessor: Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din31_28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din31_28`] module"]
#[doc(alias = "GPIOC_DIN31_28")]
pub type GpiocDin31_28 = crate::Reg<gpioc_din31_28::GpiocDin31_28Spec>;
#[doc = "Data input 31 to 28"]
pub mod gpioc_din31_28;
#[doc = "GPIOC_DIN31_0 (r) register accessor: Data input 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din31_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_din31_0`] module"]
#[doc(alias = "GPIOC_DIN31_0")]
pub type GpiocDin31_0 = crate::Reg<gpioc_din31_0::GpiocDin31_0Spec>;
#[doc = "Data input 31 to 0"]
pub mod gpioc_din31_0;
#[doc = "GPIOC_POLARITY15_0 (rw) register accessor: Polarity 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_polarity15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_polarity15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_polarity15_0`] module"]
#[doc(alias = "GPIOC_POLARITY15_0")]
pub type GpiocPolarity15_0 = crate::Reg<gpioc_polarity15_0::GpiocPolarity15_0Spec>;
#[doc = "Polarity 15 to 0"]
pub mod gpioc_polarity15_0;
#[doc = "GPIOC_POLARITY31_16 (rw) register accessor: Polarity 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_polarity31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_polarity31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_polarity31_16`] module"]
#[doc(alias = "GPIOC_POLARITY31_16")]
pub type GpiocPolarity31_16 = crate::Reg<gpioc_polarity31_16::GpiocPolarity31_16Spec>;
#[doc = "Polarity 31 to 16"]
pub mod gpioc_polarity31_16;
#[doc = "GPIOC_CTL (rw) register accessor: FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_ctl`] module"]
#[doc(alias = "GPIOC_CTL")]
pub type GpiocCtl = crate::Reg<gpioc_ctl::GpiocCtlSpec>;
#[doc = "FAST WAKE GLOBAL EN"]
pub mod gpioc_ctl;
#[doc = "GPIOC_FASTWAKE (rw) register accessor: FAST WAKE ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_fastwake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_fastwake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_fastwake`] module"]
#[doc(alias = "GPIOC_FASTWAKE")]
pub type GpiocFastwake = crate::Reg<gpioc_fastwake::GpiocFastwakeSpec>;
#[doc = "FAST WAKE ENABLE"]
pub mod gpioc_fastwake;
#[doc = "GPIOC_SUB0CFG (rw) register accessor: Subscriber 0 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_sub0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_sub0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_sub0cfg`] module"]
#[doc(alias = "GPIOC_SUB0CFG")]
pub type GpiocSub0cfg = crate::Reg<gpioc_sub0cfg::GpiocSub0cfgSpec>;
#[doc = "Subscriber 0 configuration"]
pub mod gpioc_sub0cfg;
#[doc = "GPIOC_FILTEREN15_0 (rw) register accessor: Filter Enable 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_filteren15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_filteren15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_filteren15_0`] module"]
#[doc(alias = "GPIOC_FILTEREN15_0")]
pub type GpiocFilteren15_0 = crate::Reg<gpioc_filteren15_0::GpiocFilteren15_0Spec>;
#[doc = "Filter Enable 15 to 0"]
pub mod gpioc_filteren15_0;
#[doc = "GPIOC_FILTEREN31_16 (rw) register accessor: Filter Enable 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_filteren31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_filteren31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_filteren31_16`] module"]
#[doc(alias = "GPIOC_FILTEREN31_16")]
pub type GpiocFilteren31_16 = crate::Reg<gpioc_filteren31_16::GpiocFilteren31_16Spec>;
#[doc = "Filter Enable 31 to 16"]
pub mod gpioc_filteren31_16;
#[doc = "GPIOC_DMAMASK (rw) register accessor: DMA Write MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_dmamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dmamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_dmamask`] module"]
#[doc(alias = "GPIOC_DMAMASK")]
pub type GpiocDmamask = crate::Reg<gpioc_dmamask::GpiocDmamaskSpec>;
#[doc = "DMA Write MASK"]
pub mod gpioc_dmamask;
#[doc = "GPIOC_SUB1CFG (rw) register accessor: Subscriber 1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_sub1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_sub1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc_sub1cfg`] module"]
#[doc(alias = "GPIOC_SUB1CFG")]
pub type GpiocSub1cfg = crate::Reg<gpioc_sub1cfg::GpiocSub1cfgSpec>;
#[doc = "Subscriber 1 configuration"]
pub mod gpioc_sub1cfg;
