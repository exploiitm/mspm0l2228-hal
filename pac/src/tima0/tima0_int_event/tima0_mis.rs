#[doc = "Register `TIMA0_MIS` reader"]
pub type R = crate::R<Tima0MisSpec>;
#[doc = "Zero event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Z {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Z> for bool {
    #[inline(always)]
    fn from(variant: Z) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Z` reader - Zero event generated an interrupt."]
pub type ZR = crate::BitReader<Z>;
impl ZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Z {
        match self.bits {
            false => Z::Clr,
            true => Z::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Z::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Z::Set
    }
}
#[doc = "Load event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<L> for bool {
    #[inline(always)]
    fn from(variant: L) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` reader - Load event generated an interrupt."]
pub type LR = crate::BitReader<L>;
impl LR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L {
        match self.bits {
            false => L::Clr,
            true => L::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == L::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == L::Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd0 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd0> for bool {
    #[inline(always)]
    fn from(variant: Ccd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD0` reader - Capture or compare down event generated an interrupt CCP0"]
pub type Ccd0R = crate::BitReader<Ccd0>;
impl Ccd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd0 {
        match self.bits {
            false => Ccd0::Clr,
            true => Ccd0::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd0::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd0::Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd1 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd1> for bool {
    #[inline(always)]
    fn from(variant: Ccd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD1` reader - Capture or compare down event generated an interrupt CCP1"]
pub type Ccd1R = crate::BitReader<Ccd1>;
impl Ccd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd1 {
        match self.bits {
            false => Ccd1::Clr,
            true => Ccd1::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd1::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd1::Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd2 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd2> for bool {
    #[inline(always)]
    fn from(variant: Ccd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD2` reader - Capture or compare down event generated an interrupt CCP2"]
pub type Ccd2R = crate::BitReader<Ccd2>;
impl Ccd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd2 {
        match self.bits {
            false => Ccd2::Clr,
            true => Ccd2::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd2::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd2::Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd3 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd3> for bool {
    #[inline(always)]
    fn from(variant: Ccd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD3` reader - Capture or compare down event generated an interrupt CCP3"]
pub type Ccd3R = crate::BitReader<Ccd3>;
impl Ccd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd3 {
        match self.bits {
            false => Ccd3::Clr,
            true => Ccd3::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd3::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd3::Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu0 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu0> for bool {
    #[inline(always)]
    fn from(variant: Ccu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU0` reader - Capture or compare up event generated an interrupt CCP0"]
pub type Ccu0R = crate::BitReader<Ccu0>;
impl Ccu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu0 {
        match self.bits {
            false => Ccu0::Clr,
            true => Ccu0::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu0::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu0::Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu1 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu1> for bool {
    #[inline(always)]
    fn from(variant: Ccu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU1` reader - Capture or compare up event generated an interrupt CCP1"]
pub type Ccu1R = crate::BitReader<Ccu1>;
impl Ccu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu1 {
        match self.bits {
            false => Ccu1::Clr,
            true => Ccu1::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu1::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu1::Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu2 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu2> for bool {
    #[inline(always)]
    fn from(variant: Ccu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU2` reader - Capture or compare up event generated an interrupt CCP2"]
pub type Ccu2R = crate::BitReader<Ccu2>;
impl Ccu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu2 {
        match self.bits {
            false => Ccu2::Clr,
            true => Ccu2::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu2::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu2::Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu3 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu3> for bool {
    #[inline(always)]
    fn from(variant: Ccu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU3` reader - Capture or compare up event generated an interrupt CCP3"]
pub type Ccu3R = crate::BitReader<Ccu3>;
impl Ccu3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu3 {
        match self.bits {
            false => Ccu3::Clr,
            true => Ccu3::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu3::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu3::Set
    }
}
#[doc = "Compare down event generated an interrupt CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd4 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd4> for bool {
    #[inline(always)]
    fn from(variant: Ccd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD4` reader - Compare down event generated an interrupt CCP4"]
pub type Ccd4R = crate::BitReader<Ccd4>;
impl Ccd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd4 {
        match self.bits {
            false => Ccd4::Clr,
            true => Ccd4::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd4::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd4::Set
    }
}
#[doc = "Compare down event generated an interrupt CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd5 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd5> for bool {
    #[inline(always)]
    fn from(variant: Ccd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD5` reader - Compare down event generated an interrupt CCP5"]
pub type Ccd5R = crate::BitReader<Ccd5>;
impl Ccd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd5 {
        match self.bits {
            false => Ccd5::Clr,
            true => Ccd5::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd5::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd5::Set
    }
}
#[doc = "Compare up event generated an interrupt CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu4 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu4> for bool {
    #[inline(always)]
    fn from(variant: Ccu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU4` reader - Compare up event generated an interrupt CCP4"]
pub type Ccu4R = crate::BitReader<Ccu4>;
impl Ccu4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu4 {
        match self.bits {
            false => Ccu4::Clr,
            true => Ccu4::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu4::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu4::Set
    }
}
#[doc = "Compare up event generated an interrupt CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu5 {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu5> for bool {
    #[inline(always)]
    fn from(variant: Ccu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU5` reader - Compare up event generated an interrupt CCP5"]
pub type Ccu5R = crate::BitReader<Ccu5>;
impl Ccu5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu5 {
        match self.bits {
            false => Ccu5::Clr,
            true => Ccu5::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu5::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu5::Set
    }
}
#[doc = "Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<F> for bool {
    #[inline(always)]
    fn from(variant: F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F` reader - Fault"]
pub type FR = crate::BitReader<F>;
impl FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F {
        match self.bits {
            false => F::Clr,
            true => F::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == F::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == F::Set
    }
}
#[doc = "Trigger overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tov {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Tov> for bool {
    #[inline(always)]
    fn from(variant: Tov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOV` reader - Trigger overflow"]
pub type TovR = crate::BitReader<Tov>;
impl TovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tov {
        match self.bits {
            false => Tov::Clr,
            true => Tov::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tov::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tov::Set
    }
}
#[doc = "Repeat Counter Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repc {
    #[doc = "0: Event Cleared"]
    Clr = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Repc> for bool {
    #[inline(always)]
    fn from(variant: Repc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPC` reader - Repeat Counter Zero"]
pub type RepcR = crate::BitReader<Repc>;
impl RepcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Repc {
        match self.bits {
            false => Repc::Clr,
            true => Repc::Set,
        }
    }
    #[doc = "Event Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Repc::Clr
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Repc::Set
    }
}
impl R {
    #[doc = "Bit 0 - Zero event generated an interrupt."]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load event generated an interrupt."]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ccd0(&self) -> Ccd0R {
        Ccd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ccd1(&self) -> Ccd1R {
        Ccd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture or compare down event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn ccd2(&self) -> Ccd2R {
        Ccd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture or compare down event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn ccd3(&self) -> Ccd3R {
        Ccd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ccu0(&self) -> Ccu0R {
        Ccu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ccu1(&self) -> Ccu1R {
        Ccu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture or compare up event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn ccu2(&self) -> Ccu2R {
        Ccu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture or compare up event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn ccu3(&self) -> Ccu3R {
        Ccu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare down event generated an interrupt CCP4"]
    #[inline(always)]
    pub fn ccd4(&self) -> Ccd4R {
        Ccd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare down event generated an interrupt CCP5"]
    #[inline(always)]
    pub fn ccd5(&self) -> Ccd5R {
        Ccd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare up event generated an interrupt CCP4"]
    #[inline(always)]
    pub fn ccu4(&self) -> Ccu4R {
        Ccu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare up event generated an interrupt CCP5"]
    #[inline(always)]
    pub fn ccu5(&self) -> Ccu5R {
        Ccu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault"]
    #[inline(always)]
    pub fn f(&self) -> FR {
        FR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero"]
    #[inline(always)]
    pub fn repc(&self) -> RepcR {
        RepcR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0MisSpec;
impl crate::RegisterSpec for Tima0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_mis::R`](R) reader structure"]
impl crate::Readable for Tima0MisSpec {}
#[doc = "`reset()` method sets TIMA0_MIS to value 0"]
impl crate::Resettable for Tima0MisSpec {}
