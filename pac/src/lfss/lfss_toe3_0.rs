#[doc = "Register `LFSS_TOE3_0` reader"]
pub type R = crate::R<LfssToe3_0Spec>;
#[doc = "Register `LFSS_TOE3_0` writer"]
pub type W = crate::W<LfssToe3_0Spec>;
#[doc = "Enables data output for tamper I/O 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio0 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio0> for bool {
    #[inline(always)]
    fn from(variant: Tio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO0` reader - Enables data output for tamper I/O 0"]
pub type Tio0R = crate::BitReader<Tio0>;
impl Tio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio0 {
        match self.bits {
            false => Tio0::Disable,
            true => Tio0::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio0::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio0::Enable
    }
}
#[doc = "Field `TIO0` writer - Enables data output for tamper I/O 0"]
pub type Tio0W<'a, REG> = crate::BitWriter<'a, REG, Tio0>;
impl<'a, REG> Tio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio1 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio1> for bool {
    #[inline(always)]
    fn from(variant: Tio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO1` reader - Enables data output for tamper I/O 1"]
pub type Tio1R = crate::BitReader<Tio1>;
impl Tio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio1 {
        match self.bits {
            false => Tio1::Disable,
            true => Tio1::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio1::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio1::Enable
    }
}
#[doc = "Field `TIO1` writer - Enables data output for tamper I/O 1"]
pub type Tio1W<'a, REG> = crate::BitWriter<'a, REG, Tio1>;
impl<'a, REG> Tio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio2 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio2> for bool {
    #[inline(always)]
    fn from(variant: Tio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO2` reader - Enables data output for tamper I/O 2"]
pub type Tio2R = crate::BitReader<Tio2>;
impl Tio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio2 {
        match self.bits {
            false => Tio2::Disable,
            true => Tio2::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio2::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio2::Enable
    }
}
#[doc = "Field `TIO2` writer - Enables data output for tamper I/O 2"]
pub type Tio2W<'a, REG> = crate::BitWriter<'a, REG, Tio2>;
impl<'a, REG> Tio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio3 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio3> for bool {
    #[inline(always)]
    fn from(variant: Tio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO3` reader - Enables data output for tamper I/O 3"]
pub type Tio3R = crate::BitReader<Tio3>;
impl Tio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio3 {
        match self.bits {
            false => Tio3::Disable,
            true => Tio3::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio3::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio3::Enable
    }
}
#[doc = "Field `TIO3` writer - Enables data output for tamper I/O 3"]
pub type Tio3W<'a, REG> = crate::BitWriter<'a, REG, Tio3>;
impl<'a, REG> Tio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables data output for tamper I/O 0"]
    #[inline(always)]
    pub fn tio0(&self) -> Tio0R {
        Tio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 1"]
    #[inline(always)]
    pub fn tio1(&self) -> Tio1R {
        Tio1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 2"]
    #[inline(always)]
    pub fn tio2(&self) -> Tio2R {
        Tio2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 3"]
    #[inline(always)]
    pub fn tio3(&self) -> Tio3R {
        Tio3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables data output for tamper I/O 0"]
    #[inline(always)]
    pub fn tio0(&mut self) -> Tio0W<'_, LfssToe3_0Spec> {
        Tio0W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 1"]
    #[inline(always)]
    pub fn tio1(&mut self) -> Tio1W<'_, LfssToe3_0Spec> {
        Tio1W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 2"]
    #[inline(always)]
    pub fn tio2(&mut self) -> Tio2W<'_, LfssToe3_0Spec> {
        Tio2W::new(self, 16)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 3"]
    #[inline(always)]
    pub fn tio3(&mut self) -> Tio3W<'_, LfssToe3_0Spec> {
        Tio3W::new(self, 24)
    }
}
#[doc = "Tamper Output Enable 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssToe3_0Spec;
impl crate::RegisterSpec for LfssToe3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_toe3_0::R`](R) reader structure"]
impl crate::Readable for LfssToe3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_toe3_0::W`](W) writer structure"]
impl crate::Writable for LfssToe3_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOE3_0 to value 0"]
impl crate::Resettable for LfssToe3_0Spec {}
