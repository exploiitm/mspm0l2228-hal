#[doc = "Register `SYSCTL_SYSOSCTRIMUSER` reader"]
pub type R = crate::R<SysctlSysosctrimuserSpec>;
#[doc = "Register `SYSCTL_SYSOSCTRIMUSER` writer"]
pub type W = crate::W<SysctlSysosctrimuserSpec>;
#[doc = "FREQ specifies the target user-trimmed frequency for SYSOSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freq {
    #[doc = "1: 16MHz user frequency"]
    Sysosc16m = 1,
    #[doc = "2: 24MHz user frequency"]
    Sysosc24m = 2,
}
impl From<Freq> for u8 {
    #[inline(always)]
    fn from(variant: Freq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freq {
    type Ux = u8;
}
impl crate::IsEnum for Freq {}
#[doc = "Field `FREQ` reader - FREQ specifies the target user-trimmed frequency for SYSOSC."]
pub type FreqR = crate::FieldReader<Freq>;
impl FreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freq> {
        match self.bits {
            1 => Some(Freq::Sysosc16m),
            2 => Some(Freq::Sysosc24m),
            _ => None,
        }
    }
    #[doc = "16MHz user frequency"]
    #[inline(always)]
    pub fn is_sysosc16m(&self) -> bool {
        *self == Freq::Sysosc16m
    }
    #[doc = "24MHz user frequency"]
    #[inline(always)]
    pub fn is_sysosc24m(&self) -> bool {
        *self == Freq::Sysosc24m
    }
}
#[doc = "Field `FREQ` writer - FREQ specifies the target user-trimmed frequency for SYSOSC."]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freq>;
impl<'a, REG> FreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16MHz user frequency"]
    #[inline(always)]
    pub fn sysosc16m(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sysosc16m)
    }
    #[doc = "24MHz user frequency"]
    #[inline(always)]
    pub fn sysosc24m(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sysosc24m)
    }
}
#[doc = "Field `CAP` reader - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
pub type CapR = crate::FieldReader;
#[doc = "Field `CAP` writer - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
pub type CapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESCOARSE` reader - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
pub type RescoarseR = crate::FieldReader;
#[doc = "Field `RESCOARSE` writer - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
pub type RescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESFINE` reader - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
pub type ResfineR = crate::FieldReader;
#[doc = "Field `RESFINE` writer - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
pub type ResfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDIV` reader - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
pub type RdivR = crate::FieldReader<u16>;
#[doc = "Field `RDIV` writer - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
pub type RdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - FREQ specifies the target user-trimmed frequency for SYSOSC."]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:13 - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn rescoarse(&self) -> RescoarseR {
        RescoarseR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn resfine(&self) -> ResfineR {
        ResfineR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:28 - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn rdiv(&self) -> RdivR {
        RdivR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - FREQ specifies the target user-trimmed frequency for SYSOSC."]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, SysctlSysosctrimuserSpec> {
        FreqW::new(self, 0)
    }
    #[doc = "Bits 4:6 - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn cap(&mut self) -> CapW<'_, SysctlSysosctrimuserSpec> {
        CapW::new(self, 4)
    }
    #[doc = "Bits 8:13 - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn rescoarse(&mut self) -> RescoarseW<'_, SysctlSysosctrimuserSpec> {
        RescoarseW::new(self, 8)
    }
    #[doc = "Bits 16:19 - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn resfine(&mut self) -> ResfineW<'_, SysctlSysosctrimuserSpec> {
        ResfineW::new(self, 16)
    }
    #[doc = "Bits 20:28 - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn rdiv(&mut self) -> RdivW<'_, SysctlSysosctrimuserSpec> {
        RdivW::new(self, 20)
    }
}
#[doc = "SYSOSC user-specified trim\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sysosctrimuser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysosctrimuser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSysosctrimuserSpec;
impl crate::RegisterSpec for SysctlSysosctrimuserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_sysosctrimuser::R`](R) reader structure"]
impl crate::Readable for SysctlSysosctrimuserSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_sysosctrimuser::W`](W) writer structure"]
impl crate::Writable for SysctlSysosctrimuserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SYSOSCTRIMUSER to value 0"]
impl crate::Resettable for SysctlSysosctrimuserSpec {}
