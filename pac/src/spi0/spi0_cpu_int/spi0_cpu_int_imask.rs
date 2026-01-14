#[doc = "Register `SPI0_CPU_INT_IMASK` reader"]
pub type R = crate::R<Spi0CpuIntImaskSpec>;
#[doc = "Register `SPI0_CPU_INT_IMASK` writer"]
pub type W = crate::W<Spi0CpuIntImaskSpec>;
#[doc = "RXFIFO overflow event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxfifoOvf {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<RxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: RxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO_OVF` reader - RXFIFO overflow event mask."]
pub type RxfifoOvfR = crate::BitReader<RxfifoOvf>;
impl RxfifoOvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxfifoOvf {
        match self.bits {
            false => RxfifoOvf::Clr,
            true => RxfifoOvf::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == RxfifoOvf::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RxfifoOvf::Set
    }
}
#[doc = "Field `RXFIFO_OVF` writer - RXFIFO overflow event mask."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, RxfifoOvf>;
impl<'a, REG> RxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Set)
    }
}
#[doc = "Parity error event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Parity error event mask."]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            false => Per::Clr,
            true => Per::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Per::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Per::Set
    }
}
#[doc = "Field `PER` writer - Parity error event mask."]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Set)
    }
}
#[doc = "Enable SPI Receive Time-Out event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - Enable SPI Receive Time-Out event mask."]
pub type RtoutR = crate::BitReader<Rtout>;
impl RtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtout {
        match self.bits {
            false => Rtout::Clr,
            true => Rtout::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtout::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtout::Set
    }
}
#[doc = "Field `RTOUT` writer - Enable SPI Receive Time-Out event mask."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
}
#[doc = "Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            false => Rx::Clr,
            true => Rx::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rx::Set
    }
}
#[doc = "Field `RX` writer - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Set)
    }
}
#[doc = "Transmit FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Transmit FIFO event mask."]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::Clr,
            true => Tx::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tx::Set
    }
}
#[doc = "Field `TX` writer - Transmit FIFO event mask."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Set)
    }
}
#[doc = "Transmit FIFO Empty event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty event mask."]
pub type TxemptyR = crate::BitReader<Txempty>;
impl TxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempty {
        match self.bits {
            false => Txempty::Clr,
            true => Txempty::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txempty::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txempty::Set
    }
}
#[doc = "Field `TXEMPTY` writer - Transmit FIFO Empty event mask."]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG, Txempty>;
impl<'a, REG> TxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Set)
    }
}
#[doc = "SPI Idle event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - SPI Idle event mask."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::Clr,
            true => Idle::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Idle::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idle::Set
    }
}
#[doc = "Field `IDLE` writer - SPI Idle event mask."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Set)
    }
}
#[doc = "DMA Done 1 event for RX event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<DmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_RX` reader - DMA Done 1 event for RX event mask."]
pub type DmaDoneRxR = crate::BitReader<DmaDoneRx>;
impl DmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneRx {
        match self.bits {
            false => DmaDoneRx::Clr,
            true => DmaDoneRx::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneRx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneRx::Set
    }
}
#[doc = "Field `DMA_DONE_RX` writer - DMA Done 1 event for RX event mask."]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneRx>;
impl<'a, REG> DmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Set)
    }
}
#[doc = "DMA Done 1 event for TX event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<DmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_TX` reader - DMA Done 1 event for TX event mask."]
pub type DmaDoneTxR = crate::BitReader<DmaDoneTx>;
impl DmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneTx {
        match self.bits {
            false => DmaDoneTx::Clr,
            true => DmaDoneTx::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneTx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneTx::Set
    }
}
#[doc = "Field `DMA_DONE_TX` writer - DMA Done 1 event for TX event mask."]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneTx>;
impl<'a, REG> DmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Set)
    }
}
#[doc = "TX FIFO underflow interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxfifoUnf {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<TxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: TxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFO_UNF` reader - TX FIFO underflow interrupt mask"]
pub type TxfifoUnfR = crate::BitReader<TxfifoUnf>;
impl TxfifoUnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxfifoUnf {
        match self.bits {
            false => TxfifoUnf::Clr,
            true => TxfifoUnf::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TxfifoUnf::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TxfifoUnf::Set
    }
}
#[doc = "Field `TXFIFO_UNF` writer - TX FIFO underflow interrupt mask"]
pub type TxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG, TxfifoUnf>;
impl<'a, REG> TxfifoUnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TxfifoUnf::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxfifoUnf::Set)
    }
}
#[doc = "RX FIFO Full Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfull {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rxfull> for bool {
    #[inline(always)]
    fn from(variant: Rxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL` reader - RX FIFO Full Interrupt Mask"]
pub type RxfullR = crate::BitReader<Rxfull>;
impl RxfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfull {
        match self.bits {
            false => Rxfull::Clr,
            true => Rxfull::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxfull::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxfull::Set
    }
}
#[doc = "Field `RXFULL` writer - RX FIFO Full Interrupt Mask"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG, Rxfull>;
impl<'a, REG> RxfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfull::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfull::Set)
    }
}
impl R {
    #[doc = "Bit 0 - RXFIFO overflow event mask."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error event mask."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty event mask."]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Idle event mask."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Done 1 event for RX event mask."]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Done 1 event for TX event mask."]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DmaDoneTxR {
        DmaDoneTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn txfifo_unf(&self) -> TxfifoUnfR {
        TxfifoUnfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO overflow event mask."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, Spi0CpuIntImaskSpec> {
        RxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity error event mask."]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, Spi0CpuIntImaskSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Spi0CpuIntImaskSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, Spi0CpuIntImaskSpec> {
        RxW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, Spi0CpuIntImaskSpec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty event mask."]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, Spi0CpuIntImaskSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI Idle event mask."]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, Spi0CpuIntImaskSpec> {
        IdleW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Done 1 event for RX event mask."]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, Spi0CpuIntImaskSpec> {
        DmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - DMA Done 1 event for TX event mask."]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, Spi0CpuIntImaskSpec> {
        DmaDoneTxW::new(self, 8)
    }
    #[doc = "Bit 9 - TX FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn txfifo_unf(&mut self) -> TxfifoUnfW<'_, Spi0CpuIntImaskSpec> {
        TxfifoUnfW::new(self, 9)
    }
    #[doc = "Bit 10 - RX FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, Spi0CpuIntImaskSpec> {
        RxfullW::new(self, 10)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_cpu_int_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_cpu_int_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0CpuIntImaskSpec;
impl crate::RegisterSpec for Spi0CpuIntImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_cpu_int_imask::R`](R) reader structure"]
impl crate::Readable for Spi0CpuIntImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`spi0_cpu_int_imask::W`](W) writer structure"]
impl crate::Writable for Spi0CpuIntImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0_CPU_INT_IMASK to value 0"]
impl crate::Resettable for Spi0CpuIntImaskSpec {}
