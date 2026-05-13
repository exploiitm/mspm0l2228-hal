#[doc = "Register `I2C0_CPU_INT_RIS` reader"]
pub type R = crate::R<I2c0CpuIntRisSpec>;
#[doc = "Controller Receive Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxdone {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Crxdone> for bool {
    #[inline(always)]
    fn from(variant: Crxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXDONE` reader - Controller Receive Transaction completed Interrupt"]
pub type CrxdoneR = crate::BitReader<Crxdone>;
impl CrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crxdone {
        match self.bits {
            false => Crxdone::Clr,
            true => Crxdone::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxdone::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxdone::Set
    }
}
#[doc = "Controller Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxdone {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ctxdone> for bool {
    #[inline(always)]
    fn from(variant: Ctxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXDONE` reader - Controller Transmit Transaction completed Interrupt"]
pub type CtxdoneR = crate::BitReader<Ctxdone>;
impl CtxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctxdone {
        match self.bits {
            false => Ctxdone::Clr,
            true => Ctxdone::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxdone::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxdone::Set
    }
}
#[doc = "Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Crxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Crxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOTRG` reader - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
pub type CrxfifotrgR = crate::BitReader<Crxfifotrg>;
impl CrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crxfifotrg {
        match self.bits {
            false => Crxfifotrg::Clr,
            true => Crxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxfifotrg::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxfifotrg::Set
    }
}
#[doc = "Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ctxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ctxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXFIFOTRG` reader - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
pub type CtxfifotrgR = crate::BitReader<Ctxfifotrg>;
impl CtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctxfifotrg {
        match self.bits {
            false => Ctxfifotrg::Clr,
            true => Ctxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxfifotrg::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxfifotrg::Set
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifofull {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Crxfifofull> for bool {
    #[inline(always)]
    fn from(variant: Crxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type CrxfifofullR = crate::BitReader<Crxfifofull>;
impl CrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crxfifofull {
        match self.bits {
            false => Crxfifofull::Clr,
            true => Crxfifofull::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxfifofull::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxfifofull::Set
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxempty {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ctxempty> for bool {
    #[inline(always)]
    fn from(variant: Ctxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type CtxemptyR = crate::BitReader<Ctxempty>;
impl CtxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctxempty {
        match self.bits {
            false => Ctxempty::Clr,
            true => Ctxempty::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxempty::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxempty::Set
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnack {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Cnack> for bool {
    #[inline(always)]
    fn from(variant: Cnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNACK` reader - Address/Data NACK Interrupt"]
pub type CnackR = crate::BitReader<Cnack>;
impl CnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnack {
        match self.bits {
            false => Cnack::Clr,
            true => Cnack::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cnack::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cnack::Set
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstart {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Cstart> for bool {
    #[inline(always)]
    fn from(variant: Cstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTART` reader - START Detection Interrupt"]
pub type CstartR = crate::BitReader<Cstart>;
impl CstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstart {
        match self.bits {
            false => Cstart::Clr,
            true => Cstart::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cstart::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cstart::Set
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Cstop> for bool {
    #[inline(always)]
    fn from(variant: Cstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP` reader - STOP Detection Interrupt"]
pub type CstopR = crate::BitReader<Cstop>;
impl CstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop {
        match self.bits {
            false => Cstop::Clr,
            true => Cstop::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cstop::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cstop::Set
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carblost {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Carblost> for bool {
    #[inline(always)]
    fn from(variant: Carblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARBLOST` reader - Arbitration Lost Interrupt"]
pub type CarblostR = crate::BitReader<Carblost>;
impl CarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carblost {
        match self.bits {
            false => Carblost::Clr,
            true => Carblost::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Carblost::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Carblost::Set
    }
}
#[doc = "DMA Done on Event Channel TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdmaDoneTx {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<CdmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: CdmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMA_DONE_TX` reader - DMA Done on Event Channel TX"]
pub type CdmaDoneTxR = crate::BitReader<CdmaDoneTx>;
impl CdmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CdmaDoneTx {
        match self.bits {
            false => CdmaDoneTx::Clr,
            true => CdmaDoneTx::Set,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == CdmaDoneTx::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CdmaDoneTx::Set
    }
}
#[doc = "DMA Done on Event Channel RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdmaDoneRx {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<CdmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: CdmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMA_DONE_RX` reader - DMA Done on Event Channel RX"]
pub type CdmaDoneRxR = crate::BitReader<CdmaDoneRx>;
impl CdmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CdmaDoneRx {
        match self.bits {
            false => CdmaDoneRx::Clr,
            true => CdmaDoneRx::Set,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == CdmaDoneRx::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CdmaDoneRx::Set
    }
}
#[doc = "Controller RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpecRxErr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt Occured"]
    Set = 1,
}
impl From<CpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: CpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPEC_RX_ERR` reader - Controller RX Pec Error Interrupt"]
pub type CpecRxErrR = crate::BitReader<CpecRxErr>;
impl CpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpecRxErr {
        match self.bits {
            false => CpecRxErr::Clr,
            true => CpecRxErr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == CpecRxErr::Clr
    }
    #[doc = "Interrupt Occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CpecRxErr::Set
    }
}
#[doc = "Timeout A Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeouta {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Timeouta> for bool {
    #[inline(always)]
    fn from(variant: Timeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTA` reader - Timeout A Interrupt"]
pub type TimeoutaR = crate::BitReader<Timeouta>;
impl TimeoutaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeouta {
        match self.bits {
            false => Timeouta::Clr,
            true => Timeouta::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Timeouta::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Timeouta::Set
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeoutb {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Timeoutb> for bool {
    #[inline(always)]
    fn from(variant: Timeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTB` reader - Timeout B Interrupt"]
pub type TimeoutbR = crate::BitReader<Timeoutb>;
impl TimeoutbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeoutb {
        match self.bits {
            false => Timeoutb::Clr,
            true => Timeoutb::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Timeoutb::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Timeoutb::Set
    }
}
#[doc = "Target Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxdone {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Trxdone> for bool {
    #[inline(always)]
    fn from(variant: Trxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXDONE` reader - Target Receive Data Interrupt Signals that a byte has been received"]
pub type TrxdoneR = crate::BitReader<Trxdone>;
impl TrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trxdone {
        match self.bits {
            false => Trxdone::Clr,
            true => Trxdone::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trxdone::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxdone::Set
    }
}
#[doc = "Target Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxdone {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ttxdone> for bool {
    #[inline(always)]
    fn from(variant: Ttxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXDONE` reader - Target Transmit Transaction completed Interrupt"]
pub type TtxdoneR = crate::BitReader<Ttxdone>;
impl TtxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttxdone {
        match self.bits {
            false => Ttxdone::Clr,
            true => Ttxdone::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxdone::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxdone::Set
    }
}
#[doc = "Target Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Trxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Trxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOTRG` reader - Target Receive FIFO Trigger"]
pub type TrxfifotrgR = crate::BitReader<Trxfifotrg>;
impl TrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trxfifotrg {
        match self.bits {
            false => Trxfifotrg::Clr,
            true => Trxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trxfifotrg::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxfifotrg::Set
    }
}
#[doc = "Target Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ttxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ttxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXFIFOTRG` reader - Target Transmit FIFO Trigger"]
pub type TtxfifotrgR = crate::BitReader<Ttxfifotrg>;
impl TtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttxfifotrg {
        match self.bits {
            false => Ttxfifotrg::Clr,
            true => Ttxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxfifotrg::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxfifotrg::Set
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifofull {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Trxfifofull> for bool {
    #[inline(always)]
    fn from(variant: Trxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type TrxfifofullR = crate::BitReader<Trxfifofull>;
impl TrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trxfifofull {
        match self.bits {
            false => Trxfifofull::Clr,
            true => Trxfifofull::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trxfifofull::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxfifofull::Set
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxempty {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ttxempty> for bool {
    #[inline(always)]
    fn from(variant: Ttxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type TtxemptyR = crate::BitReader<Ttxempty>;
impl TtxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttxempty {
        match self.bits {
            false => Ttxempty::Clr,
            true => Ttxempty::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxempty::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxempty::Set
    }
}
#[doc = "Start Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstart {
    #[doc = "0: Clear interrupt"]
    Clr = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Tstart> for bool {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTART` reader - Start Condition Interrupt"]
pub type TstartR = crate::BitReader<Tstart>;
impl TstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstart {
        match self.bits {
            false => Tstart::Clr,
            true => Tstart::Set,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstart::Clr
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstart::Set
    }
}
#[doc = "Stop Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstop {
    #[doc = "0: Clear Interrupt"]
    Clr = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Tstop> for bool {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTOP` reader - Stop Condition Interrupt"]
pub type TstopR = crate::BitReader<Tstop>;
impl TstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstop {
        match self.bits {
            false => Tstop::Clr,
            true => Tstop::Set,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstop::Clr
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstop::Set
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgencall {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Tgencall> for bool {
    #[inline(always)]
    fn from(variant: Tgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGENCALL` reader - General Call Interrupt"]
pub type TgencallR = crate::BitReader<Tgencall>;
impl TgencallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tgencall {
        match self.bits {
            false => Tgencall::Clr,
            true => Tgencall::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tgencall::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tgencall::Set
    }
}
#[doc = "DMA Done on Event Channel TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdmaDoneTx {
    #[doc = "0: Clear interrupt"]
    Clr = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TdmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: TdmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMA_DONE_TX` reader - DMA Done on Event Channel TX"]
pub type TdmaDoneTxR = crate::BitReader<TdmaDoneTx>;
impl TdmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdmaDoneTx {
        match self.bits {
            false => TdmaDoneTx::Clr,
            true => TdmaDoneTx::Set,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TdmaDoneTx::Clr
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TdmaDoneTx::Set
    }
}
#[doc = "DMA Done on Event Channel RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdmaDoneRx {
    #[doc = "0: Clear interrupt"]
    Clr = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TdmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: TdmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMA_DONE_RX` reader - DMA Done on Event Channel RX"]
pub type TdmaDoneRxR = crate::BitReader<TdmaDoneRx>;
impl TdmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdmaDoneRx {
        match self.bits {
            false => TdmaDoneRx::Clr,
            true => TdmaDoneRx::Set,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TdmaDoneRx::Clr
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TdmaDoneRx::Set
    }
}
#[doc = "Target RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpecRxErr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt ocuured"]
    Set = 1,
}
impl From<TpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: TpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEC_RX_ERR` reader - Target RX Pec Error Interrupt"]
pub type TpecRxErrR = crate::BitReader<TpecRxErr>;
impl TpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TpecRxErr {
        match self.bits {
            false => TpecRxErr::Clr,
            true => TpecRxErr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TpecRxErr::Clr
    }
    #[doc = "Interrupt ocuured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TpecRxErr::Set
    }
}
#[doc = "Target TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TtxUnfl {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<TtxUnfl> for bool {
    #[inline(always)]
    fn from(variant: TtxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTX_UNFL` reader - Target TX FIFO underflow"]
pub type TtxUnflR = crate::BitReader<TtxUnfl>;
impl TtxUnflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TtxUnfl {
        match self.bits {
            false => TtxUnfl::Clr,
            true => TtxUnfl::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TtxUnfl::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TtxUnfl::Set
    }
}
#[doc = "Target RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrxOvfl {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt Occured"]
    Set = 1,
}
impl From<TrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: TrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRX_OVFL` reader - Target RX FIFO overflow"]
pub type TrxOvflR = crate::BitReader<TrxOvfl>;
impl TrxOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TrxOvfl {
        match self.bits {
            false => TrxOvfl::Clr,
            true => TrxOvfl::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TrxOvfl::Clr
    }
    #[doc = "Interrupt Occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TrxOvfl::Set
    }
}
#[doc = "Target Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tarblost {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Tarblost> for bool {
    #[inline(always)]
    fn from(variant: Tarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARBLOST` reader - Target Arbitration Lost"]
pub type TarblostR = crate::BitReader<Tarblost>;
impl TarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tarblost {
        match self.bits {
            false => Tarblost::Clr,
            true => Tarblost::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tarblost::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tarblost::Set
    }
}
#[doc = "Interrupt overflow interrupt It is set when CSTART or CSTOP interrupts overflow i.e. occur twice without being serviced\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrOvfl {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<IntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_OVFL` reader - Interrupt overflow interrupt It is set when CSTART or CSTOP interrupts overflow i.e. occur twice without being serviced"]
pub type IntrOvflR = crate::BitReader<IntrOvfl>;
impl IntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrOvfl {
        match self.bits {
            false => IntrOvfl::Clr,
            true => IntrOvfl::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IntrOvfl::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IntrOvfl::Set
    }
}
impl R {
    #[doc = "Bit 0 - Controller Receive Transaction completed Interrupt"]
    #[inline(always)]
    pub fn crxdone(&self) -> CrxdoneR {
        CrxdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controller Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn ctxdone(&self) -> CtxdoneR {
        CtxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&self) -> CrxfifotrgR {
        CrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&self) -> CtxfifotrgR {
        CtxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn crxfifofull(&self) -> CrxfifofullR {
        CrxfifofullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn ctxempty(&self) -> CtxemptyR {
        CtxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn cnack(&self) -> CnackR {
        CnackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn cstart(&self) -> CstartR {
        CstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn cstop(&self) -> CstopR {
        CstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn carblost(&self) -> CarblostR {
        CarblostR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn cdma_done_tx(&self) -> CdmaDoneTxR {
        CdmaDoneTxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn cdma_done_rx(&self) -> CdmaDoneRxR {
        CdmaDoneRxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controller RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn cpec_rx_err(&self) -> CpecRxErrR {
        CpecRxErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn timeouta(&self) -> TimeoutaR {
        TimeoutaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TimeoutbR {
        TimeoutbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Target Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn trxdone(&self) -> TrxdoneR {
        TrxdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Target Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn ttxdone(&self) -> TtxdoneR {
        TtxdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&self) -> TrxfifotrgR {
        TrxfifotrgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&self) -> TtxfifotrgR {
        TtxfifotrgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn trxfifofull(&self) -> TrxfifofullR {
        TrxfifofullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn ttxempty(&self) -> TtxemptyR {
        TtxemptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn tgencall(&self) -> TgencallR {
        TgencallR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn tdma_done_tx(&self) -> TdmaDoneTxR {
        TdmaDoneTxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn tdma_done_rx(&self) -> TdmaDoneRxR {
        TdmaDoneRxR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Target RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn tpec_rx_err(&self) -> TpecRxErrR {
        TpecRxErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Target TX FIFO underflow"]
    #[inline(always)]
    pub fn ttx_unfl(&self) -> TtxUnflR {
        TtxUnflR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Target RX FIFO overflow"]
    #[inline(always)]
    pub fn trx_ovfl(&self) -> TrxOvflR {
        TrxOvflR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Target Arbitration Lost"]
    #[inline(always)]
    pub fn tarblost(&self) -> TarblostR {
        TarblostR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt overflow interrupt It is set when CSTART or CSTOP interrupts overflow i.e. occur twice without being serviced"]
    #[inline(always)]
    pub fn intr_ovfl(&self) -> IntrOvflR {
        IntrOvflR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cpu_int_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0CpuIntRisSpec;
impl crate::RegisterSpec for I2c0CpuIntRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_cpu_int_ris::R`](R) reader structure"]
impl crate::Readable for I2c0CpuIntRisSpec {}
#[doc = "`reset()` method sets I2C0_CPU_INT_RIS to value 0"]
impl crate::Resettable for I2c0CpuIntRisSpec {}
