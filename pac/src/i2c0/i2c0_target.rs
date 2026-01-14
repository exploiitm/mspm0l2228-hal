#[repr(C)]
#[doc = "I2C0_TARGET\\[%s\\]"]
#[doc(alias = "I2C0_TARGET")]
pub struct I2c0Target {
    i2c0_toar: I2c0Toar,
    i2c0_toar2: I2c0Toar2,
    i2c0_tctr: I2c0Tctr,
    i2c0_tsr: I2c0Tsr,
    i2c0_trxdata: I2c0Trxdata,
    i2c0_ttxdata: I2c0Ttxdata,
    i2c0_tackctl: I2c0Tackctl,
    i2c0_tfifoctl: I2c0Tfifoctl,
    i2c0_tfifosr: I2c0Tfifosr,
    i2c0_target_pecctl: I2c0TargetPecctl,
    i2c0_target_pecsr: I2c0TargetPecsr,
}
impl I2c0Target {
    #[doc = "0x00 - I2C Target Own Address"]
    #[inline(always)]
    pub const fn i2c0_toar(&self) -> &I2c0Toar {
        &self.i2c0_toar
    }
    #[doc = "0x04 - I2C Target Own Address 2"]
    #[inline(always)]
    pub const fn i2c0_toar2(&self) -> &I2c0Toar2 {
        &self.i2c0_toar2
    }
    #[doc = "0x08 - I2C Target Control Register"]
    #[inline(always)]
    pub const fn i2c0_tctr(&self) -> &I2c0Tctr {
        &self.i2c0_tctr
    }
    #[doc = "0x0c - I2C Target Status Register"]
    #[inline(always)]
    pub const fn i2c0_tsr(&self) -> &I2c0Tsr {
        &self.i2c0_tsr
    }
    #[doc = "0x10 - I2C Target RXData"]
    #[inline(always)]
    pub const fn i2c0_trxdata(&self) -> &I2c0Trxdata {
        &self.i2c0_trxdata
    }
    #[doc = "0x14 - I2C Target TXData"]
    #[inline(always)]
    pub const fn i2c0_ttxdata(&self) -> &I2c0Ttxdata {
        &self.i2c0_ttxdata
    }
    #[doc = "0x18 - I2C Target ACK Control"]
    #[inline(always)]
    pub const fn i2c0_tackctl(&self) -> &I2c0Tackctl {
        &self.i2c0_tackctl
    }
    #[doc = "0x1c - I2C Target FIFO Control"]
    #[inline(always)]
    pub const fn i2c0_tfifoctl(&self) -> &I2c0Tfifoctl {
        &self.i2c0_tfifoctl
    }
    #[doc = "0x20 - I2C Target FIFO Status Register"]
    #[inline(always)]
    pub const fn i2c0_tfifosr(&self) -> &I2c0Tfifosr {
        &self.i2c0_tfifosr
    }
    #[doc = "0x24 - I2C Target PEC control register"]
    #[inline(always)]
    pub const fn i2c0_target_pecctl(&self) -> &I2c0TargetPecctl {
        &self.i2c0_target_pecctl
    }
    #[doc = "0x28 - I2C Target PEC status register"]
    #[inline(always)]
    pub const fn i2c0_target_pecsr(&self) -> &I2c0TargetPecsr {
        &self.i2c0_target_pecsr
    }
}
#[doc = "I2C0_TOAR (rw) register accessor: I2C Target Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_toar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_toar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_toar`] module"]
#[doc(alias = "I2C0_TOAR")]
pub type I2c0Toar = crate::Reg<i2c0_toar::I2c0ToarSpec>;
#[doc = "I2C Target Own Address"]
pub mod i2c0_toar;
#[doc = "I2C0_TOAR2 (rw) register accessor: I2C Target Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_toar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_toar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_toar2`] module"]
#[doc(alias = "I2C0_TOAR2")]
pub type I2c0Toar2 = crate::Reg<i2c0_toar2::I2c0Toar2Spec>;
#[doc = "I2C Target Own Address 2"]
pub mod i2c0_toar2;
#[doc = "I2C0_TCTR (rw) register accessor: I2C Target Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_tctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_tctr`] module"]
#[doc(alias = "I2C0_TCTR")]
pub type I2c0Tctr = crate::Reg<i2c0_tctr::I2c0TctrSpec>;
#[doc = "I2C Target Control Register"]
pub mod i2c0_tctr;
#[doc = "I2C0_TSR (r) register accessor: I2C Target Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_tsr`] module"]
#[doc(alias = "I2C0_TSR")]
pub type I2c0Tsr = crate::Reg<i2c0_tsr::I2c0TsrSpec>;
#[doc = "I2C Target Status Register"]
pub mod i2c0_tsr;
#[doc = "I2C0_TRXDATA (r) register accessor: I2C Target RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_trxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_trxdata`] module"]
#[doc(alias = "I2C0_TRXDATA")]
pub type I2c0Trxdata = crate::Reg<i2c0_trxdata::I2c0TrxdataSpec>;
#[doc = "I2C Target RXData"]
pub mod i2c0_trxdata;
#[doc = "I2C0_TTXDATA (rw) register accessor: I2C Target TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ttxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ttxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ttxdata`] module"]
#[doc(alias = "I2C0_TTXDATA")]
pub type I2c0Ttxdata = crate::Reg<i2c0_ttxdata::I2c0TtxdataSpec>;
#[doc = "I2C Target TXData"]
pub mod i2c0_ttxdata;
#[doc = "I2C0_TACKCTL (rw) register accessor: I2C Target ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tackctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_tackctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_tackctl`] module"]
#[doc(alias = "I2C0_TACKCTL")]
pub type I2c0Tackctl = crate::Reg<i2c0_tackctl::I2c0TackctlSpec>;
#[doc = "I2C Target ACK Control"]
pub mod i2c0_tackctl;
#[doc = "I2C0_TFIFOCTL (rw) register accessor: I2C Target FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_tfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_tfifoctl`] module"]
#[doc(alias = "I2C0_TFIFOCTL")]
pub type I2c0Tfifoctl = crate::Reg<i2c0_tfifoctl::I2c0TfifoctlSpec>;
#[doc = "I2C Target FIFO Control"]
pub mod i2c0_tfifoctl;
#[doc = "I2C0_TFIFOSR (r) register accessor: I2C Target FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_tfifosr`] module"]
#[doc(alias = "I2C0_TFIFOSR")]
pub type I2c0Tfifosr = crate::Reg<i2c0_tfifosr::I2c0TfifosrSpec>;
#[doc = "I2C Target FIFO Status Register"]
pub mod i2c0_tfifosr;
#[doc = "I2C0_TARGET_PECCTL (rw) register accessor: I2C Target PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_target_pecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_target_pecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_target_pecctl`] module"]
#[doc(alias = "I2C0_TARGET_PECCTL")]
pub type I2c0TargetPecctl = crate::Reg<i2c0_target_pecctl::I2c0TargetPecctlSpec>;
#[doc = "I2C Target PEC control register"]
pub mod i2c0_target_pecctl;
#[doc = "I2C0_TARGET_PECSR (r) register accessor: I2C Target PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_target_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_target_pecsr`] module"]
#[doc(alias = "I2C0_TARGET_PECSR")]
pub type I2c0TargetPecsr = crate::Reg<i2c0_target_pecsr::I2c0TargetPecsrSpec>;
#[doc = "I2C Target PEC status register"]
pub mod i2c0_target_pecsr;
