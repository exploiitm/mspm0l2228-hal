#[doc = "Register `SYSCTL_SYSSTATUS` reader"]
pub type R = crate::R<SysctlSysstatusSpec>;
#[doc = "FLASHDED indicates if a flash ECC double bit error was detected (DED).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashded {
    #[doc = "0: No flash ECC double bit error detected"]
    False = 0,
    #[doc = "1: Flash ECC double bit error detected"]
    True = 1,
}
impl From<Flashded> for bool {
    #[inline(always)]
    fn from(variant: Flashded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHDED` reader - FLASHDED indicates if a flash ECC double bit error was detected (DED)."]
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
    #[doc = "No flash ECC double bit error detected"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Flashded::False
    }
    #[doc = "Flash ECC double bit error detected"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Flashded::True
    }
}
#[doc = "FLASHSEC indicates if a flash ECC single bit error was detected and corrected (SEC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashsec {
    #[doc = "0: No flash ECC single bit error detected"]
    False = 0,
    #[doc = "1: Flash ECC single bit error was detected and corrected"]
    True = 1,
}
impl From<Flashsec> for bool {
    #[inline(always)]
    fn from(variant: Flashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHSEC` reader - FLASHSEC indicates if a flash ECC single bit error was detected and corrected (SEC)."]
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
    #[doc = "No flash ECC single bit error detected"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Flashsec::False
    }
    #[doc = "Flash ECC single bit error was detected and corrected"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Flashsec::True
    }
}
#[doc = "BORCURTHRESHOLD indicates the active brown-out reset supply monitor configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Borcurthreshold {
    #[doc = "0: Default minimum threshold; a BOR0- violation triggers a BOR"]
    Bormin = 0,
    #[doc = "1: A BOR1- violation generates a BORLVL interrupt"]
    Borlevel1 = 1,
    #[doc = "2: A BOR2- violation generates a BORLVL interrupt"]
    Borlevel2 = 2,
    #[doc = "3: A BOR3- violation generates a BORLVL interrupt"]
    Borlevel3 = 3,
}
impl From<Borcurthreshold> for u8 {
    #[inline(always)]
    fn from(variant: Borcurthreshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Borcurthreshold {
    type Ux = u8;
}
impl crate::IsEnum for Borcurthreshold {}
#[doc = "Field `BORCURTHRESHOLD` reader - BORCURTHRESHOLD indicates the active brown-out reset supply monitor configuration."]
pub type BorcurthresholdR = crate::FieldReader<Borcurthreshold>;
impl BorcurthresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borcurthreshold {
        match self.bits {
            0 => Borcurthreshold::Bormin,
            1 => Borcurthreshold::Borlevel1,
            2 => Borcurthreshold::Borlevel2,
            3 => Borcurthreshold::Borlevel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Default minimum threshold; a BOR0- violation triggers a BOR"]
    #[inline(always)]
    pub fn is_bormin(&self) -> bool {
        *self == Borcurthreshold::Bormin
    }
    #[doc = "A BOR1- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn is_borlevel1(&self) -> bool {
        *self == Borcurthreshold::Borlevel1
    }
    #[doc = "A BOR2- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn is_borlevel2(&self) -> bool {
        *self == Borcurthreshold::Borlevel2
    }
    #[doc = "A BOR3- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn is_borlevel3(&self) -> bool {
        *self == Borcurthreshold::Borlevel3
    }
}
#[doc = "BORLVL indicates if a BOR event occured and the BOR threshold was switched to BOR0 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borlvl {
    #[doc = "0: No BOR violation occured"]
    False = 0,
    #[doc = "1: A BOR violation occured and the BOR threshold was switched to BOR0"]
    True = 1,
}
impl From<Borlvl> for bool {
    #[inline(always)]
    fn from(variant: Borlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORLVL` reader - BORLVL indicates if a BOR event occured and the BOR threshold was switched to BOR0 by hardware."]
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
    #[doc = "No BOR violation occured"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Borlvl::False
    }
    #[doc = "A BOR violation occured and the BOR threshold was switched to BOR0"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Borlvl::True
    }
}
#[doc = "ANACPUMPGOOD is set by hardware when the VBOOST analog mux charge pump is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anacpumpgood {
    #[doc = "0: VBOOST is not ready"]
    False = 0,
    #[doc = "1: VBOOST is ready"]
    True = 1,
}
impl From<Anacpumpgood> for bool {
    #[inline(always)]
    fn from(variant: Anacpumpgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACPUMPGOOD` reader - ANACPUMPGOOD is set by hardware when the VBOOST analog mux charge pump is ready."]
pub type AnacpumpgoodR = crate::BitReader<Anacpumpgood>;
impl AnacpumpgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anacpumpgood {
        match self.bits {
            false => Anacpumpgood::False,
            true => Anacpumpgood::True,
        }
    }
    #[doc = "VBOOST is not ready"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Anacpumpgood::False
    }
    #[doc = "VBOOST is ready"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Anacpumpgood::True
    }
}
#[doc = "PMUIREFGOOD is set by hardware when the PMU current reference is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmuirefgood {
    #[doc = "0: IREF is not ready"]
    False = 0,
    #[doc = "1: IREF is ready"]
    True = 1,
}
impl From<Pmuirefgood> for bool {
    #[inline(always)]
    fn from(variant: Pmuirefgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUIREFGOOD` reader - PMUIREFGOOD is set by hardware when the PMU current reference is ready."]
pub type PmuirefgoodR = crate::BitReader<Pmuirefgood>;
impl PmuirefgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmuirefgood {
        match self.bits {
            false => Pmuirefgood::False,
            true => Pmuirefgood::True,
        }
    }
    #[doc = "IREF is not ready"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Pmuirefgood::False
    }
    #[doc = "IREF is ready"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Pmuirefgood::True
    }
}
#[doc = "VBATGOOD is set by hardware when the VBAT Power Domain is valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatgood {
    #[doc = "0: VBAT Power Domain is not valid"]
    False = 0,
    #[doc = "1: VBAT Power Domain is valid"]
    True = 1,
}
impl From<Vbatgood> for bool {
    #[inline(always)]
    fn from(variant: Vbatgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATGOOD` reader - VBATGOOD is set by hardware when the VBAT Power Domain is valid."]
pub type VbatgoodR = crate::BitReader<Vbatgood>;
impl VbatgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatgood {
        match self.bits {
            false => Vbatgood::False,
            true => Vbatgood::True,
        }
    }
    #[doc = "VBAT Power Domain is not valid"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Vbatgood::False
    }
    #[doc = "VBAT Power Domain is valid"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Vbatgood::True
    }
}
#[doc = "EXTRSTPINDIS indicates when user has disabled the use of external reset pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extrstpindis {
    #[doc = "0: External Reset Pin Enabled"]
    False = 0,
    #[doc = "1: External Reset Pin Disabled"]
    True = 1,
}
impl From<Extrstpindis> for bool {
    #[inline(always)]
    fn from(variant: Extrstpindis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTRSTPINDIS` reader - EXTRSTPINDIS indicates when user has disabled the use of external reset pin"]
pub type ExtrstpindisR = crate::BitReader<Extrstpindis>;
impl ExtrstpindisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extrstpindis {
        match self.bits {
            false => Extrstpindis::False,
            true => Extrstpindis::True,
        }
    }
    #[doc = "External Reset Pin Enabled"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Extrstpindis::False
    }
    #[doc = "External Reset Pin Disabled"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Extrstpindis::True
    }
}
#[doc = "SWDCFGDIS indicates when user has disabled the use of SWD Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swdcfgdis {
    #[doc = "0: SWD Port Enabled"]
    False = 0,
    #[doc = "1: SWD Port Disabled"]
    True = 1,
}
impl From<Swdcfgdis> for bool {
    #[inline(always)]
    fn from(variant: Swdcfgdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDCFGDIS` reader - SWDCFGDIS indicates when user has disabled the use of SWD Port"]
pub type SwdcfgdisR = crate::BitReader<Swdcfgdis>;
impl SwdcfgdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swdcfgdis {
        match self.bits {
            false => Swdcfgdis::False,
            true => Swdcfgdis::True,
        }
    }
    #[doc = "SWD Port Enabled"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Swdcfgdis::False
    }
    #[doc = "SWD Port Disabled"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Swdcfgdis::True
    }
}
#[doc = "SHDNIOLOCK indicates when IO is locked due to SHUTDOWN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shdniolock {
    #[doc = "0: IO IS NOT Locked due to SHUTDOWN"]
    False = 0,
    #[doc = "1: IO IS Locked due to SHUTDOWN"]
    True = 1,
}
impl From<Shdniolock> for bool {
    #[inline(always)]
    fn from(variant: Shdniolock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHDNIOLOCK` reader - SHDNIOLOCK indicates when IO is locked due to SHUTDOWN"]
pub type ShdniolockR = crate::BitReader<Shdniolock>;
impl ShdniolockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shdniolock {
        match self.bits {
            false => Shdniolock::False,
            true => Shdniolock::True,
        }
    }
    #[doc = "IO IS NOT Locked due to SHUTDOWN"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Shdniolock::False
    }
    #[doc = "IO IS Locked due to SHUTDOWN"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Shdniolock::True
    }
}
#[doc = "Field `REBOOTATTEMPTS` reader - REBOOTATTEMPTS indicates the number of boot attempts taken before the user application starts."]
pub type RebootattemptsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - FLASHDED indicates if a flash ECC double bit error was detected (DED)."]
    #[inline(always)]
    pub fn flashded(&self) -> FlashdedR {
        FlashdedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASHSEC indicates if a flash ECC single bit error was detected and corrected (SEC)."]
    #[inline(always)]
    pub fn flashsec(&self) -> FlashsecR {
        FlashsecR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - BORCURTHRESHOLD indicates the active brown-out reset supply monitor configuration."]
    #[inline(always)]
    pub fn borcurthreshold(&self) -> BorcurthresholdR {
        BorcurthresholdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - BORLVL indicates if a BOR event occured and the BOR threshold was switched to BOR0 by hardware."]
    #[inline(always)]
    pub fn borlvl(&self) -> BorlvlR {
        BorlvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ANACPUMPGOOD is set by hardware when the VBOOST analog mux charge pump is ready."]
    #[inline(always)]
    pub fn anacpumpgood(&self) -> AnacpumpgoodR {
        AnacpumpgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PMUIREFGOOD is set by hardware when the PMU current reference is ready."]
    #[inline(always)]
    pub fn pmuirefgood(&self) -> PmuirefgoodR {
        PmuirefgoodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VBATGOOD is set by hardware when the VBAT Power Domain is valid."]
    #[inline(always)]
    pub fn vbatgood(&self) -> VbatgoodR {
        VbatgoodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTRSTPINDIS indicates when user has disabled the use of external reset pin"]
    #[inline(always)]
    pub fn extrstpindis(&self) -> ExtrstpindisR {
        ExtrstpindisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SWDCFGDIS indicates when user has disabled the use of SWD Port"]
    #[inline(always)]
    pub fn swdcfgdis(&self) -> SwdcfgdisR {
        SwdcfgdisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SHDNIOLOCK indicates when IO is locked due to SHUTDOWN"]
    #[inline(always)]
    pub fn shdniolock(&self) -> ShdniolockR {
        ShdniolockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 30:31 - REBOOTATTEMPTS indicates the number of boot attempts taken before the user application starts."]
    #[inline(always)]
    pub fn rebootattempts(&self) -> RebootattemptsR {
        RebootattemptsR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "System status information\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sysstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSysstatusSpec;
impl crate::RegisterSpec for SysctlSysstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_sysstatus::R`](R) reader structure"]
impl crate::Readable for SysctlSysstatusSpec {}
#[doc = "`reset()` method sets SYSCTL_SYSSTATUS to value 0"]
impl crate::Resettable for SysctlSysstatusSpec {}
