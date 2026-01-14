#[doc = "Register `SYSCTL_CLKSTATUS` reader"]
pub type R = crate::R<SysctlClkstatusSpec>;
#[doc = "SYSOSCFREQ indicates the current SYSOSC operating frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sysoscfreq {
    #[doc = "0: SYSOSC is at base frequency (32MHz)"]
    Sysosc32m = 0,
    #[doc = "1: SYSOSC is at low frequency (4MHz)"]
    Sysosc4m = 1,
    #[doc = "2: SYSOSC is at the user-trimmed frequency (16 or 24MHz)"]
    Sysoscuser = 2,
    #[doc = "3: Reserved"]
    Sysoscturbo = 3,
}
impl From<Sysoscfreq> for u8 {
    #[inline(always)]
    fn from(variant: Sysoscfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysoscfreq {
    type Ux = u8;
}
impl crate::IsEnum for Sysoscfreq {}
#[doc = "Field `SYSOSCFREQ` reader - SYSOSCFREQ indicates the current SYSOSC operating frequency."]
pub type SysoscfreqR = crate::FieldReader<Sysoscfreq>;
impl SysoscfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysoscfreq {
        match self.bits {
            0 => Sysoscfreq::Sysosc32m,
            1 => Sysoscfreq::Sysosc4m,
            2 => Sysoscfreq::Sysoscuser,
            3 => Sysoscfreq::Sysoscturbo,
            _ => unreachable!(),
        }
    }
    #[doc = "SYSOSC is at base frequency (32MHz)"]
    #[inline(always)]
    pub fn is_sysosc32m(&self) -> bool {
        *self == Sysoscfreq::Sysosc32m
    }
    #[doc = "SYSOSC is at low frequency (4MHz)"]
    #[inline(always)]
    pub fn is_sysosc4m(&self) -> bool {
        *self == Sysoscfreq::Sysosc4m
    }
    #[doc = "SYSOSC is at the user-trimmed frequency (16 or 24MHz)"]
    #[inline(always)]
    pub fn is_sysoscuser(&self) -> bool {
        *self == Sysoscfreq::Sysoscuser
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_sysoscturbo(&self) -> bool {
        *self == Sysoscfreq::Sysoscturbo
    }
}
#[doc = "HSCLKMUX indicates if MCLK is currently sourced from the high-speed clock (HSCLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkmux {
    #[doc = "0: MCLK is not sourced from HSCLK"]
    Sysosc = 0,
    #[doc = "1: MCLK is sourced from HSCLK"]
    Hsclk = 1,
}
impl From<Hsclkmux> for bool {
    #[inline(always)]
    fn from(variant: Hsclkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKMUX` reader - HSCLKMUX indicates if MCLK is currently sourced from the high-speed clock (HSCLK)."]
pub type HsclkmuxR = crate::BitReader<Hsclkmux>;
impl HsclkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclkmux {
        match self.bits {
            false => Hsclkmux::Sysosc,
            true => Hsclkmux::Hsclk,
        }
    }
    #[doc = "MCLK is not sourced from HSCLK"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Hsclkmux::Sysosc
    }
    #[doc = "MCLK is sourced from HSCLK"]
    #[inline(always)]
    pub fn is_hsclk(&self) -> bool {
        *self == Hsclkmux::Hsclk
    }
}
#[doc = "LFCLKMUX indicates if LFCLK is sourced from the internal LFOSC, the low frequency crystal (LFXT), or the LFCLK_IN digital clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfclkmux {
    #[doc = "0: LFCLK is sourced from the internal LFOSC"]
    Lfosc = 0,
    #[doc = "1: LFCLK is sourced from the LFXT (crystal)"]
    Lfxt = 1,
    #[doc = "2: LFCLK is sourced from LFCLK_IN (external digital clock input)"]
    Exlf = 2,
}
impl From<Lfclkmux> for u8 {
    #[inline(always)]
    fn from(variant: Lfclkmux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfclkmux {
    type Ux = u8;
}
impl crate::IsEnum for Lfclkmux {}
#[doc = "Field `LFCLKMUX` reader - LFCLKMUX indicates if LFCLK is sourced from the internal LFOSC, the low frequency crystal (LFXT), or the LFCLK_IN digital clock input."]
pub type LfclkmuxR = crate::FieldReader<Lfclkmux>;
impl LfclkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfclkmux> {
        match self.bits {
            0 => Some(Lfclkmux::Lfosc),
            1 => Some(Lfclkmux::Lfxt),
            2 => Some(Lfclkmux::Exlf),
            _ => None,
        }
    }
    #[doc = "LFCLK is sourced from the internal LFOSC"]
    #[inline(always)]
    pub fn is_lfosc(&self) -> bool {
        *self == Lfclkmux::Lfosc
    }
    #[doc = "LFCLK is sourced from the LFXT (crystal)"]
    #[inline(always)]
    pub fn is_lfxt(&self) -> bool {
        *self == Lfclkmux::Lfxt
    }
    #[doc = "LFCLK is sourced from LFCLK_IN (external digital clock input)"]
    #[inline(always)]
    pub fn is_exlf(&self) -> bool {
        *self == Lfclkmux::Exlf
    }
}
#[doc = "HFCLKGOOD indicates that the HFCLK started correctly. When the HFXT is started or HFCLK_IN is selected as the HFCLK source, this bit will be set by hardware if a valid HFCLK is detected, and cleared if HFCLK is not operating within the expected range.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkgood {
    #[doc = "0: HFCLK did not start correctly"]
    False = 0,
    #[doc = "1: HFCLK started correctly"]
    True = 1,
}
impl From<Hfclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKGOOD` reader - HFCLKGOOD indicates that the HFCLK started correctly. When the HFXT is started or HFCLK_IN is selected as the HFCLK source, this bit will be set by hardware if a valid HFCLK is detected, and cleared if HFCLK is not operating within the expected range."]
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
    #[doc = "HFCLK did not start correctly"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hfclkgood::False
    }
    #[doc = "HFCLK started correctly"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hfclkgood::True
    }
}
#[doc = "LFXTGOOD indicates if the LFXT started correctly. When the LFXT is started, LFXTGOOD is cleared by hardware. After the startup settling time has expired, the LFXT status is tested. If the LFXT started successfully the LFXTGOOD bit is set, else it is left cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtgood {
    #[doc = "0: LFXT did not start correctly"]
    False = 0,
    #[doc = "1: LFXT started correctly"]
    True = 1,
}
impl From<Lfxtgood> for bool {
    #[inline(always)]
    fn from(variant: Lfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTGOOD` reader - LFXTGOOD indicates if the LFXT started correctly. When the LFXT is started, LFXTGOOD is cleared by hardware. After the startup settling time has expired, the LFXT status is tested. If the LFXT started successfully the LFXTGOOD bit is set, else it is left cleared."]
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
    #[doc = "LFXT did not start correctly"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Lfxtgood::False
    }
    #[doc = "LFXT started correctly"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Lfxtgood::True
    }
}
#[doc = "LFOSCGOOD indicates when the LFOSC startup has completed and the LFOSC is ready for use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfoscgood {
    #[doc = "0: LFOSC is not ready"]
    False = 0,
    #[doc = "1: LFOSC is ready"]
    True = 1,
}
impl From<Lfoscgood> for bool {
    #[inline(always)]
    fn from(variant: Lfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFOSCGOOD` reader - LFOSCGOOD indicates when the LFOSC startup has completed and the LFOSC is ready for use."]
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
    #[doc = "LFOSC is not ready"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Lfoscgood::False
    }
    #[doc = "LFOSC is ready"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Lfoscgood::True
    }
}
#[doc = "HSCLKSOFF is set when the high speed clock sources (SYSPLL, HFCLK) are disabled or dead. It is the logical AND of HFCLKOFF and SYSPLLOFF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclksoff {
    #[doc = "0: SYSPLL, HFCLK, or both were started correctly and remain enabled"]
    False = 0,
    #[doc = "1: SYSPLL and HFCLK are both either off or dead"]
    True = 1,
}
impl From<Hsclksoff> for bool {
    #[inline(always)]
    fn from(variant: Hsclksoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKSOFF` reader - HSCLKSOFF is set when the high speed clock sources (SYSPLL, HFCLK) are disabled or dead. It is the logical AND of HFCLKOFF and SYSPLLOFF."]
pub type HsclksoffR = crate::BitReader<Hsclksoff>;
impl HsclksoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclksoff {
        match self.bits {
            false => Hsclksoff::False,
            true => Hsclksoff::True,
        }
    }
    #[doc = "SYSPLL, HFCLK, or both were started correctly and remain enabled"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hsclksoff::False
    }
    #[doc = "SYSPLL and HFCLK are both either off or dead"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hsclksoff::True
    }
}
#[doc = "HFCLKOFF indicates if the HFCLK is disabled or was dead at startup. When the HFCLK is started, HFCLKOFF is cleared by hardware. Following startup of the HFCLK, if the HFCLK startup monitor determines that the HFCLK was not started correctly, HFCLKOFF is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkoff {
    #[doc = "0: HFCLK started correctly and is enabled"]
    False = 0,
    #[doc = "1: HFCLK is disabled or was dead at startup"]
    True = 1,
}
impl From<Hfclkoff> for bool {
    #[inline(always)]
    fn from(variant: Hfclkoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKOFF` reader - HFCLKOFF indicates if the HFCLK is disabled or was dead at startup. When the HFCLK is started, HFCLKOFF is cleared by hardware. Following startup of the HFCLK, if the HFCLK startup monitor determines that the HFCLK was not started correctly, HFCLKOFF is set."]
pub type HfclkoffR = crate::BitReader<Hfclkoff>;
impl HfclkoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkoff {
        match self.bits {
            false => Hfclkoff::False,
            true => Hfclkoff::True,
        }
    }
    #[doc = "HFCLK started correctly and is enabled"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hfclkoff::False
    }
    #[doc = "HFCLK is disabled or was dead at startup"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hfclkoff::True
    }
}
#[doc = "CURHSCLKSEL indicates the current clock source for HSCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curhsclksel {
    #[doc = "0: HSCLK is currently sourced from the SYSPLL"]
    Syspll = 0,
    #[doc = "1: HSCLK is currently sourced from the HFCLK"]
    Hfclk = 1,
}
impl From<Curhsclksel> for bool {
    #[inline(always)]
    fn from(variant: Curhsclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURHSCLKSEL` reader - CURHSCLKSEL indicates the current clock source for HSCLK."]
pub type CurhsclkselR = crate::BitReader<Curhsclksel>;
impl CurhsclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curhsclksel {
        match self.bits {
            false => Curhsclksel::Syspll,
            true => Curhsclksel::Hfclk,
        }
    }
    #[doc = "HSCLK is currently sourced from the SYSPLL"]
    #[inline(always)]
    pub fn is_syspll(&self) -> bool {
        *self == Curhsclksel::Syspll
    }
    #[doc = "HSCLK is currently sourced from the HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Curhsclksel::Hfclk
    }
}
#[doc = "CURMCLKSEL indicates if MCLK is currently sourced from LFCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curmclksel {
    #[doc = "0: MCLK is not sourced from LFCLK"]
    Notlfclk = 0,
    #[doc = "1: MCLK is sourced from LFCLK"]
    Lfclk = 1,
}
impl From<Curmclksel> for bool {
    #[inline(always)]
    fn from(variant: Curmclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURMCLKSEL` reader - CURMCLKSEL indicates if MCLK is currently sourced from LFCLK."]
pub type CurmclkselR = crate::BitReader<Curmclksel>;
impl CurmclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curmclksel {
        match self.bits {
            false => Curmclksel::Notlfclk,
            true => Curmclksel::Lfclk,
        }
    }
    #[doc = "MCLK is not sourced from LFCLK"]
    #[inline(always)]
    pub fn is_notlfclk(&self) -> bool {
        *self == Curmclksel::Notlfclk
    }
    #[doc = "MCLK is sourced from LFCLK"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == Curmclksel::Lfclk
    }
}
#[doc = "HSCLKDEAD is set by hardware if the selected source for HSCLK was started but did not start successfully.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkdead {
    #[doc = "0: The HSCLK source was not started or started correctly"]
    False = 0,
    #[doc = "1: The HSCLK source did not start correctly"]
    True = 1,
}
impl From<Hsclkdead> for bool {
    #[inline(always)]
    fn from(variant: Hsclkdead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKDEAD` reader - HSCLKDEAD is set by hardware if the selected source for HSCLK was started but did not start successfully."]
pub type HsclkdeadR = crate::BitReader<Hsclkdead>;
impl HsclkdeadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclkdead {
        match self.bits {
            false => Hsclkdead::False,
            true => Hsclkdead::True,
        }
    }
    #[doc = "The HSCLK source was not started or started correctly"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hsclkdead::False
    }
    #[doc = "The HSCLK source did not start correctly"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hsclkdead::True
    }
}
#[doc = "HSCLKGOOD is set by hardware if the selected clock source for HSCLK started successfully.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkgood {
    #[doc = "0: The HSCLK source did not start correctly"]
    False = 0,
    #[doc = "1: The HSCLK source started correctly"]
    True = 1,
}
impl From<Hsclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKGOOD` reader - HSCLKGOOD is set by hardware if the selected clock source for HSCLK started successfully."]
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
    #[doc = "The HSCLK source did not start correctly"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hsclkgood::False
    }
    #[doc = "The HSCLK source started correctly"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hsclkgood::True
    }
}
#[doc = "LFCLKFAIL indicates when the continous LFCLK monitor detects a LFXT or LFCLK_IN clock stuck failure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkfail {
    #[doc = "0: No LFCLK fault detected"]
    False = 0,
    #[doc = "1: LFCLK stuck fault detected"]
    True = 1,
}
impl From<Lfclkfail> for bool {
    #[inline(always)]
    fn from(variant: Lfclkfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKFAIL` reader - LFCLKFAIL indicates when the continous LFCLK monitor detects a LFXT or LFCLK_IN clock stuck failure."]
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
    #[doc = "No LFCLK fault detected"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Lfclkfail::False
    }
    #[doc = "LFCLK stuck fault detected"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Lfclkfail::True
    }
}
#[doc = "FCLMODE indicates if the SYSOSC frequency correction loop (FCL) is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fclmode {
    #[doc = "0: SYSOSC FCL is disabled"]
    Disabled = 0,
    #[doc = "1: SYSOSC FCL is enabled"]
    Enabled = 1,
}
impl From<Fclmode> for bool {
    #[inline(always)]
    fn from(variant: Fclmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCLMODE` reader - FCLMODE indicates if the SYSOSC frequency correction loop (FCL) is enabled."]
pub type FclmodeR = crate::BitReader<Fclmode>;
impl FclmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fclmode {
        match self.bits {
            false => Fclmode::Disabled,
            true => Fclmode::Enabled,
        }
    }
    #[doc = "SYSOSC FCL is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fclmode::Disabled
    }
    #[doc = "SYSOSC FCL is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fclmode::Enabled
    }
}
#[doc = "FCCDONE indicates when a frequency clock counter capture is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fccdone {
    #[doc = "0: FCC capture is not done"]
    Notdone = 0,
    #[doc = "1: FCC capture is done"]
    Done = 1,
}
impl From<Fccdone> for bool {
    #[inline(always)]
    fn from(variant: Fccdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCDONE` reader - FCCDONE indicates when a frequency clock counter capture is complete."]
pub type FccdoneR = crate::BitReader<Fccdone>;
impl FccdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fccdone {
        match self.bits {
            false => Fccdone::Notdone,
            true => Fccdone::Done,
        }
    }
    #[doc = "FCC capture is not done"]
    #[inline(always)]
    pub fn is_notdone(&self) -> bool {
        *self == Fccdone::Notdone
    }
    #[doc = "FCC capture is done"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Fccdone::Done
    }
}
#[doc = "HFCLKBLKUPD indicates when writes to the HFCLKCLKCFG register are blocked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkblkupd {
    #[doc = "0: Writes to HFCLKCLKCFG are allowed"]
    False = 0,
    #[doc = "1: Writes to HFCLKCLKCFG are blocked"]
    True = 1,
}
impl From<Hfclkblkupd> for bool {
    #[inline(always)]
    fn from(variant: Hfclkblkupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKBLKUPD` reader - HFCLKBLKUPD indicates when writes to the HFCLKCLKCFG register are blocked."]
pub type HfclkblkupdR = crate::BitReader<Hfclkblkupd>;
impl HfclkblkupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkblkupd {
        match self.bits {
            false => Hfclkblkupd::False,
            true => Hfclkblkupd::True,
        }
    }
    #[doc = "Writes to HFCLKCLKCFG are allowed"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Hfclkblkupd::False
    }
    #[doc = "Writes to HFCLKCLKCFG are blocked"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Hfclkblkupd::True
    }
}
#[doc = "ANACLKERR is set when the device clock configuration does not support an enabled analog peripheral mode and the analog peripheral may not be functioning as expected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anaclkerr {
    #[doc = "0: No analog clock errors detected"]
    False = 0,
    #[doc = "1: Analog clock error detected"]
    True = 1,
}
impl From<Anaclkerr> for bool {
    #[inline(always)]
    fn from(variant: Anaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACLKERR` reader - ANACLKERR is set when the device clock configuration does not support an enabled analog peripheral mode and the analog peripheral may not be functioning as expected."]
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
    #[doc = "No analog clock errors detected"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Anaclkerr::False
    }
    #[doc = "Analog clock error detected"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Anaclkerr::True
    }
}
impl R {
    #[doc = "Bits 0:1 - SYSOSCFREQ indicates the current SYSOSC operating frequency."]
    #[inline(always)]
    pub fn sysoscfreq(&self) -> SysoscfreqR {
        SysoscfreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - HSCLKMUX indicates if MCLK is currently sourced from the high-speed clock (HSCLK)."]
    #[inline(always)]
    pub fn hsclkmux(&self) -> HsclkmuxR {
        HsclkmuxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LFCLKMUX indicates if LFCLK is sourced from the internal LFOSC, the low frequency crystal (LFXT), or the LFCLK_IN digital clock input."]
    #[inline(always)]
    pub fn lfclkmux(&self) -> LfclkmuxR {
        LfclkmuxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - HFCLKGOOD indicates that the HFCLK started correctly. When the HFXT is started or HFCLK_IN is selected as the HFCLK source, this bit will be set by hardware if a valid HFCLK is detected, and cleared if HFCLK is not operating within the expected range."]
    #[inline(always)]
    pub fn hfclkgood(&self) -> HfclkgoodR {
        HfclkgoodR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - LFXTGOOD indicates if the LFXT started correctly. When the LFXT is started, LFXTGOOD is cleared by hardware. After the startup settling time has expired, the LFXT status is tested. If the LFXT started successfully the LFXTGOOD bit is set, else it is left cleared."]
    #[inline(always)]
    pub fn lfxtgood(&self) -> LfxtgoodR {
        LfxtgoodR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LFOSCGOOD indicates when the LFOSC startup has completed and the LFOSC is ready for use."]
    #[inline(always)]
    pub fn lfoscgood(&self) -> LfoscgoodR {
        LfoscgoodR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HSCLKSOFF is set when the high speed clock sources (SYSPLL, HFCLK) are disabled or dead. It is the logical AND of HFCLKOFF and SYSPLLOFF."]
    #[inline(always)]
    pub fn hsclksoff(&self) -> HsclksoffR {
        HsclksoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HFCLKOFF indicates if the HFCLK is disabled or was dead at startup. When the HFCLK is started, HFCLKOFF is cleared by hardware. Following startup of the HFCLK, if the HFCLK startup monitor determines that the HFCLK was not started correctly, HFCLKOFF is set."]
    #[inline(always)]
    pub fn hfclkoff(&self) -> HfclkoffR {
        HfclkoffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CURHSCLKSEL indicates the current clock source for HSCLK."]
    #[inline(always)]
    pub fn curhsclksel(&self) -> CurhsclkselR {
        CurhsclkselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CURMCLKSEL indicates if MCLK is currently sourced from LFCLK."]
    #[inline(always)]
    pub fn curmclksel(&self) -> CurmclkselR {
        CurmclkselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - HSCLKDEAD is set by hardware if the selected source for HSCLK was started but did not start successfully."]
    #[inline(always)]
    pub fn hsclkdead(&self) -> HsclkdeadR {
        HsclkdeadR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HSCLKGOOD is set by hardware if the selected clock source for HSCLK started successfully."]
    #[inline(always)]
    pub fn hsclkgood(&self) -> HsclkgoodR {
        HsclkgoodR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - LFCLKFAIL indicates when the continous LFCLK monitor detects a LFXT or LFCLK_IN clock stuck failure."]
    #[inline(always)]
    pub fn lfclkfail(&self) -> LfclkfailR {
        LfclkfailR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FCLMODE indicates if the SYSOSC frequency correction loop (FCL) is enabled."]
    #[inline(always)]
    pub fn fclmode(&self) -> FclmodeR {
        FclmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FCCDONE indicates when a frequency clock counter capture is complete."]
    #[inline(always)]
    pub fn fccdone(&self) -> FccdoneR {
        FccdoneR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - HFCLKBLKUPD indicates when writes to the HFCLKCLKCFG register are blocked."]
    #[inline(always)]
    pub fn hfclkblkupd(&self) -> HfclkblkupdR {
        HfclkblkupdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - ANACLKERR is set when the device clock configuration does not support an enabled analog peripheral mode and the analog peripheral may not be functioning as expected."]
    #[inline(always)]
    pub fn anaclkerr(&self) -> AnaclkerrR {
        AnaclkerrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Clock module (CKM) status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_clkstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlClkstatusSpec;
impl crate::RegisterSpec for SysctlClkstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_clkstatus::R`](R) reader structure"]
impl crate::Readable for SysctlClkstatusSpec {}
#[doc = "`reset()` method sets SYSCTL_CLKSTATUS to value 0"]
impl crate::Resettable for SysctlClkstatusSpec {}
