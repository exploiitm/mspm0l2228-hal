#[doc = "Register `SPI1_CPU_INT_ICLR` writer"]
pub type W = crate::W<Spi1CpuIntIclrSpec>;
#[doc = "Clear RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxfifoOvf {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<RxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: RxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO_OVF` writer - Clear RXFIFO overflow event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Clr)
    }
}
#[doc = "Clear Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` writer - Clear Parity error event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Clr)
    }
}
#[doc = "Clear SPI Receive Time-Out Event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Clr = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Clear SPI Receive Time-Out Event."]
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
}
#[doc = "Clear Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` writer - Clear Receive FIFO event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Clr)
    }
}
#[doc = "Clear Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` writer - Clear Transmit FIFO event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Clr)
    }
}
#[doc = "Clear Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` writer - Clear Transmit FIFO Empty event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Clr)
    }
}
#[doc = "Clear SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` writer - Clear SPI IDLE mode event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Clr)
    }
}
#[doc = "Clear DMA Done 1 event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<DmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_RX` writer - Clear DMA Done 1 event for RX."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Clr)
    }
}
#[doc = "Clear DMA Done 1 event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<DmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_TX` writer - Clear DMA Done 1 event for TX."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Clr)
    }
}
#[doc = "Clear TXFIFO underflow event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxfifoUnf {
    #[doc = "0: Writing has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<TxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: TxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFO_UNF` writer - Clear TXFIFO underflow event"]
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
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(TxfifoUnf::Clr)
    }
}
#[doc = "Clear RX FIFO underflow event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfull {
    #[doc = "0: Writing has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rxfull> for bool {
    #[inline(always)]
    fn from(variant: Rxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL` writer - Clear RX FIFO underflow event"]
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
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfull::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear RXFIFO overflow event."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, Spi1CpuIntIclrSpec> {
        RxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Parity error event."]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, Spi1CpuIntIclrSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear SPI Receive Time-Out Event."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Spi1CpuIntIclrSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, Spi1CpuIntIclrSpec> {
        RxW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, Spi1CpuIntIclrSpec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, Spi1CpuIntIclrSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear SPI IDLE mode event."]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, Spi1CpuIntIclrSpec> {
        IdleW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear DMA Done 1 event for RX."]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, Spi1CpuIntIclrSpec> {
        DmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear DMA Done 1 event for TX."]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, Spi1CpuIntIclrSpec> {
        DmaDoneTxW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear TXFIFO underflow event"]
    #[inline(always)]
    pub fn txfifo_unf(&mut self) -> TxfifoUnfW<'_, Spi1CpuIntIclrSpec> {
        TxfifoUnfW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear RX FIFO underflow event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, Spi1CpuIntIclrSpec> {
        RxfullW::new(self, 10)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_cpu_int_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1CpuIntIclrSpec;
impl crate::RegisterSpec for Spi1CpuIntIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi1_cpu_int_iclr::W`](W) writer structure"]
impl crate::Writable for Spi1CpuIntIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_CPU_INT_ICLR to value 0"]
impl crate::Resettable for Spi1CpuIntIclrSpec {}
