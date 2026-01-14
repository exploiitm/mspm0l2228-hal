#[doc = "Register `DEBUGSS_EVT_MODE` reader"]
pub type R = crate::R<DebugssEvtModeSpec>;
#[doc = "Event line mode select for peripheral events\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Int0Cfg {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Software = 1,
    #[doc = "2: The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    Hardware = 2,
}
impl From<Int0Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Int0Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Int0Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Int0Cfg {}
#[doc = "Field `INT0_CFG` reader - Event line mode select for peripheral events"]
pub type Int0CfgR = crate::FieldReader<Int0Cfg>;
impl Int0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Int0Cfg> {
        match self.bits {
            0 => Some(Int0Cfg::Disable),
            1 => Some(Int0Cfg::Software),
            2 => Some(Int0Cfg::Hardware),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Int0Cfg::Disable
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Int0Cfg::Software
    }
    #[doc = "The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Int0Cfg::Hardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select for peripheral events"]
    #[inline(always)]
    pub fn int0_cfg(&self) -> Int0CfgR {
        Int0CfgR::new((self.bits & 3) as u8)
    }
}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_evt_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssEvtModeSpec;
impl crate::RegisterSpec for DebugssEvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_evt_mode::R`](R) reader structure"]
impl crate::Readable for DebugssEvtModeSpec {}
#[doc = "`reset()` method sets DEBUGSS_EVT_MODE to value 0"]
impl crate::Resettable for DebugssEvtModeSpec {}
