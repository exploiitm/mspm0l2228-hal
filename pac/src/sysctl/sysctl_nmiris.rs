#[doc = "Register `SYSCTL_NMIRIS` reader"]
pub type R = crate::R<SysctlNmirisSpec>;
#[doc = "Raw status of the BORLVL NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borlvl {
    #[doc = "0: No interrupt pending"]
    False = 0,
    #[doc = "1: Interrupt pending"]
    True = 1,
}
impl From<Borlvl> for bool {
    #[inline(always)]
    fn from(variant: Borlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORLVL` reader - Raw status of the BORLVL NMI"]
pub type BorlvlR = crate::BitReader<Borlvl>;
impl BorlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borlvl {
        match self.bits {
            false => Borlvl::False,
            true => Borlvl::True,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Borlvl::False
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Borlvl::True
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0 {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Wwdt0> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0` reader - Watch Dog 0 Fault"]
pub type Wwdt0R = crate::BitReader<Wwdt0>;
impl Wwdt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdt0 {
        match self.bits {
            false => Wwdt0::False,
            true => Wwdt0::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Wwdt0::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Wwdt0::True
    }
}
#[doc = "LFXT-EXLF Monitor Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkfail {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Lfclkfail> for bool {
    #[inline(always)]
    fn from(variant: Lfclkfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKFAIL` reader - LFXT-EXLF Monitor Fail"]
pub type LfclkfailR = crate::BitReader<Lfclkfail>;
impl LfclkfailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfclkfail {
        match self.bits {
            false => Lfclkfail::False,
            true => Lfclkfail::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Lfclkfail::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Lfclkfail::True
    }
}
#[doc = "Flash Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashded {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Flashded> for bool {
    #[inline(always)]
    fn from(variant: Flashded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHDED` reader - Flash Double Error Detect"]
pub type FlashdedR = crate::BitReader<Flashded>;
impl FlashdedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashded {
        match self.bits {
            false => Flashded::False,
            true => Flashded::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Flashded::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Flashded::True
    }
}
#[doc = "SRAM Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramded {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Sramded> for bool {
    #[inline(always)]
    fn from(variant: Sramded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMDED` reader - SRAM Double Error Detect"]
pub type SramdedR = crate::BitReader<Sramded>;
impl SramdedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramded {
        match self.bits {
            false => Sramded::False,
            true => Sramded::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Sramded::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Sramded::True
    }
}
#[doc = "VBAT Power Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatdn {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Vbatdn> for bool {
    #[inline(always)]
    fn from(variant: Vbatdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATDN` reader - VBAT Power Off"]
pub type VbatdnR = crate::BitReader<Vbatdn>;
impl VbatdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatdn {
        match self.bits {
            false => Vbatdn::False,
            true => Vbatdn::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Vbatdn::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Vbatdn::True
    }
}
#[doc = "VBAT Power On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatup {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Vbatup> for bool {
    #[inline(always)]
    fn from(variant: Vbatup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATUP` reader - VBAT Power On"]
pub type VbatupR = crate::BitReader<Vbatup>;
impl VbatupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatup {
        match self.bits {
            false => Vbatup::False,
            true => Vbatup::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Vbatup::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Vbatup::True
    }
}
impl R {
    #[doc = "Bit 0 - Raw status of the BORLVL NMI"]
    #[inline(always)]
    pub fn borlvl(&self) -> BorlvlR {
        BorlvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn wwdt0(&self) -> Wwdt0R {
        Wwdt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFXT-EXLF Monitor Fail"]
    #[inline(always)]
    pub fn lfclkfail(&self) -> LfclkfailR {
        LfclkfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn flashded(&self) -> FlashdedR {
        FlashdedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM Double Error Detect"]
    #[inline(always)]
    pub fn sramded(&self) -> SramdedR {
        SramdedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VBAT Power Off"]
    #[inline(always)]
    pub fn vbatdn(&self) -> VbatdnR {
        VbatdnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VBAT Power On"]
    #[inline(always)]
    pub fn vbatup(&self) -> VbatupR {
        VbatupR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "NMI raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_nmiris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlNmirisSpec;
impl crate::RegisterSpec for SysctlNmirisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_nmiris::R`](R) reader structure"]
impl crate::Readable for SysctlNmirisSpec {}
#[doc = "`reset()` method sets SYSCTL_NMIRIS to value 0"]
impl crate::Resettable for SysctlNmirisSpec {}
