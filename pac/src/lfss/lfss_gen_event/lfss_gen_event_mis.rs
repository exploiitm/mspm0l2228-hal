#[doc = "Register `LFSS_GEN_EVENT_MIS` reader"]
pub type R = crate::R<LfssGenEventMisSpec>;
#[doc = "RTC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdy {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtcrdy::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtcrdy::Set
    }
}
#[doc = "RTC time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctev {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtctev::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtctev::Set
    }
}
#[doc = "RTC alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtca1 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtca1::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtca1::Set
    }
}
#[doc = "RTC alarm 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtca2 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtca2::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtca2::Set
    }
}
#[doc = "RTC prescale timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0ps {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rt0ps::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rt0ps::Set
    }
}
#[doc = "RTC prescale timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1ps {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rt1ps::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rt1ps::Set
    }
}
#[doc = "RTC prescale timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt2ps {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rt2ps::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rt2ps::Set
    }
}
#[doc = "Time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsevt {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tsevt::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tsevt::Set
    }
}
#[doc = "Tamper I/O 0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio0 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio0::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio0::Set
    }
}
#[doc = "Tamper I/O 1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio1 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio1::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio1::Set
    }
}
#[doc = "Tamper I/O 2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio2 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio2::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio2::Set
    }
}
#[doc = "Tamper I/O 3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio3 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio3::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio3::Set
    }
}
#[doc = "Tamper I/O 4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio4 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio4::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio4::Set
    }
}
#[doc = "Tamper I/O 5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio5 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio5::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio5::Set
    }
}
#[doc = "Tamper I/O 6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio6 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio6::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio6::Set
    }
}
#[doc = "Tamper I/O 7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio7 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio7::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio7::Set
    }
}
#[doc = "Tamper I/O 8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio8 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio8::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio8::Set
    }
}
#[doc = "Tamper I/O 9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio9 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio9::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio9::Set
    }
}
#[doc = "Tamper I/O 10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio10 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio10::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio10::Set
    }
}
#[doc = "Tamper I/O 11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio11 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio11::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio11::Set
    }
}
#[doc = "Tamper I/O 12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio12 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio12::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio12::Set
    }
}
#[doc = "Tamper I/O 13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio13 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio13::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio13::Set
    }
}
#[doc = "Tamper I/O 14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio14 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio14::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio14::Set
    }
}
#[doc = "Tamper I/O 15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio15 {
    #[doc = "0: Interrupt did not occur or is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
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
    #[doc = "Interrupt did not occur or is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tio15::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tio15::Set
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
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_gen_event_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssGenEventMisSpec;
impl crate::RegisterSpec for LfssGenEventMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_gen_event_mis::R`](R) reader structure"]
impl crate::Readable for LfssGenEventMisSpec {}
#[doc = "`reset()` method sets LFSS_GEN_EVENT_MIS to value 0"]
impl crate::Resettable for LfssGenEventMisSpec {}
