#[doc = "Register `GPIOA_DIN11_8` reader"]
pub type R = crate::R<GpioaDin11_8Spec>;
#[doc = "This bit reads the data input value of DIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` reader - This bit reads the data input value of DIO8."]
pub type Dio8R = crate::BitReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            false => Dio8::Zero,
            true => Dio8::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio8::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio8::One
    }
}
#[doc = "This bit reads the data input value of DIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` reader - This bit reads the data input value of DIO9."]
pub type Dio9R = crate::BitReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            false => Dio9::Zero,
            true => Dio9::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio9::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio9::One
    }
}
#[doc = "This bit reads the data input value of DIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` reader - This bit reads the data input value of DIO10."]
pub type Dio10R = crate::BitReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            false => Dio10::Zero,
            true => Dio10::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio10::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio10::One
    }
}
#[doc = "This bit reads the data input value of DIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` reader - This bit reads the data input value of DIO11."]
pub type Dio11R = crate::BitReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            false => Dio11::Zero,
            true => Dio11::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio11::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio11::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO8."]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO9."]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO10."]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO11."]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_din11_8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaDin11_8Spec;
impl crate::RegisterSpec for GpioaDin11_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa_din11_8::R`](R) reader structure"]
impl crate::Readable for GpioaDin11_8Spec {}
#[doc = "`reset()` method sets GPIOA_DIN11_8 to value 0"]
impl crate::Resettable for GpioaDin11_8Spec {}
