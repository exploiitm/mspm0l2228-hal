#[doc = "Register `SYSCTL_SYSOSCFCLCTL` writer"]
pub type W = crate::W<SysctlSysoscfclctlSpec>;
#[doc = "Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setusefcl {
    #[doc = "1: Enable the SYSOSC FCL"]
    True = 1,
}
impl From<Setusefcl> for bool {
    #[inline(always)]
    fn from(variant: Setusefcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETUSEFCL` writer - Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST."]
pub type SetusefclW<'a, REG> = crate::BitWriter<'a, REG, Setusefcl>;
impl<'a, REG> SetusefclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the SYSOSC FCL"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Setusefcl::True)
    }
}
#[doc = "Set SETUSEEXRES to specify that an external resistor will be used for the FCL. An appropriate resistor must be populated on the ROSC pin. This state is locked until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setuseexres {
    #[doc = "1: Enable the SYSOSC external Resistor"]
    True = 1,
}
impl From<Setuseexres> for bool {
    #[inline(always)]
    fn from(variant: Setuseexres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETUSEEXRES` writer - Set SETUSEEXRES to specify that an external resistor will be used for the FCL. An appropriate resistor must be populated on the ROSC pin. This state is locked until the next BOOTRST."]
pub type SetuseexresW<'a, REG> = crate::BitWriter<'a, REG, Setuseexres>;
impl<'a, REG> SetuseexresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the SYSOSC external Resistor"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Setuseexres::True)
    }
}
impl W {
    #[doc = "Bit 0 - Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST."]
    #[inline(always)]
    pub fn setusefcl(&mut self) -> SetusefclW<'_, SysctlSysoscfclctlSpec> {
        SetusefclW::new(self, 0)
    }
    #[doc = "Bit 1 - Set SETUSEEXRES to specify that an external resistor will be used for the FCL. An appropriate resistor must be populated on the ROSC pin. This state is locked until the next BOOTRST."]
    #[inline(always)]
    pub fn setuseexres(&mut self) -> SetuseexresW<'_, SysctlSysoscfclctlSpec> {
        SetuseexresW::new(self, 1)
    }
}
#[doc = "SYSOSC frequency correction loop (FCL) ROSC enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysoscfclctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSysoscfclctlSpec;
impl crate::RegisterSpec for SysctlSysoscfclctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_sysoscfclctl::W`](W) writer structure"]
impl crate::Writable for SysctlSysoscfclctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SYSOSCFCLCTL to value 0"]
impl crate::Resettable for SysctlSysoscfclctlSpec {}
