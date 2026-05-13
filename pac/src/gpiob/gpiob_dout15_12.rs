#[doc = "Register `GPIOB_DOUT15_12` writer"]
pub type W = crate::W<GpiobDout15_12Spec>;
#[doc = "This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` writer - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dio12>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` writer - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dio13>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` writer - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dio14>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` writer - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dio15>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, GpiobDout15_12Spec> {
        Dio12W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, GpiobDout15_12Spec> {
        Dio13W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, GpiobDout15_12Spec> {
        Dio14W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, GpiobDout15_12Spec> {
        Dio15W::new(self, 24)
    }
}
#[doc = "Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout15_12::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiobDout15_12Spec;
impl crate::RegisterSpec for GpiobDout15_12Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpiob_dout15_12::W`](W) writer structure"]
impl crate::Writable for GpiobDout15_12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB_DOUT15_12 to value 0"]
impl crate::Resettable for GpiobDout15_12Spec {}
