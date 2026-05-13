#[doc = "Register `GPIOC_CPU_INT_ICLR` writer"]
pub type W = crate::W<GpiocCpuIntIclrSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO0 in RIS register"]
    Clr = 1,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` writer - DIO0 event"]
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
    #[doc = "Clears DIO0 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Clr)
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO1 in RIS register"]
    Clr = 1,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` writer - DIO1 event"]
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
    #[doc = "Clears DIO1 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Clr)
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO2 in RIS register"]
    Clr = 1,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` writer - DIO2 event"]
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
    #[doc = "Clears DIO2 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Clr)
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO3 in RIS register"]
    Clr = 1,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` writer - DIO3 event"]
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
    #[doc = "Clears DIO3 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Clr)
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO4 in RIS register"]
    Clr = 1,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` writer - DIO4 event"]
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
    #[doc = "Clears DIO4 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Clr)
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO5 in RIS register"]
    Clr = 1,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` writer - DIO5 event"]
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
    #[doc = "Clears DIO5 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Clr)
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO6 in RIS register"]
    Clr = 1,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` writer - DIO6 event"]
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
    #[doc = "Clears DIO6 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Clr)
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO7 in RIS register"]
    Clr = 1,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` writer - DIO7 event"]
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
    #[doc = "Clears DIO7 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Clr)
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO8 in RIS register"]
    Clr = 1,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` writer - DIO8 event"]
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
    #[doc = "Clears DIO8 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Clr)
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO9 in RIS register"]
    Clr = 1,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` writer - DIO9 event"]
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
    #[doc = "Clears DIO9 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Clr)
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO10 in RIS register"]
    Clr = 1,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` writer - DIO10 event"]
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
    #[doc = "Clears DIO10 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Clr)
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO11 in RIS register"]
    Clr = 1,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` writer - DIO11 event"]
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
    #[doc = "Clears DIO11 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Clr)
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO12 in RIS register"]
    Clr = 1,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` writer - DIO12 event"]
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
    #[doc = "Clears DIO12 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Clr)
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO13 in RIS register"]
    Clr = 1,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` writer - DIO13 event"]
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
    #[doc = "Clears DIO13 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Clr)
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO14 in RIS register"]
    Clr = 1,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` writer - DIO14 event"]
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
    #[doc = "Clears DIO14 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Clr)
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO15 in RIS register"]
    Clr = 1,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` writer - DIO15 event"]
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
    #[doc = "Clears DIO15 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Clr)
    }
}
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO16 in RIS register"]
    Clr = 1,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` writer - DIO16 event"]
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
    #[doc = "Clears DIO16 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Clr)
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO17 in RIS register"]
    Clr = 1,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` writer - DIO17 event"]
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
    #[doc = "Clears DIO17 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Clr)
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO18 in RIS register"]
    Clr = 1,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` writer - DIO18 event"]
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
    #[doc = "Clears DIO18 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Clr)
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO19 in RIS register"]
    Clr = 1,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` writer - DIO19 event"]
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
    #[doc = "Clears DIO19 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Clr)
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO20 in RIS register"]
    Clr = 1,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` writer - DIO20 event"]
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
    #[doc = "Clears DIO20 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Clr)
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO21 in RIS register"]
    Clr = 1,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` writer - DIO21 event"]
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
    #[doc = "Clears DIO21 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Clr)
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO22 in RIS register"]
    Clr = 1,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` writer - DIO22 event"]
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
    #[doc = "Clears DIO22 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Clr)
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO23 in RIS register"]
    Clr = 1,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` writer - DIO23 event"]
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
    #[doc = "Clears DIO23 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Clr)
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO24 in RIS register"]
    Clr = 1,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` writer - DIO24 event"]
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
    #[doc = "Clears DIO24 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Clr)
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO25 in RIS register"]
    Clr = 1,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` writer - DIO25 event"]
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
    #[doc = "Clears DIO25 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Clr)
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO26 in RIS register"]
    Clr = 1,
}
impl From<Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO26` writer - DIO26 event"]
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
    #[doc = "Clears DIO26 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Clr)
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO27 in RIS register"]
    Clr = 1,
}
impl From<Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO27` writer - DIO27 event"]
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
    #[doc = "Clears DIO27 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Clr)
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO28 in RIS register"]
    Clr = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` writer - DIO28 event"]
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
    #[doc = "Clears DIO28 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Clr)
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO29 in RIS register"]
    Clr = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` writer - DIO29 event"]
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
    #[doc = "Clears DIO29 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Clr)
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO30 in RIS register"]
    Clr = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` writer - DIO30 event"]
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
    #[doc = "Clears DIO30 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Clr)
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears DIO31 in RIS register"]
    Clr = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` writer - DIO31 event"]
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
    #[doc = "Clears DIO31 in RIS register"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpiocCpuIntIclrSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpiocCpuIntIclrSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpiocCpuIntIclrSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpiocCpuIntIclrSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpiocCpuIntIclrSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpiocCpuIntIclrSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpiocCpuIntIclrSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpiocCpuIntIclrSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpiocCpuIntIclrSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpiocCpuIntIclrSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpiocCpuIntIclrSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpiocCpuIntIclrSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpiocCpuIntIclrSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpiocCpuIntIclrSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpiocCpuIntIclrSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpiocCpuIntIclrSpec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, GpiocCpuIntIclrSpec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, GpiocCpuIntIclrSpec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, GpiocCpuIntIclrSpec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, GpiocCpuIntIclrSpec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, GpiocCpuIntIclrSpec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, GpiocCpuIntIclrSpec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, GpiocCpuIntIclrSpec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, GpiocCpuIntIclrSpec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, GpiocCpuIntIclrSpec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, GpiocCpuIntIclrSpec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, GpiocCpuIntIclrSpec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, GpiocCpuIntIclrSpec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, GpiocCpuIntIclrSpec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, GpiocCpuIntIclrSpec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, GpiocCpuIntIclrSpec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, GpiocCpuIntIclrSpec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_cpu_int_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocCpuIntIclrSpec;
impl crate::RegisterSpec for GpiocCpuIntIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_cpu_int_iclr::W`](W) writer structure"]
impl crate::Writable for GpiocCpuIntIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_CPU_INT_ICLR to value 0"]
impl crate::Resettable for GpiocCpuIntIclrSpec {}
