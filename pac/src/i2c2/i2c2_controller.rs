#[repr(C)]
#[doc = "I2C2_CONTROLLER\\[%s\\]"]
#[doc(alias = "I2C2_CONTROLLER")]
pub struct I2c2Controller {
    i2c2_csa: I2c2Csa,
    i2c2_cctr: I2c2Cctr,
    i2c2_csr: I2c2Csr,
    i2c2_crxdata: I2c2Crxdata,
    i2c2_ctxdata: I2c2Ctxdata,
    i2c2_ctpr: I2c2Ctpr,
    i2c2_ccr: I2c2Ccr,
    _reserved7: [u8; 0x08],
    i2c2_cbmon: I2c2Cbmon,
    i2c2_cfifoctl: I2c2Cfifoctl,
    i2c2_cfifosr: I2c2Cfifosr,
    i2c2_controller_i2cpecctl: I2c2ControllerI2cpecctl,
    i2c2_controller_pecsr: I2c2ControllerPecsr,
}
impl I2c2Controller {
    #[doc = "0x00 - I2C Controller Target Address Register"]
    #[inline(always)]
    pub const fn i2c2_csa(&self) -> &I2c2Csa {
        &self.i2c2_csa
    }
    #[doc = "0x04 - I2C Controller Control Register"]
    #[inline(always)]
    pub const fn i2c2_cctr(&self) -> &I2c2Cctr {
        &self.i2c2_cctr
    }
    #[doc = "0x08 - I2C Controller Status Register"]
    #[inline(always)]
    pub const fn i2c2_csr(&self) -> &I2c2Csr {
        &self.i2c2_csr
    }
    #[doc = "0x0c - I2C Controller RXData"]
    #[inline(always)]
    pub const fn i2c2_crxdata(&self) -> &I2c2Crxdata {
        &self.i2c2_crxdata
    }
    #[doc = "0x10 - I2C Controller TXData"]
    #[inline(always)]
    pub const fn i2c2_ctxdata(&self) -> &I2c2Ctxdata {
        &self.i2c2_ctxdata
    }
    #[doc = "0x14 - I2C Controller Timer Period"]
    #[inline(always)]
    pub const fn i2c2_ctpr(&self) -> &I2c2Ctpr {
        &self.i2c2_ctpr
    }
    #[doc = "0x18 - I2C Controller Configuration"]
    #[inline(always)]
    pub const fn i2c2_ccr(&self) -> &I2c2Ccr {
        &self.i2c2_ccr
    }
    #[doc = "0x24 - I2C Controller Bus Monitor"]
    #[inline(always)]
    pub const fn i2c2_cbmon(&self) -> &I2c2Cbmon {
        &self.i2c2_cbmon
    }
    #[doc = "0x28 - I2C Controller FIFO Control"]
    #[inline(always)]
    pub const fn i2c2_cfifoctl(&self) -> &I2c2Cfifoctl {
        &self.i2c2_cfifoctl
    }
    #[doc = "0x2c - I2C Controller FIFO Status Register"]
    #[inline(always)]
    pub const fn i2c2_cfifosr(&self) -> &I2c2Cfifosr {
        &self.i2c2_cfifosr
    }
    #[doc = "0x30 - I2C Controller PEC control register"]
    #[inline(always)]
    pub const fn i2c2_controller_i2cpecctl(&self) -> &I2c2ControllerI2cpecctl {
        &self.i2c2_controller_i2cpecctl
    }
    #[doc = "0x34 - I2C Controller PEC status register"]
    #[inline(always)]
    pub const fn i2c2_controller_pecsr(&self) -> &I2c2ControllerPecsr {
        &self.i2c2_controller_pecsr
    }
}
#[doc = "I2C2_CSA (rw) register accessor: I2C Controller Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_csa`] module"]
#[doc(alias = "I2C2_CSA")]
pub type I2c2Csa = crate::Reg<i2c2_csa::I2c2CsaSpec>;
#[doc = "I2C Controller Target Address Register"]
pub mod i2c2_csa;
#[doc = "I2C2_CCTR (rw) register accessor: I2C Controller Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_cctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_cctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_cctr`] module"]
#[doc(alias = "I2C2_CCTR")]
pub type I2c2Cctr = crate::Reg<i2c2_cctr::I2c2CctrSpec>;
#[doc = "I2C Controller Control Register"]
pub mod i2c2_cctr;
#[doc = "I2C2_CSR (r) register accessor: I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_csr`] module"]
#[doc(alias = "I2C2_CSR")]
pub type I2c2Csr = crate::Reg<i2c2_csr::I2c2CsrSpec>;
#[doc = "I2C Controller Status Register"]
pub mod i2c2_csr;
#[doc = "I2C2_CRXDATA (r) register accessor: I2C Controller RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_crxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_crxdata`] module"]
#[doc(alias = "I2C2_CRXDATA")]
pub type I2c2Crxdata = crate::Reg<i2c2_crxdata::I2c2CrxdataSpec>;
#[doc = "I2C Controller RXData"]
pub mod i2c2_crxdata;
#[doc = "I2C2_CTXDATA (rw) register accessor: I2C Controller TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_ctxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_ctxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_ctxdata`] module"]
#[doc(alias = "I2C2_CTXDATA")]
pub type I2c2Ctxdata = crate::Reg<i2c2_ctxdata::I2c2CtxdataSpec>;
#[doc = "I2C Controller TXData"]
pub mod i2c2_ctxdata;
#[doc = "I2C2_CTPR (rw) register accessor: I2C Controller Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_ctpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_ctpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_ctpr`] module"]
#[doc(alias = "I2C2_CTPR")]
pub type I2c2Ctpr = crate::Reg<i2c2_ctpr::I2c2CtprSpec>;
#[doc = "I2C Controller Timer Period"]
pub mod i2c2_ctpr;
#[doc = "I2C2_CCR (rw) register accessor: I2C Controller Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_ccr`] module"]
#[doc(alias = "I2C2_CCR")]
pub type I2c2Ccr = crate::Reg<i2c2_ccr::I2c2CcrSpec>;
#[doc = "I2C Controller Configuration"]
pub mod i2c2_ccr;
#[doc = "I2C2_CBMON (r) register accessor: I2C Controller Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_cbmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_cbmon`] module"]
#[doc(alias = "I2C2_CBMON")]
pub type I2c2Cbmon = crate::Reg<i2c2_cbmon::I2c2CbmonSpec>;
#[doc = "I2C Controller Bus Monitor"]
pub mod i2c2_cbmon;
#[doc = "I2C2_CFIFOCTL (rw) register accessor: I2C Controller FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_cfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_cfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_cfifoctl`] module"]
#[doc(alias = "I2C2_CFIFOCTL")]
pub type I2c2Cfifoctl = crate::Reg<i2c2_cfifoctl::I2c2CfifoctlSpec>;
#[doc = "I2C Controller FIFO Control"]
pub mod i2c2_cfifoctl;
#[doc = "I2C2_CFIFOSR (r) register accessor: I2C Controller FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_cfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_cfifosr`] module"]
#[doc(alias = "I2C2_CFIFOSR")]
pub type I2c2Cfifosr = crate::Reg<i2c2_cfifosr::I2c2CfifosrSpec>;
#[doc = "I2C Controller FIFO Status Register"]
pub mod i2c2_cfifosr;
#[doc = "I2C2_CONTROLLER_I2CPECCTL (rw) register accessor: I2C Controller PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_controller_i2cpecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_controller_i2cpecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_controller_i2cpecctl`] module"]
#[doc(alias = "I2C2_CONTROLLER_I2CPECCTL")]
pub type I2c2ControllerI2cpecctl =
    crate::Reg<i2c2_controller_i2cpecctl::I2c2ControllerI2cpecctlSpec>;
#[doc = "I2C Controller PEC control register"]
pub mod i2c2_controller_i2cpecctl;
#[doc = "I2C2_CONTROLLER_PECSR (r) register accessor: I2C Controller PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_controller_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_controller_pecsr`] module"]
#[doc(alias = "I2C2_CONTROLLER_PECSR")]
pub type I2c2ControllerPecsr = crate::Reg<i2c2_controller_pecsr::I2c2ControllerPecsrSpec>;
#[doc = "I2C Controller PEC status register"]
pub mod i2c2_controller_pecsr;
