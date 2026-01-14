#[doc = "Register `GPIOC_DOUT11_8` writer"]
pub type W = crate::W<GpiocDout11_8Spec>;
#[doc = "This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` writer - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dio8>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` writer - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dio9>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` writer - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dio10>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` writer - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dio11>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, GpiocDout11_8Spec> {
        Dio8W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, GpiocDout11_8Spec> {
        Dio9W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, GpiocDout11_8Spec> {
        Dio10W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, GpiocDout11_8Spec> {
        Dio11W::new(self, 24)
    }
}
#[doc = "Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_dout11_8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocDout11_8Spec;
impl crate::RegisterSpec for GpiocDout11_8Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_dout11_8::W`](W) writer structure"]
impl crate::Writable for GpiocDout11_8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_DOUT11_8 to value 0"]
impl crate::Resettable for GpiocDout11_8Spec {}
