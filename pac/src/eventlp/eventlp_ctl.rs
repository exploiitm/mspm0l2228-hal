#[doc = "Register `EVENTLP_CTL` reader"]
pub type R = crate::R<EventlpCtlSpec>;
#[doc = "Register `EVENTLP_CTL` writer"]
pub type W = crate::W<EventlpCtlSpec>;
#[doc = "Enable overwrite of config even if resources are already configured. By default, a configuration cannot be overwritten.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OvrwrEn {
    #[doc = "5: Overwrite is disabled. Mode is same as in UNASG aperture."]
    Disable = 5,
    #[doc = "10: Ovewrite is enabled"]
    Enabled = 10,
}
impl From<OvrwrEn> for u8 {
    #[inline(always)]
    fn from(variant: OvrwrEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OvrwrEn {
    type Ux = u8;
}
impl crate::IsEnum for OvrwrEn {}
#[doc = "Field `OVRWR_EN` reader - Enable overwrite of config even if resources are already configured. By default, a configuration cannot be overwritten."]
pub type OvrwrEnR = crate::FieldReader<OvrwrEn>;
impl OvrwrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OvrwrEn> {
        match self.bits {
            5 => Some(OvrwrEn::Disable),
            10 => Some(OvrwrEn::Enabled),
            _ => None,
        }
    }
    #[doc = "Overwrite is disabled. Mode is same as in UNASG aperture."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OvrwrEn::Disable
    }
    #[doc = "Ovewrite is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OvrwrEn::Enabled
    }
}
#[doc = "Field `OVRWR_EN` writer - Enable overwrite of config even if resources are already configured. By default, a configuration cannot be overwritten."]
pub type OvrwrEnW<'a, REG> = crate::FieldWriter<'a, REG, 4, OvrwrEn>;
impl<'a, REG> OvrwrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Overwrite is disabled. Mode is same as in UNASG aperture."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OvrwrEn::Disable)
    }
    #[doc = "Ovewrite is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OvrwrEn::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Enable overwrite of config even if resources are already configured. By default, a configuration cannot be overwritten."]
    #[inline(always)]
    pub fn ovrwr_en(&self) -> OvrwrEnR {
        OvrwrEnR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enable overwrite of config even if resources are already configured. By default, a configuration cannot be overwritten."]
    #[inline(always)]
    pub fn ovrwr_en(&mut self) -> OvrwrEnW<'_, EventlpCtlSpec> {
        OvrwrEnW::new(self, 0)
    }
}
#[doc = "Event Manager control register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpCtlSpec;
impl crate::RegisterSpec for EventlpCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eventlp_ctl::R`](R) reader structure"]
impl crate::Readable for EventlpCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_ctl::W`](W) writer structure"]
impl crate::Writable for EventlpCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_CTL to value 0"]
impl crate::Resettable for EventlpCtlSpec {}
