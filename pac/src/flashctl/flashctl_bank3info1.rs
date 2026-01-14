#[doc = "Register `FLASHCTL_BANK3INFO1` reader"]
pub type R = crate::R<FlashctlBank3info1Spec>;
#[doc = "Non-main region size in sectors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nonmainsize {
    #[doc = "0: Minimum value of \\[NONMAINSIZE\\]"]
    Minsectors = 0,
    #[doc = "32: Maximum value of \\[NONMAINSIZE\\]"]
    Maxsectors = 32,
}
impl From<Nonmainsize> for u8 {
    #[inline(always)]
    fn from(variant: Nonmainsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nonmainsize {
    type Ux = u8;
}
impl crate::IsEnum for Nonmainsize {}
#[doc = "Field `NONMAINSIZE` reader - Non-main region size in sectors"]
pub type NonmainsizeR = crate::FieldReader<Nonmainsize>;
impl NonmainsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nonmainsize> {
        match self.bits {
            0 => Some(Nonmainsize::Minsectors),
            32 => Some(Nonmainsize::Maxsectors),
            _ => None,
        }
    }
    #[doc = "Minimum value of \\[NONMAINSIZE\\]"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Nonmainsize::Minsectors
    }
    #[doc = "Maximum value of \\[NONMAINSIZE\\]"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Nonmainsize::Maxsectors
    }
}
#[doc = "Trim region size in sectors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trimsize {
    #[doc = "0: Minimum value of \\[TRIMSIZE\\]"]
    Minsectors = 0,
    #[doc = "32: Maximum value of \\[TRIMSIZE\\]"]
    Maxsectors = 32,
}
impl From<Trimsize> for u8 {
    #[inline(always)]
    fn from(variant: Trimsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trimsize {
    type Ux = u8;
}
impl crate::IsEnum for Trimsize {}
#[doc = "Field `TRIMSIZE` reader - Trim region size in sectors"]
pub type TrimsizeR = crate::FieldReader<Trimsize>;
impl TrimsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trimsize> {
        match self.bits {
            0 => Some(Trimsize::Minsectors),
            32 => Some(Trimsize::Maxsectors),
            _ => None,
        }
    }
    #[doc = "Minimum value of \\[TRIMSIZE\\]"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Trimsize::Minsectors
    }
    #[doc = "Maximum value of \\[TRIMSIZE\\]"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Trimsize::Maxsectors
    }
}
#[doc = "Engr region size in sectors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Engrsize {
    #[doc = "0: Minimum value of \\[ENGRSIZE\\]"]
    Minsectors = 0,
    #[doc = "32: Maximum value of \\[ENGRSIZE\\]"]
    Maxsectors = 32,
}
impl From<Engrsize> for u8 {
    #[inline(always)]
    fn from(variant: Engrsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Engrsize {
    type Ux = u8;
}
impl crate::IsEnum for Engrsize {}
#[doc = "Field `ENGRSIZE` reader - Engr region size in sectors"]
pub type EngrsizeR = crate::FieldReader<Engrsize>;
impl EngrsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Engrsize> {
        match self.bits {
            0 => Some(Engrsize::Minsectors),
            32 => Some(Engrsize::Maxsectors),
            _ => None,
        }
    }
    #[doc = "Minimum value of \\[ENGRSIZE\\]"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Engrsize::Minsectors
    }
    #[doc = "Maximum value of \\[ENGRSIZE\\]"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Engrsize::Maxsectors
    }
}
impl R {
    #[doc = "Bits 0:7 - Non-main region size in sectors"]
    #[inline(always)]
    pub fn nonmainsize(&self) -> NonmainsizeR {
        NonmainsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Trim region size in sectors"]
    #[inline(always)]
    pub fn trimsize(&self) -> TrimsizeR {
        TrimsizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Engr region size in sectors"]
    #[inline(always)]
    pub fn engrsize(&self) -> EngrsizeR {
        EngrsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Bank Information Register 1 for Bank 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank3info1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlBank3info1Spec;
impl crate::RegisterSpec for FlashctlBank3info1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_bank3info1::R`](R) reader structure"]
impl crate::Readable for FlashctlBank3info1Spec {}
#[doc = "`reset()` method sets FLASHCTL_BANK3INFO1 to value 0"]
impl crate::Resettable for FlashctlBank3info1Spec {}
