#[doc = "Register `SYSCTL_RESETLEVEL` reader"]
pub type R = crate::R<SysctlResetlevelSpec>;
#[doc = "Register `SYSCTL_RESETLEVEL` writer"]
pub type W = crate::W<SysctlResetlevelSpec>;
#[doc = "LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Level {
    #[doc = "0: Issue a SYSRST (CPU plus peripherals only)"]
    Cpu = 0,
    #[doc = "1: Issue a BOOTRST (CPU, peripherals, and boot configuration routine)"]
    Boot = 1,
    #[doc = "2: Issue a SYSRST and enter the boot strap loader (BSL)"]
    Bootloaderentry = 2,
    #[doc = "3: Issue a power-on reset (POR)"]
    Por = 3,
    #[doc = "4: Issue a SYSRST and exit the boot strap loader (BSL)"]
    Bootloaderexit = 4,
}
impl From<Level> for u8 {
    #[inline(always)]
    fn from(variant: Level) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Level {
    type Ux = u8;
}
impl crate::IsEnum for Level {}
#[doc = "Field `LEVEL` reader - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
pub type LevelR = crate::FieldReader<Level>;
impl LevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Level> {
        match self.bits {
            0 => Some(Level::Cpu),
            1 => Some(Level::Boot),
            2 => Some(Level::Bootloaderentry),
            3 => Some(Level::Por),
            4 => Some(Level::Bootloaderexit),
            _ => None,
        }
    }
    #[doc = "Issue a SYSRST (CPU plus peripherals only)"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Level::Cpu
    }
    #[doc = "Issue a BOOTRST (CPU, peripherals, and boot configuration routine)"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == Level::Boot
    }
    #[doc = "Issue a SYSRST and enter the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn is_bootloaderentry(&self) -> bool {
        *self == Level::Bootloaderentry
    }
    #[doc = "Issue a power-on reset (POR)"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == Level::Por
    }
    #[doc = "Issue a SYSRST and exit the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn is_bootloaderexit(&self) -> bool {
        *self == Level::Bootloaderexit
    }
}
#[doc = "Field `LEVEL` writer - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Level>;
impl<'a, REG> LevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue a SYSRST (CPU plus peripherals only)"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Cpu)
    }
    #[doc = "Issue a BOOTRST (CPU, peripherals, and boot configuration routine)"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Boot)
    }
    #[doc = "Issue a SYSRST and enter the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn bootloaderentry(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Bootloaderentry)
    }
    #[doc = "Issue a power-on reset (POR)"]
    #[inline(always)]
    pub fn por(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Por)
    }
    #[doc = "Issue a SYSRST and exit the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn bootloaderexit(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Bootloaderexit)
    }
}
impl R {
    #[doc = "Bits 0:2 - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, SysctlResetlevelSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Reset level for application-triggered reset command\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_resetlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_resetlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlResetlevelSpec;
impl crate::RegisterSpec for SysctlResetlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_resetlevel::R`](R) reader structure"]
impl crate::Readable for SysctlResetlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_resetlevel::W`](W) writer structure"]
impl crate::Writable for SysctlResetlevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_RESETLEVEL to value 0"]
impl crate::Resettable for SysctlResetlevelSpec {}
