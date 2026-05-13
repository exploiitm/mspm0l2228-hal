#[doc = "Register `GPIOB_GEN_EVENT1_RIS` reader"]
pub type R = crate::R<GpiobGenEvent1RisSpec>;
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: DIO16 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO16 event occurred"]
    Set = 1,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - DIO16 event"]
pub type Dio16R = crate::BitReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            false => Dio16::Clr,
            true => Dio16::Set,
        }
    }
    #[doc = "DIO16 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio16::Clr
    }
    #[doc = "DIO16 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio16::Set
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: DIO17 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO17 event occurred"]
    Set = 1,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - DIO17 event"]
pub type Dio17R = crate::BitReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            false => Dio17::Clr,
            true => Dio17::Set,
        }
    }
    #[doc = "DIO17 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio17::Clr
    }
    #[doc = "DIO17 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio17::Set
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: DIO18 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO18 event occurred"]
    Set = 1,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - DIO18 event"]
pub type Dio18R = crate::BitReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            false => Dio18::Clr,
            true => Dio18::Set,
        }
    }
    #[doc = "DIO18 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio18::Clr
    }
    #[doc = "DIO18 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio18::Set
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: DIO19 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO19 event occurred"]
    Set = 1,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - DIO19 event"]
pub type Dio19R = crate::BitReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            false => Dio19::Clr,
            true => Dio19::Set,
        }
    }
    #[doc = "DIO19 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio19::Clr
    }
    #[doc = "DIO19 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio19::Set
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: DIO20 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO20 event occurred"]
    Set = 1,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - DIO20 event"]
pub type Dio20R = crate::BitReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            false => Dio20::Clr,
            true => Dio20::Set,
        }
    }
    #[doc = "DIO20 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio20::Clr
    }
    #[doc = "DIO20 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio20::Set
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: DIO21 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO21 event occurred"]
    Set = 1,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - DIO21 event"]
pub type Dio21R = crate::BitReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            false => Dio21::Clr,
            true => Dio21::Set,
        }
    }
    #[doc = "DIO21 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio21::Clr
    }
    #[doc = "DIO21 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio21::Set
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: DIO22 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO22 event occurred"]
    Set = 1,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - DIO22 event"]
pub type Dio22R = crate::BitReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            false => Dio22::Clr,
            true => Dio22::Set,
        }
    }
    #[doc = "DIO22 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio22::Clr
    }
    #[doc = "DIO22 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio22::Set
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: DIO23 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO23 event occurred"]
    Set = 1,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - DIO23 event"]
pub type Dio23R = crate::BitReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            false => Dio23::Clr,
            true => Dio23::Set,
        }
    }
    #[doc = "DIO23 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio23::Clr
    }
    #[doc = "DIO23 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio23::Set
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: DIO24 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO24 event occurred"]
    Set = 1,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - DIO24 event"]
pub type Dio24R = crate::BitReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            false => Dio24::Clr,
            true => Dio24::Set,
        }
    }
    #[doc = "DIO24 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio24::Clr
    }
    #[doc = "DIO24 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio24::Set
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: DIO25 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO25 event occurred"]
    Set = 1,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - DIO25 event"]
pub type Dio25R = crate::BitReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            false => Dio25::Clr,
            true => Dio25::Set,
        }
    }
    #[doc = "DIO25 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio25::Clr
    }
    #[doc = "DIO25 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio25::Set
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: DIO26 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO26 event occurred"]
    Set = 1,
}
impl From<Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO26` reader - DIO26 event"]
pub type Dio26R = crate::BitReader<Dio26>;
impl Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio26 {
        match self.bits {
            false => Dio26::Clr,
            true => Dio26::Set,
        }
    }
    #[doc = "DIO26 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio26::Clr
    }
    #[doc = "DIO26 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio26::Set
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: DIO27 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO27 event occurred"]
    Set = 1,
}
impl From<Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO27` reader - DIO27 event"]
pub type Dio27R = crate::BitReader<Dio27>;
impl Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio27 {
        match self.bits {
            false => Dio27::Clr,
            true => Dio27::Set,
        }
    }
    #[doc = "DIO27 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio27::Clr
    }
    #[doc = "DIO27 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio27::Set
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: DIO28 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO28 event occurred"]
    Set = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` reader - DIO28 event"]
pub type Dio28R = crate::BitReader<Dio28>;
impl Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio28 {
        match self.bits {
            false => Dio28::Clr,
            true => Dio28::Set,
        }
    }
    #[doc = "DIO28 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio28::Clr
    }
    #[doc = "DIO28 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio28::Set
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: DIO29 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO29 event occurred"]
    Set = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` reader - DIO29 event"]
pub type Dio29R = crate::BitReader<Dio29>;
impl Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio29 {
        match self.bits {
            false => Dio29::Clr,
            true => Dio29::Set,
        }
    }
    #[doc = "DIO29 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio29::Clr
    }
    #[doc = "DIO29 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio29::Set
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: DIO30 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO30 event occurred"]
    Set = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` reader - DIO30 event"]
pub type Dio30R = crate::BitReader<Dio30>;
impl Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio30 {
        match self.bits {
            false => Dio30::Clr,
            true => Dio30::Set,
        }
    }
    #[doc = "DIO30 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio30::Clr
    }
    #[doc = "DIO30 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio30::Set
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: DIO31 event did not occur"]
    Clr = 0,
    #[doc = "1: DIO31 event occurred"]
    Set = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` reader - DIO31 event"]
pub type Dio31R = crate::BitReader<Dio31>;
impl Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio31 {
        match self.bits {
            false => Dio31::Clr,
            true => Dio31::Set,
        }
    }
    #[doc = "DIO31 event did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio31::Clr
    }
    #[doc = "DIO31 event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio31::Set
    }
}
impl R {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiobGenEvent1RisSpec;
impl crate::RegisterSpec for GpiobGenEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_gen_event1_ris::R`](R) reader structure"]
impl crate::Readable for GpiobGenEvent1RisSpec {}
#[doc = "`reset()` method sets GPIOB_GEN_EVENT1_RIS to value 0"]
impl crate::Resettable for GpiobGenEvent1RisSpec {}
