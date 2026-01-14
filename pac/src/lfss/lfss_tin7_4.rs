#[doc = "Register `LFSS_TIN7_4` reader"]
pub type R = crate::R<LfssTin7_4Spec>;
#[doc = "This bit reads the data input value of tamper I/O 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio4 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio4> for bool {
    #[inline(always)]
    fn from(variant: Tio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO4` reader - This bit reads the data input value of tamper I/O 4."]
pub type Tio4R = crate::BitReader<Tio4>;
impl Tio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio4 {
        match self.bits {
            false => Tio4::Zero,
            true => Tio4::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio4::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio4::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio5 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio5> for bool {
    #[inline(always)]
    fn from(variant: Tio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO5` reader - This bit reads the data input value of tamper I/O 5."]
pub type Tio5R = crate::BitReader<Tio5>;
impl Tio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio5 {
        match self.bits {
            false => Tio5::Zero,
            true => Tio5::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio5::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio5::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio6 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio6> for bool {
    #[inline(always)]
    fn from(variant: Tio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO6` reader - This bit reads the data input value of tamper I/O 6."]
pub type Tio6R = crate::BitReader<Tio6>;
impl Tio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio6 {
        match self.bits {
            false => Tio6::Zero,
            true => Tio6::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio6::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio6::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio7 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio7> for bool {
    #[inline(always)]
    fn from(variant: Tio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO7` reader - This bit reads the data input value of tamper I/O 7."]
pub type Tio7R = crate::BitReader<Tio7>;
impl Tio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio7 {
        match self.bits {
            false => Tio7::Zero,
            true => Tio7::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio7::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio7::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of tamper I/O 4."]
    #[inline(always)]
    pub fn tio4(&self) -> Tio4R {
        Tio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of tamper I/O 5."]
    #[inline(always)]
    pub fn tio5(&self) -> Tio5R {
        Tio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of tamper I/O 6."]
    #[inline(always)]
    pub fn tio6(&self) -> Tio6R {
        Tio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of tamper I/O 7."]
    #[inline(always)]
    pub fn tio7(&self) -> Tio7R {
        Tio7R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin7_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTin7_4Spec;
impl crate::RegisterSpec for LfssTin7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tin7_4::R`](R) reader structure"]
impl crate::Readable for LfssTin7_4Spec {}
#[doc = "`reset()` method sets LFSS_TIN7_4 to value 0"]
impl crate::Resettable for LfssTin7_4Spec {}
