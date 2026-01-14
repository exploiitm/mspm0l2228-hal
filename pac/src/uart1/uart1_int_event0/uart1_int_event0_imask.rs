#[doc = "Register `UART1_INT_EVENT0_IMASK` reader"]
pub type R = crate::R<Uart1IntEvent0ImaskSpec>;
#[doc = "Register `UART1_INT_EVENT0_IMASK` writer"]
pub type W = crate::W<Uart1IntEvent0ImaskSpec>;
#[doc = "Enable UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `RTOUT` reader - Enable UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Field `RTOUT` writer - Enable UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Enable UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmerr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Frmerr> for bool {
    #[inline(always)]
    fn from(variant: Frmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` reader - Enable UART Framing Error Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Frmerr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Frmerr::Set
    }
}
#[doc = "Field `FRMERR` writer - Enable UART Framing Error Interrupt."]
pub type FrmerrW<'a, REG> = crate::BitWriter<'a, REG, Frmerr>;
impl<'a, REG> FrmerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Frmerr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Frmerr::Set)
    }
}
#[doc = "Enable UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parerr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Parerr> for bool {
    #[inline(always)]
    fn from(variant: Parerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - Enable UART Parity Error Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Parerr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Parerr::Set
    }
}
#[doc = "Field `PARERR` writer - Enable UART Parity Error Interrupt."]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG, Parerr>;
impl<'a, REG> ParerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Parerr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Parerr::Set)
    }
}
#[doc = "Enable UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkerr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Brkerr> for bool {
    #[inline(always)]
    fn from(variant: Brkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKERR` reader - Enable UART Break Error Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Brkerr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Brkerr::Set
    }
}
#[doc = "Field `BRKERR` writer - Enable UART Break Error Interrupt."]
pub type BrkerrW<'a, REG> = crate::BitWriter<'a, REG, Brkerr>;
impl<'a, REG> BrkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Brkerr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Brkerr::Set)
    }
}
#[doc = "Enable UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrerr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ovrerr> for bool {
    #[inline(always)]
    fn from(variant: Ovrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRERR` reader - Enable UART Receive Overrun Error Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ovrerr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ovrerr::Set
    }
}
#[doc = "Field `OVRERR` writer - Enable UART Receive Overrun Error Interrupt."]
pub type OvrerrW<'a, REG> = crate::BitWriter<'a, REG, Ovrerr>;
impl<'a, REG> OvrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrerr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrerr::Set)
    }
}
#[doc = "Enable Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxne {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rxne> for bool {
    #[inline(always)]
    fn from(variant: Rxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Enable Negative Edge on UARTxRXD Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxne::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxne::Set
    }
}
#[doc = "Field `RXNE` writer - Enable Negative Edge on UARTxRXD Interrupt."]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG, Rxne>;
impl<'a, REG> RxneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxne::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxne::Set)
    }
}
#[doc = "Enable Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpe {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rxpe> for bool {
    #[inline(always)]
    fn from(variant: Rxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPE` reader - Enable Positive Edge on UARTxRXD Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxpe::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxpe::Set
    }
}
#[doc = "Field `RXPE` writer - Enable Positive Edge on UARTxRXD Interrupt."]
pub type RxpeW<'a, REG> = crate::BitWriter<'a, REG, Rxpe>;
impl<'a, REG> RxpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpe::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpe::Set)
    }
}
#[doc = "Enable LIN Capture 0 / Match Interrupt .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linc0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Linc0> for bool {
    #[inline(always)]
    fn from(variant: Linc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINC0` reader - Enable LIN Capture 0 / Match Interrupt ."]
pub type Linc0R = crate::BitReader<Linc0>;
impl Linc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linc0 {
        match self.bits {
            false => Linc0::Clr,
            true => Linc0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Linc0::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Linc0::Set
    }
}
#[doc = "Field `LINC0` writer - Enable LIN Capture 0 / Match Interrupt ."]
pub type Linc0W<'a, REG> = crate::BitWriter<'a, REG, Linc0>;
impl<'a, REG> Linc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Linc0::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Linc0::Set)
    }
}
#[doc = "Enable LIN Capture 1 Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linc1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Linc1> for bool {
    #[inline(always)]
    fn from(variant: Linc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINC1` reader - Enable LIN Capture 1 Interrupt."]
pub type Linc1R = crate::BitReader<Linc1>;
impl Linc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linc1 {
        match self.bits {
            false => Linc1::Clr,
            true => Linc1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Linc1::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Linc1::Set
    }
}
#[doc = "Field `LINC1` writer - Enable LIN Capture 1 Interrupt."]
pub type Linc1W<'a, REG> = crate::BitWriter<'a, REG, Linc1>;
impl<'a, REG> Linc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Linc1::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Linc1::Set)
    }
}
#[doc = "Enable LIN Hardware Counter Overflow Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linovf {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Linovf> for bool {
    #[inline(always)]
    fn from(variant: Linovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINOVF` reader - Enable LIN Hardware Counter Overflow Interrupt."]
pub type LinovfR = crate::BitReader<Linovf>;
impl LinovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linovf {
        match self.bits {
            false => Linovf::Clr,
            true => Linovf::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Linovf::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Linovf::Set
    }
}
#[doc = "Field `LINOVF` writer - Enable LIN Hardware Counter Overflow Interrupt."]
pub type LinovfW<'a, REG> = crate::BitWriter<'a, REG, Linovf>;
impl<'a, REG> LinovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Linovf::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Linovf::Set)
    }
}
#[doc = "Enable UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` reader - Enable UART Receive Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxint::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxint::Set
    }
}
#[doc = "Field `RXINT` writer - Enable UART Receive Interrupt."]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG, Rxint>;
impl<'a, REG> RxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Set)
    }
}
#[doc = "Enable UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` reader - Enable UART Transmit Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txint::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txint::Set
    }
}
#[doc = "Field `TXINT` writer - Enable UART Transmit Interrupt."]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG, Txint>;
impl<'a, REG> TxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Set)
    }
}
#[doc = "Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eot {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Eot> for bool {
    #[inline(always)]
    fn from(variant: Eot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Eot::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Eot::Set
    }
}
#[doc = "Field `EOT` writer - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG, Eot>;
impl<'a, REG> EotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Eot::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Eot::Set)
    }
}
#[doc = "Enable Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrMatch {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<AddrMatch> for bool {
    #[inline(always)]
    fn from(variant: AddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_MATCH` reader - Enable Address Match Interrupt."]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == AddrMatch::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AddrMatch::Set
    }
}
#[doc = "Field `ADDR_MATCH` writer - Enable Address Match Interrupt."]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG, AddrMatch>;
impl<'a, REG> AddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::Set)
    }
}
#[doc = "Enable UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: Interrupt disabled"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Enable UART Clear to Send Modem Interrupt."]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cts::Set
    }
}
#[doc = "Field `CTS` writer - Enable UART Clear to Send Modem Interrupt."]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG, Cts>;
impl<'a, REG> CtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Set)
    }
}
#[doc = "Enable DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "0: Interrupt disabled"]
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
#[doc = "Field `DMA_DONE_RX` reader - Enable DMA Done on RX Event Channel Interrupt"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneRx::Set
    }
}
#[doc = "Field `DMA_DONE_RX` writer - Enable DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneRx>;
impl<'a, REG> DmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
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
#[doc = "Enable DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "0: Interrupt disabled"]
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
#[doc = "Field `DMA_DONE_TX` reader - Enable DMA Done on TX Event Channel Interrupt"]
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
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneTx::Set
    }
}
#[doc = "Field `DMA_DONE_TX` writer - Enable DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneTx>;
impl<'a, REG> DmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
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
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nerr {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
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
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Nerr::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Nerr::Set
    }
}
#[doc = "Field `NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrW<'a, REG> = crate::BitWriter<'a, REG, Nerr>;
impl<'a, REG> NerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Nerr::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Nerr::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&self) -> BrkerrR {
        BrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&self) -> OvrerrR {
        OvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&self) -> RxpeR {
        RxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn linc0(&self) -> Linc0R {
        Linc0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn linc1(&self) -> Linc1R {
        Linc1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn linovf(&self) -> LinovfR {
        LinovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable DMA Done on TX Event Channel Interrupt"]
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
impl W {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Uart1IntEvent0ImaskSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FrmerrW<'_, Uart1IntEvent0ImaskSpec> {
        FrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<'_, Uart1IntEvent0ImaskSpec> {
        ParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&mut self) -> BrkerrW<'_, Uart1IntEvent0ImaskSpec> {
        BrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&mut self) -> OvrerrW<'_, Uart1IntEvent0ImaskSpec> {
        OvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, Uart1IntEvent0ImaskSpec> {
        RxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&mut self) -> RxpeW<'_, Uart1IntEvent0ImaskSpec> {
        RxpeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn linc0(&mut self) -> Linc0W<'_, Uart1IntEvent0ImaskSpec> {
        Linc0W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn linc1(&mut self) -> Linc1W<'_, Uart1IntEvent0ImaskSpec> {
        Linc1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn linovf(&mut self) -> LinovfW<'_, Uart1IntEvent0ImaskSpec> {
        LinovfW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, Uart1IntEvent0ImaskSpec> {
        RxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, Uart1IntEvent0ImaskSpec> {
        TxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EotW<'_, Uart1IntEvent0ImaskSpec> {
        EotW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<'_, Uart1IntEvent0ImaskSpec> {
        AddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, Uart1IntEvent0ImaskSpec> {
        CtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, Uart1IntEvent0ImaskSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, Uart1IntEvent0ImaskSpec> {
        DmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&mut self) -> NerrW<'_, Uart1IntEvent0ImaskSpec> {
        NerrW::new(self, 17)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1IntEvent0ImaskSpec;
impl crate::RegisterSpec for Uart1IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_int_event0_imask::R`](R) reader structure"]
impl crate::Readable for Uart1IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_int_event0_imask::W`](W) writer structure"]
impl crate::Writable for Uart1IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for Uart1IntEvent0ImaskSpec {}
