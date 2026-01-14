#[doc = "Register `LFSS_TOUT7_4` reader"]
pub type R = crate::R<LfssTout7_4Spec>;
#[doc = "Register `LFSS_TOUT7_4` writer"]
pub type W = crate::W<LfssTout7_4Spec>;
#[doc = "This bit sets the value of the pin tamper I/O 4 (TIO4) when the output is enabled through TOE4 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio4 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio4> for bool {
    #[inline(always)]
    fn from(variant: Tio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO4` reader - This bit sets the value of the pin tamper I/O 4 (TIO4) when the output is enabled through TOE4 register."]
pub type Tio4R = crate::BitReader<Tio4>;
impl Tio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio4 {
        match self.bits {
            false => Tio4::Zero,
            true => Tio4::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio4::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio4::One
    }
}
#[doc = "Field `TIO4` writer - This bit sets the value of the pin tamper I/O 4 (TIO4) when the output is enabled through TOE4 register."]
pub type Tio4W<'a, REG> = crate::BitWriter<'a, REG, Tio4>;
impl<'a, REG> Tio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 5 (TIO5) when the output is enabled through TOE5 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio5 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio5> for bool {
    #[inline(always)]
    fn from(variant: Tio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO5` reader - This bit sets the value of the pin tamper I/O 5 (TIO5) when the output is enabled through TOE5 register."]
pub type Tio5R = crate::BitReader<Tio5>;
impl Tio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio5 {
        match self.bits {
            false => Tio5::Zero,
            true => Tio5::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio5::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio5::One
    }
}
#[doc = "Field `TIO5` writer - This bit sets the value of the pin tamper I/O 5 (TIO5) when the output is enabled through TOE5 register."]
pub type Tio5W<'a, REG> = crate::BitWriter<'a, REG, Tio5>;
impl<'a, REG> Tio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 2 (TIO6) when the output is enabled through TOE6 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio6 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio6> for bool {
    #[inline(always)]
    fn from(variant: Tio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO6` reader - This bit sets the value of the pin tamper I/O 2 (TIO6) when the output is enabled through TOE6 register."]
pub type Tio6R = crate::BitReader<Tio6>;
impl Tio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio6 {
        match self.bits {
            false => Tio6::Zero,
            true => Tio6::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio6::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio6::One
    }
}
#[doc = "Field `TIO6` writer - This bit sets the value of the pin tamper I/O 2 (TIO6) when the output is enabled through TOE6 register."]
pub type Tio6W<'a, REG> = crate::BitWriter<'a, REG, Tio6>;
impl<'a, REG> Tio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 7 (TIO7) when the output is enabled through TOE7 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio7 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio7> for bool {
    #[inline(always)]
    fn from(variant: Tio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO7` reader - This bit sets the value of the pin tamper I/O 7 (TIO7) when the output is enabled through TOE7 register."]
pub type Tio7R = crate::BitReader<Tio7>;
impl Tio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio7 {
        match self.bits {
            false => Tio7::Zero,
            true => Tio7::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio7::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio7::One
    }
}
#[doc = "Field `TIO7` writer - This bit sets the value of the pin tamper I/O 7 (TIO7) when the output is enabled through TOE7 register."]
pub type Tio7W<'a, REG> = crate::BitWriter<'a, REG, Tio7>;
impl<'a, REG> Tio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::One)
    }
}
impl R {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 4 (TIO4) when the output is enabled through TOE4 register."]
    #[inline(always)]
    pub fn tio4(&self) -> Tio4R {
        Tio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 5 (TIO5) when the output is enabled through TOE5 register."]
    #[inline(always)]
    pub fn tio5(&self) -> Tio5R {
        Tio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 2 (TIO6) when the output is enabled through TOE6 register."]
    #[inline(always)]
    pub fn tio6(&self) -> Tio6R {
        Tio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 7 (TIO7) when the output is enabled through TOE7 register."]
    #[inline(always)]
    pub fn tio7(&self) -> Tio7R {
        Tio7R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 4 (TIO4) when the output is enabled through TOE4 register."]
    #[inline(always)]
    pub fn tio4(&mut self) -> Tio4W<'_, LfssTout7_4Spec> {
        Tio4W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 5 (TIO5) when the output is enabled through TOE5 register."]
    #[inline(always)]
    pub fn tio5(&mut self) -> Tio5W<'_, LfssTout7_4Spec> {
        Tio5W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 2 (TIO6) when the output is enabled through TOE6 register."]
    #[inline(always)]
    pub fn tio6(&mut self) -> Tio6W<'_, LfssTout7_4Spec> {
        Tio6W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 7 (TIO7) when the output is enabled through TOE7 register."]
    #[inline(always)]
    pub fn tio7(&mut self) -> Tio7W<'_, LfssTout7_4Spec> {
        Tio7W::new(self, 24)
    }
}
#[doc = "Tamper Output 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout7_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout7_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTout7_4Spec;
impl crate::RegisterSpec for LfssTout7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tout7_4::R`](R) reader structure"]
impl crate::Readable for LfssTout7_4Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tout7_4::W`](W) writer structure"]
impl crate::Writable for LfssTout7_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOUT7_4 to value 0"]
impl crate::Resettable for LfssTout7_4Spec {}
