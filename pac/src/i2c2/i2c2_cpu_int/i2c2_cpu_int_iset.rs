#[doc = "Register `I2C2_CPU_INT_ISET` writer"]
pub type W = crate::W<I2c2CpuIntIsetSpec>;
#[doc = "Controller Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxdone {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Crxdone> for bool {
    #[inline(always)]
    fn from(variant: Crxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXDONE` writer - Controller Receive Data Interrupt Signals that a byte has been received"]
pub type CrxdoneW<'a, REG> = crate::BitWriter<'a, REG, Crxdone>;
impl<'a, REG> CrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Crxdone::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxdone::Set)
    }
}
#[doc = "Controller Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxdone {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Ctxdone> for bool {
    #[inline(always)]
    fn from(variant: Ctxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXDONE` writer - Controller Transmit Transaction completed Interrupt"]
pub type CtxdoneW<'a, REG> = crate::BitWriter<'a, REG, Ctxdone>;
impl<'a, REG> CtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxdone::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxdone::Set)
    }
}
#[doc = "Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Crxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Crxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOTRG` writer - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
pub type CrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Crxfifotrg>;
impl<'a, REG> CrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::Set)
    }
}
#[doc = "Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Ctxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ctxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXFIFOTRG` writer - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
pub type CtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Ctxfifotrg>;
impl<'a, REG> CtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::Set)
    }
}
#[doc = "RXFIFO full event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifofull {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Crxfifofull> for bool {
    #[inline(always)]
    fn from(variant: Crxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOFULL` writer - RXFIFO full event."]
pub type CrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, Crxfifofull>;
impl<'a, REG> CrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifofull::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifofull::Set)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxempty {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Ctxempty> for bool {
    #[inline(always)]
    fn from(variant: Ctxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type CtxemptyW<'a, REG> = crate::BitWriter<'a, REG, Ctxempty>;
impl<'a, REG> CtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxempty::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxempty::Set)
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnack {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Cnack> for bool {
    #[inline(always)]
    fn from(variant: Cnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNACK` writer - Address/Data NACK Interrupt"]
pub type CnackW<'a, REG> = crate::BitWriter<'a, REG, Cnack>;
impl<'a, REG> CnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Cnack::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cnack::Set)
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstart {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Cstart> for bool {
    #[inline(always)]
    fn from(variant: Cstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTART` writer - START Detection Interrupt"]
pub type CstartW<'a, REG> = crate::BitWriter<'a, REG, Cstart>;
impl<'a, REG> CstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Cstart::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cstart::Set)
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Cstop> for bool {
    #[inline(always)]
    fn from(variant: Cstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP` writer - STOP Detection Interrupt"]
pub type CstopW<'a, REG> = crate::BitWriter<'a, REG, Cstop>;
impl<'a, REG> CstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::Set)
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carblost {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Carblost> for bool {
    #[inline(always)]
    fn from(variant: Carblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARBLOST` writer - Arbitration Lost Interrupt"]
pub type CarblostW<'a, REG> = crate::BitWriter<'a, REG, Carblost>;
impl<'a, REG> CarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Carblost::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Carblost::Set)
    }
}
#[doc = "DMA Done on Event Channel TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdmaDoneTx {
    #[doc = "0: Interrupt disabled"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<CdmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: CdmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMA_DONE_TX` writer - DMA Done on Event Channel TX"]
pub type CdmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, CdmaDoneTx>;
impl<'a, REG> CdmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneTx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneTx::Set)
    }
}
#[doc = "DMA Done on Event Channel RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdmaDoneRx {
    #[doc = "0: Interrupt disabled"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<CdmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: CdmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMA_DONE_RX` writer - DMA Done on Event Channel RX"]
pub type CdmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, CdmaDoneRx>;
impl<'a, REG> CdmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneRx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneRx::Set)
    }
}
#[doc = "Controller RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpecRxErr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<CpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: CpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPEC_RX_ERR` writer - Controller RX Pec Error Interrupt"]
pub type CpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, CpecRxErr>;
impl<'a, REG> CpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CpecRxErr::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CpecRxErr::Set)
    }
}
#[doc = "Timeout A interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeouta {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Timeouta> for bool {
    #[inline(always)]
    fn from(variant: Timeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTA` writer - Timeout A interrupt"]
pub type TimeoutaW<'a, REG> = crate::BitWriter<'a, REG, Timeouta>;
impl<'a, REG> TimeoutaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouta::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouta::Set)
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeoutb {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Timeoutb> for bool {
    #[inline(always)]
    fn from(variant: Timeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTB` writer - Timeout B Interrupt"]
pub type TimeoutbW<'a, REG> = crate::BitWriter<'a, REG, Timeoutb>;
impl<'a, REG> TimeoutbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutb::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutb::Set)
    }
}
#[doc = "Target Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxdone {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Trxdone> for bool {
    #[inline(always)]
    fn from(variant: Trxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXDONE` writer - Target Receive Data Interrupt Signals that a byte has been received"]
pub type TrxdoneW<'a, REG> = crate::BitWriter<'a, REG, Trxdone>;
impl<'a, REG> TrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trxdone::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxdone::Set)
    }
}
#[doc = "Target Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxdone {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Ttxdone> for bool {
    #[inline(always)]
    fn from(variant: Ttxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXDONE` writer - Target Transmit Transaction completed Interrupt"]
pub type TtxdoneW<'a, REG> = crate::BitWriter<'a, REG, Ttxdone>;
impl<'a, REG> TtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxdone::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxdone::Set)
    }
}
#[doc = "Target Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Trxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Trxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOTRG` writer - Target Receive FIFO Trigger"]
pub type TrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Trxfifotrg>;
impl<'a, REG> TrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::Set)
    }
}
#[doc = "Target Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Ttxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ttxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXFIFOTRG` writer - Target Transmit FIFO Trigger"]
pub type TtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Ttxfifotrg>;
impl<'a, REG> TtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::Set)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifofull {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Trxfifofull> for bool {
    #[inline(always)]
    fn from(variant: Trxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type TrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, Trxfifofull>;
impl<'a, REG> TrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifofull::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifofull::Set)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxempty {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Ttxempty> for bool {
    #[inline(always)]
    fn from(variant: Ttxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type TtxemptyW<'a, REG> = crate::BitWriter<'a, REG, Ttxempty>;
impl<'a, REG> TtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxempty::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxempty::Set)
    }
}
#[doc = "Start Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstart {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Tstart> for bool {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTART` writer - Start Condition Interrupt"]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Set)
    }
}
#[doc = "Stop Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstop {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Tstop> for bool {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTOP` writer - Stop Condition Interrupt"]
pub type TstopW<'a, REG> = crate::BitWriter<'a, REG, Tstop>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Set)
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgencall {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Tgencall> for bool {
    #[inline(always)]
    fn from(variant: Tgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGENCALL` writer - General Call Interrupt"]
pub type TgencallW<'a, REG> = crate::BitWriter<'a, REG, Tgencall>;
impl<'a, REG> TgencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tgencall::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tgencall::Set)
    }
}
#[doc = "DMA Done on Event Channel TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdmaDoneTx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TdmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: TdmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMA_DONE_TX` writer - DMA Done on Event Channel TX"]
pub type TdmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, TdmaDoneTx>;
impl<'a, REG> TdmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneTx::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneTx::Set)
    }
}
#[doc = "DMA Done on Event Channel RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdmaDoneRx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TdmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: TdmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMA_DONE_RX` writer - DMA Done on Event Channel RX"]
pub type TdmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, TdmaDoneRx>;
impl<'a, REG> TdmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneRx::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneRx::Set)
    }
}
#[doc = "Target RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpecRxErr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: TpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEC_RX_ERR` writer - Target RX Pec Error Interrupt"]
pub type TpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, TpecRxErr>;
impl<'a, REG> TpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TpecRxErr::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TpecRxErr::Set)
    }
}
#[doc = "Target TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TtxUnfl {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TtxUnfl> for bool {
    #[inline(always)]
    fn from(variant: TtxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTX_UNFL` writer - Target TX FIFO underflow"]
pub type TtxUnflW<'a, REG> = crate::BitWriter<'a, REG, TtxUnfl>;
impl<'a, REG> TtxUnflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TtxUnfl::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TtxUnfl::Set)
    }
}
#[doc = "Target RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrxOvfl {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: TrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRX_OVFL` writer - Target RX FIFO overflow"]
pub type TrxOvflW<'a, REG> = crate::BitWriter<'a, REG, TrxOvfl>;
impl<'a, REG> TrxOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TrxOvfl::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TrxOvfl::Set)
    }
}
#[doc = "Target Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tarblost {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Tarblost> for bool {
    #[inline(always)]
    fn from(variant: Tarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARBLOST` writer - Target Arbitration Lost"]
pub type TarblostW<'a, REG> = crate::BitWriter<'a, REG, Tarblost>;
impl<'a, REG> TarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tarblost::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tarblost::Set)
    }
}
#[doc = "Interrupt overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrOvfl {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<IntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_OVFL` writer - Interrupt overflow"]
pub type IntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IntrOvfl>;
impl<'a, REG> IntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntrOvfl::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(IntrOvfl::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Controller Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn crxdone(&mut self) -> CrxdoneW<'_, I2c2CpuIntIsetSpec> {
        CrxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Controller Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn ctxdone(&mut self) -> CtxdoneW<'_, I2c2CpuIntIsetSpec> {
        CtxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&mut self) -> CrxfifotrgW<'_, I2c2CpuIntIsetSpec> {
        CrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&mut self) -> CtxfifotrgW<'_, I2c2CpuIntIsetSpec> {
        CtxfifotrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event."]
    #[inline(always)]
    pub fn crxfifofull(&mut self) -> CrxfifofullW<'_, I2c2CpuIntIsetSpec> {
        CrxfifofullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn ctxempty(&mut self) -> CtxemptyW<'_, I2c2CpuIntIsetSpec> {
        CtxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn cnack(&mut self) -> CnackW<'_, I2c2CpuIntIsetSpec> {
        CnackW::new(self, 7)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn cstart(&mut self) -> CstartW<'_, I2c2CpuIntIsetSpec> {
        CstartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn cstop(&mut self) -> CstopW<'_, I2c2CpuIntIsetSpec> {
        CstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn carblost(&mut self) -> CarblostW<'_, I2c2CpuIntIsetSpec> {
        CarblostW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn cdma_done_tx(&mut self) -> CdmaDoneTxW<'_, I2c2CpuIntIsetSpec> {
        CdmaDoneTxW::new(self, 11)
    }
    #[doc = "Bit 12 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn cdma_done_rx(&mut self) -> CdmaDoneRxW<'_, I2c2CpuIntIsetSpec> {
        CdmaDoneRxW::new(self, 12)
    }
    #[doc = "Bit 13 - Controller RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn cpec_rx_err(&mut self) -> CpecRxErrW<'_, I2c2CpuIntIsetSpec> {
        CpecRxErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Timeout A interrupt"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, I2c2CpuIntIsetSpec> {
        TimeoutaW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, I2c2CpuIntIsetSpec> {
        TimeoutbW::new(self, 15)
    }
    #[doc = "Bit 16 - Target Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn trxdone(&mut self) -> TrxdoneW<'_, I2c2CpuIntIsetSpec> {
        TrxdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Target Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn ttxdone(&mut self) -> TtxdoneW<'_, I2c2CpuIntIsetSpec> {
        TtxdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&mut self) -> TrxfifotrgW<'_, I2c2CpuIntIsetSpec> {
        TrxfifotrgW::new(self, 18)
    }
    #[doc = "Bit 19 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&mut self) -> TtxfifotrgW<'_, I2c2CpuIntIsetSpec> {
        TtxfifotrgW::new(self, 19)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn trxfifofull(&mut self) -> TrxfifofullW<'_, I2c2CpuIntIsetSpec> {
        TrxfifofullW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn ttxempty(&mut self) -> TtxemptyW<'_, I2c2CpuIntIsetSpec> {
        TtxemptyW::new(self, 21)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<'_, I2c2CpuIntIsetSpec> {
        TstartW::new(self, 22)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<'_, I2c2CpuIntIsetSpec> {
        TstopW::new(self, 23)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn tgencall(&mut self) -> TgencallW<'_, I2c2CpuIntIsetSpec> {
        TgencallW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn tdma_done_tx(&mut self) -> TdmaDoneTxW<'_, I2c2CpuIntIsetSpec> {
        TdmaDoneTxW::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn tdma_done_rx(&mut self) -> TdmaDoneRxW<'_, I2c2CpuIntIsetSpec> {
        TdmaDoneRxW::new(self, 26)
    }
    #[doc = "Bit 27 - Target RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn tpec_rx_err(&mut self) -> TpecRxErrW<'_, I2c2CpuIntIsetSpec> {
        TpecRxErrW::new(self, 27)
    }
    #[doc = "Bit 28 - Target TX FIFO underflow"]
    #[inline(always)]
    pub fn ttx_unfl(&mut self) -> TtxUnflW<'_, I2c2CpuIntIsetSpec> {
        TtxUnflW::new(self, 28)
    }
    #[doc = "Bit 29 - Target RX FIFO overflow"]
    #[inline(always)]
    pub fn trx_ovfl(&mut self) -> TrxOvflW<'_, I2c2CpuIntIsetSpec> {
        TrxOvflW::new(self, 29)
    }
    #[doc = "Bit 30 - Target Arbitration Lost"]
    #[inline(always)]
    pub fn tarblost(&mut self) -> TarblostW<'_, I2c2CpuIntIsetSpec> {
        TarblostW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt overflow"]
    #[inline(always)]
    pub fn intr_ovfl(&mut self) -> IntrOvflW<'_, I2c2CpuIntIsetSpec> {
        IntrOvflW::new(self, 31)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_cpu_int_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2CpuIntIsetSpec;
impl crate::RegisterSpec for I2c2CpuIntIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c2_cpu_int_iset::W`](W) writer structure"]
impl crate::Writable for I2c2CpuIntIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2_CPU_INT_ISET to value 0"]
impl crate::Resettable for I2c2CpuIntIsetSpec {}
