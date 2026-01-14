#[doc = "Register `LFSS_TSSTAT` reader"]
pub type R = crate::R<LfssTsstatSpec>;
#[doc = "Tamper I/O 0 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt0 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt0> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT0` reader - Tamper I/O 0 caused time stamp event"]
pub type Tstioevt0R = crate::BitReader<Tstioevt0>;
impl Tstioevt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt0 {
        match self.bits {
            false => Tstioevt0::Clr,
            true => Tstioevt0::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt0::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt0::Set
    }
}
#[doc = "Tamper I/O 1 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt1 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt1> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT1` reader - Tamper I/O 1 caused time stamp event"]
pub type Tstioevt1R = crate::BitReader<Tstioevt1>;
impl Tstioevt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt1 {
        match self.bits {
            false => Tstioevt1::Clr,
            true => Tstioevt1::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt1::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt1::Set
    }
}
#[doc = "Tamper I/O 2 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt2 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt2> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT2` reader - Tamper I/O 2 caused time stamp event"]
pub type Tstioevt2R = crate::BitReader<Tstioevt2>;
impl Tstioevt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt2 {
        match self.bits {
            false => Tstioevt2::Clr,
            true => Tstioevt2::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt2::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt2::Set
    }
}
#[doc = "Tamper I/O 3 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt3 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt3> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT3` reader - Tamper I/O 3 caused time stamp event"]
pub type Tstioevt3R = crate::BitReader<Tstioevt3>;
impl Tstioevt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt3 {
        match self.bits {
            false => Tstioevt3::Clr,
            true => Tstioevt3::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt3::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt3::Set
    }
}
#[doc = "Tamper I/O 4 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt4 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt4> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT4` reader - Tamper I/O 4 caused time stamp event"]
pub type Tstioevt4R = crate::BitReader<Tstioevt4>;
impl Tstioevt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt4 {
        match self.bits {
            false => Tstioevt4::Clr,
            true => Tstioevt4::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt4::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt4::Set
    }
}
#[doc = "Tamper I/O 5 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt5 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt5> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT5` reader - Tamper I/O 5 caused time stamp event"]
pub type Tstioevt5R = crate::BitReader<Tstioevt5>;
impl Tstioevt5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt5 {
        match self.bits {
            false => Tstioevt5::Clr,
            true => Tstioevt5::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt5::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt5::Set
    }
}
#[doc = "Tamper I/O 6 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt6 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt6> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT6` reader - Tamper I/O 6 caused time stamp event"]
pub type Tstioevt6R = crate::BitReader<Tstioevt6>;
impl Tstioevt6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt6 {
        match self.bits {
            false => Tstioevt6::Clr,
            true => Tstioevt6::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt6::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt6::Set
    }
}
#[doc = "Tamper I/O 7 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt7 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt7> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT7` reader - Tamper I/O 7 caused time stamp event"]
pub type Tstioevt7R = crate::BitReader<Tstioevt7>;
impl Tstioevt7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt7 {
        match self.bits {
            false => Tstioevt7::Clr,
            true => Tstioevt7::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt7::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt7::Set
    }
}
#[doc = "Tamper I/O 8 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt8 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt8> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT8` reader - Tamper I/O 8 caused time stamp event"]
pub type Tstioevt8R = crate::BitReader<Tstioevt8>;
impl Tstioevt8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt8 {
        match self.bits {
            false => Tstioevt8::Clr,
            true => Tstioevt8::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt8::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt8::Set
    }
}
#[doc = "Tamper I/O 9 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt9 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt9> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT9` reader - Tamper I/O 9 caused time stamp event"]
pub type Tstioevt9R = crate::BitReader<Tstioevt9>;
impl Tstioevt9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt9 {
        match self.bits {
            false => Tstioevt9::Clr,
            true => Tstioevt9::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt9::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt9::Set
    }
}
#[doc = "Tamper I/O 10 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt10 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt10> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT10` reader - Tamper I/O 10 caused time stamp event"]
pub type Tstioevt10R = crate::BitReader<Tstioevt10>;
impl Tstioevt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt10 {
        match self.bits {
            false => Tstioevt10::Clr,
            true => Tstioevt10::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt10::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt10::Set
    }
}
#[doc = "Tamper I/O 11 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt11 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt11> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT11` reader - Tamper I/O 11 caused time stamp event"]
pub type Tstioevt11R = crate::BitReader<Tstioevt11>;
impl Tstioevt11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt11 {
        match self.bits {
            false => Tstioevt11::Clr,
            true => Tstioevt11::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt11::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt11::Set
    }
}
#[doc = "Tamper I/O 12 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt12 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt12> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT12` reader - Tamper I/O 12 caused time stamp event"]
pub type Tstioevt12R = crate::BitReader<Tstioevt12>;
impl Tstioevt12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt12 {
        match self.bits {
            false => Tstioevt12::Clr,
            true => Tstioevt12::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt12::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt12::Set
    }
}
#[doc = "Tamper I/O 13 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt13 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt13> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT13` reader - Tamper I/O 13 caused time stamp event"]
pub type Tstioevt13R = crate::BitReader<Tstioevt13>;
impl Tstioevt13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt13 {
        match self.bits {
            false => Tstioevt13::Clr,
            true => Tstioevt13::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt13::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt13::Set
    }
}
#[doc = "Tamper I/O 14 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt14 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt14> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT14` reader - Tamper I/O 14 caused time stamp event"]
pub type Tstioevt14R = crate::BitReader<Tstioevt14>;
impl Tstioevt14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt14 {
        match self.bits {
            false => Tstioevt14::Clr,
            true => Tstioevt14::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt14::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt14::Set
    }
}
#[doc = "Tamper I/O 15 caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioevt15 {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tstioevt15> for bool {
    #[inline(always)]
    fn from(variant: Tstioevt15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEVT15` reader - Tamper I/O 15 caused time stamp event"]
pub type Tstioevt15R = crate::BitReader<Tstioevt15>;
impl Tstioevt15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioevt15 {
        match self.bits {
            false => Tstioevt15::Clr,
            true => Tstioevt15::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tstioevt15::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tstioevt15::Set
    }
}
#[doc = "Loss of VDD caused time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsvddevt {
    #[doc = "0: no event detected"]
    Clr = 0,
    #[doc = "1: event detected"]
    Set = 1,
}
impl From<Tsvddevt> for bool {
    #[inline(always)]
    fn from(variant: Tsvddevt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVDDEVT` reader - Loss of VDD caused time stamp event"]
pub type TsvddevtR = crate::BitReader<Tsvddevt>;
impl TsvddevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsvddevt {
        match self.bits {
            false => Tsvddevt::Clr,
            true => Tsvddevt::Set,
        }
    }
    #[doc = "no event detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tsvddevt::Clr
    }
    #[doc = "event detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tsvddevt::Set
    }
}
impl R {
    #[doc = "Bit 0 - Tamper I/O 0 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt0(&self) -> Tstioevt0R {
        Tstioevt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper I/O 1 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt1(&self) -> Tstioevt1R {
        Tstioevt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper I/O 2 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt2(&self) -> Tstioevt2R {
        Tstioevt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper I/O 3 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt3(&self) -> Tstioevt3R {
        Tstioevt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper I/O 4 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt4(&self) -> Tstioevt4R {
        Tstioevt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper I/O 5 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt5(&self) -> Tstioevt5R {
        Tstioevt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper I/O 6 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt6(&self) -> Tstioevt6R {
        Tstioevt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper I/O 7 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt7(&self) -> Tstioevt7R {
        Tstioevt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper I/O 8 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt8(&self) -> Tstioevt8R {
        Tstioevt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper I/O 9 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt9(&self) -> Tstioevt9R {
        Tstioevt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tamper I/O 10 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt10(&self) -> Tstioevt10R {
        Tstioevt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tamper I/O 11 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt11(&self) -> Tstioevt11R {
        Tstioevt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tamper I/O 12 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt12(&self) -> Tstioevt12R {
        Tstioevt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper I/O 13 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt13(&self) -> Tstioevt13R {
        Tstioevt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper I/O 14 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt14(&self) -> Tstioevt14R {
        Tstioevt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tamper I/O 15 caused time stamp event"]
    #[inline(always)]
    pub fn tstioevt15(&self) -> Tstioevt15R {
        Tstioevt15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Loss of VDD caused time stamp event"]
    #[inline(always)]
    pub fn tsvddevt(&self) -> TsvddevtR {
        TsvddevtR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Time Stamp Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsstatSpec;
impl crate::RegisterSpec for LfssTsstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tsstat::R`](R) reader structure"]
impl crate::Readable for LfssTsstatSpec {}
#[doc = "`reset()` method sets LFSS_TSSTAT to value 0"]
impl crate::Resettable for LfssTsstatSpec {}
