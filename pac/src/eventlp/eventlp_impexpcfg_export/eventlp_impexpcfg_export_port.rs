#[doc = "Register `EVENTLP_IMPEXPCFG_EXPORT_PORT` reader"]
pub type R = crate::R<EventlpImpexpcfgExportPortSpec>;
#[doc = "Register `EVENTLP_IMPEXPCFG_EXPORT_PORT` writer"]
pub type W = crate::W<EventlpImpexpcfgExportPortSpec>;
#[doc = "Field `CHANID` reader - Channel ID for import side to connect to"]
pub type ChanidR = crate::FieldReader;
#[doc = "Field `CHANID` writer - Channel ID for import side to connect to"]
pub type ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel ID for import side to connect to"]
    #[inline(always)]
    pub fn chanid(&self) -> ChanidR {
        ChanidR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel ID for import side to connect to"]
    #[inline(always)]
    pub fn chanid(&mut self) -> ChanidW<'_, EventlpImpexpcfgExportPortSpec> {
        ChanidW::new(self, 0)
    }
}
#[doc = "Export channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_impexpcfg_export_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_impexpcfg_export_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpImpexpcfgExportPortSpec;
impl crate::RegisterSpec for EventlpImpexpcfgExportPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_impexpcfg_export_port::R`](R) reader structure"]
impl crate::Readable for EventlpImpexpcfgExportPortSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_impexpcfg_export_port::W`](W) writer structure"]
impl crate::Writable for EventlpImpexpcfgExportPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_IMPEXPCFG_EXPORT_PORT to value 0"]
impl crate::Resettable for EventlpImpexpcfgExportPortSpec {}
