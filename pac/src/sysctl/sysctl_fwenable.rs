#[doc = "Register `SYSCTL_FWENABLE` writer"]
pub type W = crate::W<SysctlFwenableSpec>;
#[doc = "1: Flash Read Execute Protection Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flrxprot {
    #[doc = "1: Turn On Flash Read-eXecute Protection"]
    Enable = 1,
}
impl From<Flrxprot> for bool {
    #[inline(always)]
    fn from(variant: Flrxprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLRXPROT` writer - 1: Flash Read Execute Protection Active"]
pub type FlrxprotW<'a, REG> = crate::BitWriter<'a, REG, Flrxprot>;
impl<'a, REG> FlrxprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turn On Flash Read-eXecute Protection"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Flrxprot::Enable)
    }
}
#[doc = "1: Flash Read IP ProtectionActive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flipprot {
    #[doc = "1: Turn On Flash IP Protection"]
    Enable = 1,
}
impl From<Flipprot> for bool {
    #[inline(always)]
    fn from(variant: Flipprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIPPROT` writer - 1: Flash Read IP ProtectionActive"]
pub type FlipprotW<'a, REG> = crate::BitWriter<'a, REG, Flipprot>;
impl<'a, REG> FlipprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turn On Flash IP Protection"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Flipprot::Enable)
    }
}
#[doc = "1: Blocks Writes from Changing SRAMBOUNDARY MMR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramboundarylock {
    #[doc = "1: SRAMBOUNDARY MMR Locked"]
    Enable = 1,
}
impl From<Sramboundarylock> for bool {
    #[inline(always)]
    fn from(variant: Sramboundarylock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMBOUNDARYLOCK` writer - 1: Blocks Writes from Changing SRAMBOUNDARY MMR"]
pub type SramboundarylockW<'a, REG> = crate::BitWriter<'a, REG, Sramboundarylock>;
impl<'a, REG> SramboundarylockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMBOUNDARY MMR Locked"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sramboundarylock::Enable)
    }
}
impl W {
    #[doc = "Bit 4 - 1: Flash Read Execute Protection Active"]
    #[inline(always)]
    pub fn flrxprot(&mut self) -> FlrxprotW<'_, SysctlFwenableSpec> {
        FlrxprotW::new(self, 4)
    }
    #[doc = "Bit 6 - 1: Flash Read IP ProtectionActive"]
    #[inline(always)]
    pub fn flipprot(&mut self) -> FlipprotW<'_, SysctlFwenableSpec> {
        FlipprotW::new(self, 6)
    }
    #[doc = "Bit 8 - 1: Blocks Writes from Changing SRAMBOUNDARY MMR"]
    #[inline(always)]
    pub fn sramboundarylock(&mut self) -> SramboundarylockW<'_, SysctlFwenableSpec> {
        SramboundarylockW::new(self, 8)
    }
}
#[doc = "Security Firewall Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fwenable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlFwenableSpec;
impl crate::RegisterSpec for SysctlFwenableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_fwenable::W`](W) writer structure"]
impl crate::Writable for SysctlFwenableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_FWENABLE to value 0"]
impl crate::Resettable for SysctlFwenableSpec {}
