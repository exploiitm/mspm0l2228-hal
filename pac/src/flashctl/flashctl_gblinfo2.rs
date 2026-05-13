#[doc = "Register `FLASHCTL_GBLINFO2` reader"]
pub type R = crate::R<FlashctlGblinfo2Spec>;
#[doc = "Field `DATAREGISTERS` reader - Number of data registers present."]
pub type DataregistersR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of data registers present."]
    #[inline(always)]
    pub fn dataregisters(&self) -> DataregistersR {
        DataregistersR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Global Information Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_gblinfo2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlGblinfo2Spec;
impl crate::RegisterSpec for FlashctlGblinfo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_gblinfo2::R`](R) reader structure"]
impl crate::Readable for FlashctlGblinfo2Spec {}
#[doc = "`reset()` method sets FLASHCTL_GBLINFO2 to value 0"]
impl crate::Resettable for FlashctlGblinfo2Spec {}
