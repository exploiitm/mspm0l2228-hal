#[repr(C)]
#[doc = "I2C2_TARGET\\[%s\\]"]
#[doc(alias = "I2C2_TARGET")]
pub struct I2c2Target {
    i2c2_toar: I2c2Toar,
    i2c2_toar2: I2c2Toar2,
    i2c2_tctr: I2c2Tctr,
    i2c2_tsr: I2c2Tsr,
    i2c2_trxdata: I2c2Trxdata,
    i2c2_ttxdata: I2c2Ttxdata,
    i2c2_tackctl: I2c2Tackctl,
    i2c2_tfifoctl: I2c2Tfifoctl,
    i2c2_tfifosr: I2c2Tfifosr,
    i2c2_target_pecctl: I2c2TargetPecctl,
    i2c2_target_pecsr: I2c2TargetPecsr,
}
impl I2c2Target {
    #[doc = "0x00 - I2C Target Own Address"]
    #[inline(always)]
    pub const fn i2c2_toar(&self) -> &I2c2Toar {
        &self.i2c2_toar
    }
    #[doc = "0x04 - I2C Target Own Address 2"]
    #[inline(always)]
    pub const fn i2c2_toar2(&self) -> &I2c2Toar2 {
        &self.i2c2_toar2
    }
    #[doc = "0x08 - I2C Target Control Register"]
    #[inline(always)]
    pub const fn i2c2_tctr(&self) -> &I2c2Tctr {
        &self.i2c2_tctr
    }
    #[doc = "0x0c - I2C Target Status Register"]
    #[inline(always)]
    pub const fn i2c2_tsr(&self) -> &I2c2Tsr {
        &self.i2c2_tsr
    }
    #[doc = "0x10 - I2C Target RXData"]
    #[inline(always)]
    pub const fn i2c2_trxdata(&self) -> &I2c2Trxdata {
        &self.i2c2_trxdata
    }
    #[doc = "0x14 - I2C Target TXData"]
    #[inline(always)]
    pub const fn i2c2_ttxdata(&self) -> &I2c2Ttxdata {
        &self.i2c2_ttxdata
    }
    #[doc = "0x18 - I2C Target ACK Control"]
    #[inline(always)]
    pub const fn i2c2_tackctl(&self) -> &I2c2Tackctl {
        &self.i2c2_tackctl
    }
    #[doc = "0x1c - I2C Target FIFO Control"]
    #[inline(always)]
    pub const fn i2c2_tfifoctl(&self) -> &I2c2Tfifoctl {
        &self.i2c2_tfifoctl
    }
    #[doc = "0x20 - I2C Target FIFO Status Register"]
    #[inline(always)]
    pub const fn i2c2_tfifosr(&self) -> &I2c2Tfifosr {
        &self.i2c2_tfifosr
    }
    #[doc = "0x24 - I2C Target PEC control register"]
    #[inline(always)]
    pub const fn i2c2_target_pecctl(&self) -> &I2c2TargetPecctl {
        &self.i2c2_target_pecctl
    }
    #[doc = "0x28 - I2C Target PEC status register"]
    #[inline(always)]
    pub const fn i2c2_target_pecsr(&self) -> &I2c2TargetPecsr {
        &self.i2c2_target_pecsr
    }
}
#[doc = "I2C2_TOAR (rw) register accessor: I2C Target Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_toar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_toar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_toar`] module"]
#[doc(alias = "I2C2_TOAR")]
pub type I2c2Toar = crate::Reg<i2c2_toar::I2c2ToarSpec>;
#[doc = "I2C Target Own Address"]
pub mod i2c2_toar;
#[doc = "I2C2_TOAR2 (rw) register accessor: I2C Target Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_toar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_toar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_toar2`] module"]
#[doc(alias = "I2C2_TOAR2")]
pub type I2c2Toar2 = crate::Reg<i2c2_toar2::I2c2Toar2Spec>;
#[doc = "I2C Target Own Address 2"]
pub mod i2c2_toar2;
#[doc = "I2C2_TCTR (rw) register accessor: I2C Target Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_tctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_tctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_tctr`] module"]
#[doc(alias = "I2C2_TCTR")]
pub type I2c2Tctr = crate::Reg<i2c2_tctr::I2c2TctrSpec>;
#[doc = "I2C Target Control Register"]
pub mod i2c2_tctr;
#[doc = "I2C2_TSR (r) register accessor: I2C Target Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_tsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_tsr`] module"]
#[doc(alias = "I2C2_TSR")]
pub type I2c2Tsr = crate::Reg<i2c2_tsr::I2c2TsrSpec>;
#[doc = "I2C Target Status Register"]
pub mod i2c2_tsr;
#[doc = "I2C2_TRXDATA (r) register accessor: I2C Target RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_trxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_trxdata`] module"]
#[doc(alias = "I2C2_TRXDATA")]
pub type I2c2Trxdata = crate::Reg<i2c2_trxdata::I2c2TrxdataSpec>;
#[doc = "I2C Target RXData"]
pub mod i2c2_trxdata;
#[doc = "I2C2_TTXDATA (rw) register accessor: I2C Target TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_ttxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_ttxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_ttxdata`] module"]
#[doc(alias = "I2C2_TTXDATA")]
pub type I2c2Ttxdata = crate::Reg<i2c2_ttxdata::I2c2TtxdataSpec>;
#[doc = "I2C Target TXData"]
pub mod i2c2_ttxdata;
#[doc = "I2C2_TACKCTL (rw) register accessor: I2C Target ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_tackctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_tackctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_tackctl`] module"]
#[doc(alias = "I2C2_TACKCTL")]
pub type I2c2Tackctl = crate::Reg<i2c2_tackctl::I2c2TackctlSpec>;
#[doc = "I2C Target ACK Control"]
pub mod i2c2_tackctl;
#[doc = "I2C2_TFIFOCTL (rw) register accessor: I2C Target FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_tfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_tfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_tfifoctl`] module"]
#[doc(alias = "I2C2_TFIFOCTL")]
pub type I2c2Tfifoctl = crate::Reg<i2c2_tfifoctl::I2c2TfifoctlSpec>;
#[doc = "I2C Target FIFO Control"]
pub mod i2c2_tfifoctl;
#[doc = "I2C2_TFIFOSR (r) register accessor: I2C Target FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_tfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_tfifosr`] module"]
#[doc(alias = "I2C2_TFIFOSR")]
pub type I2c2Tfifosr = crate::Reg<i2c2_tfifosr::I2c2TfifosrSpec>;
#[doc = "I2C Target FIFO Status Register"]
pub mod i2c2_tfifosr;
#[doc = "I2C2_TARGET_PECCTL (rw) register accessor: I2C Target PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_target_pecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_target_pecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_target_pecctl`] module"]
#[doc(alias = "I2C2_TARGET_PECCTL")]
pub type I2c2TargetPecctl = crate::Reg<i2c2_target_pecctl::I2c2TargetPecctlSpec>;
#[doc = "I2C Target PEC control register"]
pub mod i2c2_target_pecctl;
#[doc = "I2C2_TARGET_PECSR (r) register accessor: I2C Target PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_target_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2_target_pecsr`] module"]
#[doc(alias = "I2C2_TARGET_PECSR")]
pub type I2c2TargetPecsr = crate::Reg<i2c2_target_pecsr::I2c2TargetPecsrSpec>;
#[doc = "I2C Target PEC status register"]
pub mod i2c2_target_pecsr;
