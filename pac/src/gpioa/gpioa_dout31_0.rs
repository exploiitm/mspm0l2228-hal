#[doc = "Register `GPIOA_DOUT31_0` reader"]
pub type R = crate::R<GpioaDout31_0Spec>;
#[doc = "Register `GPIOA_DOUT31_0` writer"]
pub type W = crate::W<GpioaDout31_0Spec>;
#[doc = "This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` reader - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dio0R = crate::BitReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            false => Dio0::Zero,
            true => Dio0::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio0::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio0::One
    }
}
#[doc = "Field `DIO0` writer - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` reader - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dio1R = crate::BitReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            false => Dio1::Zero,
            true => Dio1::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio1::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio1::One
    }
}
#[doc = "Field `DIO1` writer - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` reader - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dio2R = crate::BitReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            false => Dio2::Zero,
            true => Dio2::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio2::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio2::One
    }
}
#[doc = "Field `DIO2` writer - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` reader - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dio3R = crate::BitReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            false => Dio3::Zero,
            true => Dio3::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio3::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio3::One
    }
}
#[doc = "Field `DIO3` writer - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` reader - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dio4R = crate::BitReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            false => Dio4::Zero,
            true => Dio4::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio4::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio4::One
    }
}
#[doc = "Field `DIO4` writer - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` reader - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dio5R = crate::BitReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            false => Dio5::Zero,
            true => Dio5::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio5::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio5::One
    }
}
#[doc = "Field `DIO5` writer - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` reader - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dio6R = crate::BitReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            false => Dio6::Zero,
            true => Dio6::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio6::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio6::One
    }
}
#[doc = "Field `DIO6` writer - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` reader - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dio7R = crate::BitReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            false => Dio7::Zero,
            true => Dio7::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio7::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio7::One
    }
}
#[doc = "Field `DIO7` writer - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` reader - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dio8R = crate::BitReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            false => Dio8::Zero,
            true => Dio8::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio8::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio8::One
    }
}
#[doc = "Field `DIO8` writer - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dio8>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` reader - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dio9R = crate::BitReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            false => Dio9::Zero,
            true => Dio9::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio9::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio9::One
    }
}
#[doc = "Field `DIO9` writer - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dio9>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` reader - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dio10R = crate::BitReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            false => Dio10::Zero,
            true => Dio10::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio10::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio10::One
    }
}
#[doc = "Field `DIO10` writer - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dio10>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` reader - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dio11R = crate::BitReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            false => Dio11::Zero,
            true => Dio11::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio11::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio11::One
    }
}
#[doc = "Field `DIO11` writer - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dio11>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` reader - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dio12R = crate::BitReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            false => Dio12::Zero,
            true => Dio12::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio12::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio12::One
    }
}
#[doc = "Field `DIO12` writer - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dio12>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` reader - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dio13R = crate::BitReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            false => Dio13::Zero,
            true => Dio13::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio13::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio13::One
    }
}
#[doc = "Field `DIO13` writer - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dio13>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` reader - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dio14R = crate::BitReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            false => Dio14::Zero,
            true => Dio14::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio14::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio14::One
    }
}
#[doc = "Field `DIO14` writer - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dio14>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` reader - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dio15R = crate::BitReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            false => Dio15::Zero,
            true => Dio15::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio15::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio15::One
    }
}
#[doc = "Field `DIO15` writer - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dio15>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dio16R = crate::BitReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            false => Dio16::Zero,
            true => Dio16::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio16::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio16::One
    }
}
#[doc = "Field `DIO16` writer - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dio17R = crate::BitReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            false => Dio17::Zero,
            true => Dio17::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio17::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio17::One
    }
}
#[doc = "Field `DIO17` writer - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dio18R = crate::BitReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            false => Dio18::Zero,
            true => Dio18::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio18::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio18::One
    }
}
#[doc = "Field `DIO18` writer - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dio19R = crate::BitReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            false => Dio19::Zero,
            true => Dio19::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio19::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio19::One
    }
}
#[doc = "Field `DIO19` writer - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dio20R = crate::BitReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            false => Dio20::Zero,
            true => Dio20::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio20::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio20::One
    }
}
#[doc = "Field `DIO20` writer - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dio21R = crate::BitReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            false => Dio21::Zero,
            true => Dio21::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio21::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio21::One
    }
}
#[doc = "Field `DIO21` writer - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dio22R = crate::BitReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            false => Dio22::Zero,
            true => Dio22::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio22::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio22::One
    }
}
#[doc = "Field `DIO22` writer - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dio23R = crate::BitReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            false => Dio23::Zero,
            true => Dio23::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio23::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio23::One
    }
}
#[doc = "Field `DIO23` writer - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dio24R = crate::BitReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            false => Dio24::Zero,
            true => Dio24::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio24::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio24::One
    }
}
#[doc = "Field `DIO24` writer - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dio25R = crate::BitReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            false => Dio25::Zero,
            true => Dio25::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio25::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio25::One
    }
}
#[doc = "Field `DIO25` writer - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO26` reader - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dio26R = crate::BitReader<Dio26>;
impl Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio26 {
        match self.bits {
            false => Dio26::Zero,
            true => Dio26::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio26::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio26::One
    }
}
#[doc = "Field `DIO26` writer - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG, Dio26>;
impl<'a, REG> Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO27` reader - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dio27R = crate::BitReader<Dio27>;
impl Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio27 {
        match self.bits {
            false => Dio27::Zero,
            true => Dio27::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio27::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio27::One
    }
}
#[doc = "Field `DIO27` writer - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG, Dio27>;
impl<'a, REG> Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` reader - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dio28R = crate::BitReader<Dio28>;
impl Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio28 {
        match self.bits {
            false => Dio28::Zero,
            true => Dio28::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio28::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio28::One
    }
}
#[doc = "Field `DIO28` writer - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG, Dio28>;
impl<'a, REG> Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` reader - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dio29R = crate::BitReader<Dio29>;
impl Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio29 {
        match self.bits {
            false => Dio29::Zero,
            true => Dio29::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio29::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio29::One
    }
}
#[doc = "Field `DIO29` writer - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG, Dio29>;
impl<'a, REG> Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` reader - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dio30R = crate::BitReader<Dio30>;
impl Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio30 {
        match self.bits {
            false => Dio30::Zero,
            true => Dio30::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio30::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio30::One
    }
}
#[doc = "Field `DIO30` writer - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG, Dio30>;
impl<'a, REG> Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` reader - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dio31R = crate::BitReader<Dio31>;
impl Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio31 {
        match self.bits {
            false => Dio31::Zero,
            true => Dio31::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio31::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio31::One
    }
}
#[doc = "Field `DIO31` writer - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG, Dio31>;
impl<'a, REG> Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::One)
    }
}
impl R {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpioaDout31_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpioaDout31_0Spec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpioaDout31_0Spec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpioaDout31_0Spec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpioaDout31_0Spec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpioaDout31_0Spec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpioaDout31_0Spec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpioaDout31_0Spec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpioaDout31_0Spec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpioaDout31_0Spec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpioaDout31_0Spec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpioaDout31_0Spec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpioaDout31_0Spec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpioaDout31_0Spec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpioaDout31_0Spec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpioaDout31_0Spec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, GpioaDout31_0Spec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, GpioaDout31_0Spec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, GpioaDout31_0Spec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, GpioaDout31_0Spec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, GpioaDout31_0Spec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, GpioaDout31_0Spec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, GpioaDout31_0Spec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, GpioaDout31_0Spec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, GpioaDout31_0Spec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, GpioaDout31_0Spec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, GpioaDout31_0Spec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, GpioaDout31_0Spec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, GpioaDout31_0Spec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, GpioaDout31_0Spec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, GpioaDout31_0Spec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, GpioaDout31_0Spec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Data output 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_dout31_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout31_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaDout31_0Spec;
impl crate::RegisterSpec for GpioaDout31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa_dout31_0::R`](R) reader structure"]
impl crate::Readable for GpioaDout31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa_dout31_0::W`](W) writer structure"]
impl crate::Writable for GpioaDout31_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA_DOUT31_0 to value 0"]
impl crate::Resettable for GpioaDout31_0Spec {}
