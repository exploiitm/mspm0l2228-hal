#[doc = "Register `SYSCTL_FWEPROTMAIN` reader"]
pub type R = crate::R<SysctlFweprotmainSpec>;
#[doc = "Register `SYSCTL_FWEPROTMAIN` writer"]
pub type W = crate::W<SysctlFweprotmainSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "1 Sector Write-Erase per bit starting at address 0x0 of flash\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fweprotmain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fweprotmain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFweprotmainSpec;
impl crate::RegisterSpec for SysctlFweprotmainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_fweprotmain::R`](R) reader structure"]
impl crate::Readable for SysctlFweprotmainSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_fweprotmain::W`](W) writer structure"]
impl crate::Writable for SysctlFweprotmainSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FWEPROTMAIN to value 0"]
impl crate::Resettable for SysctlFweprotmainSpec {}
