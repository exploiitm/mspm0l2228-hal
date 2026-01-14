#[doc = "Register `TIMA0_CPSV` reader"]
pub type R = crate::R<Tima0CpsvSpec>;
#[doc = "Field `CPSVAL` reader - Current Prescale Count Value"]
pub type CpsvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current Prescale Count Value"]
    #[inline(always)]
    pub fn cpsval(&self) -> CpsvalR {
        CpsvalR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cpsv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0CpsvSpec;
impl crate::RegisterSpec for Tima0CpsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_cpsv::R`](R) reader structure"]
impl crate::Readable for Tima0CpsvSpec {}
#[doc = "`reset()` method sets TIMA0_CPSV to value 0"]
impl crate::Resettable for Tima0CpsvSpec {}
