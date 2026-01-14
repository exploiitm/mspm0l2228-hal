#[doc = "Register `SYSCTL_FIPPROTMAINSTART` reader"]
pub type R = crate::R<SysctlFipprotmainstartSpec>;
#[doc = "Register `SYSCTL_FIPPROTMAINSTART` writer"]
pub type W = crate::W<SysctlFipprotmainstartSpec>;
#[doc = "Field `ADDR` reader - Flash IP Protection Start Address 64B granularity"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Flash IP Protection Start Address 64B granularity"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 6:21 - Flash IP Protection Start Address 64B granularity"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 6) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:21 - Flash IP Protection Start Address 64B granularity"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SysctlFipprotmainstartSpec> {
        AddrW::new(self, 6)
    }
}
#[doc = "Flash IP Protection Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fipprotmainstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fipprotmainstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFipprotmainstartSpec;
impl crate::RegisterSpec for SysctlFipprotmainstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_fipprotmainstart::R`](R) reader structure"]
impl crate::Readable for SysctlFipprotmainstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_fipprotmainstart::W`](W) writer structure"]
impl crate::Writable for SysctlFipprotmainstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FIPPROTMAINSTART to value 0"]
impl crate::Resettable for SysctlFipprotmainstartSpec {}
