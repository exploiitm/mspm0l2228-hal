#[doc = "Register `LFSS_TIN15_12` reader"]
pub type R = crate::R<LfssTin15_12Spec>;
#[doc = "This bit reads the data input value of tamper I/O 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio12 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio12> for bool {
    #[inline(always)]
    fn from(variant: Tio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO12` reader - This bit reads the data input value of tamper I/O 12."]
pub type Tio12R = crate::BitReader<Tio12>;
impl Tio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio12 {
        match self.bits {
            false => Tio12::Zero,
            true => Tio12::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio12::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio12::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio13 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio13> for bool {
    #[inline(always)]
    fn from(variant: Tio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO13` reader - This bit reads the data input value of tamper I/O 13."]
pub type Tio13R = crate::BitReader<Tio13>;
impl Tio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio13 {
        match self.bits {
            false => Tio13::Zero,
            true => Tio13::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio13::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio13::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio14 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio14> for bool {
    #[inline(always)]
    fn from(variant: Tio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO14` reader - This bit reads the data input value of tamper I/O 14."]
pub type Tio14R = crate::BitReader<Tio14>;
impl Tio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio14 {
        match self.bits {
            false => Tio14::Zero,
            true => Tio14::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio14::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio14::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio15 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio15> for bool {
    #[inline(always)]
    fn from(variant: Tio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO15` reader - This bit reads the data input value of tamper I/O 15."]
pub type Tio15R = crate::BitReader<Tio15>;
impl Tio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio15 {
        match self.bits {
            false => Tio15::Zero,
            true => Tio15::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio15::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio15::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of tamper I/O 12."]
    #[inline(always)]
    pub fn tio12(&self) -> Tio12R {
        Tio12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of tamper I/O 13."]
    #[inline(always)]
    pub fn tio13(&self) -> Tio13R {
        Tio13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of tamper I/O 14."]
    #[inline(always)]
    pub fn tio14(&self) -> Tio14R {
        Tio14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of tamper I/O 15."]
    #[inline(always)]
    pub fn tio15(&self) -> Tio15R {
        Tio15R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin15_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTin15_12Spec;
impl crate::RegisterSpec for LfssTin15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tin15_12::R`](R) reader structure"]
impl crate::Readable for LfssTin15_12Spec {}
#[doc = "`reset()` method sets LFSS_TIN15_12 to value 0"]
impl crate::Resettable for LfssTin15_12Spec {}
