#[doc = "Register `GPIOB_DOUT27_24` writer"]
pub type W = crate::W<GpiobDout27_24Spec>;
#[doc = "This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` writer - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` writer - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio26 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO26` writer - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG, Dio26>;
impl<'a, REG> Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio27 {
    #[doc = "0: Output is set to 0"]
    Zero = 0,
    #[doc = "1: Output is set to 1"]
    One = 1,
}
impl From<Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO27` writer - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG, Dio27>;
impl<'a, REG> Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Zero)
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, GpiobDout27_24Spec> {
        Dio24W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, GpiobDout27_24Spec> {
        Dio25W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, GpiobDout27_24Spec> {
        Dio26W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, GpiobDout27_24Spec> {
        Dio27W::new(self, 24)
    }
}
#[doc = "Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_dout27_24::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiobDout27_24Spec;
impl crate::RegisterSpec for GpiobDout27_24Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpiob_dout27_24::W`](W) writer structure"]
impl crate::Writable for GpiobDout27_24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB_DOUT27_24 to value 0"]
impl crate::Resettable for GpiobDout27_24Spec {}
