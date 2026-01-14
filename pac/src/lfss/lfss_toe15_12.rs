#[doc = "Register `LFSS_TOE15_12` reader"]
pub type R = crate::R<LfssToe15_12Spec>;
#[doc = "Register `LFSS_TOE15_12` writer"]
pub type W = crate::W<LfssToe15_12Spec>;
#[doc = "Enables data output for tamper I/O 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio12 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio12> for bool {
    #[inline(always)]
    fn from(variant: Tio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO12` reader - Enables data output for tamper I/O 12"]
pub type Tio12R = crate::BitReader<Tio12>;
impl Tio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio12 {
        match self.bits {
            false => Tio12::Disable,
            true => Tio12::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio12::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio12::Enable
    }
}
#[doc = "Field `TIO12` writer - Enables data output for tamper I/O 12"]
pub type Tio12W<'a, REG> = crate::BitWriter<'a, REG, Tio12>;
impl<'a, REG> Tio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio13 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio13> for bool {
    #[inline(always)]
    fn from(variant: Tio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO13` reader - Enables data output for tamper I/O 13"]
pub type Tio13R = crate::BitReader<Tio13>;
impl Tio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio13 {
        match self.bits {
            false => Tio13::Disable,
            true => Tio13::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio13::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio13::Enable
    }
}
#[doc = "Field `TIO13` writer - Enables data output for tamper I/O 13"]
pub type Tio13W<'a, REG> = crate::BitWriter<'a, REG, Tio13>;
impl<'a, REG> Tio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio14 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio14> for bool {
    #[inline(always)]
    fn from(variant: Tio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO14` reader - Enables data output for tamper I/O 14"]
pub type Tio14R = crate::BitReader<Tio14>;
impl Tio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio14 {
        match self.bits {
            false => Tio14::Disable,
            true => Tio14::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio14::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio14::Enable
    }
}
#[doc = "Field `TIO14` writer - Enables data output for tamper I/O 14"]
pub type Tio14W<'a, REG> = crate::BitWriter<'a, REG, Tio14>;
impl<'a, REG> Tio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::Enable)
    }
}
#[doc = "Enables data output for tamper I/O 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio15 {
    #[doc = "0: output disabled"]
    Disable = 0,
    #[doc = "1: output enabled"]
    Enable = 1,
}
impl From<Tio15> for bool {
    #[inline(always)]
    fn from(variant: Tio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO15` reader - Enables data output for tamper I/O 15"]
pub type Tio15R = crate::BitReader<Tio15>;
impl Tio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio15 {
        match self.bits {
            false => Tio15::Disable,
            true => Tio15::Enable,
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tio15::Disable
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tio15::Enable
    }
}
#[doc = "Field `TIO15` writer - Enables data output for tamper I/O 15"]
pub type Tio15W<'a, REG> = crate::BitWriter<'a, REG, Tio15>;
impl<'a, REG> Tio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::Disable)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables data output for tamper I/O 12"]
    #[inline(always)]
    pub fn tio12(&self) -> Tio12R {
        Tio12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 13"]
    #[inline(always)]
    pub fn tio13(&self) -> Tio13R {
        Tio13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 14"]
    #[inline(always)]
    pub fn tio14(&self) -> Tio14R {
        Tio14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 15"]
    #[inline(always)]
    pub fn tio15(&self) -> Tio15R {
        Tio15R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables data output for tamper I/O 12"]
    #[inline(always)]
    pub fn tio12(&mut self) -> Tio12W<'_, LfssToe15_12Spec> {
        Tio12W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables data output for tamper I/O 13"]
    #[inline(always)]
    pub fn tio13(&mut self) -> Tio13W<'_, LfssToe15_12Spec> {
        Tio13W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables data output for tamper I/O 14"]
    #[inline(always)]
    pub fn tio14(&mut self) -> Tio14W<'_, LfssToe15_12Spec> {
        Tio14W::new(self, 16)
    }
    #[doc = "Bit 24 - Enables data output for tamper I/O 15"]
    #[inline(always)]
    pub fn tio15(&mut self) -> Tio15W<'_, LfssToe15_12Spec> {
        Tio15W::new(self, 24)
    }
}
#[doc = "Tamper Output Enable 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe15_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe15_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssToe15_12Spec;
impl crate::RegisterSpec for LfssToe15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_toe15_12::R`](R) reader structure"]
impl crate::Readable for LfssToe15_12Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_toe15_12::W`](W) writer structure"]
impl crate::Writable for LfssToe15_12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOE15_12 to value 0"]
impl crate::Resettable for LfssToe15_12Spec {}
