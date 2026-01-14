#[doc = "Register `TIMA0_ICLR` writer"]
pub type W = crate::W<Tima0IclrSpec>;
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
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd2 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccd2> for bool {
    #[inline(always)]
    fn from(variant: Ccd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD2` writer - Capture or compare down event CLEAR"]
pub type Ccd2W<'a, REG> = crate::BitWriter<'a, REG, Ccd2>;
impl<'a, REG> Ccd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd2::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd2::Clr)
    }
}
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd3 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccd3> for bool {
    #[inline(always)]
    fn from(variant: Ccd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD3` writer - Capture or compare down event CLEAR"]
pub type Ccd3W<'a, REG> = crate::BitWriter<'a, REG, Ccd3>;
impl<'a, REG> Ccd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd3::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd3::Clr)
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
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu2 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccu2> for bool {
    #[inline(always)]
    fn from(variant: Ccu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU2` writer - Capture or compare up event CLEAR"]
pub type Ccu2W<'a, REG> = crate::BitWriter<'a, REG, Ccu2>;
impl<'a, REG> Ccu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu2::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu2::Clr)
    }
}
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu3 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccu3> for bool {
    #[inline(always)]
    fn from(variant: Ccu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU3` writer - Capture or compare up event CLEAR"]
pub type Ccu3W<'a, REG> = crate::BitWriter<'a, REG, Ccu3>;
impl<'a, REG> Ccu3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu3::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu3::Clr)
    }
}
#[doc = "Compare down event 4 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd4 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccd4> for bool {
    #[inline(always)]
    fn from(variant: Ccd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD4` writer - Compare down event 4 CLEAR"]
pub type Ccd4W<'a, REG> = crate::BitWriter<'a, REG, Ccd4>;
impl<'a, REG> Ccd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd4::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd4::Clr)
    }
}
#[doc = "Compare down event 5 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd5 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccd5> for bool {
    #[inline(always)]
    fn from(variant: Ccd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD5` writer - Compare down event 5 CLEAR"]
pub type Ccd5W<'a, REG> = crate::BitWriter<'a, REG, Ccd5>;
impl<'a, REG> Ccd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd5::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd5::Clr)
    }
}
#[doc = "Compare up event 4 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu4 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccu4> for bool {
    #[inline(always)]
    fn from(variant: Ccu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU4` writer - Compare up event 4 CLEAR"]
pub type Ccu4W<'a, REG> = crate::BitWriter<'a, REG, Ccu4>;
impl<'a, REG> Ccu4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu4::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu4::Clr)
    }
}
#[doc = "Compare up event 5 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu5 {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Ccu5> for bool {
    #[inline(always)]
    fn from(variant: Ccu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU5` writer - Compare up event 5 CLEAR"]
pub type Ccu5W<'a, REG> = crate::BitWriter<'a, REG, Ccu5>;
impl<'a, REG> Ccu5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu5::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu5::Clr)
    }
}
#[doc = "Fault event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<F> for bool {
    #[inline(always)]
    fn from(variant: F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F` writer - Fault event CLEAR"]
pub type FW<'a, REG> = crate::BitWriter<'a, REG, F>;
impl<'a, REG> FW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(F::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(F::Clr)
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
#[doc = "Repeat Counter Zero event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repc {
    #[doc = "0: Writing 0 has no effect."]
    NoEffect = 0,
    #[doc = "1: Event Clear"]
    Clr = 1,
}
impl From<Repc> for bool {
    #[inline(always)]
    fn from(variant: Repc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPC` writer - Repeat Counter Zero event CLEAR"]
pub type RepcW<'a, REG> = crate::BitWriter<'a, REG, Repc>;
impl<'a, REG> RepcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Repc::NoEffect)
    }
    #[doc = "Event Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Repc::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Zero event CLEAR"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, Tima0IclrSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load event CLEAR"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, Tima0IclrSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, Tima0IclrSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, Tima0IclrSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn ccd2(&mut self) -> Ccd2W<'_, Tima0IclrSpec> {
        Ccd2W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn ccd3(&mut self) -> Ccd3W<'_, Tima0IclrSpec> {
        Ccd3W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, Tima0IclrSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, Tima0IclrSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn ccu2(&mut self) -> Ccu2W<'_, Tima0IclrSpec> {
        Ccu2W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn ccu3(&mut self) -> Ccu3W<'_, Tima0IclrSpec> {
        Ccu3W::new(self, 11)
    }
    #[doc = "Bit 12 - Compare down event 4 CLEAR"]
    #[inline(always)]
    pub fn ccd4(&mut self) -> Ccd4W<'_, Tima0IclrSpec> {
        Ccd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare down event 5 CLEAR"]
    #[inline(always)]
    pub fn ccd5(&mut self) -> Ccd5W<'_, Tima0IclrSpec> {
        Ccd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare up event 4 CLEAR"]
    #[inline(always)]
    pub fn ccu4(&mut self) -> Ccu4W<'_, Tima0IclrSpec> {
        Ccu4W::new(self, 14)
    }
    #[doc = "Bit 15 - Compare up event 5 CLEAR"]
    #[inline(always)]
    pub fn ccu5(&mut self) -> Ccu5W<'_, Tima0IclrSpec> {
        Ccu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault event CLEAR"]
    #[inline(always)]
    pub fn f(&mut self) -> FW<'_, Tima0IclrSpec> {
        FW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow event CLEAR"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, Tima0IclrSpec> {
        TovW::new(self, 25)
    }
    #[doc = "Bit 26 - Repeat Counter Zero event CLEAR"]
    #[inline(always)]
    pub fn repc(&mut self) -> RepcW<'_, Tima0IclrSpec> {
        RepcW::new(self, 26)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0IclrSpec;
impl crate::RegisterSpec for Tima0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tima0_iclr::W`](W) writer structure"]
impl crate::Writable for Tima0IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_ICLR to value 0"]
impl crate::Resettable for Tima0IclrSpec {}
