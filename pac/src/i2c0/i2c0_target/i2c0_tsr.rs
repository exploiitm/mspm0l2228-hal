#[doc = "Register `I2C0_TSR` reader"]
pub type R = crate::R<I2c0TsrSpec>;
#[doc = "Receive Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rreq {
    #[doc = "0: No outstanding receive data."]
    Cleared = 0,
    #[doc = "1: The I2C controller has outstanding receive data from the I2C Controller and is using clock stretching to delay the Controller until the data has been read from the TRXDATA FIFO (Target RX FIFO is full)."]
    Set = 1,
}
impl From<Rreq> for bool {
    #[inline(always)]
    fn from(variant: Rreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RREQ` reader - Receive Request"]
pub type RreqR = crate::BitReader<Rreq>;
impl RreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rreq {
        match self.bits {
            false => Rreq::Cleared,
            true => Rreq::Set,
        }
    }
    #[doc = "No outstanding receive data."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Rreq::Cleared
    }
    #[doc = "The I2C controller has outstanding receive data from the I2C Controller and is using clock stretching to delay the Controller until the data has been read from the TRXDATA FIFO (Target RX FIFO is full)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rreq::Set
    }
}
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Treq {
    #[doc = "0: No outstanding transmit request."]
    Cleared = 0,
    #[doc = "1: The I2C controller has been addressed as a Target transmitter and is using clock stretching to delay the Controller until data has been written to the TTXDATA FIFO (Target TX FIFO is empty)."]
    Set = 1,
}
impl From<Treq> for bool {
    #[inline(always)]
    fn from(variant: Treq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TREQ` reader - Transmit Request"]
pub type TreqR = crate::BitReader<Treq>;
impl TreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Treq {
        match self.bits {
            false => Treq::Cleared,
            true => Treq::Set,
        }
    }
    #[doc = "No outstanding transmit request."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Treq::Cleared
    }
    #[doc = "The I2C controller has been addressed as a Target transmitter and is using clock stretching to delay the Controller until data has been written to the TTXDATA FIFO (Target TX FIFO is empty)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Treq::Set
    }
}
#[doc = "Target FSM is in Rx MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxmode {
    #[doc = "0: The Target State Machine is not in the RX_DATA, RX_ACK, RX_WAIT, RX_ACK_WAIT or ADDR_ACK state with the bus direction set to write."]
    Cleared = 0,
    #[doc = "1: The Target State Machine is in the RX_DATA, RX_ACK, RX_WAIT, RX_ACK_WAIT or ADDR_ACK state with the bus direction set to write."]
    Set = 1,
}
impl From<Rxmode> for bool {
    #[inline(always)]
    fn from(variant: Rxmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXMODE` reader - Target FSM is in Rx MODE"]
pub type RxmodeR = crate::BitReader<Rxmode>;
impl RxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxmode {
        match self.bits {
            false => Rxmode::Cleared,
            true => Rxmode::Set,
        }
    }
    #[doc = "The Target State Machine is not in the RX_DATA, RX_ACK, RX_WAIT, RX_ACK_WAIT or ADDR_ACK state with the bus direction set to write."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Rxmode::Cleared
    }
    #[doc = "The Target State Machine is in the RX_DATA, RX_ACK, RX_WAIT, RX_ACK_WAIT or ADDR_ACK state with the bus direction set to write."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxmode::Set
    }
}
#[doc = "OAR2 Address Matched This bit gets reevaluated after every address comparison.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oar2sel {
    #[doc = "0: Either the OAR2 address is not matched or the match is in legacy mode."]
    Cleared = 0,
    #[doc = "1: OAR2 address matched and ACKed by the Target."]
    Set = 1,
}
impl From<Oar2sel> for bool {
    #[inline(always)]
    fn from(variant: Oar2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAR2SEL` reader - OAR2 Address Matched This bit gets reevaluated after every address comparison."]
pub type Oar2selR = crate::BitReader<Oar2sel>;
impl Oar2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oar2sel {
        match self.bits {
            false => Oar2sel::Cleared,
            true => Oar2sel::Set,
        }
    }
    #[doc = "Either the OAR2 address is not matched or the match is in legacy mode."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Oar2sel::Cleared
    }
    #[doc = "OAR2 address matched and ACKed by the Target."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Oar2sel::Set
    }
}
#[doc = "Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qcmdst {
    #[doc = "0: The last transaction was a normal transaction or a transaction has not occurred."]
    Cleared = 0,
    #[doc = "1: The last transaction was a Quick Command transaction."]
    Set = 1,
}
impl From<Qcmdst> for bool {
    #[inline(always)]
    fn from(variant: Qcmdst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QCMDST` reader - Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction"]
pub type QcmdstR = crate::BitReader<Qcmdst>;
impl QcmdstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qcmdst {
        match self.bits {
            false => Qcmdst::Cleared,
            true => Qcmdst::Set,
        }
    }
    #[doc = "The last transaction was a normal transaction or a transaction has not occurred."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Qcmdst::Cleared
    }
    #[doc = "The last transaction was a Quick Command transaction."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Qcmdst::Set
    }
}
#[doc = "Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qcmdrw {
    #[doc = "0: Quick command was a write"]
    Cleared = 0,
    #[doc = "1: Quick command was a read"]
    Set = 1,
}
impl From<Qcmdrw> for bool {
    #[inline(always)]
    fn from(variant: Qcmdrw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QCMDRW` reader - Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read"]
pub type QcmdrwR = crate::BitReader<Qcmdrw>;
impl QcmdrwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qcmdrw {
        match self.bits {
            false => Qcmdrw::Cleared,
            true => Qcmdrw::Set,
        }
    }
    #[doc = "Quick command was a write"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Qcmdrw::Cleared
    }
    #[doc = "Quick command was a read"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Qcmdrw::Set
    }
}
#[doc = "I2C bus is busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busbsy {
    #[doc = "0: The I2C Bus is not busy"]
    Cleared = 0,
    #[doc = "1: The I2C Bus is busy. This is cleared on a timeout."]
    Set = 1,
}
impl From<Busbsy> for bool {
    #[inline(always)]
    fn from(variant: Busbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSBSY` reader - I2C bus is busy"]
pub type BusbsyR = crate::BitReader<Busbsy>;
impl BusbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busbsy {
        match self.bits {
            false => Busbsy::Cleared,
            true => Busbsy::Set,
        }
    }
    #[doc = "The I2C Bus is not busy"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Busbsy::Cleared
    }
    #[doc = "The I2C Bus is busy. This is cleared on a timeout."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Busbsy::Set
    }
}
#[doc = "Target FSM is in TX MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txmode {
    #[doc = "0: The Target State Machine is not in TX_DATA, TX_WAIT, TX_ACK or ADDR_ACK state with the bus direction set to read."]
    Cleared = 0,
    #[doc = "1: The Target State Machine is in TX_DATA, TX_WAIT, TX_ACK or ADDR_ACK state with the bus direction set to read."]
    Set = 1,
}
impl From<Txmode> for bool {
    #[inline(always)]
    fn from(variant: Txmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXMODE` reader - Target FSM is in TX MODE"]
pub type TxmodeR = crate::BitReader<Txmode>;
impl TxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txmode {
        match self.bits {
            false => Txmode::Cleared,
            true => Txmode::Set,
        }
    }
    #[doc = "The Target State Machine is not in TX_DATA, TX_WAIT, TX_ACK or ADDR_ACK state with the bus direction set to read."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Txmode::Cleared
    }
    #[doc = "The Target State Machine is in TX_DATA, TX_WAIT, TX_ACK or ADDR_ACK state with the bus direction set to read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txmode::Set
    }
}
#[doc = "Stale Tx FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaleTxfifo {
    #[doc = "0: Tx FIFO is not stale"]
    Cleared = 0,
    #[doc = "1: The TX FIFO is stale. This occurs when the TX FIFO was not emptied during the previous I2C transaction."]
    Set = 1,
}
impl From<StaleTxfifo> for bool {
    #[inline(always)]
    fn from(variant: StaleTxfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALE_TXFIFO` reader - Stale Tx FIFO"]
pub type StaleTxfifoR = crate::BitReader<StaleTxfifo>;
impl StaleTxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StaleTxfifo {
        match self.bits {
            false => StaleTxfifo::Cleared,
            true => StaleTxfifo::Set,
        }
    }
    #[doc = "Tx FIFO is not stale"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == StaleTxfifo::Cleared
    }
    #[doc = "The TX FIFO is stale. This occurs when the TX FIFO was not emptied during the previous I2C transaction."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == StaleTxfifo::Set
    }
}
#[doc = "Field `ADDRMATCH` reader - Indicates the address for which Target address match happened"]
pub type AddrmatchR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn rreq(&self) -> RreqR {
        RreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn treq(&self) -> TreqR {
        TreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Target FSM is in Rx MODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RxmodeR {
        RxmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OAR2 Address Matched This bit gets reevaluated after every address comparison."]
    #[inline(always)]
    pub fn oar2sel(&self) -> Oar2selR {
        Oar2selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction"]
    #[inline(always)]
    pub fn qcmdst(&self) -> QcmdstR {
        QcmdstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read"]
    #[inline(always)]
    pub fn qcmdrw(&self) -> QcmdrwR {
        QcmdrwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C bus is busy"]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Target FSM is in TX MODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TxmodeR {
        TxmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stale Tx FIFO"]
    #[inline(always)]
    pub fn stale_txfifo(&self) -> StaleTxfifoR {
        StaleTxfifoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:18 - Indicates the address for which Target address match happened"]
    #[inline(always)]
    pub fn addrmatch(&self) -> AddrmatchR {
        AddrmatchR::new(((self.bits >> 9) & 0x03ff) as u16)
    }
}
#[doc = "I2C Target Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0TsrSpec;
impl crate::RegisterSpec for I2c0TsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_tsr::R`](R) reader structure"]
impl crate::Readable for I2c0TsrSpec {}
#[doc = "`reset()` method sets I2C0_TSR to value 0"]
impl crate::Resettable for I2c0TsrSpec {}
