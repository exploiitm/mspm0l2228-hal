#[doc = "Register `TIMG4_IMASK` reader"]
pub type R = crate::R<Timg4ImaskSpec>;
#[doc = "Register `TIMG4_IMASK` writer"]
pub type W = crate::W<Timg4ImaskSpec>;
#[doc = "Zero Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Z {
    #[doc = "0: Disable Event"]
    Clr = 0,
    #[doc = "1: Enable Event"]
    Set = 1,
}
impl From<Z> for bool {
    #[inline(always)]
    fn from(variant: Z) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Z` reader - Zero Event mask"]
pub type ZR = crate::BitReader<Z>;
impl ZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Z {
        match self.bits {
            false => Z::Clr,
            true => Z::Set,
        }
    }
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Z::Clr
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Z::Set
    }
}
#[doc = "Field `Z` writer - Zero Event mask"]
pub type ZW<'a, REG> = crate::BitWriter<'a, REG, Z>;
impl<'a, REG> ZW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Z::Clr)
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Z::Set)
    }
}
#[doc = "Load Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<L> for bool {
    #[inline(always)]
    fn from(variant: L) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` reader - Load Event mask"]
pub type LR = crate::BitReader<L>;
impl LR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L {
        match self.bits {
            false => L::Clr,
            true => L::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == L::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == L::Set
    }
}
#[doc = "Field `L` writer - Load Event mask"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG, L>;
impl<'a, REG> LW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(L::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(L::Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd0> for bool {
    #[inline(always)]
    fn from(variant: Ccd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD0` reader - Capture or Compare DN event mask CCP0"]
pub type Ccd0R = crate::BitReader<Ccd0>;
impl Ccd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd0 {
        match self.bits {
            false => Ccd0::Clr,
            true => Ccd0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd0::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd0::Set
    }
}
#[doc = "Field `CCD0` writer - Capture or Compare DN event mask CCP0"]
pub type Ccd0W<'a, REG> = crate::BitWriter<'a, REG, Ccd0>;
impl<'a, REG> Ccd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd1> for bool {
    #[inline(always)]
    fn from(variant: Ccd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD1` reader - Capture or Compare DN event mask CCP1"]
pub type Ccd1R = crate::BitReader<Ccd1>;
impl Ccd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd1 {
        match self.bits {
            false => Ccd1::Clr,
            true => Ccd1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd1::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd1::Set
    }
}
#[doc = "Field `CCD1` writer - Capture or Compare DN event mask CCP1"]
pub type Ccd1W<'a, REG> = crate::BitWriter<'a, REG, Ccd1>;
impl<'a, REG> Ccd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu0> for bool {
    #[inline(always)]
    fn from(variant: Ccu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU0` reader - Capture or Compare UP event mask CCP0"]
pub type Ccu0R = crate::BitReader<Ccu0>;
impl Ccu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu0 {
        match self.bits {
            false => Ccu0::Clr,
            true => Ccu0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu0::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu0::Set
    }
}
#[doc = "Field `CCU0` writer - Capture or Compare UP event mask CCP0"]
pub type Ccu0W<'a, REG> = crate::BitWriter<'a, REG, Ccu0>;
impl<'a, REG> Ccu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu1> for bool {
    #[inline(always)]
    fn from(variant: Ccu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU1` reader - Capture or Compare UP event mask CCP1"]
pub type Ccu1R = crate::BitReader<Ccu1>;
impl Ccu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu1 {
        match self.bits {
            false => Ccu1::Clr,
            true => Ccu1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu1::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu1::Set
    }
}
#[doc = "Field `CCU1` writer - Capture or Compare UP event mask CCP1"]
pub type Ccu1W<'a, REG> = crate::BitWriter<'a, REG, Ccu1>;
impl<'a, REG> Ccu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::Set)
    }
}
#[doc = "Trigger Overflow Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tov {
    #[doc = "0: Disable Event"]
    Clr = 0,
    #[doc = "1: Enable Event"]
    Set = 1,
}
impl From<Tov> for bool {
    #[inline(always)]
    fn from(variant: Tov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOV` reader - Trigger Overflow Event mask"]
pub type TovR = crate::BitReader<Tov>;
impl TovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tov {
        match self.bits {
            false => Tov::Clr,
            true => Tov::Set,
        }
    }
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tov::Clr
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tov::Set
    }
}
#[doc = "Field `TOV` writer - Trigger Overflow Event mask"]
pub type TovW<'a, REG> = crate::BitWriter<'a, REG, Tov>;
impl<'a, REG> TovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::Clr)
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or Compare DN event mask CCP0"]
    #[inline(always)]
    pub fn ccd0(&self) -> Ccd0R {
        Ccd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or Compare DN event mask CCP1"]
    #[inline(always)]
    pub fn ccd1(&self) -> Ccd1R {
        Ccd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or Compare UP event mask CCP0"]
    #[inline(always)]
    pub fn ccu0(&self) -> Ccu0R {
        Ccu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or Compare UP event mask CCP1"]
    #[inline(always)]
    pub fn ccu1(&self) -> Ccu1R {
        Ccu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, Timg4ImaskSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, Timg4ImaskSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or Compare DN event mask CCP0"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, Timg4ImaskSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or Compare DN event mask CCP1"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, Timg4ImaskSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 8 - Capture or Compare UP event mask CCP0"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, Timg4ImaskSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or Compare UP event mask CCP1"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, Timg4ImaskSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, Timg4ImaskSpec> {
        TovW::new(self, 25)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg4ImaskSpec;
impl crate::RegisterSpec for Timg4ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg4_imask::R`](R) reader structure"]
impl crate::Readable for Timg4ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`timg4_imask::W`](W) writer structure"]
impl crate::Writable for Timg4ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG4_IMASK to value 0"]
impl crate::Resettable for Timg4ImaskSpec {}
