#[doc = "Register `GPIOC_DIN31_28` reader"]
pub type R = crate::R<GpiocDin31_28Spec>;
#[doc = "This bit reads the data input value of DIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio28 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO28` reader - This bit reads the data input value of DIO28."]
pub type Dio28R = crate::BitReader<Dio28>;
impl Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio28 {
        match self.bits {
            false => Dio28::Zero,
            true => Dio28::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio28::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio28::One
    }
}
#[doc = "This bit reads the data input value of DIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio29 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO29` reader - This bit reads the data input value of DIO29."]
pub type Dio29R = crate::BitReader<Dio29>;
impl Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio29 {
        match self.bits {
            false => Dio29::Zero,
            true => Dio29::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio29::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio29::One
    }
}
#[doc = "This bit reads the data input value of DIO30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio30 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO30` reader - This bit reads the data input value of DIO30."]
pub type Dio30R = crate::BitReader<Dio30>;
impl Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio30 {
        match self.bits {
            false => Dio30::Zero,
            true => Dio30::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio30::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio30::One
    }
}
#[doc = "This bit reads the data input value of DIO31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio31 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO31` reader - This bit reads the data input value of DIO31."]
pub type Dio31R = crate::BitReader<Dio31>;
impl Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio31 {
        match self.bits {
            false => Dio31::Zero,
            true => Dio31::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio31::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio31::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO28."]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO29."]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO30."]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO31."]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din31_28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocDin31_28Spec;
impl crate::RegisterSpec for GpiocDin31_28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_din31_28::R`](R) reader structure"]
impl crate::Readable for GpiocDin31_28Spec {}
#[doc = "`reset()` method sets GPIOC_DIN31_28 to value 0"]
impl crate::Resettable for GpiocDin31_28Spec {}
