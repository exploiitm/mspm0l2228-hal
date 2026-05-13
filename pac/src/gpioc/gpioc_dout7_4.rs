#[doc = "Register `GPIOC_DOUT7_4` writer"]
pub type W = crate::W<GpiocDout7_4Spec>;
#[doc = "This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` writer - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` writer - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` writer - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` writer - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, GpiocDout7_4Spec> {
        Dio4W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, GpiocDout7_4Spec> {
        Dio5W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, GpiocDout7_4Spec> {
        Dio6W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, GpiocDout7_4Spec> {
        Dio7W::new(self, 24)
    }
}
#[doc = "Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout7_4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocDout7_4Spec;
impl crate::RegisterSpec for GpiocDout7_4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_dout7_4::W`](W) writer structure"]
impl crate::Writable for GpiocDout7_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_DOUT7_4 to value 0"]
impl crate::Resettable for GpiocDout7_4Spec {}
