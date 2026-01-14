#[doc = "Register `LFSS_EVT_MODE` reader"]
pub type R = crate::R<LfssEvtModeSpec>;
#[doc = "Event line mode 0 select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evt0Cfg {
    #[doc = "1: The interrupt or event line is in software mode. The software ISR clears the associated RIS flag."]
    Software = 1,
}
impl From<Evt0Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Evt0Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evt0Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Evt0Cfg {}
#[doc = "Field `EVT0_CFG` reader - Event line mode 0 select"]
pub type Evt0CfgR = crate::FieldReader<Evt0Cfg>;
impl Evt0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evt0Cfg> {
        match self.bits {
            1 => Some(Evt0Cfg::Software),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is in software mode. The software ISR clears the associated RIS flag."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Evt0Cfg::Software
    }
}
#[doc = "Event line mode 1 select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evt1Cfg {
    #[doc = "2: The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    Hardware = 2,
}
impl From<Evt1Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Evt1Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evt1Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Evt1Cfg {}
#[doc = "Field `EVT1_CFG` reader - Event line mode 1 select"]
pub type Evt1CfgR = crate::FieldReader<Evt1Cfg>;
impl Evt1CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evt1Cfg> {
        match self.bits {
            2 => Some(Evt1Cfg::Hardware),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Evt1Cfg::Hardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode 0 select"]
    #[inline(always)]
    pub fn evt0_cfg(&self) -> Evt0CfgR {
        Evt0CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event line mode 1 select"]
    #[inline(always)]
    pub fn evt1_cfg(&self) -> Evt1CfgR {
        Evt1CfgR::new(((self.bits >> 2) & 3) as u8)
    }
}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_evt_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssEvtModeSpec;
impl crate::RegisterSpec for LfssEvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_evt_mode::R`](R) reader structure"]
impl crate::Readable for LfssEvtModeSpec {}
#[doc = "`reset()` method sets LFSS_EVT_MODE to value 0"]
impl crate::Resettable for LfssEvtModeSpec {}
