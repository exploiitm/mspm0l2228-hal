#[doc = "Register `I2C0_CPU_INT_IMASK` reader"]
pub type R = crate::R<I2c0CpuIntImaskSpec>;
#[doc = "Register `I2C0_CPU_INT_IMASK` writer"]
pub type W = crate::W<I2c0CpuIntImaskSpec>;
#[doc = "Controller Receive Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxdone {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxdone::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxdone::Set
    }
}
#[doc = "Field `CRXDONE` writer - Controller Receive Transaction completed Interrupt"]
pub type CrxdoneW<'a, REG> = crate::BitWriter<'a, REG, Crxdone>;
impl<'a, REG> CrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Crxdone::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxdone::Set)
    }
}
#[doc = "Controller Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxdone {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxdone::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxdone::Set
    }
}
#[doc = "Field `CTXDONE` writer - Controller Transmit Transaction completed Interrupt"]
pub type CtxdoneW<'a, REG> = crate::BitWriter<'a, REG, Ctxdone>;
impl<'a, REG> CtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxdone::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxdone::Set)
    }
}
#[doc = "Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::Set)
    }
}
#[doc = "Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::Set)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifofull {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxfifofull::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxfifofull::Set
    }
}
#[doc = "Field `CRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type CrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, Crxfifofull>;
impl<'a, REG> CrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifofull::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifofull::Set)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxempty {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxempty::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxempty::Set
    }
}
#[doc = "Field `CTXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type CtxemptyW<'a, REG> = crate::BitWriter<'a, REG, Ctxempty>;
impl<'a, REG> CtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxempty::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxempty::Set)
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnack {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cnack::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cnack::Set
    }
}
#[doc = "Field `CNACK` writer - Address/Data NACK Interrupt"]
pub type CnackW<'a, REG> = crate::BitWriter<'a, REG, Cnack>;
impl<'a, REG> CnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cnack::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cnack::Set)
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstart {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cstart::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cstart::Set
    }
}
#[doc = "Field `CSTART` writer - START Detection Interrupt"]
pub type CstartW<'a, REG> = crate::BitWriter<'a, REG, Cstart>;
impl<'a, REG> CstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cstart::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cstart::Set)
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cstop::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cstop::Set
    }
}
#[doc = "Field `CSTOP` writer - STOP Detection Interrupt"]
pub type CstopW<'a, REG> = crate::BitWriter<'a, REG, Cstop>;
impl<'a, REG> CstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::Set)
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carblost {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Carblost::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Carblost::Set
    }
}
#[doc = "Field `CARBLOST` writer - Arbitration Lost Interrupt"]
pub type CarblostW<'a, REG> = crate::BitWriter<'a, REG, Carblost>;
impl<'a, REG> CarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Carblost::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Carblost::Set)
    }
}
#[doc = "DMA Done on Event Channel TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdmaDoneTx {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CdmaDoneTx::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneTx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneTx::Set)
    }
}
#[doc = "DMA Done on Event Channel RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdmaDoneRx {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CdmaDoneRx::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneRx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CdmaDoneRx::Set)
    }
}
#[doc = "Controller RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpecRxErr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == CpecRxErr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CpecRxErr::Set
    }
}
#[doc = "Field `CPEC_RX_ERR` writer - Controller RX Pec Error Interrupt"]
pub type CpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, CpecRxErr>;
impl<'a, REG> CpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(CpecRxErr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CpecRxErr::Set)
    }
}
#[doc = "Timeout A Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeouta {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Timeouta::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Timeouta::Set
    }
}
#[doc = "Field `TIMEOUTA` writer - Timeout A Interrupt"]
pub type TimeoutaW<'a, REG> = crate::BitWriter<'a, REG, Timeouta>;
impl<'a, REG> TimeoutaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouta::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouta::Set)
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeoutb {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Timeoutb::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Timeoutb::Set
    }
}
#[doc = "Field `TIMEOUTB` writer - Timeout B Interrupt"]
pub type TimeoutbW<'a, REG> = crate::BitWriter<'a, REG, Timeoutb>;
impl<'a, REG> TimeoutbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutb::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutb::Set)
    }
}
#[doc = "Target Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxdone {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trxdone::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxdone::Set
    }
}
#[doc = "Field `TRXDONE` writer - Target Receive Data Interrupt Signals that a byte has been received"]
pub type TrxdoneW<'a, REG> = crate::BitWriter<'a, REG, Trxdone>;
impl<'a, REG> TrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trxdone::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxdone::Set)
    }
}
#[doc = "Target Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxdone {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxdone::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxdone::Set
    }
}
#[doc = "Field `TTXDONE` writer - Target Transmit Transaction completed Interrupt"]
pub type TtxdoneW<'a, REG> = crate::BitWriter<'a, REG, Ttxdone>;
impl<'a, REG> TtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxdone::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxdone::Set)
    }
}
#[doc = "Target Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::Set)
    }
}
#[doc = "Target Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::Set)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an Target RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifofull {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Trxfifofull> for bool {
    #[inline(always)]
    fn from(variant: Trxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an Target RX FIFO is full."]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxfifofull::Set
    }
}
#[doc = "Field `TRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an Target RX FIFO is full."]
pub type TrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, Trxfifofull>;
impl<'a, REG> TrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifofull::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifofull::Set)
    }
}
#[doc = "Target Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxempty {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ttxempty> for bool {
    #[inline(always)]
    fn from(variant: Ttxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXEMPTY` reader - Target Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxempty::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxempty::Set
    }
}
#[doc = "Field `TTXEMPTY` writer - Target Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type TtxemptyW<'a, REG> = crate::BitWriter<'a, REG, Ttxempty>;
impl<'a, REG> TtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxempty::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxempty::Set)
    }
}
#[doc = "Start Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstart {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstart::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstart::Set
    }
}
#[doc = "Field `TSTART` writer - Start Condition Interrupt"]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Set)
    }
}
#[doc = "Stop Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstop {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstop::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstop::Set
    }
}
#[doc = "Field `TSTOP` writer - Stop Condition Interrupt"]
pub type TstopW<'a, REG> = crate::BitWriter<'a, REG, Tstop>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Set)
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgencall {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tgencall::Set
    }
}
#[doc = "Field `TGENCALL` writer - General Call Interrupt"]
pub type TgencallW<'a, REG> = crate::BitWriter<'a, REG, Tgencall>;
impl<'a, REG> TgencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tgencall::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tgencall::Set)
    }
}
#[doc = "Target DMA Done on Event Channel TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdmaDoneTx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<TdmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: TdmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMA_DONE_TX` reader - Target DMA Done on Event Channel TX"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TdmaDoneTx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TdmaDoneTx::Set
    }
}
#[doc = "Field `TDMA_DONE_TX` writer - Target DMA Done on Event Channel TX"]
pub type TdmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, TdmaDoneTx>;
impl<'a, REG> TdmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneTx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneTx::Set)
    }
}
#[doc = "Target DMA Done on Event Channel RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdmaDoneRx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<TdmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: TdmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMA_DONE_RX` reader - Target DMA Done on Event Channel RX"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TdmaDoneRx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TdmaDoneRx::Set
    }
}
#[doc = "Field `TDMA_DONE_RX` writer - Target DMA Done on Event Channel RX"]
pub type TdmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, TdmaDoneRx>;
impl<'a, REG> TdmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneRx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TdmaDoneRx::Set)
    }
}
#[doc = "Target RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpecRxErr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TpecRxErr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TpecRxErr::Set
    }
}
#[doc = "Field `TPEC_RX_ERR` writer - Target RX Pec Error Interrupt"]
pub type TpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, TpecRxErr>;
impl<'a, REG> TpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TpecRxErr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TpecRxErr::Set)
    }
}
#[doc = "Target TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TtxUnfl {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TtxUnfl::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TtxUnfl::Set
    }
}
#[doc = "Field `TTX_UNFL` writer - Target TX FIFO underflow"]
pub type TtxUnflW<'a, REG> = crate::BitWriter<'a, REG, TtxUnfl>;
impl<'a, REG> TtxUnflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TtxUnfl::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TtxUnfl::Set)
    }
}
#[doc = "Target RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrxOvfl {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TrxOvfl::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TrxOvfl::Set
    }
}
#[doc = "Field `TRX_OVFL` writer - Target RX FIFO overflow"]
pub type TrxOvflW<'a, REG> = crate::BitWriter<'a, REG, TrxOvfl>;
impl<'a, REG> TrxOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TrxOvfl::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TrxOvfl::Set)
    }
}
#[doc = "Target Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tarblost {
    #[doc = "0: Clear Set Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tarblost::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tarblost::Set
    }
}
#[doc = "Field `TARBLOST` writer - Target Arbitration Lost"]
pub type TarblostW<'a, REG> = crate::BitWriter<'a, REG, Tarblost>;
impl<'a, REG> TarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Set Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tarblost::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tarblost::Set)
    }
}
#[doc = "Interrupt Overflow Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrOvfl {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<IntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_OVFL` reader - Interrupt Overflow Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == IntrOvfl::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IntrOvfl::Set
    }
}
#[doc = "Field `INTR_OVFL` writer - Interrupt Overflow Interrupt Mask"]
pub type IntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IntrOvfl>;
impl<'a, REG> IntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntrOvfl::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(IntrOvfl::Set)
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
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an Target RX FIFO is full."]
    #[inline(always)]
    pub fn trxfifofull(&self) -> TrxfifofullR {
        TrxfifofullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Target Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
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
    #[doc = "Bit 25 - Target DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn tdma_done_tx(&self) -> TdmaDoneTxR {
        TdmaDoneTxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Target DMA Done on Event Channel RX"]
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
    #[doc = "Bit 31 - Interrupt Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn intr_ovfl(&self) -> IntrOvflR {
        IntrOvflR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controller Receive Transaction completed Interrupt"]
    #[inline(always)]
    pub fn crxdone(&mut self) -> CrxdoneW<'_, I2c0CpuIntImaskSpec> {
        CrxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Controller Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn ctxdone(&mut self) -> CtxdoneW<'_, I2c0CpuIntImaskSpec> {
        CtxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&mut self) -> CrxfifotrgW<'_, I2c0CpuIntImaskSpec> {
        CrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&mut self) -> CtxfifotrgW<'_, I2c0CpuIntImaskSpec> {
        CtxfifotrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn crxfifofull(&mut self) -> CrxfifofullW<'_, I2c0CpuIntImaskSpec> {
        CrxfifofullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn ctxempty(&mut self) -> CtxemptyW<'_, I2c0CpuIntImaskSpec> {
        CtxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn cnack(&mut self) -> CnackW<'_, I2c0CpuIntImaskSpec> {
        CnackW::new(self, 7)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn cstart(&mut self) -> CstartW<'_, I2c0CpuIntImaskSpec> {
        CstartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn cstop(&mut self) -> CstopW<'_, I2c0CpuIntImaskSpec> {
        CstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn carblost(&mut self) -> CarblostW<'_, I2c0CpuIntImaskSpec> {
        CarblostW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn cdma_done_tx(&mut self) -> CdmaDoneTxW<'_, I2c0CpuIntImaskSpec> {
        CdmaDoneTxW::new(self, 11)
    }
    #[doc = "Bit 12 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn cdma_done_rx(&mut self) -> CdmaDoneRxW<'_, I2c0CpuIntImaskSpec> {
        CdmaDoneRxW::new(self, 12)
    }
    #[doc = "Bit 13 - Controller RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn cpec_rx_err(&mut self) -> CpecRxErrW<'_, I2c0CpuIntImaskSpec> {
        CpecRxErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, I2c0CpuIntImaskSpec> {
        TimeoutaW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, I2c0CpuIntImaskSpec> {
        TimeoutbW::new(self, 15)
    }
    #[doc = "Bit 16 - Target Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn trxdone(&mut self) -> TrxdoneW<'_, I2c0CpuIntImaskSpec> {
        TrxdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Target Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn ttxdone(&mut self) -> TtxdoneW<'_, I2c0CpuIntImaskSpec> {
        TtxdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&mut self) -> TrxfifotrgW<'_, I2c0CpuIntImaskSpec> {
        TrxfifotrgW::new(self, 18)
    }
    #[doc = "Bit 19 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&mut self) -> TtxfifotrgW<'_, I2c0CpuIntImaskSpec> {
        TtxfifotrgW::new(self, 19)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an Target RX FIFO is full."]
    #[inline(always)]
    pub fn trxfifofull(&mut self) -> TrxfifofullW<'_, I2c0CpuIntImaskSpec> {
        TrxfifofullW::new(self, 20)
    }
    #[doc = "Bit 21 - Target Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn ttxempty(&mut self) -> TtxemptyW<'_, I2c0CpuIntImaskSpec> {
        TtxemptyW::new(self, 21)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<'_, I2c0CpuIntImaskSpec> {
        TstartW::new(self, 22)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<'_, I2c0CpuIntImaskSpec> {
        TstopW::new(self, 23)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn tgencall(&mut self) -> TgencallW<'_, I2c0CpuIntImaskSpec> {
        TgencallW::new(self, 24)
    }
    #[doc = "Bit 25 - Target DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn tdma_done_tx(&mut self) -> TdmaDoneTxW<'_, I2c0CpuIntImaskSpec> {
        TdmaDoneTxW::new(self, 25)
    }
    #[doc = "Bit 26 - Target DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn tdma_done_rx(&mut self) -> TdmaDoneRxW<'_, I2c0CpuIntImaskSpec> {
        TdmaDoneRxW::new(self, 26)
    }
    #[doc = "Bit 27 - Target RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn tpec_rx_err(&mut self) -> TpecRxErrW<'_, I2c0CpuIntImaskSpec> {
        TpecRxErrW::new(self, 27)
    }
    #[doc = "Bit 28 - Target TX FIFO underflow"]
    #[inline(always)]
    pub fn ttx_unfl(&mut self) -> TtxUnflW<'_, I2c0CpuIntImaskSpec> {
        TtxUnflW::new(self, 28)
    }
    #[doc = "Bit 29 - Target RX FIFO overflow"]
    #[inline(always)]
    pub fn trx_ovfl(&mut self) -> TrxOvflW<'_, I2c0CpuIntImaskSpec> {
        TrxOvflW::new(self, 29)
    }
    #[doc = "Bit 30 - Target Arbitration Lost"]
    #[inline(always)]
    pub fn tarblost(&mut self) -> TarblostW<'_, I2c0CpuIntImaskSpec> {
        TarblostW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn intr_ovfl(&mut self) -> IntrOvflW<'_, I2c0CpuIntImaskSpec> {
        IntrOvflW::new(self, 31)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cpu_int_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_cpu_int_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0CpuIntImaskSpec;
impl crate::RegisterSpec for I2c0CpuIntImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_cpu_int_imask::R`](R) reader structure"]
impl crate::Readable for I2c0CpuIntImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0_cpu_int_imask::W`](W) writer structure"]
impl crate::Writable for I2c0CpuIntImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_CPU_INT_IMASK to value 0"]
impl crate::Resettable for I2c0CpuIntImaskSpec {}
