#[doc = "Register `GPIOC_GEN_EVENT1_ISET` writer"]
pub type W = crate::W<GpiocGenEvent1IsetSpec>;
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO16 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO16 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Set)
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO17 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO17 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Set)
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO18 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO18 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Set)
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO19 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO19 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Set)
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO20 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO20 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Set)
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO21 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO21 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Set)
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO22 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO22 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Set)
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO23 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO23 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Set)
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO24 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO24 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Set)
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO25 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO25 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Set)
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO26 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO26 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Set)
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO27 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO27 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Set)
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO28 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO28 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Set)
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO29 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO29 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Set)
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO30 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO30 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Set)
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets DIO31 in RIS register"]
    Set = 1,
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
    #[doc = "Sets DIO31 in RIS register"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Set)
    }
}
impl W {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, GpiocGenEvent1IsetSpec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, GpiocGenEvent1IsetSpec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, GpiocGenEvent1IsetSpec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, GpiocGenEvent1IsetSpec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, GpiocGenEvent1IsetSpec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, GpiocGenEvent1IsetSpec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, GpiocGenEvent1IsetSpec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, GpiocGenEvent1IsetSpec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, GpiocGenEvent1IsetSpec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, GpiocGenEvent1IsetSpec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, GpiocGenEvent1IsetSpec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, GpiocGenEvent1IsetSpec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, GpiocGenEvent1IsetSpec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, GpiocGenEvent1IsetSpec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, GpiocGenEvent1IsetSpec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, GpiocGenEvent1IsetSpec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocGenEvent1IsetSpec;
impl crate::RegisterSpec for GpiocGenEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_gen_event1_iset::W`](W) writer structure"]
impl crate::Writable for GpiocGenEvent1IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_GEN_EVENT1_ISET to value 0"]
impl crate::Resettable for GpiocGenEvent1IsetSpec {}
