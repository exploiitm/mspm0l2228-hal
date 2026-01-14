#[doc = "Register `TIMG8_ICLR` writer"]
pub type W = crate::W<Timg8IclrSpec>;
#[doc = "Zero event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Z {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Z> for bool {
    #[inline(always)]
    fn from(variant: Z) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Z` writer - Zero event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Z::Clr)
    }
}
#[doc = "Load event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<L> for bool {
    #[inline(always)]
    fn from(variant: L) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` writer - Load event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(L::Clr)
    }
}
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd0 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccd0> for bool {
    #[inline(always)]
    fn from(variant: Ccd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD0` writer - Capture or compare down event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::Clr)
    }
}
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd1 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccd1> for bool {
    #[inline(always)]
    fn from(variant: Ccd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD1` writer - Capture or compare down event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::Clr)
    }
}
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu0 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccu0> for bool {
    #[inline(always)]
    fn from(variant: Ccu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU0` writer - Capture or compare up event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::Clr)
    }
}
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu1 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccu1> for bool {
    #[inline(always)]
    fn from(variant: Ccu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU1` writer - Capture or compare up event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::Clr)
    }
}
#[doc = "Trigger Overflow event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tov {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Tov> for bool {
    #[inline(always)]
    fn from(variant: Tov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOV` writer - Trigger Overflow event CLEAR"]
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
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::Clr)
    }
}
#[doc = "Direction Change event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dc {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Dc> for bool {
    #[inline(always)]
    fn from(variant: Dc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC` writer - Direction Change event CLEAR"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG, Dc>;
impl<'a, REG> DcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dc::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dc::Clr)
    }
}
#[doc = "QEIERR event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qeierr {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Qeierr> for bool {
    #[inline(always)]
    fn from(variant: Qeierr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QEIERR` writer - QEIERR event CLEAR"]
pub type QeierrW<'a, REG> = crate::BitWriter<'a, REG, Qeierr>;
impl<'a, REG> QeierrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Qeierr::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Qeierr::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Zero event CLEAR"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, Timg8IclrSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load event CLEAR"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, Timg8IclrSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, Timg8IclrSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, Timg8IclrSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 8 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, Timg8IclrSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, Timg8IclrSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 25 - Trigger Overflow event CLEAR"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, Timg8IclrSpec> {
        TovW::new(self, 25)
    }
    #[doc = "Bit 27 - Direction Change event CLEAR"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<'_, Timg8IclrSpec> {
        DcW::new(self, 27)
    }
    #[doc = "Bit 28 - QEIERR event CLEAR"]
    #[inline(always)]
    pub fn qeierr(&mut self) -> QeierrW<'_, Timg8IclrSpec> {
        QeierrW::new(self, 28)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg8IclrSpec;
impl crate::RegisterSpec for Timg8IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timg8_iclr::W`](W) writer structure"]
impl crate::Writable for Timg8IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG8_ICLR to value 0"]
impl crate::Resettable for Timg8IclrSpec {}
