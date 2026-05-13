#[doc = "Register `SYSCTL_INITDONE` writer"]
pub type W = crate::W<SysctlInitdoneSpec>;
#[doc = "INITCODE writes 1 for PASS, left unwritten a timeout will occur if not blocked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    #[doc = "1: INITCODE PASS"]
    True = 1,
}
impl From<Pass> for bool {
    #[inline(always)]
    fn from(variant: Pass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PASS` writer - INITCODE writes 1 for PASS, left unwritten a timeout will occur if not blocked"]
pub type PassW<'a, REG> = crate::BitWriter<'a, REG, Pass>;
impl<'a, REG> PassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INITCODE PASS"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Pass::True)
    }
}
impl W {
    #[doc = "Bit 0 - INITCODE writes 1 for PASS, left unwritten a timeout will occur if not blocked"]
    #[inline(always)]
    pub fn pass(&mut self) -> PassW<'_, SysctlInitdoneSpec> {
        PassW::new(self, 0)
    }
}
#[doc = "INITCODE PASS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_initdone::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlInitdoneSpec;
impl crate::RegisterSpec for SysctlInitdoneSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_initdone::W`](W) writer structure"]
impl crate::Writable for SysctlInitdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_INITDONE to value 0"]
impl crate::Resettable for SysctlInitdoneSpec {}
