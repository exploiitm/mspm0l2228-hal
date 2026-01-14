#[doc = "Register `GPIOA_DOUT23_20` writer"]
pub type W = crate::W<GpioaDout23_20Spec>;
#[doc = "This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` writer - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` writer - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` writer - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` writer - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, GpioaDout23_20Spec> {
        Dio20W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, GpioaDout23_20Spec> {
        Dio21W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, GpioaDout23_20Spec> {
        Dio22W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, GpioaDout23_20Spec> {
        Dio23W::new(self, 24)
    }
}
#[doc = "Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout23_20::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaDout23_20Spec;
impl crate::RegisterSpec for GpioaDout23_20Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioa_dout23_20::W`](W) writer structure"]
impl crate::Writable for GpioaDout23_20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA_DOUT23_20 to value 0"]
impl crate::Resettable for GpioaDout23_20Spec {}
