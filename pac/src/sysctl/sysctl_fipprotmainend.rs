#[doc = "Register `SYSCTL_FIPPROTMAINEND` reader"]
pub type R = crate::R<SysctlFipprotmainendSpec>;
#[doc = "Register `SYSCTL_FIPPROTMAINEND` writer"]
pub type W = crate::W<SysctlFipprotmainendSpec>;
#[doc = "Field `ADDR` reader - Flash IP Protection End Address 64B granularity"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Flash IP Protection End Address 64B granularity"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 6:21 - Flash IP Protection End Address 64B granularity"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 6) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:21 - Flash IP Protection End Address 64B granularity"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SysctlFipprotmainendSpec> {
        AddrW::new(self, 6)
    }
}
#[doc = "Flash IP Protection End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fipprotmainend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fipprotmainend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFipprotmainendSpec;
impl crate::RegisterSpec for SysctlFipprotmainendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_fipprotmainend::R`](R) reader structure"]
impl crate::Readable for SysctlFipprotmainendSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_fipprotmainend::W`](W) writer structure"]
impl crate::Writable for SysctlFipprotmainendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FIPPROTMAINEND to value 0"]
impl crate::Resettable for SysctlFipprotmainendSpec {}
