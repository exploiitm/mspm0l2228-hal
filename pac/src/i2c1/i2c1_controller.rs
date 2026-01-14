#[repr(C)]
#[doc = "I2C1_CONTROLLER\\[%s\\]"]
#[doc(alias = "I2C1_CONTROLLER")]
pub struct I2c1Controller {
    i2c1_csa: I2c1Csa,
    i2c1_cctr: I2c1Cctr,
    i2c1_csr: I2c1Csr,
    i2c1_crxdata: I2c1Crxdata,
    i2c1_ctxdata: I2c1Ctxdata,
    i2c1_ctpr: I2c1Ctpr,
    i2c1_ccr: I2c1Ccr,
    _reserved7: [u8; 0x08],
    i2c1_cbmon: I2c1Cbmon,
    i2c1_cfifoctl: I2c1Cfifoctl,
    i2c1_cfifosr: I2c1Cfifosr,
    i2c1_controller_i2cpecctl: I2c1ControllerI2cpecctl,
    i2c1_controller_pecsr: I2c1ControllerPecsr,
}
impl I2c1Controller {
    #[doc = "0x00 - I2C Controller Target Address Register"]
    #[inline(always)]
    pub const fn i2c1_csa(&self) -> &I2c1Csa {
        &self.i2c1_csa
    }
    #[doc = "0x04 - I2C Controller Control Register"]
    #[inline(always)]
    pub const fn i2c1_cctr(&self) -> &I2c1Cctr {
        &self.i2c1_cctr
    }
    #[doc = "0x08 - I2C Controller Status Register"]
    #[inline(always)]
    pub const fn i2c1_csr(&self) -> &I2c1Csr {
        &self.i2c1_csr
    }
    #[doc = "0x0c - I2C Controller RXData"]
    #[inline(always)]
    pub const fn i2c1_crxdata(&self) -> &I2c1Crxdata {
        &self.i2c1_crxdata
    }
    #[doc = "0x10 - I2C Controller TXData"]
    #[inline(always)]
    pub const fn i2c1_ctxdata(&self) -> &I2c1Ctxdata {
        &self.i2c1_ctxdata
    }
    #[doc = "0x14 - I2C Controller Timer Period"]
    #[inline(always)]
    pub const fn i2c1_ctpr(&self) -> &I2c1Ctpr {
        &self.i2c1_ctpr
    }
    #[doc = "0x18 - I2C Controller Configuration"]
    #[inline(always)]
    pub const fn i2c1_ccr(&self) -> &I2c1Ccr {
        &self.i2c1_ccr
    }
    #[doc = "0x24 - I2C Controller Bus Monitor"]
    #[inline(always)]
    pub const fn i2c1_cbmon(&self) -> &I2c1Cbmon {
        &self.i2c1_cbmon
    }
    #[doc = "0x28 - I2C Controller FIFO Control"]
    #[inline(always)]
    pub const fn i2c1_cfifoctl(&self) -> &I2c1Cfifoctl {
        &self.i2c1_cfifoctl
    }
    #[doc = "0x2c - I2C Controller FIFO Status Register"]
    #[inline(always)]
    pub const fn i2c1_cfifosr(&self) -> &I2c1Cfifosr {
        &self.i2c1_cfifosr
    }
    #[doc = "0x30 - I2C Controller PEC control register"]
    #[inline(always)]
    pub const fn i2c1_controller_i2cpecctl(&self) -> &I2c1ControllerI2cpecctl {
        &self.i2c1_controller_i2cpecctl
    }
    #[doc = "0x34 - I2C Controller PEC status register"]
    #[inline(always)]
    pub const fn i2c1_controller_pecsr(&self) -> &I2c1ControllerPecsr {
        &self.i2c1_controller_pecsr
    }
}
#[doc = "I2C1_CSA (rw) register accessor: I2C Controller Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_csa`] module"]
#[doc(alias = "I2C1_CSA")]
pub type I2c1Csa = crate::Reg<i2c1_csa::I2c1CsaSpec>;
#[doc = "I2C Controller Target Address Register"]
pub mod i2c1_csa;
#[doc = "I2C1_CCTR (rw) register accessor: I2C Controller Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_cctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cctr`] module"]
#[doc(alias = "I2C1_CCTR")]
pub type I2c1Cctr = crate::Reg<i2c1_cctr::I2c1CctrSpec>;
#[doc = "I2C Controller Control Register"]
pub mod i2c1_cctr;
#[doc = "I2C1_CSR (r) register accessor: I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_csr`] module"]
#[doc(alias = "I2C1_CSR")]
pub type I2c1Csr = crate::Reg<i2c1_csr::I2c1CsrSpec>;
#[doc = "I2C Controller Status Register"]
pub mod i2c1_csr;
#[doc = "I2C1_CRXDATA (r) register accessor: I2C Controller RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_crxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_crxdata`] module"]
#[doc(alias = "I2C1_CRXDATA")]
pub type I2c1Crxdata = crate::Reg<i2c1_crxdata::I2c1CrxdataSpec>;
#[doc = "I2C Controller RXData"]
pub mod i2c1_crxdata;
#[doc = "I2C1_CTXDATA (rw) register accessor: I2C Controller TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctxdata`] module"]
#[doc(alias = "I2C1_CTXDATA")]
pub type I2c1Ctxdata = crate::Reg<i2c1_ctxdata::I2c1CtxdataSpec>;
#[doc = "I2C Controller TXData"]
pub mod i2c1_ctxdata;
#[doc = "I2C1_CTPR (rw) register accessor: I2C Controller Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctpr`] module"]
#[doc(alias = "I2C1_CTPR")]
pub type I2c1Ctpr = crate::Reg<i2c1_ctpr::I2c1CtprSpec>;
#[doc = "I2C Controller Timer Period"]
pub mod i2c1_ctpr;
#[doc = "I2C1_CCR (rw) register accessor: I2C Controller Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ccr`] module"]
#[doc(alias = "I2C1_CCR")]
pub type I2c1Ccr = crate::Reg<i2c1_ccr::I2c1CcrSpec>;
#[doc = "I2C Controller Configuration"]
pub mod i2c1_ccr;
#[doc = "I2C1_CBMON (r) register accessor: I2C Controller Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cbmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cbmon`] module"]
#[doc(alias = "I2C1_CBMON")]
pub type I2c1Cbmon = crate::Reg<i2c1_cbmon::I2c1CbmonSpec>;
#[doc = "I2C Controller Bus Monitor"]
pub mod i2c1_cbmon;
#[doc = "I2C1_CFIFOCTL (rw) register accessor: I2C Controller FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_cfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cfifoctl`] module"]
#[doc(alias = "I2C1_CFIFOCTL")]
pub type I2c1Cfifoctl = crate::Reg<i2c1_cfifoctl::I2c1CfifoctlSpec>;
#[doc = "I2C Controller FIFO Control"]
pub mod i2c1_cfifoctl;
#[doc = "I2C1_CFIFOSR (r) register accessor: I2C Controller FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_cfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_cfifosr`] module"]
#[doc(alias = "I2C1_CFIFOSR")]
pub type I2c1Cfifosr = crate::Reg<i2c1_cfifosr::I2c1CfifosrSpec>;
#[doc = "I2C Controller FIFO Status Register"]
pub mod i2c1_cfifosr;
#[doc = "I2C1_CONTROLLER_I2CPECCTL (rw) register accessor: I2C Controller PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_controller_i2cpecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_controller_i2cpecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_controller_i2cpecctl`] module"]
#[doc(alias = "I2C1_CONTROLLER_I2CPECCTL")]
pub type I2c1ControllerI2cpecctl =
    crate::Reg<i2c1_controller_i2cpecctl::I2c1ControllerI2cpecctlSpec>;
#[doc = "I2C Controller PEC control register"]
pub mod i2c1_controller_i2cpecctl;
#[doc = "I2C1_CONTROLLER_PECSR (r) register accessor: I2C Controller PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_controller_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_controller_pecsr`] module"]
#[doc(alias = "I2C1_CONTROLLER_PECSR")]
pub type I2c1ControllerPecsr = crate::Reg<i2c1_controller_pecsr::I2c1ControllerPecsrSpec>;
#[doc = "I2C Controller PEC status register"]
pub mod i2c1_controller_pecsr;
