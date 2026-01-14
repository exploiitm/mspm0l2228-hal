#[doc = "Register `EVENTLP_IMPEXPCFG_IMPORT_PORT` reader"]
pub type R = crate::R<EventlpImpexpcfgImportPortSpec>;
#[doc = "Register `EVENTLP_IMPEXPCFG_IMPORT_PORT` writer"]
pub type W = crate::W<EventlpImpexpcfgImportPortSpec>;
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
    pub fn chanid(&mut self) -> ChanidW<'_, EventlpImpexpcfgImportPortSpec> {
        ChanidW::new(self, 0)
    }
}
#[doc = "Import channel ID registe\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_impexpcfg_import_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_impexpcfg_import_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpImpexpcfgImportPortSpec;
impl crate::RegisterSpec for EventlpImpexpcfgImportPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_impexpcfg_import_port::R`](R) reader structure"]
impl crate::Readable for EventlpImpexpcfgImportPortSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_impexpcfg_import_port::W`](W) writer structure"]
impl crate::Writable for EventlpImpexpcfgImportPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_IMPEXPCFG_IMPORT_PORT to value 0"]
impl crate::Resettable for EventlpImpexpcfgImportPortSpec {}
