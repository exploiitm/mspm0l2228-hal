#[doc = "Register `GPIOB_GEN_EVENT0_IMASK` reader"]
pub type R = crate::R<GpiobGenEvent0ImaskSpec>;
#[doc = "Register `GPIOB_GEN_EVENT0_IMASK` writer"]
pub type W = crate::W<GpiobGenEvent0ImaskSpec>;
#[doc = "DIO0 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` reader - DIO0 event mask"]
pub type Dio0R = crate::BitReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            false => Dio0::Clr,
            true => Dio0::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio0::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio0::Set
    }
}
#[doc = "Field `DIO0` writer - DIO0 event mask"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Set)
    }
}
#[doc = "DIO1 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` reader - DIO1 event mask"]
pub type Dio1R = crate::BitReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            false => Dio1::Clr,
            true => Dio1::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio1::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio1::Set
    }
}
#[doc = "Field `DIO1` writer - DIO1 event mask"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Set)
    }
}
#[doc = "DIO2 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` reader - DIO2 event mask"]
pub type Dio2R = crate::BitReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            false => Dio2::Clr,
            true => Dio2::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio2::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio2::Set
    }
}
#[doc = "Field `DIO2` writer - DIO2 event mask"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Set)
    }
}
#[doc = "DIO3 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` reader - DIO3 event mask"]
pub type Dio3R = crate::BitReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            false => Dio3::Clr,
            true => Dio3::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio3::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio3::Set
    }
}
#[doc = "Field `DIO3` writer - DIO3 event mask"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Set)
    }
}
#[doc = "DIO4 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` reader - DIO4 event mask"]
pub type Dio4R = crate::BitReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            false => Dio4::Clr,
            true => Dio4::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio4::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio4::Set
    }
}
#[doc = "Field `DIO4` writer - DIO4 event mask"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Set)
    }
}
#[doc = "DIO5 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` reader - DIO5 event mask"]
pub type Dio5R = crate::BitReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            false => Dio5::Clr,
            true => Dio5::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio5::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio5::Set
    }
}
#[doc = "Field `DIO5` writer - DIO5 event mask"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Set)
    }
}
#[doc = "DIO6 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` reader - DIO6 event mask"]
pub type Dio6R = crate::BitReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            false => Dio6::Clr,
            true => Dio6::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio6::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio6::Set
    }
}
#[doc = "Field `DIO6` writer - DIO6 event mask"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Set)
    }
}
#[doc = "DIO7 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` reader - DIO7 event mask"]
pub type Dio7R = crate::BitReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            false => Dio7::Clr,
            true => Dio7::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio7::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio7::Set
    }
}
#[doc = "Field `DIO7` writer - DIO7 event mask"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Set)
    }
}
#[doc = "DIO8 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` reader - DIO8 event mask"]
pub type Dio8R = crate::BitReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            false => Dio8::Clr,
            true => Dio8::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio8::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio8::Set
    }
}
#[doc = "Field `DIO8` writer - DIO8 event mask"]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dio8>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Set)
    }
}
#[doc = "DIO9 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` reader - DIO9 event mask"]
pub type Dio9R = crate::BitReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            false => Dio9::Clr,
            true => Dio9::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio9::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio9::Set
    }
}
#[doc = "Field `DIO9` writer - DIO9 event mask"]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dio9>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Set)
    }
}
#[doc = "DIO10 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` reader - DIO10 event mask"]
pub type Dio10R = crate::BitReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            false => Dio10::Clr,
            true => Dio10::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio10::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio10::Set
    }
}
#[doc = "Field `DIO10` writer - DIO10 event mask"]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dio10>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Set)
    }
}
#[doc = "DIO11 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` reader - DIO11 event mask"]
pub type Dio11R = crate::BitReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            false => Dio11::Clr,
            true => Dio11::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio11::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio11::Set
    }
}
#[doc = "Field `DIO11` writer - DIO11 event mask"]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dio11>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Set)
    }
}
#[doc = "DIO12 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` reader - DIO12 event mask"]
pub type Dio12R = crate::BitReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            false => Dio12::Clr,
            true => Dio12::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio12::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio12::Set
    }
}
#[doc = "Field `DIO12` writer - DIO12 event mask"]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dio12>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Set)
    }
}
#[doc = "DIO13 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` reader - DIO13 event mask"]
pub type Dio13R = crate::BitReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            false => Dio13::Clr,
            true => Dio13::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio13::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio13::Set
    }
}
#[doc = "Field `DIO13` writer - DIO13 event mask"]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dio13>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Set)
    }
}
#[doc = "DIO14 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` reader - DIO14 event mask"]
pub type Dio14R = crate::BitReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            false => Dio14::Clr,
            true => Dio14::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio14::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio14::Set
    }
}
#[doc = "Field `DIO14` writer - DIO14 event mask"]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dio14>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Set)
    }
}
#[doc = "DIO15 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` reader - DIO15 event mask"]
pub type Dio15R = crate::BitReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            false => Dio15::Clr,
            true => Dio15::Set,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio15::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio15::Set
    }
}
#[doc = "Field `DIO15` writer - DIO15 event mask"]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dio15>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Set)
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpiobGenEvent0ImaskSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpiobGenEvent0ImaskSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpiobGenEvent0ImaskSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpiobGenEvent0ImaskSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpiobGenEvent0ImaskSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpiobGenEvent0ImaskSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpiobGenEvent0ImaskSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpiobGenEvent0ImaskSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpiobGenEvent0ImaskSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpiobGenEvent0ImaskSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpiobGenEvent0ImaskSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpiobGenEvent0ImaskSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpiobGenEvent0ImaskSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpiobGenEvent0ImaskSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpiobGenEvent0ImaskSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpiobGenEvent0ImaskSpec> {
        Dio15W::new(self, 15)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_gen_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_gen_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiobGenEvent0ImaskSpec;
impl crate::RegisterSpec for GpiobGenEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_gen_event0_imask::R`](R) reader structure"]
impl crate::Readable for GpiobGenEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiob_gen_event0_imask::W`](W) writer structure"]
impl crate::Writable for GpiobGenEvent0ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB_GEN_EVENT0_IMASK to value 0"]
impl crate::Resettable for GpiobGenEvent0ImaskSpec {}
