#[doc = "Register `GPIOC_CPU_INT_IMASK` reader"]
pub type R = crate::R<GpiocCpuIntImaskSpec>;
#[doc = "Register `GPIOC_CPU_INT_IMASK` writer"]
pub type W = crate::W<GpiocCpuIntImaskSpec>;
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
#[doc = "DIO16 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - DIO16 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio16::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio16::Set
    }
}
#[doc = "Field `DIO16` writer - DIO16 event mask"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Set)
    }
}
#[doc = "DIO17 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - DIO17 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio17::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio17::Set
    }
}
#[doc = "Field `DIO17` writer - DIO17 event mask"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Set)
    }
}
#[doc = "DIO18 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - DIO18 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio18::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio18::Set
    }
}
#[doc = "Field `DIO18` writer - DIO18 event mask"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Set)
    }
}
#[doc = "DIO19 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - DIO19 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio19::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio19::Set
    }
}
#[doc = "Field `DIO19` writer - DIO19 event mask"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Set)
    }
}
#[doc = "DIO20 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - DIO20 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio20::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio20::Set
    }
}
#[doc = "Field `DIO20` writer - DIO20 event mask"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Set)
    }
}
#[doc = "DIO21 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - DIO21 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio21::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio21::Set
    }
}
#[doc = "Field `DIO21` writer - DIO21 event mask"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Set)
    }
}
#[doc = "DIO22 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - DIO22 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio22::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio22::Set
    }
}
#[doc = "Field `DIO22` writer - DIO22 event mask"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Set)
    }
}
#[doc = "DIO23 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - DIO23 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio23::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio23::Set
    }
}
#[doc = "Field `DIO23` writer - DIO23 event mask"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Set)
    }
}
#[doc = "DIO24 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - DIO24 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio24::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio24::Set
    }
}
#[doc = "Field `DIO24` writer - DIO24 event mask"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Set)
    }
}
#[doc = "DIO25 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - DIO25 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio25::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio25::Set
    }
}
#[doc = "Field `DIO25` writer - DIO25 event mask"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Set)
    }
}
#[doc = "DIO26 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO26` reader - DIO26 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio26::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio26::Set
    }
}
#[doc = "Field `DIO26` writer - DIO26 event mask"]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG, Dio26>;
impl<'a, REG> Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Set)
    }
}
#[doc = "DIO27 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO27` reader - DIO27 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio27::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio27::Set
    }
}
#[doc = "Field `DIO27` writer - DIO27 event mask"]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG, Dio27>;
impl<'a, REG> Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Set)
    }
}
#[doc = "DIO28 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` reader - DIO28 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio28::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio28::Set
    }
}
#[doc = "Field `DIO28` writer - DIO28 event mask"]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG, Dio28>;
impl<'a, REG> Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Set)
    }
}
#[doc = "DIO29 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` reader - DIO29 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio29::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio29::Set
    }
}
#[doc = "Field `DIO29` writer - DIO29 event mask"]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG, Dio29>;
impl<'a, REG> Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Set)
    }
}
#[doc = "DIO30 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` reader - DIO30 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio30::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio30::Set
    }
}
#[doc = "Field `DIO30` writer - DIO30 event mask"]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG, Dio30>;
impl<'a, REG> Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Set)
    }
}
#[doc = "DIO31 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: Event is masked"]
    Clr = 0,
    #[doc = "1: Event is unmasked"]
    Set = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` reader - DIO31 event mask"]
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
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dio31::Clr
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dio31::Set
    }
}
#[doc = "Field `DIO31` writer - DIO31 event mask"]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG, Dio31>;
impl<'a, REG> Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Clr)
    }
    #[doc = "Event is unmasked"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Set)
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
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpiocCpuIntImaskSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpiocCpuIntImaskSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpiocCpuIntImaskSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpiocCpuIntImaskSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpiocCpuIntImaskSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpiocCpuIntImaskSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpiocCpuIntImaskSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpiocCpuIntImaskSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpiocCpuIntImaskSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpiocCpuIntImaskSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpiocCpuIntImaskSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpiocCpuIntImaskSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpiocCpuIntImaskSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpiocCpuIntImaskSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpiocCpuIntImaskSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpiocCpuIntImaskSpec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, GpiocCpuIntImaskSpec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, GpiocCpuIntImaskSpec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, GpiocCpuIntImaskSpec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, GpiocCpuIntImaskSpec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, GpiocCpuIntImaskSpec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, GpiocCpuIntImaskSpec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, GpiocCpuIntImaskSpec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, GpiocCpuIntImaskSpec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, GpiocCpuIntImaskSpec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, GpiocCpuIntImaskSpec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, GpiocCpuIntImaskSpec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, GpiocCpuIntImaskSpec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, GpiocCpuIntImaskSpec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, GpiocCpuIntImaskSpec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, GpiocCpuIntImaskSpec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, GpiocCpuIntImaskSpec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_cpu_int_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_cpu_int_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocCpuIntImaskSpec;
impl crate::RegisterSpec for GpiocCpuIntImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_cpu_int_imask::R`](R) reader structure"]
impl crate::Readable for GpiocCpuIntImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioc_cpu_int_imask::W`](W) writer structure"]
impl crate::Writable for GpiocCpuIntImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_CPU_INT_IMASK to value 0"]
impl crate::Resettable for GpiocCpuIntImaskSpec {}
