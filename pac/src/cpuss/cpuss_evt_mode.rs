#[doc = "Register `CPUSS_EVT_MODE` reader"]
pub type R = crate::R<CpussEvtModeSpec>;
#[doc = "Event line mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntCfg {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: Event handled by software. Software must clear the associated RIS flag."]
    Software = 1,
    #[doc = "2: Event handled by hardware. The hardware (another module) clears automatically the associated RIS flag."]
    Hardware = 2,
}
impl From<IntCfg> for u8 {
    #[inline(always)]
    fn from(variant: IntCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntCfg {
    type Ux = u8;
}
impl crate::IsEnum for IntCfg {}
#[doc = "Field `INT_CFG` reader - Event line mode select"]
pub type IntCfgR = crate::FieldReader<IntCfg>;
impl IntCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntCfg> {
        match self.bits {
            0 => Some(IntCfg::Disable),
            1 => Some(IntCfg::Software),
            2 => Some(IntCfg::Hardware),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IntCfg::Disable
    }
    #[doc = "Event handled by software. Software must clear the associated RIS flag."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IntCfg::Software
    }
    #[doc = "Event handled by hardware. The hardware (another module) clears automatically the associated RIS flag."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IntCfg::Hardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select"]
    #[inline(always)]
    pub fn int_cfg(&self) -> IntCfgR {
        IntCfgR::new((self.bits & 3) as u8)
    }
}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_evt_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussEvtModeSpec;
impl crate::RegisterSpec for CpussEvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_evt_mode::R`](R) reader structure"]
impl crate::Readable for CpussEvtModeSpec {}
#[doc = "`reset()` method sets CPUSS_EVT_MODE to value 0"]
impl crate::Resettable for CpussEvtModeSpec {}
