#[doc = "Register `SYSCTL_FRXPROTMAINEND` reader"]
pub type R = crate::R<SysctlFrxprotmainendSpec>;
#[doc = "Register `SYSCTL_FRXPROTMAINEND` writer"]
pub type W = crate::W<SysctlFrxprotmainendSpec>;
#[doc = "Field `ADDR` reader - Flash RX Protection End Address 64B granularity"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Flash RX Protection End Address 64B granularity"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 6:21 - Flash RX Protection End Address 64B granularity"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 6) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:21 - Flash RX Protection End Address 64B granularity"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SysctlFrxprotmainendSpec> {
        AddrW::new(self, 6)
    }
}
#[doc = "Flash RX Protection End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_frxprotmainend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_frxprotmainend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFrxprotmainendSpec;
impl crate::RegisterSpec for SysctlFrxprotmainendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_frxprotmainend::R`](R) reader structure"]
impl crate::Readable for SysctlFrxprotmainendSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_frxprotmainend::W`](W) writer structure"]
impl crate::Writable for SysctlFrxprotmainendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FRXPROTMAINEND to value 0"]
impl crate::Resettable for SysctlFrxprotmainendSpec {}
