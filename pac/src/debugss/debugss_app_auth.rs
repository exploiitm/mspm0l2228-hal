#[doc = "Register `DEBUGSS_APP_AUTH` reader"]
pub type R = crate::R<DebugssAppAuthSpec>;
#[doc = "Controls invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: Invasive debug disabled"]
    Disable = 0,
    #[doc = "1: Invasive debug enabled"]
    Enable = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Controls invasive debug enable."]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::Disable,
            true => Dbgen::Enable,
        }
    }
    #[doc = "Invasive debug disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbgen::Disable
    }
    #[doc = "Invasive debug enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbgen::Enable
    }
}
#[doc = "Controls non-invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niden {
    #[doc = "0: Non-invasive debug disabled"]
    Disable = 0,
    #[doc = "1: Non-invasive debug enabled"]
    Enable = 1,
}
impl From<Niden> for bool {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` reader - Controls non-invasive debug enable."]
pub type NidenR = crate::BitReader<Niden>;
impl NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niden {
        match self.bits {
            false => Niden::Disable,
            true => Niden::Enable,
        }
    }
    #[doc = "Non-invasive debug disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Niden::Disable
    }
    #[doc = "Non-invasive debug enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Niden::Enable
    }
}
#[doc = "Secure invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiden {
    #[doc = "0: Invasive debug disabled"]
    Disable = 0,
    #[doc = "1: Invasive debug enabled"]
    Enable = 1,
}
impl From<Spiden> for bool {
    #[inline(always)]
    fn from(variant: Spiden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN` reader - Secure invasive debug enable."]
pub type SpidenR = crate::BitReader<Spiden>;
impl SpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiden {
        match self.bits {
            false => Spiden::Disable,
            true => Spiden::Enable,
        }
    }
    #[doc = "Invasive debug disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spiden::Disable
    }
    #[doc = "Invasive debug enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spiden::Enable
    }
}
#[doc = "Secure non-invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spniden {
    #[doc = "0: Invasive debug disabled"]
    Disable = 0,
    #[doc = "1: Invasive debug enabled"]
    Enable = 1,
}
impl From<Spniden> for bool {
    #[inline(always)]
    fn from(variant: Spniden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN` reader - Secure non-invasive debug enable."]
pub type SpnidenR = crate::BitReader<Spniden>;
impl SpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spniden {
        match self.bits {
            false => Spniden::Disable,
            true => Spniden::Enable,
        }
    }
    #[doc = "Invasive debug disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spniden::Disable
    }
    #[doc = "Invasive debug enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spniden::Enable
    }
}
impl R {
    #[doc = "Bit 0 - Controls invasive debug enable."]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls non-invasive debug enable."]
    #[inline(always)]
    pub fn niden(&self) -> NidenR {
        NidenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure invasive debug enable."]
    #[inline(always)]
    pub fn spiden(&self) -> SpidenR {
        SpidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure non-invasive debug enable."]
    #[inline(always)]
    pub fn spniden(&self) -> SpnidenR {
        SpnidenR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Application CPU0 authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_app_auth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssAppAuthSpec;
impl crate::RegisterSpec for DebugssAppAuthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_app_auth::R`](R) reader structure"]
impl crate::Readable for DebugssAppAuthSpec {}
#[doc = "`reset()` method sets DEBUGSS_APP_AUTH to value 0"]
impl crate::Resettable for DebugssAppAuthSpec {}
