#[doc = "Register `TIMG4_ISET` writer"]
pub type W = crate::W<Timg4IsetSpec>;
#[doc = "Zero event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Z {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Z> for bool {
    #[inline(always)]
    fn from(variant: Z) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Z` writer - Zero event SET"]
pub type ZW<'a, REG> = crate::BitWriter<'a, REG, Z>;
impl<'a, REG> ZW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Z::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Z::Set)
    }
}
#[doc = "Load event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<L> for bool {
    #[inline(always)]
    fn from(variant: L) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` writer - Load event SET"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG, L>;
impl<'a, REG> LW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(L::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(L::Set)
    }
}
#[doc = "Capture or compare down event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd0 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd0> for bool {
    #[inline(always)]
    fn from(variant: Ccd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD0` writer - Capture or compare down event SET"]
pub type Ccd0W<'a, REG> = crate::BitWriter<'a, REG, Ccd0>;
impl<'a, REG> Ccd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::Set)
    }
}
#[doc = "Capture or compare down event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd1 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccd1> for bool {
    #[inline(always)]
    fn from(variant: Ccd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD1` writer - Capture or compare down event SET"]
pub type Ccd1W<'a, REG> = crate::BitWriter<'a, REG, Ccd1>;
impl<'a, REG> Ccd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::Set)
    }
}
#[doc = "Capture or compare up event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu0 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu0> for bool {
    #[inline(always)]
    fn from(variant: Ccu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU0` writer - Capture or compare up event SET"]
pub type Ccu0W<'a, REG> = crate::BitWriter<'a, REG, Ccu0>;
impl<'a, REG> Ccu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::Set)
    }
}
#[doc = "Capture or compare up event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu1 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Ccu1> for bool {
    #[inline(always)]
    fn from(variant: Ccu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU1` writer - Capture or compare up event SET"]
pub type Ccu1W<'a, REG> = crate::BitWriter<'a, REG, Ccu1>;
impl<'a, REG> Ccu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::Set)
    }
}
#[doc = "Trigger Overflow event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tov {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Set"]
    Set = 1,
}
impl From<Tov> for bool {
    #[inline(always)]
    fn from(variant: Tov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOV` writer - Trigger Overflow event SET"]
pub type TovW<'a, REG> = crate::BitWriter<'a, REG, Tov>;
impl<'a, REG> TovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::NoEffect)
    }
    #[doc = "Event Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Zero event SET"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, Timg4IsetSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load event SET"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, Timg4IsetSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, Timg4IsetSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, Timg4IsetSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 8 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, Timg4IsetSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, Timg4IsetSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 25 - Trigger Overflow event SET"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, Timg4IsetSpec> {
        TovW::new(self, 25)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg4IsetSpec;
impl crate::RegisterSpec for Timg4IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timg4_iset::W`](W) writer structure"]
impl crate::Writable for Timg4IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG4_ISET to value 0"]
impl crate::Resettable for Timg4IsetSpec {}
