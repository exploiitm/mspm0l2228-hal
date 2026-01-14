#[doc = "Register `LFSS_TOUT3_0` reader"]
pub type R = crate::R<LfssTout3_0Spec>;
#[doc = "Register `LFSS_TOUT3_0` writer"]
pub type W = crate::W<LfssTout3_0Spec>;
#[doc = "This bit sets the value of the pin tamper I/O 0 (TIO0) when the output is enabled through TOE0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio0 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio0> for bool {
    #[inline(always)]
    fn from(variant: Tio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO0` reader - This bit sets the value of the pin tamper I/O 0 (TIO0) when the output is enabled through TOE0 register."]
pub type Tio0R = crate::BitReader<Tio0>;
impl Tio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio0 {
        match self.bits {
            false => Tio0::Zero,
            true => Tio0::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio0::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio0::One
    }
}
#[doc = "Field `TIO0` writer - This bit sets the value of the pin tamper I/O 0 (TIO0) when the output is enabled through TOE0 register."]
pub type Tio0W<'a, REG> = crate::BitWriter<'a, REG, Tio0>;
impl<'a, REG> Tio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 1 (TIO1) when the output is enabled through TOE1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio1 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio1> for bool {
    #[inline(always)]
    fn from(variant: Tio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO1` reader - This bit sets the value of the pin tamper I/O 1 (TIO1) when the output is enabled through TOE1 register."]
pub type Tio1R = crate::BitReader<Tio1>;
impl Tio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio1 {
        match self.bits {
            false => Tio1::Zero,
            true => Tio1::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio1::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio1::One
    }
}
#[doc = "Field `TIO1` writer - This bit sets the value of the pin tamper I/O 1 (TIO1) when the output is enabled through TOE1 register."]
pub type Tio1W<'a, REG> = crate::BitWriter<'a, REG, Tio1>;
impl<'a, REG> Tio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 2 (TIO0) when the output is enabled through TOE2 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio2 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio2> for bool {
    #[inline(always)]
    fn from(variant: Tio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO2` reader - This bit sets the value of the pin tamper I/O 2 (TIO0) when the output is enabled through TOE2 register."]
pub type Tio2R = crate::BitReader<Tio2>;
impl Tio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio2 {
        match self.bits {
            false => Tio2::Zero,
            true => Tio2::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio2::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio2::One
    }
}
#[doc = "Field `TIO2` writer - This bit sets the value of the pin tamper I/O 2 (TIO0) when the output is enabled through TOE2 register."]
pub type Tio2W<'a, REG> = crate::BitWriter<'a, REG, Tio2>;
impl<'a, REG> Tio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::One)
    }
}
#[doc = "This bit sets the value of the pin tamper I/O 3 (TIO3) when the output is enabled through TOE3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio3 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Tio3> for bool {
    #[inline(always)]
    fn from(variant: Tio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO3` reader - This bit sets the value of the pin tamper I/O 3 (TIO3) when the output is enabled through TOE3 register."]
pub type Tio3R = crate::BitReader<Tio3>;
impl Tio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio3 {
        match self.bits {
            false => Tio3::Zero,
            true => Tio3::One,
        }
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio3::Zero
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio3::One
    }
}
#[doc = "Field `TIO3` writer - This bit sets the value of the pin tamper I/O 3 (TIO3) when the output is enabled through TOE3 register."]
pub type Tio3W<'a, REG> = crate::BitWriter<'a, REG, Tio3>;
impl<'a, REG> Tio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::One)
    }
}
impl R {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 0 (TIO0) when the output is enabled through TOE0 register."]
    #[inline(always)]
    pub fn tio0(&self) -> Tio0R {
        Tio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 1 (TIO1) when the output is enabled through TOE1 register."]
    #[inline(always)]
    pub fn tio1(&self) -> Tio1R {
        Tio1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 2 (TIO0) when the output is enabled through TOE2 register."]
    #[inline(always)]
    pub fn tio2(&self) -> Tio2R {
        Tio2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 3 (TIO3) when the output is enabled through TOE3 register."]
    #[inline(always)]
    pub fn tio3(&self) -> Tio3R {
        Tio3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin tamper I/O 0 (TIO0) when the output is enabled through TOE0 register."]
    #[inline(always)]
    pub fn tio0(&mut self) -> Tio0W<'_, LfssTout3_0Spec> {
        Tio0W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin tamper I/O 1 (TIO1) when the output is enabled through TOE1 register."]
    #[inline(always)]
    pub fn tio1(&mut self) -> Tio1W<'_, LfssTout3_0Spec> {
        Tio1W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin tamper I/O 2 (TIO0) when the output is enabled through TOE2 register."]
    #[inline(always)]
    pub fn tio2(&mut self) -> Tio2W<'_, LfssTout3_0Spec> {
        Tio2W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin tamper I/O 3 (TIO3) when the output is enabled through TOE3 register."]
    #[inline(always)]
    pub fn tio3(&mut self) -> Tio3W<'_, LfssTout3_0Spec> {
        Tio3W::new(self, 24)
    }
}
#[doc = "Tamper Output 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTout3_0Spec;
impl crate::RegisterSpec for LfssTout3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tout3_0::R`](R) reader structure"]
impl crate::Readable for LfssTout3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tout3_0::W`](W) writer structure"]
impl crate::Writable for LfssTout3_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TOUT3_0 to value 0"]
impl crate::Resettable for LfssTout3_0Spec {}
