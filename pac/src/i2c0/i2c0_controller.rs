#[repr(C)]
#[doc = "I2C0_CONTROLLER\\[%s\\]"]
#[doc(alias = "I2C0_CONTROLLER")]
pub struct I2c0Controller {
    i2c0_csa: I2c0Csa,
    i2c0_cctr: I2c0Cctr,
    i2c0_csr: I2c0Csr,
    i2c0_crxdata: I2c0Crxdata,
    i2c0_ctxdata: I2c0Ctxdata,
    i2c0_ctpr: I2c0Ctpr,
    i2c0_ccr: I2c0Ccr,
    _reserved7: [u8; 0x08],
    i2c0_cbmon: I2c0Cbmon,
    i2c0_cfifoctl: I2c0Cfifoctl,
    i2c0_cfifosr: I2c0Cfifosr,
    i2c0_controller_i2cpecctl: I2c0ControllerI2cpecctl,
    i2c0_controller_pecsr: I2c0ControllerPecsr,
}
impl I2c0Controller {
    #[doc = "0x00 - I2C Controller Target Address Register"]
    #[inline(always)]
    pub const fn i2c0_csa(&self) -> &I2c0Csa {
        &self.i2c0_csa
    }
    #[doc = "0x04 - I2C Controller Control Register"]
    #[inline(always)]
    pub const fn i2c0_cctr(&self) -> &I2c0Cctr {
        &self.i2c0_cctr
    }
    #[doc = "0x08 - I2C Controller Status Register"]
    #[inline(always)]
    pub const fn i2c0_csr(&self) -> &I2c0Csr {
        &self.i2c0_csr
    }
    #[doc = "0x0c - I2C Controller RXData"]
    #[inline(always)]
    pub const fn i2c0_crxdata(&self) -> &I2c0Crxdata {
        &self.i2c0_crxdata
    }
    #[doc = "0x10 - I2C Controller TXData"]
    #[inline(always)]
    pub const fn i2c0_ctxdata(&self) -> &I2c0Ctxdata {
        &self.i2c0_ctxdata
    }
    #[doc = "0x14 - I2C Controller Timer Period"]
    #[inline(always)]
    pub const fn i2c0_ctpr(&self) -> &I2c0Ctpr {
        &self.i2c0_ctpr
    }
    #[doc = "0x18 - I2C Controller Configuration"]
    #[inline(always)]
    pub const fn i2c0_ccr(&self) -> &I2c0Ccr {
        &self.i2c0_ccr
    }
    #[doc = "0x24 - I2C Controller Bus Monitor"]
    #[inline(always)]
    pub const fn i2c0_cbmon(&self) -> &I2c0Cbmon {
        &self.i2c0_cbmon
    }
    #[doc = "0x28 - I2C Controller FIFO Control"]
    #[inline(always)]
    pub const fn i2c0_cfifoctl(&self) -> &I2c0Cfifoctl {
        &self.i2c0_cfifoctl
    }
    #[doc = "0x2c - I2C Controller FIFO Status Register"]
    #[inline(always)]
    pub const fn i2c0_cfifosr(&self) -> &I2c0Cfifosr {
        &self.i2c0_cfifosr
    }
    #[doc = "0x30 - I2C Controller PEC control register"]
    #[inline(always)]
    pub const fn i2c0_controller_i2cpecctl(&self) -> &I2c0ControllerI2cpecctl {
        &self.i2c0_controller_i2cpecctl
    }
    #[doc = "0x34 - I2C Controller PEC status register"]
    #[inline(always)]
    pub const fn i2c0_controller_pecsr(&self) -> &I2c0ControllerPecsr {
        &self.i2c0_controller_pecsr
    }
}
#[doc = "I2C0_CSA (rw) register accessor: I2C Controller Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_csa`] module"]
#[doc(alias = "I2C0_CSA")]
pub type I2c0Csa = crate::Reg<i2c0_csa::I2c0CsaSpec>;
#[doc = "I2C Controller Target Address Register"]
pub mod i2c0_csa;
#[doc = "I2C0_CCTR (rw) register accessor: I2C Controller Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_cctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_cctr`] module"]
#[doc(alias = "I2C0_CCTR")]
pub type I2c0Cctr = crate::Reg<i2c0_cctr::I2c0CctrSpec>;
#[doc = "I2C Controller Control Register"]
pub mod i2c0_cctr;
#[doc = "I2C0_CSR (r) register accessor: I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_csr`] module"]
#[doc(alias = "I2C0_CSR")]
pub type I2c0Csr = crate::Reg<i2c0_csr::I2c0CsrSpec>;
#[doc = "I2C Controller Status Register"]
pub mod i2c0_csr;
#[doc = "I2C0_CRXDATA (r) register accessor: I2C Controller RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_crxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_crxdata`] module"]
#[doc(alias = "I2C0_CRXDATA")]
pub type I2c0Crxdata = crate::Reg<i2c0_crxdata::I2c0CrxdataSpec>;
#[doc = "I2C Controller RXData"]
pub mod i2c0_crxdata;
#[doc = "I2C0_CTXDATA (rw) register accessor: I2C Controller TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctxdata`] module"]
#[doc(alias = "I2C0_CTXDATA")]
pub type I2c0Ctxdata = crate::Reg<i2c0_ctxdata::I2c0CtxdataSpec>;
#[doc = "I2C Controller TXData"]
pub mod i2c0_ctxdata;
#[doc = "I2C0_CTPR (rw) register accessor: I2C Controller Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctpr`] module"]
#[doc(alias = "I2C0_CTPR")]
pub type I2c0Ctpr = crate::Reg<i2c0_ctpr::I2c0CtprSpec>;
#[doc = "I2C Controller Timer Period"]
pub mod i2c0_ctpr;
#[doc = "I2C0_CCR (rw) register accessor: I2C Controller Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ccr`] module"]
#[doc(alias = "I2C0_CCR")]
pub type I2c0Ccr = crate::Reg<i2c0_ccr::I2c0CcrSpec>;
#[doc = "I2C Controller Configuration"]
pub mod i2c0_ccr;
#[doc = "I2C0_CBMON (r) register accessor: I2C Controller Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cbmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_cbmon`] module"]
#[doc(alias = "I2C0_CBMON")]
pub type I2c0Cbmon = crate::Reg<i2c0_cbmon::I2c0CbmonSpec>;
#[doc = "I2C Controller Bus Monitor"]
pub mod i2c0_cbmon;
#[doc = "I2C0_CFIFOCTL (rw) register accessor: I2C Controller FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_cfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_cfifoctl`] module"]
#[doc(alias = "I2C0_CFIFOCTL")]
pub type I2c0Cfifoctl = crate::Reg<i2c0_cfifoctl::I2c0CfifoctlSpec>;
#[doc = "I2C Controller FIFO Control"]
pub mod i2c0_cfifoctl;
#[doc = "I2C0_CFIFOSR (r) register accessor: I2C Controller FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_cfifosr`] module"]
#[doc(alias = "I2C0_CFIFOSR")]
pub type I2c0Cfifosr = crate::Reg<i2c0_cfifosr::I2c0CfifosrSpec>;
#[doc = "I2C Controller FIFO Status Register"]
pub mod i2c0_cfifosr;
#[doc = "I2C0_CONTROLLER_I2CPECCTL (rw) register accessor: I2C Controller PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_controller_i2cpecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_controller_i2cpecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_controller_i2cpecctl`] module"]
#[doc(alias = "I2C0_CONTROLLER_I2CPECCTL")]
pub type I2c0ControllerI2cpecctl =
    crate::Reg<i2c0_controller_i2cpecctl::I2c0ControllerI2cpecctlSpec>;
#[doc = "I2C Controller PEC control register"]
pub mod i2c0_controller_i2cpecctl;
#[doc = "I2C0_CONTROLLER_PECSR (r) register accessor: I2C Controller PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_controller_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_controller_pecsr`] module"]
#[doc(alias = "I2C0_CONTROLLER_PECSR")]
pub type I2c0ControllerPecsr = crate::Reg<i2c0_controller_pecsr::I2c0ControllerPecsrSpec>;
#[doc = "I2C Controller PEC status register"]
pub mod i2c0_controller_pecsr;
