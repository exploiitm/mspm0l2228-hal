#[doc = "Register `SYSCTL_SWDCFG` writer"]
pub type W = crate::W<SysctlSwdcfgSpec>;
#[doc = "Set DISABLE to disable the SWD function on SWD pins, allowing the SWD pins to be used as GPIO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disable {
    #[doc = "1: Disable SWD function on SWD pins"]
    True = 1,
}
impl From<Disable> for bool {
    #[inline(always)]
    fn from(variant: Disable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE` writer - Set DISABLE to disable the SWD function on SWD pins, allowing the SWD pins to be used as GPIO."]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG, Disable>;
impl<'a, REG> DisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable SWD function on SWD pins"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set DISABLE to disable the SWD function on SWD pins, allowing the SWD pins to be used as GPIO."]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, SysctlSwdcfgSpec> {
        DisableW::new(self, 0)
    }
}
#[doc = "Disable the SWD function on the SWD pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_swdcfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSwdcfgSpec;
impl crate::RegisterSpec for SysctlSwdcfgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_swdcfg::W`](W) writer structure"]
impl crate::Writable for SysctlSwdcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SWDCFG to value 0"]
impl crate::Resettable for SysctlSwdcfgSpec {}
