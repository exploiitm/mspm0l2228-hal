#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    trng_gprcm: [TrngGprcm; 1],
    _reserved1: [u8; 0x0808],
    trng_iidx: TrngIidx,
    _reserved2: [u8; 0x04],
    trng_imask: TrngImask,
    _reserved3: [u8; 0x04],
    trng_ris: TrngRis,
    _reserved4: [u8; 0x04],
    trng_mis: TrngMis,
    _reserved5: [u8; 0x04],
    trng_iset: TrngIset,
    _reserved6: [u8; 0x04],
    trng_iclr: TrngIclr,
    _reserved7: [u8; 0xb0],
    trng_desc: TrngDesc,
    trng_ctl: TrngCtl,
    trng_stat: TrngStat,
    trng_data_capture: TrngDataCapture,
    trng_test_results: TrngTestResults,
    trng_clkdivide: TrngClkdivide,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - TRNG_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn trng_gprcm(&self, n: usize) -> &TrngGprcm {
        &self.trng_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - TRNG_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn trng_gprcm_iter(&self) -> impl Iterator<Item = &TrngGprcm> {
        self.trng_gprcm.iter()
    }
    #[doc = "0x1020 - Interrupt index"]
    #[inline(always)]
    pub const fn trng_iidx(&self) -> &TrngIidx {
        &self.trng_iidx
    }
    #[doc = "0x1028 - Interrupt mask"]
    #[inline(always)]
    pub const fn trng_imask(&self) -> &TrngImask {
        &self.trng_imask
    }
    #[doc = "0x1030 - Raw interrupt status"]
    #[inline(always)]
    pub const fn trng_ris(&self) -> &TrngRis {
        &self.trng_ris
    }
    #[doc = "0x1038 - Masked interrupt status"]
    #[inline(always)]
    pub const fn trng_mis(&self) -> &TrngMis {
        &self.trng_mis
    }
    #[doc = "0x1040 - Interrupt set"]
    #[inline(always)]
    pub const fn trng_iset(&self) -> &TrngIset {
        &self.trng_iset
    }
    #[doc = "0x1048 - Interrupt clear"]
    #[inline(always)]
    pub const fn trng_iclr(&self) -> &TrngIclr {
        &self.trng_iclr
    }
    #[doc = "0x10fc - Module descriptions"]
    #[inline(always)]
    pub const fn trng_desc(&self) -> &TrngDesc {
        &self.trng_desc
    }
    #[doc = "0x1100 - Controls the command and decimation rate"]
    #[inline(always)]
    pub const fn trng_ctl(&self) -> &TrngCtl {
        &self.trng_ctl
    }
    #[doc = "0x1104 - Status register that informs health test results and last issued command"]
    #[inline(always)]
    pub const fn trng_stat(&self) -> &TrngStat {
        &self.trng_stat
    }
    #[doc = "0x1108 - Captured word buffer of RNG data"]
    #[inline(always)]
    pub const fn trng_data_capture(&self) -> &TrngDataCapture {
        &self.trng_data_capture
    }
    #[doc = "0x110c - Test results from TEST_ANA and TEST_DIG"]
    #[inline(always)]
    pub const fn trng_test_results(&self) -> &TrngTestResults {
        &self.trng_test_results
    }
    #[doc = "0x1110 - Clock Divider"]
    #[inline(always)]
    pub const fn trng_clkdivide(&self) -> &TrngClkdivide {
        &self.trng_clkdivide
    }
}
#[doc = "TRNG_GPRCM\\[%s\\]"]
pub use self::trng_gprcm::TrngGprcm;
#[doc = r"Cluster"]
#[doc = "TRNG_GPRCM\\[%s\\]"]
pub mod trng_gprcm;
#[doc = "TRNG_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_iidx`] module"]
#[doc(alias = "TRNG_IIDX")]
pub type TrngIidx = crate::Reg<trng_iidx::TrngIidxSpec>;
#[doc = "Interrupt index"]
pub mod trng_iidx;
#[doc = "TRNG_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_imask`] module"]
#[doc(alias = "TRNG_IMASK")]
pub type TrngImask = crate::Reg<trng_imask::TrngImaskSpec>;
#[doc = "Interrupt mask"]
pub mod trng_imask;
#[doc = "TRNG_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_ris`] module"]
#[doc(alias = "TRNG_RIS")]
pub type TrngRis = crate::Reg<trng_ris::TrngRisSpec>;
#[doc = "Raw interrupt status"]
pub mod trng_ris;
#[doc = "TRNG_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_mis`] module"]
#[doc(alias = "TRNG_MIS")]
pub type TrngMis = crate::Reg<trng_mis::TrngMisSpec>;
#[doc = "Masked interrupt status"]
pub mod trng_mis;
#[doc = "TRNG_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_iset`] module"]
#[doc(alias = "TRNG_ISET")]
pub type TrngIset = crate::Reg<trng_iset::TrngIsetSpec>;
#[doc = "Interrupt set"]
pub mod trng_iset;
#[doc = "TRNG_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_iclr`] module"]
#[doc(alias = "TRNG_ICLR")]
pub type TrngIclr = crate::Reg<trng_iclr::TrngIclrSpec>;
#[doc = "Interrupt clear"]
pub mod trng_iclr;
#[doc = "TRNG_DESC (r) register accessor: Module descriptions\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_desc`] module"]
#[doc(alias = "TRNG_DESC")]
pub type TrngDesc = crate::Reg<trng_desc::TrngDescSpec>;
#[doc = "Module descriptions"]
pub mod trng_desc;
#[doc = "TRNG_CTL (rw) register accessor: Controls the command and decimation rate\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_ctl`] module"]
#[doc(alias = "TRNG_CTL")]
pub type TrngCtl = crate::Reg<trng_ctl::TrngCtlSpec>;
#[doc = "Controls the command and decimation rate"]
pub mod trng_ctl;
#[doc = "TRNG_STAT (r) register accessor: Status register that informs health test results and last issued command\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_stat`] module"]
#[doc(alias = "TRNG_STAT")]
pub type TrngStat = crate::Reg<trng_stat::TrngStatSpec>;
#[doc = "Status register that informs health test results and last issued command"]
pub mod trng_stat;
#[doc = "TRNG_DATA_CAPTURE (r) register accessor: Captured word buffer of RNG data\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_data_capture::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_data_capture`] module"]
#[doc(alias = "TRNG_DATA_CAPTURE")]
pub type TrngDataCapture = crate::Reg<trng_data_capture::TrngDataCaptureSpec>;
#[doc = "Captured word buffer of RNG data"]
pub mod trng_data_capture;
#[doc = "TRNG_TEST_RESULTS (r) register accessor: Test results from TEST_ANA and TEST_DIG\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_test_results::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_test_results`] module"]
#[doc(alias = "TRNG_TEST_RESULTS")]
pub type TrngTestResults = crate::Reg<trng_test_results::TrngTestResultsSpec>;
#[doc = "Test results from TEST_ANA and TEST_DIG"]
pub mod trng_test_results;
#[doc = "TRNG_CLKDIVIDE (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_clkdivide::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_clkdivide::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_clkdivide`] module"]
#[doc(alias = "TRNG_CLKDIVIDE")]
pub type TrngClkdivide = crate::Reg<trng_clkdivide::TrngClkdivideSpec>;
#[doc = "Clock Divider"]
pub mod trng_clkdivide;
