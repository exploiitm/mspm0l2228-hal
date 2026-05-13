#[doc = "Register `SYSCTL_DEDERRADDR` reader"]
pub type R = crate::R<SysctlDederraddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Memory DED Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_dederraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlDederraddrSpec;
impl crate::RegisterSpec for SysctlDederraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_dederraddr::R`](R) reader structure"]
impl crate::Readable for SysctlDederraddrSpec {}
#[doc = "`reset()` method sets SYSCTL_DEDERRADDR to value 0"]
impl crate::Resettable for SysctlDederraddrSpec {}
