#[doc = "Register `GPIOA_DOUT31_28` writer"]
pub type W = crate::W<GpioaDout31_28Spec>;
#[doc = "This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` writer - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG, Dio28>;
impl<'a, REG> Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` writer - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG, Dio29>;
impl<'a, REG> Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` writer - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG, Dio30>;
impl<'a, REG> Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` writer - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG, Dio31>;
impl<'a, REG> Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, GpioaDout31_28Spec> {
        Dio28W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, GpioaDout31_28Spec> {
        Dio29W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, GpioaDout31_28Spec> {
        Dio30W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, GpioaDout31_28Spec> {
        Dio31W::new(self, 24)
    }
}
#[doc = "Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout31_28::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaDout31_28Spec;
impl crate::RegisterSpec for GpioaDout31_28Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioa_dout31_28::W`](W) writer structure"]
impl crate::Writable for GpioaDout31_28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA_DOUT31_28 to value 0"]
impl crate::Resettable for GpioaDout31_28Spec {}
