#[doc = "Register `GPIOA_DOUT3_0` writer"]
pub type W = crate::W<GpioaDout3_0Spec>;
#[doc = "This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` writer - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` writer - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` writer - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` writer - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, GpioaDout3_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, GpioaDout3_0Spec> {
        Dio1W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, GpioaDout3_0Spec> {
        Dio2W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, GpioaDout3_0Spec> {
        Dio3W::new(self, 24)
    }
}
#[doc = "Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_dout3_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaDout3_0Spec;
impl crate::RegisterSpec for GpioaDout3_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioa_dout3_0::W`](W) writer structure"]
impl crate::Writable for GpioaDout3_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA_DOUT3_0 to value 0"]
impl crate::Resettable for GpioaDout3_0Spec {}
