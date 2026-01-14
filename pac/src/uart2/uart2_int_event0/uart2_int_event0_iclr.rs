#[doc = "Register `UART2_INT_EVENT0_ICLR` writer"]
pub type W = crate::W<Uart2IntEvent0IclrSpec>;
#[doc = "Clear UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Clear UARTOUT Receive Time-Out Interrupt."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
}
#[doc = "Clear UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmerr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Frmerr> for bool {
    #[inline(always)]
    fn from(variant: Frmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` writer - Clear UART Framing Error Interrupt."]
pub type FrmerrW<'a, REG> = crate::BitWriter<'a, REG, Frmerr>;
impl<'a, REG> FrmerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Frmerr::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Frmerr::Clr)
    }
}
#[doc = "Clear UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parerr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Parerr> for bool {
    #[inline(always)]
    fn from(variant: Parerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` writer - Clear UART Parity Error Interrupt."]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG, Parerr>;
impl<'a, REG> ParerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Parerr::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Parerr::Clr)
    }
}
#[doc = "Clear UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkerr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Brkerr> for bool {
    #[inline(always)]
    fn from(variant: Brkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKERR` writer - Clear UART Break Error Interrupt."]
pub type BrkerrW<'a, REG> = crate::BitWriter<'a, REG, Brkerr>;
impl<'a, REG> BrkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Brkerr::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Brkerr::Clr)
    }
}
#[doc = "Clear UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrerr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Ovrerr> for bool {
    #[inline(always)]
    fn from(variant: Ovrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRERR` writer - Clear UART Receive Overrun Error Interrupt."]
pub type OvrerrW<'a, REG> = crate::BitWriter<'a, REG, Ovrerr>;
impl<'a, REG> OvrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrerr::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrerr::Clr)
    }
}
#[doc = "Clear Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxne {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rxne> for bool {
    #[inline(always)]
    fn from(variant: Rxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` writer - Clear Negative Edge on UARTxRXD Interrupt."]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG, Rxne>;
impl<'a, REG> RxneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxne::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxne::Clr)
    }
}
#[doc = "Clear Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpe {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rxpe> for bool {
    #[inline(always)]
    fn from(variant: Rxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPE` writer - Clear Positive Edge on UARTxRXD Interrupt."]
pub type RxpeW<'a, REG> = crate::BitWriter<'a, REG, Rxpe>;
impl<'a, REG> RxpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpe::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpe::Clr)
    }
}
#[doc = "Clear UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` writer - Clear UART Receive Interrupt."]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG, Rxint>;
impl<'a, REG> RxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Clr)
    }
}
#[doc = "Clear UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` writer - Clear UART Transmit Interrupt."]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG, Txint>;
impl<'a, REG> TxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Clr)
    }
}
#[doc = "Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eot {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Eot> for bool {
    #[inline(always)]
    fn from(variant: Eot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` writer - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG, Eot>;
impl<'a, REG> EotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Eot::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Eot::Clr)
    }
}
#[doc = "Clear Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrMatch {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<AddrMatch> for bool {
    #[inline(always)]
    fn from(variant: AddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_MATCH` writer - Clear Address Match Interrupt."]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG, AddrMatch>;
impl<'a, REG> AddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::Clr)
    }
}
#[doc = "Clear UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` writer - Clear UART Clear to Send Modem Interrupt."]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG, Cts>;
impl<'a, REG> CtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Clr)
    }
}
#[doc = "Clear DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "0: Interrupt disabled"]
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
#[doc = "Field `DMA_DONE_RX` writer - Clear DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneRx>;
impl<'a, REG> DmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
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
#[doc = "Clear DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "0: Interrupt disabled"]
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
#[doc = "Field `DMA_DONE_TX` writer - Clear DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneTx>;
impl<'a, REG> DmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
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
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nerr {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Nerr> for bool {
    #[inline(always)]
    fn from(variant: Nerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrW<'a, REG> = crate::BitWriter<'a, REG, Nerr>;
impl<'a, REG> NerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Nerr::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Nerr::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Uart2IntEvent0IclrSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FrmerrW<'_, Uart2IntEvent0IclrSpec> {
        FrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<'_, Uart2IntEvent0IclrSpec> {
        ParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&mut self) -> BrkerrW<'_, Uart2IntEvent0IclrSpec> {
        BrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&mut self) -> OvrerrW<'_, Uart2IntEvent0IclrSpec> {
        OvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, Uart2IntEvent0IclrSpec> {
        RxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&mut self) -> RxpeW<'_, Uart2IntEvent0IclrSpec> {
        RxpeW::new(self, 6)
    }
    #[doc = "Bit 10 - Clear UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, Uart2IntEvent0IclrSpec> {
        RxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, Uart2IntEvent0IclrSpec> {
        TxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EotW<'_, Uart2IntEvent0IclrSpec> {
        EotW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<'_, Uart2IntEvent0IclrSpec> {
        AddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, Uart2IntEvent0IclrSpec> {
        CtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, Uart2IntEvent0IclrSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, Uart2IntEvent0IclrSpec> {
        DmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&mut self) -> NerrW<'_, Uart2IntEvent0IclrSpec> {
        NerrW::new(self, 17)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2IntEvent0IclrSpec;
impl crate::RegisterSpec for Uart2IntEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart2_int_event0_iclr::W`](W) writer structure"]
impl crate::Writable for Uart2IntEvent0IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART2_INT_EVENT0_ICLR to value 0"]
impl crate::Resettable for Uart2IntEvent0IclrSpec {}
