#[doc = "Register `LFSS_TOE11_8` reader"]
pub type R = crate::R<LfssToe11_8Spec>;
#[doc = "Register `LFSS_TOE11_8` writer"]
pub type W = crate::W<LfssToe11_8Spec>;
#[doc = "Enables data output for tamper I/O 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio8 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio8> for bool {
    #[inline(always)]
    fn from(variant: Tio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO8` reader - Enables data output for tamper I/O 8"]
pub type Tio8R = crate::BitReader<Tio8>;
impl Tio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio8 {
        match self.bits {
            false => Tio8::Disable,
            true => Tio8::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio8::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio8::Enable
    }
}
#[doc = "Field `TIO8` writer - Enables data output for tamper I/O 8"]
pub type Tio8W<'a, REG> = crate::BitWriter<'a, REG, Tio8>;
impl<'a, REG> Tio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio9 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio9> for bool {
    #[inline(always)]
    fn from(variant: Tio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO9` reader - Enables data output for tamper I/O 9"]
pub type Tio9R = crate::BitReader<Tio9>;
impl Tio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio9 {
        match self.bits {
            false => Tio9::Disable,
            true => Tio9::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio9::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio9::Enable
    }
}
#[doc = "Field `TIO9` writer - Enables data output for tamper I/O 9"]
pub type Tio9W<'a, REG> = crate::BitWriter<'a, REG, Tio9>;
impl<'a, REG> Tio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio10 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio10> for bool {
    #[inline(always)]
    fn from(variant: Tio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO10` reader - Enables data output for tamper I/O 10"]
pub type Tio10R = crate::BitReader<Tio10>;
impl Tio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio10 {
        match self.bits {
            false => Tio10::Disable,
            true => Tio10::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio10::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio10::Enable
    }
}
#[doc = "Field `TIO10` writer - Enables data output for tamper I/O 10"]
pub type Tio10W<'a, REG> = crate::BitWriter<'a, REG, Tio10>;
impl<'a, REG> Tio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio11 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio11> for bool {
    #[inline(always)]
    fn from(variant: Tio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO11` reader - Enables data output for tamper I/O 11"]
pub type Tio11R = crate::BitReader<Tio11>;
impl Tio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio11 {
        match self.bits {
            false => Tio11::Disable,
            true => Tio11::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio11::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio11::Enable
    }
}
#[doc = "Field `TIO11` writer - Enables data output for tamper I/O 11"]
pub type Tio11W<'a, REG> = crate::BitWriter<'a, REG, Tio11>;
impl<'a, REG> Tio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables data output for tamper I/O 8"]
    #[inline(always)]
    pub fn tio8(&self) -> Tio8R {
        Tio8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 9"]
    #[inline(always)]
    pub fn tio9(&self) -> Tio9R {
        Tio9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 10"]
    #[inline(always)]
    pub fn tio10(&self) -> Tio10R {
        Tio10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 11"]
    #[inline(always)]
    pub fn tio11(&self) -> Tio11R {
        Tio11R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables data output for tamper I/O 8"]
    #[inline(always)]
    pub fn tio8(&mut self) -> Tio8W<'_, LfssToe11_8Spec> {
        Tio8W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 9"]
    #[inline(always)]
    pub fn tio9(&mut self) -> Tio9W<'_, LfssToe11_8Spec> {
        Tio9W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 10"]
    #[inline(always)]
    pub fn tio10(&mut self) -> Tio10W<'_, LfssToe11_8Spec> {
        Tio10W::new(self, 16)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 11"]
    #[inline(always)]
    pub fn tio11(&mut self) -> Tio11W<'_, LfssToe11_8Spec> {
        Tio11W::new(self, 24)
    }
}
#[doc = "Tamper Output Enable 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe11_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe11_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssToe11_8Spec;
impl crate::RegisterSpec for LfssToe11_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_toe11_8::R`](R) reader structure"]
impl crate::Readable for LfssToe11_8Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_toe11_8::W`](W) writer structure"]
impl crate::Writable for LfssToe11_8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOE11_8 to value 0"]
impl crate::Resettable for LfssToe11_8Spec {}
