#[doc = "Register `TIMG12_RIS` reader"]
pub type R = crate::R<Timg12RisSpec>;
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
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg12RisSpec;
impl crate::RegisterSpec for Timg12RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg12_ris::R`](R) reader structure"]
impl crate::Readable for Timg12RisSpec {}
#[doc = "`reset()` method sets TIMG12_RIS to value 0"]
impl crate::Resettable for Timg12RisSpec {}
