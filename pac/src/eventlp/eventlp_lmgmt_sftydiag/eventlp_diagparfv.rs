#[doc = "Register `EVENTLP_DIAGPARFV` reader"]
pub type R = crate::R<EventlpDiagparfvSpec>;
#[doc = "Field `INDEX` reader - Index of DIAG MMR creating the failure. NOTE: INDEX value of 1 corresponds to DIAGPAR0, 2 corresponds to DIAGPAR1 and so on."]
pub type IndexR = crate::FieldReader<u16>;
#[doc = "Field `DPINDEX` reader - Index of DIAG PASS MMR. NOTE: DPINDEX value of 1 corresponds to DIAGPAR0, 2 corresponds to DIAGPAR1 and so on."]
pub type DpindexR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Index of DIAG MMR creating the failure. NOTE: INDEX value of 1 corresponds to DIAGPAR0, 2 corresponds to DIAGPAR1 and so on."]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Index of DIAG PASS MMR. NOTE: DPINDEX value of 1 corresponds to DIAGPAR0, 2 corresponds to DIAGPAR1 and so on."]
    #[inline(always)]
    pub fn dpindex(&self) -> DpindexR {
        DpindexR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "Diagnostic Parity Fail Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_diagparfv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpDiagparfvSpec;
impl crate::RegisterSpec for EventlpDiagparfvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_diagparfv::R`](R) reader structure"]
impl crate::Readable for EventlpDiagparfvSpec {}
#[doc = "`reset()` method sets EVENTLP_DIAGPARFV to value 0"]
impl crate::Resettable for EventlpDiagparfvSpec {}
