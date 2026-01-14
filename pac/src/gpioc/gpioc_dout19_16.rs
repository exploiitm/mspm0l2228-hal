#[doc = "Register `GPIOC_DOUT19_16` writer"]
pub type W = crate::W<GpiocDout19_16Spec>;
#[doc = "This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` writer - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` writer - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` writer - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` writer - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, GpiocDout19_16Spec> {
        Dio16W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, GpiocDout19_16Spec> {
        Dio17W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, GpiocDout19_16Spec> {
        Dio18W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, GpiocDout19_16Spec> {
        Dio19W::new(self, 24)
    }
}
#[doc = "Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout19_16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocDout19_16Spec;
impl crate::RegisterSpec for GpiocDout19_16Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_dout19_16::W`](W) writer structure"]
impl crate::Writable for GpiocDout19_16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_DOUT19_16 to value 0"]
impl crate::Resettable for GpiocDout19_16Spec {}
