#[doc = "Register `LFSS_TIN11_8` reader"]
pub type R = crate::R<LfssTin11_8Spec>;
#[doc = "This bit reads the data input value of tamper I/O 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio8 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio8> for bool {
    #[inline(always)]
    fn from(variant: Tio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO8` reader - This bit reads the data input value of tamper I/O 8."]
pub type Tio8R = crate::BitReader<Tio8>;
impl Tio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio8 {
        match self.bits {
            false => Tio8::Zero,
            true => Tio8::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio8::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio8::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio9 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio9> for bool {
    #[inline(always)]
    fn from(variant: Tio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO9` reader - This bit reads the data input value of tamper I/O 9."]
pub type Tio9R = crate::BitReader<Tio9>;
impl Tio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio9 {
        match self.bits {
            false => Tio9::Zero,
            true => Tio9::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio9::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio9::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio10 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio10> for bool {
    #[inline(always)]
    fn from(variant: Tio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO10` reader - This bit reads the data input value of tamper I/O 10."]
pub type Tio10R = crate::BitReader<Tio10>;
impl Tio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio10 {
        match self.bits {
            false => Tio10::Zero,
            true => Tio10::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio10::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio10::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio11 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio11> for bool {
    #[inline(always)]
    fn from(variant: Tio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO11` reader - This bit reads the data input value of tamper I/O 11."]
pub type Tio11R = crate::BitReader<Tio11>;
impl Tio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio11 {
        match self.bits {
            false => Tio11::Zero,
            true => Tio11::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio11::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio11::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of tamper I/O 8."]
    #[inline(always)]
    pub fn tio8(&self) -> Tio8R {
        Tio8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of tamper I/O 9."]
    #[inline(always)]
    pub fn tio9(&self) -> Tio9R {
        Tio9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of tamper I/O 10."]
    #[inline(always)]
    pub fn tio10(&self) -> Tio10R {
        Tio10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of tamper I/O 11."]
    #[inline(always)]
    pub fn tio11(&self) -> Tio11R {
        Tio11R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin11_8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTin11_8Spec;
impl crate::RegisterSpec for LfssTin11_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tin11_8::R`](R) reader structure"]
impl crate::Readable for LfssTin11_8Spec {}
#[doc = "`reset()` method sets LFSS_TIN11_8 to value 0"]
impl crate::Resettable for LfssTin11_8Spec {}
