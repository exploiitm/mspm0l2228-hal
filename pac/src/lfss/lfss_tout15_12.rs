#[doc = "Register `LFSS_TOUT15_12` reader"]
pub type R = crate::R<LfssTout15_12Spec>;
#[doc = "Register `LFSS_TOUT15_12` writer"]
pub type W = crate::W<LfssTout15_12Spec>;
#[doc = "This bit sets the value of the pin tamper I/O 12 (TIO12) when the output is enabled through TOE12 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio12 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio12> for bool {
    #[inline(always)]
    fn from(variant: Tio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO12` reader - This bit sets the value of the pin tamper I/O 12 (TIO12) when the output is enabled through TOE12 register."]
pub type Tio12R = crate::BitReader<Tio12>;
impl Tio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio12 {
        match self.bits {
            false => Tio12::Zero,
            true => Tio12::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio12::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio12::One
    }
}
#[doc = "Field `TIO12` writer - This bit sets the value of the pin tamper I/O 12 (TIO12) when the output is enabled through TOE12 register."]
pub type Tio12W<'a, REG> = crate::BitWriter<'a, REG, Tio12>;
impl<'a, REG> Tio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 13 (TIO13) when the output is enabled through TOE13 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio13 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio13> for bool {
    #[inline(always)]
    fn from(variant: Tio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO13` reader - This bit sets the value of the pin tamper I/O 13 (TIO13) when the output is enabled through TOE13 register."]
pub type Tio13R = crate::BitReader<Tio13>;
impl Tio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio13 {
        match self.bits {
            false => Tio13::Zero,
            true => Tio13::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio13::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio13::One
    }
}
#[doc = "Field `TIO13` writer - This bit sets the value of the pin tamper I/O 13 (TIO13) when the output is enabled through TOE13 register."]
pub type Tio13W<'a, REG> = crate::BitWriter<'a, REG, Tio13>;
impl<'a, REG> Tio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 14 (TIO14) when the output is enabled through TOE14 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio14 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio14> for bool {
    #[inline(always)]
    fn from(variant: Tio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO14` reader - This bit sets the value of the pin tamper I/O 14 (TIO14) when the output is enabled through TOE14 register."]
pub type Tio14R = crate::BitReader<Tio14>;
impl Tio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio14 {
        match self.bits {
            false => Tio14::Zero,
            true => Tio14::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio14::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio14::One
    }
}
#[doc = "Field `TIO14` writer - This bit sets the value of the pin tamper I/O 14 (TIO14) when the output is enabled through TOE14 register."]
pub type Tio14W<'a, REG> = crate::BitWriter<'a, REG, Tio14>;
impl<'a, REG> Tio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 15 (TIO15) when the output is enabled through TOE15 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio15 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio15> for bool {
    #[inline(always)]
    fn from(variant: Tio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO15` reader - This bit sets the value of the pin tamper I/O 15 (TIO15) when the output is enabled through TOE15 register."]
pub type Tio15R = crate::BitReader<Tio15>;
impl Tio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio15 {
        match self.bits {
            false => Tio15::Zero,
            true => Tio15::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio15::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio15::One
    }
}
#[doc = "Field `TIO15` writer - This bit sets the value of the pin tamper I/O 15 (TIO15) when the output is enabled through TOE15 register."]
pub type Tio15W<'a, REG> = crate::BitWriter<'a, REG, Tio15>;
impl<'a, REG> Tio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::One)
    }
}
impl R {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 12 (TIO12) when the output is enabled through TOE12 register."]
    #[inline(always)]
    pub fn tio12(&self) -> Tio12R {
        Tio12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 13 (TIO13) when the output is enabled through TOE13 register."]
    #[inline(always)]
    pub fn tio13(&self) -> Tio13R {
        Tio13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 14 (TIO14) when the output is enabled through TOE14 register."]
    #[inline(always)]
    pub fn tio14(&self) -> Tio14R {
        Tio14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 15 (TIO15) when the output is enabled through TOE15 register."]
    #[inline(always)]
    pub fn tio15(&self) -> Tio15R {
        Tio15R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 12 (TIO12) when the output is enabled through TOE12 register."]
    #[inline(always)]
    pub fn tio12(&mut self) -> Tio12W<'_, LfssTout15_12Spec> {
        Tio12W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 13 (TIO13) when the output is enabled through TOE13 register."]
    #[inline(always)]
    pub fn tio13(&mut self) -> Tio13W<'_, LfssTout15_12Spec> {
        Tio13W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 14 (TIO14) when the output is enabled through TOE14 register."]
    #[inline(always)]
    pub fn tio14(&mut self) -> Tio14W<'_, LfssTout15_12Spec> {
        Tio14W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 15 (TIO15) when the output is enabled through TOE15 register."]
    #[inline(always)]
    pub fn tio15(&mut self) -> Tio15W<'_, LfssTout15_12Spec> {
        Tio15W::new(self, 24)
    }
}
#[doc = "Tamper Output 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout15_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout15_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTout15_12Spec;
impl crate::RegisterSpec for LfssTout15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tout15_12::R`](R) reader structure"]
impl crate::Readable for LfssTout15_12Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tout15_12::W`](W) writer structure"]
impl crate::Writable for LfssTout15_12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOUT15_12 to value 0"]
impl crate::Resettable for LfssTout15_12Spec {}
