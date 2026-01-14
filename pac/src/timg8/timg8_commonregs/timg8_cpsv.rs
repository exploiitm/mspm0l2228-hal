#[doc = "Register `TIMG8_CPSV` reader"]
pub type R = crate::R<Timg8CpsvSpec>;
#[doc = "Field `CPSVAL` reader - Current Prescale Count Value"]
pub type CpsvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current Prescale Count Value"]
    #[inline(always)]
    pub fn cpsval(&self) -> CpsvalR {
        CpsvalR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_cpsv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg8CpsvSpec;
impl crate::RegisterSpec for Timg8CpsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg8_cpsv::R`](R) reader structure"]
impl crate::Readable for Timg8CpsvSpec {}
#[doc = "`reset()` method sets TIMG8_CPSV to value 0"]
impl crate::Resettable for Timg8CpsvSpec {}
