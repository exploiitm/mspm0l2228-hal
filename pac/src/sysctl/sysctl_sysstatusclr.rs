#[doc = "Register `SYSCTL_SYSSTATUSCLR` writer"]
pub type W = crate::W<SysctlSysstatusclrSpec>;
#[doc = "Set ALLECC to clear all ECC related SYSSTATUS indicators.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allecc {
    #[doc = "1: Clear ECC error state"]
    Clear = 1,
}
impl From<Allecc> for bool {
    #[inline(always)]
    fn from(variant: Allecc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLECC` writer - Set ALLECC to clear all ECC related SYSSTATUS indicators."]
pub type AlleccW<'a, REG> = crate::BitWriter<'a, REG, Allecc>;
impl<'a, REG> AlleccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear ECC error state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Allecc::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Set ALLECC to clear all ECC related SYSSTATUS indicators."]
    #[inline(always)]
    pub fn allecc(&mut self) -> AlleccW<'_, SysctlSysstatusclrSpec> {
        AlleccW::new(self, 0)
    }
}
#[doc = "Clear sticky bits of SYSSTATUS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysstatusclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSysstatusclrSpec;
impl crate::RegisterSpec for SysctlSysstatusclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_sysstatusclr::W`](W) writer structure"]
impl crate::Writable for SysctlSysstatusclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SYSSTATUSCLR to value 0"]
impl crate::Resettable for SysctlSysstatusclrSpec {}
