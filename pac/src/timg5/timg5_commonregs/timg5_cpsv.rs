#[doc = "Register `TIMG5_CPSV` reader"]
pub type R = crate::R<Timg5CpsvSpec>;
#[doc = "Field `CPSVAL` reader - Current Prescale Count Value"]
pub type CpsvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current Prescale Count Value"]
    #[inline(always)]
    pub fn cpsval(&self) -> CpsvalR {
        CpsvalR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cpsv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg5CpsvSpec;
impl crate::RegisterSpec for Timg5CpsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg5_cpsv::R`](R) reader structure"]
impl crate::Readable for Timg5CpsvSpec {}
#[doc = "`reset()` method sets TIMG5_CPSV to value 0"]
impl crate::Resettable for Timg5CpsvSpec {}
