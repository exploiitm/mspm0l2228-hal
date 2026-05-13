#[doc = "Register `GPIOC_DOUTTGL31_0` writer"]
pub type W = crate::W<GpiocDouttgl31_0Spec>;
#[doc = "This bit is used to toggle DIO0 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` writer - This bit is used to toggle DIO0 output."]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO1 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` writer - This bit is used to toggle DIO1 output."]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO2 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` writer - This bit is used to toggle DIO2 output."]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO3 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` writer - This bit is used to toggle DIO3 output."]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO4 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` writer - This bit is used to toggle DIO4 output."]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO5 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` writer - This bit is used to toggle DIO5 output."]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO6 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` writer - This bit is used to toggle DIO6 output."]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO7 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` writer - This bit is used to toggle DIO7 output."]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO8 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` writer - This bit is used to toggle DIO8 output."]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dio8>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO9 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` writer - This bit is used to toggle DIO9 output."]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dio9>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO10 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` writer - This bit is used to toggle DIO10 output."]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dio10>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO11 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` writer - This bit is used to toggle DIO11 output."]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dio11>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO12 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` writer - This bit is used to toggle DIO12 output."]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dio12>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO13 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` writer - This bit is used to toggle DIO13 output."]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dio13>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO14 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` writer - This bit is used to toggle DIO14 output."]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dio14>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO15 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` writer - This bit is used to toggle DIO15 output."]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dio15>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO16 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` writer - This bit is used to toggle DIO16 output."]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO17 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` writer - This bit is used to toggle DIO17 output."]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO18 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` writer - This bit is used to toggle DIO18 output."]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO19 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` writer - This bit is used to toggle DIO19 output."]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO20 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` writer - This bit is used to toggle DIO20 output."]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO21 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` writer - This bit is used to toggle DIO21 output."]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO22 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` writer - This bit is used to toggle DIO22 output."]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO23 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` writer - This bit is used to toggle DIO23 output."]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO24 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` writer - This bit is used to toggle DIO24 output."]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO25 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` writer - This bit is used to toggle DIO25 output."]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO26 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO26` writer - This bit is used to toggle DIO26 output."]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG, Dio26>;
impl<'a, REG> Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO27 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO27` writer - This bit is used to toggle DIO27 output."]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG, Dio27>;
impl<'a, REG> Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO28 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` writer - This bit is used to toggle DIO28 output."]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG, Dio28>;
impl<'a, REG> Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO29 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` writer - This bit is used to toggle DIO29 output."]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG, Dio29>;
impl<'a, REG> Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO30 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` writer - This bit is used to toggle DIO30 output."]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG, Dio30>;
impl<'a, REG> Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Toggle)
    }
}
#[doc = "This bit is used to toggle DIO31 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Toggle output"]
    Toggle = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` writer - This bit is used to toggle DIO31 output."]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG, Dio31>;
impl<'a, REG> Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::NoEffect)
    }
    #[doc = "Toggle output"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Toggle)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to toggle DIO0 output."]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpiocDouttgl31_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to toggle DIO1 output."]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpiocDouttgl31_0Spec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to toggle DIO2 output."]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpiocDouttgl31_0Spec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is used to toggle DIO3 output."]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpiocDouttgl31_0Spec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is used to toggle DIO4 output."]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpiocDouttgl31_0Spec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is used to toggle DIO5 output."]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpiocDouttgl31_0Spec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is used to toggle DIO6 output."]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpiocDouttgl31_0Spec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is used to toggle DIO7 output."]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpiocDouttgl31_0Spec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit is used to toggle DIO8 output."]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpiocDouttgl31_0Spec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is used to toggle DIO9 output."]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpiocDouttgl31_0Spec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - This bit is used to toggle DIO10 output."]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpiocDouttgl31_0Spec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is used to toggle DIO11 output."]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpiocDouttgl31_0Spec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is used to toggle DIO12 output."]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpiocDouttgl31_0Spec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit is used to toggle DIO13 output."]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpiocDouttgl31_0Spec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - This bit is used to toggle DIO14 output."]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpiocDouttgl31_0Spec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - This bit is used to toggle DIO15 output."]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpiocDouttgl31_0Spec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - This bit is used to toggle DIO16 output."]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, GpiocDouttgl31_0Spec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit is used to toggle DIO17 output."]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, GpiocDouttgl31_0Spec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit is used to toggle DIO18 output."]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, GpiocDouttgl31_0Spec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - This bit is used to toggle DIO19 output."]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, GpiocDouttgl31_0Spec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit is used to toggle DIO20 output."]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, GpiocDouttgl31_0Spec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - This bit is used to toggle DIO21 output."]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, GpiocDouttgl31_0Spec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - This bit is used to toggle DIO22 output."]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, GpiocDouttgl31_0Spec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - This bit is used to toggle DIO23 output."]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, GpiocDouttgl31_0Spec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit is used to toggle DIO24 output."]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, GpiocDouttgl31_0Spec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit is used to toggle DIO25 output."]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, GpiocDouttgl31_0Spec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - This bit is used to toggle DIO26 output."]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, GpiocDouttgl31_0Spec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit is used to toggle DIO27 output."]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, GpiocDouttgl31_0Spec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit is used to toggle DIO28 output."]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, GpiocDouttgl31_0Spec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to toggle DIO29 output."]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, GpiocDouttgl31_0Spec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit is used to toggle DIO30 output."]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, GpiocDouttgl31_0Spec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is used to toggle DIO31 output."]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, GpiocDouttgl31_0Spec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_douttgl31_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocDouttgl31_0Spec;
impl crate::RegisterSpec for GpiocDouttgl31_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_douttgl31_0::W`](W) writer structure"]
impl crate::Writable for GpiocDouttgl31_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_DOUTTGL31_0 to value 0"]
impl crate::Resettable for GpiocDouttgl31_0Spec {}
