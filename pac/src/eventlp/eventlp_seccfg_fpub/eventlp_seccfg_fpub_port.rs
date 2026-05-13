#[doc = "Register `EVENTLP_SECCFG_FPUB_PORT[%s]` reader"]
pub type R = crate::R<EventlpSeccfgFpubPortSpec>;
#[doc = "Register `EVENTLP_SECCFG_FPUB_PORT[%s]` writer"]
pub type W = crate::W<EventlpSeccfgFpubPortSpec>;
#[doc = "Field `CHANID` reader - Channel ID for publisher to connect to"]
pub type ChanidR = crate::FieldReader;
#[doc = "Field `CHANID` writer - Channel ID for publisher to connect to"]
pub type ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel ID for publisher to connect to"]
    #[inline(always)]
    pub fn chanid(&self) -> ChanidR {
        ChanidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel ID for publisher to connect to"]
    #[inline(always)]
    pub fn chanid(&mut self) -> ChanidW<'_, EventlpSeccfgFpubPortSpec> {
        ChanidW::new(self, 0)
    }
}
#[doc = "Publisher channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_fpub_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_fpub_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpSeccfgFpubPortSpec;
impl crate::RegisterSpec for EventlpSeccfgFpubPortSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eventlp_seccfg_fpub_port::R`](R) reader structure"]
impl crate::Readable for EventlpSeccfgFpubPortSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_seccfg_fpub_port::W`](W) writer structure"]
impl crate::Writable for EventlpSeccfgFpubPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_SECCFG_FPUB_PORT[%s] to value 0"]
impl crate::Resettable for EventlpSeccfgFpubPortSpec {}
