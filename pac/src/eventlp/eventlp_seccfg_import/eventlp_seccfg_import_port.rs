#[doc = "Register `EVENTLP_SECCFG_IMPORT_PORT` reader"]
pub type R = crate::R<EventlpSeccfgImportPortSpec>;
#[doc = "Register `EVENTLP_SECCFG_IMPORT_PORT` writer"]
pub type W = crate::W<EventlpSeccfgImportPortSpec>;
#[doc = "Field `CHANID` reader - Channel ID for import side to connect to"]
pub type ChanidR = crate::FieldReader;
#[doc = "Field `CHANID` writer - Channel ID for import side to connect to"]
pub type ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel ID for import side to connect to"]
    #[inline(always)]
    pub fn chanid(&self) -> ChanidR {
        ChanidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel ID for import side to connect to"]
    #[inline(always)]
    pub fn chanid(&mut self) -> ChanidW<'_, EventlpSeccfgImportPortSpec> {
        ChanidW::new(self, 0)
    }
}
#[doc = "Import channel ID registe\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_import_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_import_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpSeccfgImportPortSpec;
impl crate::RegisterSpec for EventlpSeccfgImportPortSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eventlp_seccfg_import_port::R`](R) reader structure"]
impl crate::Readable for EventlpSeccfgImportPortSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_seccfg_import_port::W`](W) writer structure"]
impl crate::Writable for EventlpSeccfgImportPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_SECCFG_IMPORT_PORT to value 0"]
impl crate::Resettable for EventlpSeccfgImportPortSpec {}
