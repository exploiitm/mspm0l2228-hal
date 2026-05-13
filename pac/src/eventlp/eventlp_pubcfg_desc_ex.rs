#[doc = "Register `EVENTLP_PUBCFG_DESC_EX` reader"]
pub type R = crate::R<EventlpPubcfgDescExSpec>;
#[doc = "Field `NUM_SINGLE_CHANNEL` reader - Number of single channels contained in this instance of event manager"]
pub type NumSingleChannelR = crate::FieldReader;
#[doc = "Field `NUM_DUAL_CHANNEL` reader - Number of dual channels contained in this instance of event manager"]
pub type NumDualChannelR = crate::FieldReader;
#[doc = "Field `NUM_IMPORT` reader - Number of import ports available in this EventManager instantiation"]
pub type NumImportR = crate::FieldReader;
#[doc = "Field `NUM_EXPORT` reader - Number of export ports available in this EventManager instantiation"]
pub type NumExportR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of single channels contained in this instance of event manager"]
    #[inline(always)]
    pub fn num_single_channel(&self) -> NumSingleChannelR {
        NumSingleChannelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of dual channels contained in this instance of event manager"]
    #[inline(always)]
    pub fn num_dual_channel(&self) -> NumDualChannelR {
        NumDualChannelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of import ports available in this EventManager instantiation"]
    #[inline(always)]
    pub fn num_import(&self) -> NumImportR {
        NumImportR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of export ports available in this EventManager instantiation"]
    #[inline(always)]
    pub fn num_export(&self) -> NumExportR {
        NumExportR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Extended Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_desc_ex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpPubcfgDescExSpec;
impl crate::RegisterSpec for EventlpPubcfgDescExSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_pubcfg_desc_ex::R`](R) reader structure"]
impl crate::Readable for EventlpPubcfgDescExSpec {}
#[doc = "`reset()` method sets EVENTLP_PUBCFG_DESC_EX to value 0"]
impl crate::Resettable for EventlpPubcfgDescExSpec {}
