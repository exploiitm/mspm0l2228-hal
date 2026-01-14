#[doc = "Register `LFSS_TOUT11_8` reader"]
pub type R = crate::R<LfssTout11_8Spec>;
#[doc = "Register `LFSS_TOUT11_8` writer"]
pub type W = crate::W<LfssTout11_8Spec>;
#[doc = "This bit sets the value of the pin tamper I/O 8 (TIO8) when the output is enabled through TOE8 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio8 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio8> for bool {
    #[inline(always)]
    fn from(variant: Tio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO8` reader - This bit sets the value of the pin tamper I/O 8 (TIO8) when the output is enabled through TOE8 register."]
pub type Tio8R = crate::BitReader<Tio8>;
impl Tio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio8 {
        match self.bits {
            false => Tio8::Zero,
            true => Tio8::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio8::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio8::One
    }
}
#[doc = "Field `TIO8` writer - This bit sets the value of the pin tamper I/O 8 (TIO8) when the output is enabled through TOE8 register."]
pub type Tio8W<'a, REG> = crate::BitWriter<'a, REG, Tio8>;
impl<'a, REG> Tio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 9 (TIO9) when the output is enabled through TOE9 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio9 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio9> for bool {
    #[inline(always)]
    fn from(variant: Tio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO9` reader - This bit sets the value of the pin tamper I/O 9 (TIO9) when the output is enabled through TOE9 register."]
pub type Tio9R = crate::BitReader<Tio9>;
impl Tio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio9 {
        match self.bits {
            false => Tio9::Zero,
            true => Tio9::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio9::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio9::One
    }
}
#[doc = "Field `TIO9` writer - This bit sets the value of the pin tamper I/O 9 (TIO9) when the output is enabled through TOE9 register."]
pub type Tio9W<'a, REG> = crate::BitWriter<'a, REG, Tio9>;
impl<'a, REG> Tio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 10 (TIO10) when the output is enabled through TOE10 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio10 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio10> for bool {
    #[inline(always)]
    fn from(variant: Tio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO10` reader - This bit sets the value of the pin tamper I/O 10 (TIO10) when the output is enabled through TOE10 register."]
pub type Tio10R = crate::BitReader<Tio10>;
impl Tio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio10 {
        match self.bits {
            false => Tio10::Zero,
            true => Tio10::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio10::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio10::One
    }
}
#[doc = "Field `TIO10` writer - This bit sets the value of the pin tamper I/O 10 (TIO10) when the output is enabled through TOE10 register."]
pub type Tio10W<'a, REG> = crate::BitWriter<'a, REG, Tio10>;
impl<'a, REG> Tio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 11 (TIO11) when the output is enabled through TOE11 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio11 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio11> for bool {
    #[inline(always)]
    fn from(variant: Tio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO11` reader - This bit sets the value of the pin tamper I/O 11 (TIO11) when the output is enabled through TOE11 register."]
pub type Tio11R = crate::BitReader<Tio11>;
impl Tio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio11 {
        match self.bits {
            false => Tio11::Zero,
            true => Tio11::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio11::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio11::One
    }
}
#[doc = "Field `TIO11` writer - This bit sets the value of the pin tamper I/O 11 (TIO11) when the output is enabled through TOE11 register."]
pub type Tio11W<'a, REG> = crate::BitWriter<'a, REG, Tio11>;
impl<'a, REG> Tio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::One)
    }
}
impl R {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 8 (TIO8) when the output is enabled through TOE8 register."]
    #[inline(always)]
    pub fn tio8(&self) -> Tio8R {
        Tio8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 9 (TIO9) when the output is enabled through TOE9 register."]
    #[inline(always)]
    pub fn tio9(&self) -> Tio9R {
        Tio9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 10 (TIO10) when the output is enabled through TOE10 register."]
    #[inline(always)]
    pub fn tio10(&self) -> Tio10R {
        Tio10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 11 (TIO11) when the output is enabled through TOE11 register."]
    #[inline(always)]
    pub fn tio11(&self) -> Tio11R {
        Tio11R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 8 (TIO8) when the output is enabled through TOE8 register."]
    #[inline(always)]
    pub fn tio8(&mut self) -> Tio8W<'_, LfssTout11_8Spec> {
        Tio8W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 9 (TIO9) when the output is enabled through TOE9 register."]
    #[inline(always)]
    pub fn tio9(&mut self) -> Tio9W<'_, LfssTout11_8Spec> {
        Tio9W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 10 (TIO10) when the output is enabled through TOE10 register."]
    #[inline(always)]
    pub fn tio10(&mut self) -> Tio10W<'_, LfssTout11_8Spec> {
        Tio10W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 11 (TIO11) when the output is enabled through TOE11 register."]
    #[inline(always)]
    pub fn tio11(&mut self) -> Tio11W<'_, LfssTout11_8Spec> {
        Tio11W::new(self, 24)
    }
}
#[doc = "Tamper Output 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout11_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout11_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTout11_8Spec;
impl crate::RegisterSpec for LfssTout11_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tout11_8::R`](R) reader structure"]
impl crate::Readable for LfssTout11_8Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tout11_8::W`](W) writer structure"]
impl crate::Writable for LfssTout11_8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOUT11_8 to value 0"]
impl crate::Resettable for LfssTout11_8Spec {}
