#[doc = "Register `SYSCTL_FCC` reader"]
pub type R = crate::R<SysctlFccSpec>;
#[doc = "Field `DATA` reader - Frequency clock counter (FCC) count value."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency clock counter (FCC) count value."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Frequency clock counter (FCC) count\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fcc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFccSpec;
impl crate::RegisterSpec for SysctlFccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_fcc::R`](R) reader structure"]
impl crate::Readable for SysctlFccSpec {}
#[doc = "`reset()` method sets SYSCTL_FCC to value 0"]
impl crate::Resettable for SysctlFccSpec {}
