#[doc = "Register `UART4_EVT_MODE` reader"]
pub type R = crate::R<Uart4EvtModeSpec>;
#[doc = "Register `UART4_EVT_MODE` writer"]
pub type W = crate::W<Uart4EvtModeSpec>;
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT0\\]\n\nValue on reset: 0"]
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
#[doc = "Field `INT0_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT0\\]"]
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
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Int1Cfg {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Software = 1,
    #[doc = "2: The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    Hardware = 2,
}
impl From<Int1Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Int1Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Int1Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Int1Cfg {}
#[doc = "Field `INT1_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT1\\]"]
pub type Int1CfgR = crate::FieldReader<Int1Cfg>;
impl Int1CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Int1Cfg> {
        match self.bits {
            0 => Some(Int1Cfg::Disable),
            1 => Some(Int1Cfg::Software),
            2 => Some(Int1Cfg::Hardware),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Int1Cfg::Disable
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Int1Cfg::Software
    }
    #[doc = "The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Int1Cfg::Hardware
    }
}
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Int2Cfg {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Software = 1,
    #[doc = "2: The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    Hardware = 2,
}
impl From<Int2Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Int2Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Int2Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Int2Cfg {}
#[doc = "Field `INT2_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT2\\]"]
pub type Int2CfgR = crate::FieldReader<Int2Cfg>;
impl Int2CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Int2Cfg> {
        match self.bits {
            0 => Some(Int2Cfg::Disable),
            1 => Some(Int2Cfg::Software),
            2 => Some(Int2Cfg::Hardware),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Int2Cfg::Disable
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Int2Cfg::Software
    }
    #[doc = "The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Int2Cfg::Hardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT0\\]"]
    #[inline(always)]
    pub fn int0_cfg(&self) -> Int0CfgR {
        Int0CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT1\\]"]
    #[inline(always)]
    pub fn int1_cfg(&self) -> Int1CfgR {
        Int1CfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT2\\]"]
    #[inline(always)]
    pub fn int2_cfg(&self) -> Int2CfgR {
        Int2CfgR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_evt_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_evt_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart4EvtModeSpec;
impl crate::RegisterSpec for Uart4EvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart4_evt_mode::R`](R) reader structure"]
impl crate::Readable for Uart4EvtModeSpec {}
#[doc = "`write(|w| ..)` method takes [`uart4_evt_mode::W`](W) writer structure"]
impl crate::Writable for Uart4EvtModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART4_EVT_MODE to value 0"]
impl crate::Resettable for Uart4EvtModeSpec {}
