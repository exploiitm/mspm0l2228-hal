#[doc = "Register `SYSCTL_FLBANKSWP` writer"]
pub type W = crate::W<SysctlFlbankswpSpec>;
#[doc = "1: Use Upper Bank as Logical 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Useupper {
    #[doc = "0: Normal (default) memory map addressing scheme"]
    Disable = 0,
    #[doc = "1: Flash upper region address space swapped with lower region"]
    Enable = 1,
}
impl From<Useupper> for bool {
    #[inline(always)]
    fn from(variant: Useupper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEUPPER` writer - 1: Use Upper Bank as Logical 0"]
pub type UseupperW<'a, REG> = crate::BitWriter<'a, REG, Useupper>;
impl<'a, REG> UseupperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal (default) memory map addressing scheme"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Useupper::Disable)
    }
    #[doc = "Flash upper region address space swapped with lower region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Useupper::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Use Upper Bank as Logical 0"]
    #[inline(always)]
    pub fn useupper(&mut self) -> UseupperW<'_, SysctlFlbankswpSpec> {
        UseupperW::new(self, 0)
    }
}
#[doc = "Flash MAIN bank address swap\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_flbankswp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFlbankswpSpec;
impl crate::RegisterSpec for SysctlFlbankswpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_flbankswp::W`](W) writer structure"]
impl crate::Writable for SysctlFlbankswpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FLBANKSWP to value 0"]
impl crate::Resettable for SysctlFlbankswpSpec {}
