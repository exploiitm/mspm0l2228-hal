#[doc = "Register `SYSCTL_SRAMBOUNDARY` reader"]
pub type R = crate::R<SysctlSramboundarySpec>;
#[doc = "Register `SYSCTL_SRAMBOUNDARY` writer"]
pub type W = crate::W<SysctlSramboundarySpec>;
#[doc = "Field `ADDR` reader - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 5:19 - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 5) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:19 - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SysctlSramboundarySpec> {
        AddrW::new(self, 5)
    }
}
#[doc = "SRAM Write Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sramboundary::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sramboundary::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSramboundarySpec;
impl crate::RegisterSpec for SysctlSramboundarySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_sramboundary::R`](R) reader structure"]
impl crate::Readable for SysctlSramboundarySpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_sramboundary::W`](W) writer structure"]
impl crate::Writable for SysctlSramboundarySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SRAMBOUNDARY to value 0"]
impl crate::Resettable for SysctlSramboundarySpec {}
