#[doc = "Register `LFSS_TIN3_0` reader"]
pub type R = crate::R<LfssTin3_0Spec>;
#[doc = "This bit reads the data input value of tamper I/O 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio0 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio0> for bool {
    #[inline(always)]
    fn from(variant: Tio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO0` reader - This bit reads the data input value of tamper I/O 0."]
pub type Tio0R = crate::BitReader<Tio0>;
impl Tio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio0 {
        match self.bits {
            false => Tio0::Zero,
            true => Tio0::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio0::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio0::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio1 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio1> for bool {
    #[inline(always)]
    fn from(variant: Tio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO1` reader - This bit reads the data input value of tamper I/O 1."]
pub type Tio1R = crate::BitReader<Tio1>;
impl Tio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio1 {
        match self.bits {
            false => Tio1::Zero,
            true => Tio1::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio1::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio1::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio2 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio2> for bool {
    #[inline(always)]
    fn from(variant: Tio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO2` reader - This bit reads the data input value of tamper I/O 2."]
pub type Tio2R = crate::BitReader<Tio2>;
impl Tio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio2 {
        match self.bits {
            false => Tio2::Zero,
            true => Tio2::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio2::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio2::One
    }
}
#[doc = "This bit reads the data input value of tamper I/O 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio3 {
    #[doc = "0: Input value is 0"]
    Zero = 0,
    #[doc = "1: Input value is 1"]
    One = 1,
}
impl From<Tio3> for bool {
    #[inline(always)]
    fn from(variant: Tio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO3` reader - This bit reads the data input value of tamper I/O 3."]
pub type Tio3R = crate::BitReader<Tio3>;
impl Tio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tio3 {
        match self.bits {
            false => Tio3::Zero,
            true => Tio3::One,
        }
    }
    #[doc = "Input value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tio3::Zero
    }
    #[doc = "Input value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Tio3::One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of tamper I/O 0."]
    #[inline(always)]
    pub fn tio0(&self) -> Tio0R {
        Tio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of tamper I/O 1."]
    #[inline(always)]
    pub fn tio1(&self) -> Tio1R {
        Tio1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of tamper I/O 2."]
    #[inline(always)]
    pub fn tio2(&self) -> Tio2R {
        Tio2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of tamper I/O 3."]
    #[inline(always)]
    pub fn tio3(&self) -> Tio3R {
        Tio3R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin3_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTin3_0Spec;
impl crate::RegisterSpec for LfssTin3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tin3_0::R`](R) reader structure"]
impl crate::Readable for LfssTin3_0Spec {}
#[doc = "`reset()` method sets LFSS_TIN3_0 to value 0"]
impl crate::Resettable for LfssTin3_0Spec {}
