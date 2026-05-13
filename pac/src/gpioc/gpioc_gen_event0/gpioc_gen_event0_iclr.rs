#[doc = "Register `GPIOC_GEN_EVENT0_ICLR` writer"]
pub type W = crate::W<GpiocGenEvent0IclrSpec>;
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
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpiocGenEvent0IclrSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpiocGenEvent0IclrSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpiocGenEvent0IclrSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpiocGenEvent0IclrSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpiocGenEvent0IclrSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpiocGenEvent0IclrSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpiocGenEvent0IclrSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpiocGenEvent0IclrSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpiocGenEvent0IclrSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpiocGenEvent0IclrSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpiocGenEvent0IclrSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpiocGenEvent0IclrSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpiocGenEvent0IclrSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpiocGenEvent0IclrSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpiocGenEvent0IclrSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpiocGenEvent0IclrSpec> {
        Dio15W::new(self, 15)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_gen_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocGenEvent0IclrSpec;
impl crate::RegisterSpec for GpiocGenEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_gen_event0_iclr::W`](W) writer structure"]
impl crate::Writable for GpiocGenEvent0IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_GEN_EVENT0_ICLR to value 0"]
impl crate::Resettable for GpiocGenEvent0IclrSpec {}
