#[doc = "Register `GPIOC_DIN15_12` reader"]
pub type R = crate::R<GpiocDin15_12Spec>;
#[doc = "This bit reads the data input value of DIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` reader - This bit reads the data input value of DIO12."]
pub type Dio12R = crate::BitReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            false => Dio12::Zero,
            true => Dio12::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio12::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio12::One
    }
}
#[doc = "This bit reads the data input value of DIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` reader - This bit reads the data input value of DIO13."]
pub type Dio13R = crate::BitReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            false => Dio13::Zero,
            true => Dio13::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio13::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio13::One
    }
}
#[doc = "This bit reads the data input value of DIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` reader - This bit reads the data input value of DIO14."]
pub type Dio14R = crate::BitReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            false => Dio14::Zero,
            true => Dio14::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio14::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio14::One
    }
}
#[doc = "This bit reads the data input value of DIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` reader - This bit reads the data input value of DIO15."]
pub type Dio15R = crate::BitReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            false => Dio15::Zero,
            true => Dio15::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio15::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio15::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO12."]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO13."]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO14."]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO15."]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_din15_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocDin15_12Spec;
impl crate::RegisterSpec for GpiocDin15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_din15_12::R`](R) reader structure"]
impl crate::Readable for GpiocDin15_12Spec {}
#[doc = "`reset()` method sets GPIOC_DIN15_12 to value 0"]
impl crate::Resettable for GpiocDin15_12Spec {}
