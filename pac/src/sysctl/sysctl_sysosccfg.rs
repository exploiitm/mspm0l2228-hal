#[doc = "Register `SYSCTL_SYSOSCCFG` reader"]
pub type R = crate::R<SysctlSysosccfgSpec>;
#[doc = "Register `SYSCTL_SYSOSCCFG` writer"]
pub type W = crate::W<SysctlSysosccfgSpec>;
#[doc = "Target operating frequency for the system oscillator (SYSOSC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freq {
    #[doc = "0: Base frequency (32MHz)"]
    Sysoscbase = 0,
    #[doc = "1: Low frequency (4MHz)"]
    Sysosc4m = 1,
    #[doc = "2: User-trimmed frequency (16 or 24 MHz)"]
    Sysoscuser = 2,
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
#[doc = "Field `FREQ` reader - Target operating frequency for the system oscillator (SYSOSC)"]
pub type FreqR = crate::FieldReader<Freq>;
impl FreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freq> {
        match self.bits {
            0 => Some(Freq::Sysoscbase),
            1 => Some(Freq::Sysosc4m),
            2 => Some(Freq::Sysoscuser),
            _ => None,
        }
    }
    #[doc = "Base frequency (32MHz)"]
    #[inline(always)]
    pub fn is_sysoscbase(&self) -> bool {
        *self == Freq::Sysoscbase
    }
    #[doc = "Low frequency (4MHz)"]
    #[inline(always)]
    pub fn is_sysosc4m(&self) -> bool {
        *self == Freq::Sysosc4m
    }
    #[doc = "User-trimmed frequency (16 or 24 MHz)"]
    #[inline(always)]
    pub fn is_sysoscuser(&self) -> bool {
        *self == Freq::Sysoscuser
    }
}
#[doc = "Field `FREQ` writer - Target operating frequency for the system oscillator (SYSOSC)"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freq>;
impl<'a, REG> FreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Base frequency (32MHz)"]
    #[inline(always)]
    pub fn sysoscbase(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sysoscbase)
    }
    #[doc = "Low frequency (4MHz)"]
    #[inline(always)]
    pub fn sysosc4m(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sysosc4m)
    }
    #[doc = "User-trimmed frequency (16 or 24 MHz)"]
    #[inline(always)]
    pub fn sysoscuser(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sysoscuser)
    }
}
#[doc = "USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Use4mhzstop {
    #[doc = "0: Do not gear shift the SYSOSC to 4MHz in STOP mode"]
    Disable = 0,
    #[doc = "1: Gear shift SYSOSC to 4MHz in STOP mode"]
    Enable = 1,
}
impl From<Use4mhzstop> for bool {
    #[inline(always)]
    fn from(variant: Use4mhzstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE4MHZSTOP` reader - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
pub type Use4mhzstopR = crate::BitReader<Use4mhzstop>;
impl Use4mhzstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Use4mhzstop {
        match self.bits {
            false => Use4mhzstop::Disable,
            true => Use4mhzstop::Enable,
        }
    }
    #[doc = "Do not gear shift the SYSOSC to 4MHz in STOP mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Use4mhzstop::Disable
    }
    #[doc = "Gear shift SYSOSC to 4MHz in STOP mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Use4mhzstop::Enable
    }
}
#[doc = "Field `USE4MHZSTOP` writer - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
pub type Use4mhzstopW<'a, REG> = crate::BitWriter<'a, REG, Use4mhzstop>;
impl<'a, REG> Use4mhzstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not gear shift the SYSOSC to 4MHz in STOP mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Use4mhzstop::Disable)
    }
    #[doc = "Gear shift SYSOSC to 4MHz in STOP mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Use4mhzstop::Enable)
    }
}
#[doc = "DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disablestop {
    #[doc = "0: Do not disable SYSOSC in STOP mode"]
    Disable = 0,
    #[doc = "1: Disable SYSOSC in STOP mode and source ULPCLK from LFCLK"]
    Enable = 1,
}
impl From<Disablestop> for bool {
    #[inline(always)]
    fn from(variant: Disablestop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLESTOP` reader - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
pub type DisablestopR = crate::BitReader<Disablestop>;
impl DisablestopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disablestop {
        match self.bits {
            false => Disablestop::Disable,
            true => Disablestop::Enable,
        }
    }
    #[doc = "Do not disable SYSOSC in STOP mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Disablestop::Disable
    }
    #[doc = "Disable SYSOSC in STOP mode and source ULPCLK from LFCLK"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Disablestop::Enable
    }
}
#[doc = "Field `DISABLESTOP` writer - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
pub type DisablestopW<'a, REG> = crate::BitWriter<'a, REG, Disablestop>;
impl<'a, REG> DisablestopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not disable SYSOSC in STOP mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Disablestop::Disable)
    }
    #[doc = "Disable SYSOSC in STOP mode and source ULPCLK from LFCLK"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Disablestop::Enable)
    }
}
#[doc = "DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disable {
    #[doc = "0: Do not disable SYSOSC"]
    Disable = 0,
    #[doc = "1: Disable SYSOSC immediately and source MCLK and ULPCLK from LFCLK"]
    Enable = 1,
}
impl From<Disable> for bool {
    #[inline(always)]
    fn from(variant: Disable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE` reader - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
pub type DisableR = crate::BitReader<Disable>;
impl DisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disable {
        match self.bits {
            false => Disable::Disable,
            true => Disable::Enable,
        }
    }
    #[doc = "Do not disable SYSOSC"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Disable::Disable
    }
    #[doc = "Disable SYSOSC immediately and source MCLK and ULPCLK from LFCLK"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Disable::Enable
    }
}
#[doc = "Field `DISABLE` writer - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG, Disable>;
impl<'a, REG> DisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not disable SYSOSC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::Disable)
    }
    #[doc = "Disable SYSOSC immediately and source MCLK and ULPCLK from LFCLK"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::Enable)
    }
}
#[doc = "BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockasyncall {
    #[doc = "0: Asynchronous fast clock requests are controlled by the requesting peripheral"]
    Disable = 0,
    #[doc = "1: All asynchronous fast clock requests are blocked"]
    Enable = 1,
}
impl From<Blockasyncall> for bool {
    #[inline(always)]
    fn from(variant: Blockasyncall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKASYNCALL` reader - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
pub type BlockasyncallR = crate::BitReader<Blockasyncall>;
impl BlockasyncallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockasyncall {
        match self.bits {
            false => Blockasyncall::Disable,
            true => Blockasyncall::Enable,
        }
    }
    #[doc = "Asynchronous fast clock requests are controlled by the requesting peripheral"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blockasyncall::Disable
    }
    #[doc = "All asynchronous fast clock requests are blocked"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blockasyncall::Enable
    }
}
#[doc = "Field `BLOCKASYNCALL` writer - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
pub type BlockasyncallW<'a, REG> = crate::BitWriter<'a, REG, Blockasyncall>;
impl<'a, REG> BlockasyncallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous fast clock requests are controlled by the requesting peripheral"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasyncall::Disable)
    }
    #[doc = "All asynchronous fast clock requests are blocked"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasyncall::Enable)
    }
}
#[doc = "FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fastcpuevent {
    #[doc = "0: An interrupt to the CPU will not assert a fast clock request"]
    Disable = 0,
    #[doc = "1: An interrupt to the CPU will assert a fast clock request"]
    Enable = 1,
}
impl From<Fastcpuevent> for bool {
    #[inline(always)]
    fn from(variant: Fastcpuevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTCPUEVENT` reader - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
pub type FastcpueventR = crate::BitReader<Fastcpuevent>;
impl FastcpueventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastcpuevent {
        match self.bits {
            false => Fastcpuevent::Disable,
            true => Fastcpuevent::Enable,
        }
    }
    #[doc = "An interrupt to the CPU will not assert a fast clock request"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fastcpuevent::Disable
    }
    #[doc = "An interrupt to the CPU will assert a fast clock request"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fastcpuevent::Enable
    }
}
#[doc = "Field `FASTCPUEVENT` writer - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
pub type FastcpueventW<'a, REG> = crate::BitWriter<'a, REG, Fastcpuevent>;
impl<'a, REG> FastcpueventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An interrupt to the CPU will not assert a fast clock request"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fastcpuevent::Disable)
    }
    #[doc = "An interrupt to the CPU will assert a fast clock request"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fastcpuevent::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Target operating frequency for the system oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
    #[inline(always)]
    pub fn use4mhzstop(&self) -> Use4mhzstopR {
        Use4mhzstopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
    #[inline(always)]
    pub fn disablestop(&self) -> DisablestopR {
        DisablestopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
    #[inline(always)]
    pub fn blockasyncall(&self) -> BlockasyncallR {
        BlockasyncallR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
    #[inline(always)]
    pub fn fastcpuevent(&self) -> FastcpueventR {
        FastcpueventR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Target operating frequency for the system oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, SysctlSysosccfgSpec> {
        FreqW::new(self, 0)
    }
    #[doc = "Bit 8 - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
    #[inline(always)]
    pub fn use4mhzstop(&mut self) -> Use4mhzstopW<'_, SysctlSysosccfgSpec> {
        Use4mhzstopW::new(self, 8)
    }
    #[doc = "Bit 9 - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
    #[inline(always)]
    pub fn disablestop(&mut self) -> DisablestopW<'_, SysctlSysosccfgSpec> {
        DisablestopW::new(self, 9)
    }
    #[doc = "Bit 10 - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, SysctlSysosccfgSpec> {
        DisableW::new(self, 10)
    }
    #[doc = "Bit 16 - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
    #[inline(always)]
    pub fn blockasyncall(&mut self) -> BlockasyncallW<'_, SysctlSysosccfgSpec> {
        BlockasyncallW::new(self, 16)
    }
    #[doc = "Bit 17 - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
    #[inline(always)]
    pub fn fastcpuevent(&mut self) -> FastcpueventW<'_, SysctlSysosccfgSpec> {
        FastcpueventW::new(self, 17)
    }
}
#[doc = "SYSOSC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sysosccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysosccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSysosccfgSpec;
impl crate::RegisterSpec for SysctlSysosccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_sysosccfg::R`](R) reader structure"]
impl crate::Readable for SysctlSysosccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_sysosccfg::W`](W) writer structure"]
impl crate::Writable for SysctlSysosccfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SYSOSCCFG to value 0x0002_0000"]
impl crate::Resettable for SysctlSysosccfgSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
