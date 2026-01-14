#[doc = "Register `TIMG0_CPSV` reader"]
pub type R = crate::R<Timg0CpsvSpec>;
#[doc = "Field `CPSVAL` reader - Current Prescale Count Value"]
pub type CpsvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current Prescale Count Value"]
    #[inline(always)]
    pub fn cpsval(&self) -> CpsvalR {
        CpsvalR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_cpsv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg0CpsvSpec;
impl crate::RegisterSpec for Timg0CpsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg0_cpsv::R`](R) reader structure"]
impl crate::Readable for Timg0CpsvSpec {}
#[doc = "`reset()` method sets TIMG0_CPSV to value 0"]
impl crate::Resettable for Timg0CpsvSpec {}
