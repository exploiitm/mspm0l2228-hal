#[doc = "Register `LFSS_GEN_EVENT_IMASK` reader"]
pub type R = crate::R<LfssGenEventImaskSpec>;
#[doc = "Register `LFSS_GEN_EVENT_IMASK` writer"]
pub type W = crate::W<LfssGenEventImaskSpec>;
#[doc = "RTC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdy {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rtcrdy> for bool {
    #[inline(always)]
    fn from(variant: Rtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDY` reader - RTC ready"]
pub type RtcrdyR = crate::BitReader<Rtcrdy>;
impl RtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcrdy {
        match self.bits {
            false => Rtcrdy::Clr,
            true => Rtcrdy::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtcrdy::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtcrdy::Set
    }
}
#[doc = "Field `RTCRDY` writer - RTC ready"]
pub type RtcrdyW<'a, REG> = crate::BitWriter<'a, REG, Rtcrdy>;
impl<'a, REG> RtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdy::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdy::Set)
    }
}
#[doc = "RTC time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctev {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rtctev> for bool {
    #[inline(always)]
    fn from(variant: Rtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTEV` reader - RTC time event"]
pub type RtctevR = crate::BitReader<Rtctev>;
impl RtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctev {
        match self.bits {
            false => Rtctev::Clr,
            true => Rtctev::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtctev::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtctev::Set
    }
}
#[doc = "Field `RTCTEV` writer - RTC time event"]
pub type RtctevW<'a, REG> = crate::BitWriter<'a, REG, Rtctev>;
impl<'a, REG> RtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Set)
    }
}
#[doc = "RTC alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtca1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rtca1> for bool {
    #[inline(always)]
    fn from(variant: Rtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCA1` reader - RTC alarm 1"]
pub type Rtca1R = crate::BitReader<Rtca1>;
impl Rtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtca1 {
        match self.bits {
            false => Rtca1::Clr,
            true => Rtca1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtca1::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtca1::Set
    }
}
#[doc = "Field `RTCA1` writer - RTC alarm 1"]
pub type Rtca1W<'a, REG> = crate::BitWriter<'a, REG, Rtca1>;
impl<'a, REG> Rtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca1::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca1::Set)
    }
}
#[doc = "RTC alarm 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtca2 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rtca2> for bool {
    #[inline(always)]
    fn from(variant: Rtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCA2` reader - RTC alarm 2"]
pub type Rtca2R = crate::BitReader<Rtca2>;
impl Rtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtca2 {
        match self.bits {
            false => Rtca2::Clr,
            true => Rtca2::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtca2::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtca2::Set
    }
}
#[doc = "Field `RTCA2` writer - RTC alarm 2"]
pub type Rtca2W<'a, REG> = crate::BitWriter<'a, REG, Rtca2>;
impl<'a, REG> Rtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca2::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca2::Set)
    }
}
#[doc = "RTC prescale timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0ps {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rt0ps> for bool {
    #[inline(always)]
    fn from(variant: Rt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PS` reader - RTC prescale timer 0"]
pub type Rt0psR = crate::BitReader<Rt0ps>;
impl Rt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0ps {
        match self.bits {
            false => Rt0ps::Clr,
            true => Rt0ps::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rt0ps::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rt0ps::Set
    }
}
#[doc = "Field `RT0PS` writer - RTC prescale timer 0"]
pub type Rt0psW<'a, REG> = crate::BitWriter<'a, REG, Rt0ps>;
impl<'a, REG> Rt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ps::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ps::Set)
    }
}
#[doc = "RTC prescale timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1ps {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rt1ps> for bool {
    #[inline(always)]
    fn from(variant: Rt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PS` reader - RTC prescale timer 1"]
pub type Rt1psR = crate::BitReader<Rt1ps>;
impl Rt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1ps {
        match self.bits {
            false => Rt1ps::Clr,
            true => Rt1ps::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rt1ps::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rt1ps::Set
    }
}
#[doc = "Field `RT1PS` writer - RTC prescale timer 1"]
pub type Rt1psW<'a, REG> = crate::BitWriter<'a, REG, Rt1ps>;
impl<'a, REG> Rt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ps::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ps::Set)
    }
}
#[doc = "RTC prescale timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt2ps {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Rt2ps> for bool {
    #[inline(always)]
    fn from(variant: Rt2ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT2PS` reader - RTC prescale timer 2"]
pub type Rt2psR = crate::BitReader<Rt2ps>;
impl Rt2psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt2ps {
        match self.bits {
            false => Rt2ps::Clr,
            true => Rt2ps::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rt2ps::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rt2ps::Set
    }
}
#[doc = "Field `RT2PS` writer - RTC prescale timer 2"]
pub type Rt2psW<'a, REG> = crate::BitWriter<'a, REG, Rt2ps>;
impl<'a, REG> Rt2psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::Set)
    }
}
#[doc = "Time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsevt {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tsevt> for bool {
    #[inline(always)]
    fn from(variant: Tsevt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEVT` reader - Time stamp event"]
pub type TsevtR = crate::BitReader<Tsevt>;
impl TsevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsevt {
        match self.bits {
            false => Tsevt::Clr,
            true => Tsevt::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tsevt::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tsevt::Set
    }
}
#[doc = "Field `TSEVT` writer - Time stamp event"]
pub type TsevtW<'a, REG> = crate::BitWriter<'a, REG, Tsevt>;
impl<'a, REG> TsevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tsevt::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tsevt::Set)
    }
}
#[doc = "Tamper I/O 0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio0> for bool {
    #[inline(always)]
    fn from(variant: Tio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO0` reader - Tamper I/O 0 event"]
pub type Tio0R = crate::BitReader<Tio0>;
impl Tio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio0 {
        match self.bits {
            false => Tio0::Clr,
            true => Tio0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio0::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio0::Set
    }
}
#[doc = "Field `TIO0` writer - Tamper I/O 0 event"]
pub type Tio0W<'a, REG> = crate::BitWriter<'a, REG, Tio0>;
impl<'a, REG> Tio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::Set)
    }
}
#[doc = "Tamper I/O 1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio1> for bool {
    #[inline(always)]
    fn from(variant: Tio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO1` reader - Tamper I/O 1 event"]
pub type Tio1R = crate::BitReader<Tio1>;
impl Tio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio1 {
        match self.bits {
            false => Tio1::Clr,
            true => Tio1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio1::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio1::Set
    }
}
#[doc = "Field `TIO1` writer - Tamper I/O 1 event"]
pub type Tio1W<'a, REG> = crate::BitWriter<'a, REG, Tio1>;
impl<'a, REG> Tio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::Set)
    }
}
#[doc = "Tamper I/O 2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio2 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio2> for bool {
    #[inline(always)]
    fn from(variant: Tio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO2` reader - Tamper I/O 2 event"]
pub type Tio2R = crate::BitReader<Tio2>;
impl Tio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio2 {
        match self.bits {
            false => Tio2::Clr,
            true => Tio2::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio2::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio2::Set
    }
}
#[doc = "Field `TIO2` writer - Tamper I/O 2 event"]
pub type Tio2W<'a, REG> = crate::BitWriter<'a, REG, Tio2>;
impl<'a, REG> Tio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::Set)
    }
}
#[doc = "Tamper I/O 3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio3 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio3> for bool {
    #[inline(always)]
    fn from(variant: Tio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO3` reader - Tamper I/O 3 event"]
pub type Tio3R = crate::BitReader<Tio3>;
impl Tio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio3 {
        match self.bits {
            false => Tio3::Clr,
            true => Tio3::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio3::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio3::Set
    }
}
#[doc = "Field `TIO3` writer - Tamper I/O 3 event"]
pub type Tio3W<'a, REG> = crate::BitWriter<'a, REG, Tio3>;
impl<'a, REG> Tio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::Set)
    }
}
#[doc = "Tamper I/O 4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio4 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio4> for bool {
    #[inline(always)]
    fn from(variant: Tio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO4` reader - Tamper I/O 4 event"]
pub type Tio4R = crate::BitReader<Tio4>;
impl Tio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio4 {
        match self.bits {
            false => Tio4::Clr,
            true => Tio4::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio4::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio4::Set
    }
}
#[doc = "Field `TIO4` writer - Tamper I/O 4 event"]
pub type Tio4W<'a, REG> = crate::BitWriter<'a, REG, Tio4>;
impl<'a, REG> Tio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::Set)
    }
}
#[doc = "Tamper I/O 5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio5 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio5> for bool {
    #[inline(always)]
    fn from(variant: Tio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO5` reader - Tamper I/O 5 event"]
pub type Tio5R = crate::BitReader<Tio5>;
impl Tio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio5 {
        match self.bits {
            false => Tio5::Clr,
            true => Tio5::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio5::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio5::Set
    }
}
#[doc = "Field `TIO5` writer - Tamper I/O 5 event"]
pub type Tio5W<'a, REG> = crate::BitWriter<'a, REG, Tio5>;
impl<'a, REG> Tio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::Set)
    }
}
#[doc = "Tamper I/O 6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio6 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio6> for bool {
    #[inline(always)]
    fn from(variant: Tio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO6` reader - Tamper I/O 6 event"]
pub type Tio6R = crate::BitReader<Tio6>;
impl Tio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio6 {
        match self.bits {
            false => Tio6::Clr,
            true => Tio6::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio6::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio6::Set
    }
}
#[doc = "Field `TIO6` writer - Tamper I/O 6 event"]
pub type Tio6W<'a, REG> = crate::BitWriter<'a, REG, Tio6>;
impl<'a, REG> Tio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::Set)
    }
}
#[doc = "Tamper I/O 7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio7 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio7> for bool {
    #[inline(always)]
    fn from(variant: Tio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO7` reader - Tamper I/O 7 event"]
pub type Tio7R = crate::BitReader<Tio7>;
impl Tio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio7 {
        match self.bits {
            false => Tio7::Clr,
            true => Tio7::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio7::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio7::Set
    }
}
#[doc = "Field `TIO7` writer - Tamper I/O 7 event"]
pub type Tio7W<'a, REG> = crate::BitWriter<'a, REG, Tio7>;
impl<'a, REG> Tio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::Set)
    }
}
#[doc = "Tamper I/O 8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio8 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio8> for bool {
    #[inline(always)]
    fn from(variant: Tio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO8` reader - Tamper I/O 8 event"]
pub type Tio8R = crate::BitReader<Tio8>;
impl Tio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio8 {
        match self.bits {
            false => Tio8::Clr,
            true => Tio8::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio8::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio8::Set
    }
}
#[doc = "Field `TIO8` writer - Tamper I/O 8 event"]
pub type Tio8W<'a, REG> = crate::BitWriter<'a, REG, Tio8>;
impl<'a, REG> Tio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::Set)
    }
}
#[doc = "Tamper I/O 9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio9 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio9> for bool {
    #[inline(always)]
    fn from(variant: Tio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO9` reader - Tamper I/O 9 event"]
pub type Tio9R = crate::BitReader<Tio9>;
impl Tio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio9 {
        match self.bits {
            false => Tio9::Clr,
            true => Tio9::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio9::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio9::Set
    }
}
#[doc = "Field `TIO9` writer - Tamper I/O 9 event"]
pub type Tio9W<'a, REG> = crate::BitWriter<'a, REG, Tio9>;
impl<'a, REG> Tio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::Set)
    }
}
#[doc = "Tamper I/O 10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio10 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio10> for bool {
    #[inline(always)]
    fn from(variant: Tio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO10` reader - Tamper I/O 10 event"]
pub type Tio10R = crate::BitReader<Tio10>;
impl Tio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio10 {
        match self.bits {
            false => Tio10::Clr,
            true => Tio10::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio10::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio10::Set
    }
}
#[doc = "Field `TIO10` writer - Tamper I/O 10 event"]
pub type Tio10W<'a, REG> = crate::BitWriter<'a, REG, Tio10>;
impl<'a, REG> Tio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::Set)
    }
}
#[doc = "Tamper I/O 11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio11 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio11> for bool {
    #[inline(always)]
    fn from(variant: Tio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO11` reader - Tamper I/O 11 event"]
pub type Tio11R = crate::BitReader<Tio11>;
impl Tio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio11 {
        match self.bits {
            false => Tio11::Clr,
            true => Tio11::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio11::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio11::Set
    }
}
#[doc = "Field `TIO11` writer - Tamper I/O 11 event"]
pub type Tio11W<'a, REG> = crate::BitWriter<'a, REG, Tio11>;
impl<'a, REG> Tio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::Set)
    }
}
#[doc = "Tamper I/O 12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio12 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio12> for bool {
    #[inline(always)]
    fn from(variant: Tio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO12` reader - Tamper I/O 12 event"]
pub type Tio12R = crate::BitReader<Tio12>;
impl Tio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio12 {
        match self.bits {
            false => Tio12::Clr,
            true => Tio12::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio12::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio12::Set
    }
}
#[doc = "Field `TIO12` writer - Tamper I/O 12 event"]
pub type Tio12W<'a, REG> = crate::BitWriter<'a, REG, Tio12>;
impl<'a, REG> Tio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::Set)
    }
}
#[doc = "Tamper I/O 13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio13 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio13> for bool {
    #[inline(always)]
    fn from(variant: Tio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO13` reader - Tamper I/O 13 event"]
pub type Tio13R = crate::BitReader<Tio13>;
impl Tio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio13 {
        match self.bits {
            false => Tio13::Clr,
            true => Tio13::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio13::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio13::Set
    }
}
#[doc = "Field `TIO13` writer - Tamper I/O 13 event"]
pub type Tio13W<'a, REG> = crate::BitWriter<'a, REG, Tio13>;
impl<'a, REG> Tio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::Set)
    }
}
#[doc = "Tamper I/O 14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio14 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio14> for bool {
    #[inline(always)]
    fn from(variant: Tio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO14` reader - Tamper I/O 14 event"]
pub type Tio14R = crate::BitReader<Tio14>;
impl Tio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio14 {
        match self.bits {
            false => Tio14::Clr,
            true => Tio14::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio14::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio14::Set
    }
}
#[doc = "Field `TIO14` writer - Tamper I/O 14 event"]
pub type Tio14W<'a, REG> = crate::BitWriter<'a, REG, Tio14>;
impl<'a, REG> Tio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::Set)
    }
}
#[doc = "Tamper I/O 15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio15 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Tio15> for bool {
    #[inline(always)]
    fn from(variant: Tio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO15` reader - Tamper I/O 15 event"]
pub type Tio15R = crate::BitReader<Tio15>;
impl Tio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio15 {
        match self.bits {
            false => Tio15::Clr,
            true => Tio15::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio15::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio15::Set
    }
}
#[doc = "Field `TIO15` writer - Tamper I/O 15 event"]
pub type Tio15W<'a, REG> = crate::BitWriter<'a, REG, Tio15>;
impl<'a, REG> Tio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::Set)
    }
}
impl R {
    #[doc = "Bit 0 - RTC ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RtcrdyR {
        RtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC time event"]
    #[inline(always)]
    pub fn rtctev(&self) -> RtctevR {
        RtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC alarm 1"]
    #[inline(always)]
    pub fn rtca1(&self) -> Rtca1R {
        Rtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC alarm 2"]
    #[inline(always)]
    pub fn rtca2(&self) -> Rtca2R {
        Rtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC prescale timer 0"]
    #[inline(always)]
    pub fn rt0ps(&self) -> Rt0psR {
        Rt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC prescale timer 1"]
    #[inline(always)]
    pub fn rt1ps(&self) -> Rt1psR {
        Rt1psR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC prescale timer 2"]
    #[inline(always)]
    pub fn rt2ps(&self) -> Rt2psR {
        Rt2psR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time stamp event"]
    #[inline(always)]
    pub fn tsevt(&self) -> TsevtR {
        TsevtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper I/O 0 event"]
    #[inline(always)]
    pub fn tio0(&self) -> Tio0R {
        Tio0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper I/O 1 event"]
    #[inline(always)]
    pub fn tio1(&self) -> Tio1R {
        Tio1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tamper I/O 2 event"]
    #[inline(always)]
    pub fn tio2(&self) -> Tio2R {
        Tio2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tamper I/O 3 event"]
    #[inline(always)]
    pub fn tio3(&self) -> Tio3R {
        Tio3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tamper I/O 4 event"]
    #[inline(always)]
    pub fn tio4(&self) -> Tio4R {
        Tio4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper I/O 5 event"]
    #[inline(always)]
    pub fn tio5(&self) -> Tio5R {
        Tio5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper I/O 6 event"]
    #[inline(always)]
    pub fn tio6(&self) -> Tio6R {
        Tio6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tamper I/O 7 event"]
    #[inline(always)]
    pub fn tio7(&self) -> Tio7R {
        Tio7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper I/O 8 event"]
    #[inline(always)]
    pub fn tio8(&self) -> Tio8R {
        Tio8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper I/O 9 event"]
    #[inline(always)]
    pub fn tio9(&self) -> Tio9R {
        Tio9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper I/O 10 event"]
    #[inline(always)]
    pub fn tio10(&self) -> Tio10R {
        Tio10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tamper I/O 11 event"]
    #[inline(always)]
    pub fn tio11(&self) -> Tio11R {
        Tio11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tamper I/O 12 event"]
    #[inline(always)]
    pub fn tio12(&self) -> Tio12R {
        Tio12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tamper I/O 13 event"]
    #[inline(always)]
    pub fn tio13(&self) -> Tio13R {
        Tio13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tamper I/O 14 event"]
    #[inline(always)]
    pub fn tio14(&self) -> Tio14R {
        Tio14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tamper I/O 15 event"]
    #[inline(always)]
    pub fn tio15(&self) -> Tio15R {
        Tio15R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC ready"]
    #[inline(always)]
    pub fn rtcrdy(&mut self) -> RtcrdyW<'_, LfssGenEventImaskSpec> {
        RtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC time event"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RtctevW<'_, LfssGenEventImaskSpec> {
        RtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC alarm 1"]
    #[inline(always)]
    pub fn rtca1(&mut self) -> Rtca1W<'_, LfssGenEventImaskSpec> {
        Rtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - RTC alarm 2"]
    #[inline(always)]
    pub fn rtca2(&mut self) -> Rtca2W<'_, LfssGenEventImaskSpec> {
        Rtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC prescale timer 0"]
    #[inline(always)]
    pub fn rt0ps(&mut self) -> Rt0psW<'_, LfssGenEventImaskSpec> {
        Rt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC prescale timer 1"]
    #[inline(always)]
    pub fn rt1ps(&mut self) -> Rt1psW<'_, LfssGenEventImaskSpec> {
        Rt1psW::new(self, 5)
    }
    #[doc = "Bit 6 - RTC prescale timer 2"]
    #[inline(always)]
    pub fn rt2ps(&mut self) -> Rt2psW<'_, LfssGenEventImaskSpec> {
        Rt2psW::new(self, 6)
    }
    #[doc = "Bit 7 - Time stamp event"]
    #[inline(always)]
    pub fn tsevt(&mut self) -> TsevtW<'_, LfssGenEventImaskSpec> {
        TsevtW::new(self, 7)
    }
    #[doc = "Bit 8 - Tamper I/O 0 event"]
    #[inline(always)]
    pub fn tio0(&mut self) -> Tio0W<'_, LfssGenEventImaskSpec> {
        Tio0W::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper I/O 1 event"]
    #[inline(always)]
    pub fn tio1(&mut self) -> Tio1W<'_, LfssGenEventImaskSpec> {
        Tio1W::new(self, 9)
    }
    #[doc = "Bit 10 - Tamper I/O 2 event"]
    #[inline(always)]
    pub fn tio2(&mut self) -> Tio2W<'_, LfssGenEventImaskSpec> {
        Tio2W::new(self, 10)
    }
    #[doc = "Bit 11 - Tamper I/O 3 event"]
    #[inline(always)]
    pub fn tio3(&mut self) -> Tio3W<'_, LfssGenEventImaskSpec> {
        Tio3W::new(self, 11)
    }
    #[doc = "Bit 12 - Tamper I/O 4 event"]
    #[inline(always)]
    pub fn tio4(&mut self) -> Tio4W<'_, LfssGenEventImaskSpec> {
        Tio4W::new(self, 12)
    }
    #[doc = "Bit 13 - Tamper I/O 5 event"]
    #[inline(always)]
    pub fn tio5(&mut self) -> Tio5W<'_, LfssGenEventImaskSpec> {
        Tio5W::new(self, 13)
    }
    #[doc = "Bit 14 - Tamper I/O 6 event"]
    #[inline(always)]
    pub fn tio6(&mut self) -> Tio6W<'_, LfssGenEventImaskSpec> {
        Tio6W::new(self, 14)
    }
    #[doc = "Bit 15 - Tamper I/O 7 event"]
    #[inline(always)]
    pub fn tio7(&mut self) -> Tio7W<'_, LfssGenEventImaskSpec> {
        Tio7W::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper I/O 8 event"]
    #[inline(always)]
    pub fn tio8(&mut self) -> Tio8W<'_, LfssGenEventImaskSpec> {
        Tio8W::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper I/O 9 event"]
    #[inline(always)]
    pub fn tio9(&mut self) -> Tio9W<'_, LfssGenEventImaskSpec> {
        Tio9W::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper I/O 10 event"]
    #[inline(always)]
    pub fn tio10(&mut self) -> Tio10W<'_, LfssGenEventImaskSpec> {
        Tio10W::new(self, 18)
    }
    #[doc = "Bit 19 - Tamper I/O 11 event"]
    #[inline(always)]
    pub fn tio11(&mut self) -> Tio11W<'_, LfssGenEventImaskSpec> {
        Tio11W::new(self, 19)
    }
    #[doc = "Bit 20 - Tamper I/O 12 event"]
    #[inline(always)]
    pub fn tio12(&mut self) -> Tio12W<'_, LfssGenEventImaskSpec> {
        Tio12W::new(self, 20)
    }
    #[doc = "Bit 21 - Tamper I/O 13 event"]
    #[inline(always)]
    pub fn tio13(&mut self) -> Tio13W<'_, LfssGenEventImaskSpec> {
        Tio13W::new(self, 21)
    }
    #[doc = "Bit 22 - Tamper I/O 14 event"]
    #[inline(always)]
    pub fn tio14(&mut self) -> Tio14W<'_, LfssGenEventImaskSpec> {
        Tio14W::new(self, 22)
    }
    #[doc = "Bit 23 - Tamper I/O 15 event"]
    #[inline(always)]
    pub fn tio15(&mut self) -> Tio15W<'_, LfssGenEventImaskSpec> {
        Tio15W::new(self, 23)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_gen_event_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_gen_event_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssGenEventImaskSpec;
impl crate::RegisterSpec for LfssGenEventImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_gen_event_imask::R`](R) reader structure"]
impl crate::Readable for LfssGenEventImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_gen_event_imask::W`](W) writer structure"]
impl crate::Writable for LfssGenEventImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_GEN_EVENT_IMASK to value 0"]
impl crate::Resettable for LfssGenEventImaskSpec {}
