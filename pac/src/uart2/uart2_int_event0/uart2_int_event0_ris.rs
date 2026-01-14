#[doc = "Register `UART2_INT_EVENT0_RIS` reader"]
pub type R = crate::R<Uart2IntEvent0RisSpec>;
#[doc = "UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - UARTOUT Receive Time-Out Interrupt."]
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
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtout::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtout::Set
    }
}
#[doc = "UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmerr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Frmerr> for bool {
    #[inline(always)]
    fn from(variant: Frmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` reader - UART Framing Error Interrupt."]
pub type FrmerrR = crate::BitReader<Frmerr>;
impl FrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frmerr {
        match self.bits {
            false => Frmerr::Clr,
            true => Frmerr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Frmerr::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Frmerr::Set
    }
}
#[doc = "UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parerr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Parerr> for bool {
    #[inline(always)]
    fn from(variant: Parerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - UART Parity Error Interrupt."]
pub type ParerrR = crate::BitReader<Parerr>;
impl ParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parerr {
        match self.bits {
            false => Parerr::Clr,
            true => Parerr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Parerr::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Parerr::Set
    }
}
#[doc = "UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkerr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Brkerr> for bool {
    #[inline(always)]
    fn from(variant: Brkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKERR` reader - UART Break Error Interrupt."]
pub type BrkerrR = crate::BitReader<Brkerr>;
impl BrkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brkerr {
        match self.bits {
            false => Brkerr::Clr,
            true => Brkerr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Brkerr::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Brkerr::Set
    }
}
#[doc = "UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrerr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Ovrerr> for bool {
    #[inline(always)]
    fn from(variant: Ovrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRERR` reader - UART Receive Overrun Error Interrupt."]
pub type OvrerrR = crate::BitReader<Ovrerr>;
impl OvrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrerr {
        match self.bits {
            false => Ovrerr::Clr,
            true => Ovrerr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ovrerr::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ovrerr::Set
    }
}
#[doc = "Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxne {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Rxne> for bool {
    #[inline(always)]
    fn from(variant: Rxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Negative Edge on UARTxRXD Interrupt."]
pub type RxneR = crate::BitReader<Rxne>;
impl RxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxne {
        match self.bits {
            false => Rxne::Clr,
            true => Rxne::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxne::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxne::Set
    }
}
#[doc = "Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpe {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Rxpe> for bool {
    #[inline(always)]
    fn from(variant: Rxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPE` reader - Positive Edge on UARTxRXD Interrupt."]
pub type RxpeR = crate::BitReader<Rxpe>;
impl RxpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpe {
        match self.bits {
            false => Rxpe::Clr,
            true => Rxpe::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxpe::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxpe::Set
    }
}
#[doc = "UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` reader - UART Receive Interrupt."]
pub type RxintR = crate::BitReader<Rxint>;
impl RxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxint {
        match self.bits {
            false => Rxint::Clr,
            true => Rxint::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxint::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxint::Set
    }
}
#[doc = "UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` reader - UART Transmit Interrupt."]
pub type TxintR = crate::BitReader<Txint>;
impl TxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txint {
        match self.bits {
            false => Txint::Clr,
            true => Txint::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txint::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txint::Set
    }
}
#[doc = "UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eot {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Eot> for bool {
    #[inline(always)]
    fn from(variant: Eot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotR = crate::BitReader<Eot>;
impl EotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eot {
        match self.bits {
            false => Eot::Clr,
            true => Eot::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Eot::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Eot::Set
    }
}
#[doc = "Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrMatch {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<AddrMatch> for bool {
    #[inline(always)]
    fn from(variant: AddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_MATCH` reader - Address Match Interrupt."]
pub type AddrMatchR = crate::BitReader<AddrMatch>;
impl AddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrMatch {
        match self.bits {
            false => AddrMatch::Clr,
            true => AddrMatch::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == AddrMatch::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AddrMatch::Set
    }
}
#[doc = "UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - UART Clear to Send Modem Interrupt."]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::Clr,
            true => Cts::Set,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cts::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cts::Set
    }
}
#[doc = "DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<DmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_RX` reader - DMA Done on RX Event Channel Interrupt"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneRx::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneRx::Set
    }
}
#[doc = "DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<DmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_TX` reader - DMA Done on TX Event Channel Interrupt"]
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
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneTx::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneTx::Set
    }
}
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nerr {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Nerr> for bool {
    #[inline(always)]
    fn from(variant: Nerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrR = crate::BitReader<Nerr>;
impl NerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nerr {
        match self.bits {
            false => Nerr::Clr,
            true => Nerr::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Nerr::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Nerr::Set
    }
}
impl R {
    #[doc = "Bit 0 - UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&self) -> BrkerrR {
        BrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&self) -> OvrerrR {
        OvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&self) -> RxpeR {
        RxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DmaDoneTxR {
        DmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&self) -> NerrR {
        NerrR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_int_event0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2IntEvent0RisSpec;
impl crate::RegisterSpec for Uart2IntEvent0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_int_event0_ris::R`](R) reader structure"]
impl crate::Readable for Uart2IntEvent0RisSpec {}
#[doc = "`reset()` method sets UART2_INT_EVENT0_RIS to value 0"]
impl crate::Resettable for Uart2IntEvent0RisSpec {}
