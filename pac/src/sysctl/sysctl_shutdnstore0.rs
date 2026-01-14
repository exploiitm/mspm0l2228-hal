#[doc = "Register `SYSCTL_SHUTDNSTORE0` reader"]
pub type R = crate::R<SysctlShutdnstore0Spec>;
#[doc = "Register `SYSCTL_SHUTDNSTORE0` writer"]
pub type W = crate::W<SysctlShutdnstore0Spec>;
#[doc = "Field `DATA` reader - Shutdown storage byte 0"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Shutdown storage byte 0"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 0"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 0"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SysctlShutdnstore0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_shutdnstore0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shutdnstore0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlShutdnstore0Spec;
impl crate::RegisterSpec for SysctlShutdnstore0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_shutdnstore0::R`](R) reader structure"]
impl crate::Readable for SysctlShutdnstore0Spec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_shutdnstore0::W`](W) writer structure"]
impl crate::Writable for SysctlShutdnstore0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SHUTDNSTORE0 to value 0"]
impl crate::Resettable for SysctlShutdnstore0Spec {}
