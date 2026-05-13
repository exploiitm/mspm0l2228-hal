#[doc = "Register `DMA_EVT_MODE` reader"]
pub type R = crate::R<DmaEvtModeSpec>;
#[doc = "Register `DMA_EVT_MODE` writer"]
pub type W = crate::W<DmaEvtModeSpec>;
#[doc = "Event line mode select for event corresponding to interrupt event CPU_INT\n\nValue on reset: 0"]
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
#[doc = "Field `INT0_CFG` reader - Event line mode select for event corresponding to interrupt event CPU_INT"]
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
#[doc = "Event line mode select for event corresponding to generic event GEN_EVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evt1Cfg {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Software = 1,
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
#[doc = "Field `EVT1_CFG` reader - Event line mode select for event corresponding to generic event GEN_EVENT"]
pub type Evt1CfgR = crate::FieldReader<Evt1Cfg>;
impl Evt1CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evt1Cfg> {
        match self.bits {
            0 => Some(Evt1Cfg::Disable),
            1 => Some(Evt1Cfg::Software),
            2 => Some(Evt1Cfg::Hardware),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Evt1Cfg::Disable
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Evt1Cfg::Software
    }
    #[doc = "The interrupt or event line is in hardware mode. The hardware (another module) clears automatically the associated RIS flag."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Evt1Cfg::Hardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select for event corresponding to interrupt event CPU_INT"]
    #[inline(always)]
    pub fn int0_cfg(&self) -> Int0CfgR {
        Int0CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event line mode select for event corresponding to generic event GEN_EVENT"]
    #[inline(always)]
    pub fn evt1_cfg(&self) -> Evt1CfgR {
        Evt1CfgR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_evt_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_evt_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEvtModeSpec;
impl crate::RegisterSpec for DmaEvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_evt_mode::R`](R) reader structure"]
impl crate::Readable for DmaEvtModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_evt_mode::W`](W) writer structure"]
impl crate::Writable for DmaEvtModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_EVT_MODE to value 0"]
impl crate::Resettable for DmaEvtModeSpec {}
