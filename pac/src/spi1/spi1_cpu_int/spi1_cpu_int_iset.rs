#[doc = "Register `SPI1_CPU_INT_ISET` writer"]
pub type W = crate::W<Spi1CpuIntIsetSpec>;
#[doc = "Set RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxfifoOvf {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<RxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: RxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO_OVF` writer - Set RXFIFO overflow event."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, RxfifoOvf>;
impl<'a, REG> RxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Set)
    }
}
#[doc = "Set Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` writer - Set Parity error event."]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Per::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Set)
    }
}
#[doc = "Set SPI Receive Time-Out Event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Set SPI Receive Time-Out Event."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::NoEffect)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
}
#[doc = "Set Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` writer - Set Receive FIFO event."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Set)
    }
}
#[doc = "Set Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` writer - Set Transmit FIFO event."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Set)
    }
}
#[doc = "Set Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` writer - Set Transmit FIFO Empty event."]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG, Txempty>;
impl<'a, REG> TxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Set)
    }
}
#[doc = "Set SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` writer - Set SPI IDLE mode event."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Set)
    }
}
#[doc = "Set DMA Done 1 event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<DmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_RX` writer - Set DMA Done 1 event for RX."]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneRx>;
impl<'a, REG> DmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Set)
    }
}
#[doc = "Set DMA Done 1 event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<DmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_TX` writer - Set DMA Done 1 event for TX."]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneTx>;
impl<'a, REG> DmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Set)
    }
}
#[doc = "Set TX FIFO Underflow Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxfifoUnf {
    #[doc = "0: Writing has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<TxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: TxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFO_UNF` writer - Set TX FIFO Underflow Event"]
pub type TxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG, TxfifoUnf>;
impl<'a, REG> TxfifoUnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TxfifoUnf::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxfifoUnf::Set)
    }
}
#[doc = "Set RX FIFO Full Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfull {
    #[doc = "0: Writing has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Rxfull> for bool {
    #[inline(always)]
    fn from(variant: Rxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL` writer - Set RX FIFO Full Event"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG, Rxfull>;
impl<'a, REG> RxfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfull::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfull::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Set RXFIFO overflow event."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, Spi1CpuIntIsetSpec> {
        RxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Set Parity error event."]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, Spi1CpuIntIsetSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Set SPI Receive Time-Out Event."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Spi1CpuIntIsetSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Set Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, Spi1CpuIntIsetSpec> {
        RxW::new(self, 3)
    }
    #[doc = "Bit 4 - Set Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, Spi1CpuIntIsetSpec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, Spi1CpuIntIsetSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Set SPI IDLE mode event."]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, Spi1CpuIntIsetSpec> {
        IdleW::new(self, 6)
    }
    #[doc = "Bit 7 - Set DMA Done 1 event for RX."]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, Spi1CpuIntIsetSpec> {
        DmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - Set DMA Done 1 event for TX."]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, Spi1CpuIntIsetSpec> {
        DmaDoneTxW::new(self, 8)
    }
    #[doc = "Bit 9 - Set TX FIFO Underflow Event"]
    #[inline(always)]
    pub fn txfifo_unf(&mut self) -> TxfifoUnfW<'_, Spi1CpuIntIsetSpec> {
        TxfifoUnfW::new(self, 9)
    }
    #[doc = "Bit 10 - Set RX FIFO Full Event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, Spi1CpuIntIsetSpec> {
        RxfullW::new(self, 10)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_cpu_int_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1CpuIntIsetSpec;
impl crate::RegisterSpec for Spi1CpuIntIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi1_cpu_int_iset::W`](W) writer structure"]
impl crate::Writable for Spi1CpuIntIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_CPU_INT_ISET to value 0"]
impl crate::Resettable for Spi1CpuIntIsetSpec {}
