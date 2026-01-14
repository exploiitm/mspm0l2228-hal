#[doc = "Register `SYSCTL_MIS` reader"]
pub type R = crate::R<SysctlMisSpec>;
#[doc = "Masked status of the LFOSCGOOD interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfoscgood {
    #[doc = "0: No interrupt pending"]
    False = 0,
    #[doc = "1: Interrupt pending"]
    True = 1,
}
impl From<Lfoscgood> for bool {
    #[inline(always)]
    fn from(variant: Lfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFOSCGOOD` reader - Masked status of the LFOSCGOOD interrupt."]
pub type LfoscgoodR = crate::BitReader<Lfoscgood>;
impl LfoscgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfoscgood {
        match self.bits {
            false => Lfoscgood::False,
            true => Lfoscgood::True,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Lfoscgood::False
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Lfoscgood::True
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anaclkerr {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Anaclkerr> for bool {
    #[inline(always)]
    fn from(variant: Anaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACLKERR` reader - Analog Clocking Consistency Error"]
pub type AnaclkerrR = crate::BitReader<Anaclkerr>;
impl AnaclkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anaclkerr {
        match self.bits {
            false => Anaclkerr::False,
            true => Anaclkerr::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Anaclkerr::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Anaclkerr::True
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashsec {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Flashsec> for bool {
    #[inline(always)]
    fn from(variant: Flashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHSEC` reader - Flash Single Error Correct"]
pub type FlashsecR = crate::BitReader<Flashsec>;
impl FlashsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashsec {
        match self.bits {
            false => Flashsec::False,
            true => Flashsec::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Flashsec::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Flashsec::True
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsec {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Sramsec> for bool {
    #[inline(always)]
    fn from(variant: Sramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMSEC` reader - SRAM Single Error Correct"]
pub type SramsecR = crate::BitReader<Sramsec>;
impl SramsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsec {
        match self.bits {
            false => Sramsec::False,
            true => Sramsec::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Sramsec::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Sramsec::True
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtgood {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Lfxtgood> for bool {
    #[inline(always)]
    fn from(variant: Lfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTGOOD` reader - LFXT GOOD"]
pub type LfxtgoodR = crate::BitReader<Lfxtgood>;
impl LfxtgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtgood {
        match self.bits {
            false => Lfxtgood::False,
            true => Lfxtgood::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Lfxtgood::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Lfxtgood::True
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkgood {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Hfclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKGOOD` reader - HFCLK GOOD"]
pub type HfclkgoodR = crate::BitReader<Hfclkgood>;
impl HfclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkgood {
        match self.bits {
            false => Hfclkgood::False,
            true => Hfclkgood::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hfclkgood::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hfclkgood::True
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkgood {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Hsclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKGOOD` reader - HSCLK GOOD"]
pub type HsclkgoodR = crate::BitReader<Hsclkgood>;
impl HsclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclkgood {
        match self.bits {
            false => Hsclkgood::False,
            true => Hsclkgood::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hsclkgood::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hsclkgood::True
    }
}
impl R {
    #[doc = "Bit 0 - Masked status of the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn lfoscgood(&self) -> LfoscgoodR {
        LfoscgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn anaclkerr(&self) -> AnaclkerrR {
        AnaclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn flashsec(&self) -> FlashsecR {
        FlashsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn sramsec(&self) -> SramsecR {
        SramsecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn lfxtgood(&self) -> LfxtgoodR {
        LfxtgoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn hfclkgood(&self) -> HfclkgoodR {
        HfclkgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSCLK GOOD"]
    #[inline(always)]
    pub fn hsclkgood(&self) -> HsclkgoodR {
        HsclkgoodR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "SYSCTL masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlMisSpec;
impl crate::RegisterSpec for SysctlMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_mis::R`](R) reader structure"]
impl crate::Readable for SysctlMisSpec {}
#[doc = "`reset()` method sets SYSCTL_MIS to value 0"]
impl crate::Resettable for SysctlMisSpec {}
