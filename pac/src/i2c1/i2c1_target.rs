#[repr(C)]
#[doc = "I2C1_TARGET\\[%s\\]"]
#[doc(alias = "I2C1_TARGET")]
pub struct I2c1Target {
    i2c1_toar: I2c1Toar,
    i2c1_toar2: I2c1Toar2,
    i2c1_tctr: I2c1Tctr,
    i2c1_tsr: I2c1Tsr,
    i2c1_trxdata: I2c1Trxdata,
    i2c1_ttxdata: I2c1Ttxdata,
    i2c1_tackctl: I2c1Tackctl,
    i2c1_tfifoctl: I2c1Tfifoctl,
    i2c1_tfifosr: I2c1Tfifosr,
    i2c1_target_pecctl: I2c1TargetPecctl,
    i2c1_target_pecsr: I2c1TargetPecsr,
}
impl I2c1Target {
    #[doc = "0x00 - I2C Target Own Address"]
    #[inline(always)]
    pub const fn i2c1_toar(&self) -> &I2c1Toar {
        &self.i2c1_toar
    }
    #[doc = "0x04 - I2C Target Own Address 2"]
    #[inline(always)]
    pub const fn i2c1_toar2(&self) -> &I2c1Toar2 {
        &self.i2c1_toar2
    }
    #[doc = "0x08 - I2C Target Control Register"]
    #[inline(always)]
    pub const fn i2c1_tctr(&self) -> &I2c1Tctr {
        &self.i2c1_tctr
    }
    #[doc = "0x0c - I2C Target Status Register"]
    #[inline(always)]
    pub const fn i2c1_tsr(&self) -> &I2c1Tsr {
        &self.i2c1_tsr
    }
    #[doc = "0x10 - I2C Target RXData"]
    #[inline(always)]
    pub const fn i2c1_trxdata(&self) -> &I2c1Trxdata {
        &self.i2c1_trxdata
    }
    #[doc = "0x14 - I2C Target TXData"]
    #[inline(always)]
    pub const fn i2c1_ttxdata(&self) -> &I2c1Ttxdata {
        &self.i2c1_ttxdata
    }
    #[doc = "0x18 - I2C Target ACK Control"]
    #[inline(always)]
    pub const fn i2c1_tackctl(&self) -> &I2c1Tackctl {
        &self.i2c1_tackctl
    }
    #[doc = "0x1c - I2C Target FIFO Control"]
    #[inline(always)]
    pub const fn i2c1_tfifoctl(&self) -> &I2c1Tfifoctl {
        &self.i2c1_tfifoctl
    }
    #[doc = "0x20 - I2C Target FIFO Status Register"]
    #[inline(always)]
    pub const fn i2c1_tfifosr(&self) -> &I2c1Tfifosr {
        &self.i2c1_tfifosr
    }
    #[doc = "0x24 - I2C Target PEC control register"]
    #[inline(always)]
    pub const fn i2c1_target_pecctl(&self) -> &I2c1TargetPecctl {
        &self.i2c1_target_pecctl
    }
    #[doc = "0x28 - I2C Target PEC status register"]
    #[inline(always)]
    pub const fn i2c1_target_pecsr(&self) -> &I2c1TargetPecsr {
        &self.i2c1_target_pecsr
    }
}
#[doc = "I2C1_TOAR (rw) register accessor: I2C Target Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_toar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_toar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_toar`] module"]
#[doc(alias = "I2C1_TOAR")]
pub type I2c1Toar = crate::Reg<i2c1_toar::I2c1ToarSpec>;
#[doc = "I2C Target Own Address"]
pub mod i2c1_toar;
#[doc = "I2C1_TOAR2 (rw) register accessor: I2C Target Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_toar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_toar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_toar2`] module"]
#[doc(alias = "I2C1_TOAR2")]
pub type I2c1Toar2 = crate::Reg<i2c1_toar2::I2c1Toar2Spec>;
#[doc = "I2C Target Own Address 2"]
pub mod i2c1_toar2;
#[doc = "I2C1_TCTR (rw) register accessor: I2C Target Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_tctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_tctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_tctr`] module"]
#[doc(alias = "I2C1_TCTR")]
pub type I2c1Tctr = crate::Reg<i2c1_tctr::I2c1TctrSpec>;
#[doc = "I2C Target Control Register"]
pub mod i2c1_tctr;
#[doc = "I2C1_TSR (r) register accessor: I2C Target Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_tsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_tsr`] module"]
#[doc(alias = "I2C1_TSR")]
pub type I2c1Tsr = crate::Reg<i2c1_tsr::I2c1TsrSpec>;
#[doc = "I2C Target Status Register"]
pub mod i2c1_tsr;
#[doc = "I2C1_TRXDATA (r) register accessor: I2C Target RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_trxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_trxdata`] module"]
#[doc(alias = "I2C1_TRXDATA")]
pub type I2c1Trxdata = crate::Reg<i2c1_trxdata::I2c1TrxdataSpec>;
#[doc = "I2C Target RXData"]
pub mod i2c1_trxdata;
#[doc = "I2C1_TTXDATA (rw) register accessor: I2C Target TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ttxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ttxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ttxdata`] module"]
#[doc(alias = "I2C1_TTXDATA")]
pub type I2c1Ttxdata = crate::Reg<i2c1_ttxdata::I2c1TtxdataSpec>;
#[doc = "I2C Target TXData"]
pub mod i2c1_ttxdata;
#[doc = "I2C1_TACKCTL (rw) register accessor: I2C Target ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_tackctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_tackctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_tackctl`] module"]
#[doc(alias = "I2C1_TACKCTL")]
pub type I2c1Tackctl = crate::Reg<i2c1_tackctl::I2c1TackctlSpec>;
#[doc = "I2C Target ACK Control"]
pub mod i2c1_tackctl;
#[doc = "I2C1_TFIFOCTL (rw) register accessor: I2C Target FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_tfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_tfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_tfifoctl`] module"]
#[doc(alias = "I2C1_TFIFOCTL")]
pub type I2c1Tfifoctl = crate::Reg<i2c1_tfifoctl::I2c1TfifoctlSpec>;
#[doc = "I2C Target FIFO Control"]
pub mod i2c1_tfifoctl;
#[doc = "I2C1_TFIFOSR (r) register accessor: I2C Target FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_tfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_tfifosr`] module"]
#[doc(alias = "I2C1_TFIFOSR")]
pub type I2c1Tfifosr = crate::Reg<i2c1_tfifosr::I2c1TfifosrSpec>;
#[doc = "I2C Target FIFO Status Register"]
pub mod i2c1_tfifosr;
#[doc = "I2C1_TARGET_PECCTL (rw) register accessor: I2C Target PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_target_pecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_target_pecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_target_pecctl`] module"]
#[doc(alias = "I2C1_TARGET_PECCTL")]
pub type I2c1TargetPecctl = crate::Reg<i2c1_target_pecctl::I2c1TargetPecctlSpec>;
#[doc = "I2C Target PEC control register"]
pub mod i2c1_target_pecctl;
#[doc = "I2C1_TARGET_PECSR (r) register accessor: I2C Target PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_target_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_target_pecsr`] module"]
#[doc(alias = "I2C1_TARGET_PECSR")]
pub type I2c1TargetPecsr = crate::Reg<i2c1_target_pecsr::I2c1TargetPecsrSpec>;
#[doc = "I2C Target PEC status register"]
pub mod i2c1_target_pecsr;
