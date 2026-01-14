#[doc = "Register `EVENTLP_SECCFG_EXPORT_PORT` reader"]
pub type R = crate::R<EventlpSeccfgExportPortSpec>;
#[doc = "Register `EVENTLP_SECCFG_EXPORT_PORT` writer"]
pub type W = crate::W<EventlpSeccfgExportPortSpec>;
#[doc = "Field `CHANID` reader - Channel ID for export side to connect to"]
pub type ChanidR = crate::FieldReader;
#[doc = "Field `CHANID` writer - Channel ID for export side to connect to"]
pub type ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel ID for export side to connect to"]
    #[inline(always)]
    pub fn chanid(&self) -> ChanidR {
        ChanidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel ID for export side to connect to"]
    #[inline(always)]
    pub fn chanid(&mut self) -> ChanidW<'_, EventlpSeccfgExportPortSpec> {
        ChanidW::new(self, 0)
    }
}
#[doc = "Export channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_export_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_export_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpSeccfgExportPortSpec;
impl crate::RegisterSpec for EventlpSeccfgExportPortSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eventlp_seccfg_export_port::R`](R) reader structure"]
impl crate::Readable for EventlpSeccfgExportPortSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_seccfg_export_port::W`](W) writer structure"]
impl crate::Writable for EventlpSeccfgExportPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_SECCFG_EXPORT_PORT to value 0"]
impl crate::Resettable for EventlpSeccfgExportPortSpec {}
