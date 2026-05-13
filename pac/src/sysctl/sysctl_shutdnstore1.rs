#[doc = "Register `SYSCTL_SHUTDNSTORE1` reader"]
pub type R = crate::R<SysctlShutdnstore1Spec>;
#[doc = "Register `SYSCTL_SHUTDNSTORE1` writer"]
pub type W = crate::W<SysctlShutdnstore1Spec>;
#[doc = "Field `DATA` reader - Shutdown storage byte 1"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Shutdown storage byte 1"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 1"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 1"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SysctlShutdnstore1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_shutdnstore1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shutdnstore1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlShutdnstore1Spec;
impl crate::RegisterSpec for SysctlShutdnstore1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_shutdnstore1::R`](R) reader structure"]
impl crate::Readable for SysctlShutdnstore1Spec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_shutdnstore1::W`](W) writer structure"]
impl crate::Writable for SysctlShutdnstore1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SHUTDNSTORE1 to value 0"]
impl crate::Resettable for SysctlShutdnstore1Spec {}
