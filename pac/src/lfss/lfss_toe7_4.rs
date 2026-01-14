#[doc = "Register `LFSS_TOE7_4` reader"]
pub type R = crate::R<LfssToe7_4Spec>;
#[doc = "Register `LFSS_TOE7_4` writer"]
pub type W = crate::W<LfssToe7_4Spec>;
#[doc = "Enables data output for tamper I/O 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio4 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio4> for bool {
    #[inline(always)]
    fn from(variant: Tio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO4` reader - Enables data output for tamper I/O 4"]
pub type Tio4R = crate::BitReader<Tio4>;
impl Tio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio4 {
        match self.bits {
            false => Tio4::Disable,
            true => Tio4::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio4::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio4::Enable
    }
}
#[doc = "Field `TIO4` writer - Enables data output for tamper I/O 4"]
pub type Tio4W<'a, REG> = crate::BitWriter<'a, REG, Tio4>;
impl<'a, REG> Tio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio5 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio5> for bool {
    #[inline(always)]
    fn from(variant: Tio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO5` reader - Enables data output for tamper I/O 5"]
pub type Tio5R = crate::BitReader<Tio5>;
impl Tio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio5 {
        match self.bits {
            false => Tio5::Disable,
            true => Tio5::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio5::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio5::Enable
    }
}
#[doc = "Field `TIO5` writer - Enables data output for tamper I/O 5"]
pub type Tio5W<'a, REG> = crate::BitWriter<'a, REG, Tio5>;
impl<'a, REG> Tio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio6 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio6> for bool {
    #[inline(always)]
    fn from(variant: Tio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO6` reader - Enables data output for tamper I/O 6"]
pub type Tio6R = crate::BitReader<Tio6>;
impl Tio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio6 {
        match self.bits {
            false => Tio6::Disable,
            true => Tio6::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio6::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio6::Enable
    }
}
#[doc = "Field `TIO6` writer - Enables data output for tamper I/O 6"]
pub type Tio6W<'a, REG> = crate::BitWriter<'a, REG, Tio6>;
impl<'a, REG> Tio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio7 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio7> for bool {
    #[inline(always)]
    fn from(variant: Tio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO7` reader - Enables data output for tamper I/O 7"]
pub type Tio7R = crate::BitReader<Tio7>;
impl Tio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio7 {
        match self.bits {
            false => Tio7::Disable,
            true => Tio7::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio7::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio7::Enable
    }
}
#[doc = "Field `TIO7` writer - Enables data output for tamper I/O 7"]
pub type Tio7W<'a, REG> = crate::BitWriter<'a, REG, Tio7>;
impl<'a, REG> Tio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables data output for tamper I/O 4"]
    #[inline(always)]
    pub fn tio4(&self) -> Tio4R {
        Tio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 5"]
    #[inline(always)]
    pub fn tio5(&self) -> Tio5R {
        Tio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 6"]
    #[inline(always)]
    pub fn tio6(&self) -> Tio6R {
        Tio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 7"]
    #[inline(always)]
    pub fn tio7(&self) -> Tio7R {
        Tio7R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables data output for tamper I/O 4"]
    #[inline(always)]
    pub fn tio4(&mut self) -> Tio4W<'_, LfssToe7_4Spec> {
        Tio4W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 5"]
    #[inline(always)]
    pub fn tio5(&mut self) -> Tio5W<'_, LfssToe7_4Spec> {
        Tio5W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 6"]
    #[inline(always)]
    pub fn tio6(&mut self) -> Tio6W<'_, LfssToe7_4Spec> {
        Tio6W::new(self, 16)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 7"]
    #[inline(always)]
    pub fn tio7(&mut self) -> Tio7W<'_, LfssToe7_4Spec> {
        Tio7W::new(self, 24)
    }
}
#[doc = "Tamper Output Enable 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe7_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe7_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssToe7_4Spec;
impl crate::RegisterSpec for LfssToe7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_toe7_4::R`](R) reader structure"]
impl crate::Readable for LfssToe7_4Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_toe7_4::W`](W) writer structure"]
impl crate::Writable for LfssToe7_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOE7_4 to value 0"]
impl crate::Resettable for LfssToe7_4Spec {}
