#[doc = "Register `EVENTLP_PUBCFG_CPU_NUM[%s]` reader"]
pub type R = crate::R<EventlpPubcfgCpuNumSpec>;
#[doc = "Register `EVENTLP_PUBCFG_CPU_NUM[%s]` writer"]
pub type W = crate::W<EventlpPubcfgCpuNumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CPU connect register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_cpu_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_pubcfg_cpu_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpPubcfgCpuNumSpec;
impl crate::RegisterSpec for EventlpPubcfgCpuNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_pubcfg_cpu_num::R`](R) reader structure"]
impl crate::Readable for EventlpPubcfgCpuNumSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_pubcfg_cpu_num::W`](W) writer structure"]
impl crate::Writable for EventlpPubcfgCpuNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_PUBCFG_CPU_NUM[%s] to value 0"]
impl crate::Resettable for EventlpPubcfgCpuNumSpec {}
