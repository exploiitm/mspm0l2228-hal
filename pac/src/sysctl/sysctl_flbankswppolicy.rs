#[doc = "Register `SYSCTL_FLBANKSWPPOLICY` writer"]
pub type W = crate::W<SysctlFlbankswppolicySpec>;
#[doc = "1: Disables Policy To Allow Flash Bank Swapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disable {
    #[doc = "1: Disallow Bank Swap"]
    True = 1,
}
impl From<Disable> for bool {
    #[inline(always)]
    fn from(variant: Disable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE` writer - 1: Disables Policy To Allow Flash Bank Swapping"]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG, Disable>;
impl<'a, REG> DisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disallow Bank Swap"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::True)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Disables Policy To Allow Flash Bank Swapping"]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, SysctlFlbankswppolicySpec> {
        DisableW::new(self, 0)
    }
}
#[doc = "Flash Bank Swap Policy\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_flbankswppolicy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFlbankswppolicySpec;
impl crate::RegisterSpec for SysctlFlbankswppolicySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_flbankswppolicy::W`](W) writer structure"]
impl crate::Writable for SysctlFlbankswppolicySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FLBANKSWPPOLICY to value 0"]
impl crate::Resettable for SysctlFlbankswppolicySpec {}
