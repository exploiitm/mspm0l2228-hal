#[doc = "Register `SYSCTL_SECSTATUS` reader"]
pub type R = crate::R<SysctlSecstatusSpec>;
#[doc = "1: CSC has been completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initdone {
    #[doc = "0: INIT is not yet done"]
    No = 0,
    #[doc = "1: INIT is done"]
    Yes = 1,
}
impl From<Initdone> for bool {
    #[inline(always)]
    fn from(variant: Initdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITDONE` reader - 1: CSC has been completed"]
pub type InitdoneR = crate::BitReader<Initdone>;
impl InitdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initdone {
        match self.bits {
            false => Initdone::No,
            true => Initdone::Yes,
        }
    }
    #[doc = "INIT is not yet done"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Initdone::No
    }
    #[doc = "INIT is done"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Initdone::Yes
    }
}
#[doc = "1: CSC Exists in the system\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscexists {
    #[doc = "0: System does not have a CSC"]
    No = 0,
    #[doc = "1: System does have a CSC"]
    Yes = 1,
}
impl From<Cscexists> for bool {
    #[inline(always)]
    fn from(variant: Cscexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCEXISTS` reader - 1: CSC Exists in the system"]
pub type CscexistsR = crate::BitReader<Cscexists>;
impl CscexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscexists {
        match self.bits {
            false => Cscexists::No,
            true => Cscexists::Yes,
        }
    }
    #[doc = "System does not have a CSC"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cscexists::No
    }
    #[doc = "System does have a CSC"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cscexists::Yes
    }
}
#[doc = "1: Flash Read Execute Protection Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flrxprot {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Flrxprot> for bool {
    #[inline(always)]
    fn from(variant: Flrxprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLRXPROT` reader - 1: Flash Read Execute Protection Active"]
pub type FlrxprotR = crate::BitReader<Flrxprot>;
impl FlrxprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flrxprot {
        match self.bits {
            false => Flrxprot::Disabled,
            true => Flrxprot::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flrxprot::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flrxprot::Enabled
    }
}
#[doc = "1: Flash IP Protection Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flipprot {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Flipprot> for bool {
    #[inline(always)]
    fn from(variant: Flipprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIPPROT` reader - 1: Flash IP Protection Active"]
pub type FlipprotR = crate::BitReader<Flipprot>;
impl FlipprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flipprot {
        match self.bits {
            false => Flipprot::Disabled,
            true => Flipprot::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flipprot::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flipprot::Enabled
    }
}
#[doc = "1: SRAM Boundary MMR Locked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramboundarylock {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Sramboundarylock> for bool {
    #[inline(always)]
    fn from(variant: Sramboundarylock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMBOUNDARYLOCK` reader - 1: SRAM Boundary MMR Locked"]
pub type SramboundarylockR = crate::BitReader<Sramboundarylock>;
impl SramboundarylockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramboundarylock {
        match self.bits {
            false => Sramboundarylock::Disabled,
            true => Sramboundarylock::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sramboundarylock::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sramboundarylock::Enabled
    }
}
#[doc = "1: Upper and Lower Banks allowed to be swapped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flbankswppolicy {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Flbankswppolicy> for bool {
    #[inline(always)]
    fn from(variant: Flbankswppolicy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLBANKSWPPOLICY` reader - 1: Upper and Lower Banks allowed to be swapped"]
pub type FlbankswppolicyR = crate::BitReader<Flbankswppolicy>;
impl FlbankswppolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flbankswppolicy {
        match self.bits {
            false => Flbankswppolicy::Disabled,
            true => Flbankswppolicy::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flbankswppolicy::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flbankswppolicy::Enabled
    }
}
#[doc = "Field `FLBANKSWP` reader - 1: Upper and Lower Banks have been swapped"]
pub type FlbankswpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: CSC has been completed"]
    #[inline(always)]
    pub fn initdone(&self) -> InitdoneR {
        InitdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: CSC Exists in the system"]
    #[inline(always)]
    pub fn cscexists(&self) -> CscexistsR {
        CscexistsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Flash Read Execute Protection Active"]
    #[inline(always)]
    pub fn flrxprot(&self) -> FlrxprotR {
        FlrxprotR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Flash IP Protection Active"]
    #[inline(always)]
    pub fn flipprot(&self) -> FlipprotR {
        FlipprotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: SRAM Boundary MMR Locked"]
    #[inline(always)]
    pub fn sramboundarylock(&self) -> SramboundarylockR {
        SramboundarylockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Upper and Lower Banks allowed to be swapped"]
    #[inline(always)]
    pub fn flbankswppolicy(&self) -> FlbankswppolicyR {
        FlbankswppolicyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Upper and Lower Banks have been swapped"]
    #[inline(always)]
    pub fn flbankswp(&self) -> FlbankswpR {
        FlbankswpR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Security Configuration status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_secstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSecstatusSpec;
impl crate::RegisterSpec for SysctlSecstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_secstatus::R`](R) reader structure"]
impl crate::Readable for SysctlSecstatusSpec {}
#[doc = "`reset()` method sets SYSCTL_SECSTATUS to value 0"]
impl crate::Resettable for SysctlSecstatusSpec {}
