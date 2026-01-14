#[doc = "Register `SYSCTL_EXRSTPIN` writer"]
pub type W = crate::W<SysctlExrstpinSpec>;
#[doc = "Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disable {
    #[doc = "0: Reset function of NRST pin is enabled"]
    False = 0,
    #[doc = "1: Reset function of NRST pin is disabled"]
    True = 1,
}
impl From<Disable> for bool {
    #[inline(always)]
    fn from(variant: Disable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE` writer - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG, Disable>;
impl<'a, REG> DisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset function of NRST pin is enabled"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::False)
    }
    #[doc = "Reset function of NRST pin is disabled"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, SysctlExrstpinSpec> {
        DisableW::new(self, 0)
    }
}
#[doc = "Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_exrstpin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlExrstpinSpec;
impl crate::RegisterSpec for SysctlExrstpinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_exrstpin::W`](W) writer structure"]
impl crate::Writable for SysctlExrstpinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_EXRSTPIN to value 0"]
impl crate::Resettable for SysctlExrstpinSpec {}
