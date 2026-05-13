#[doc = "Register `SYSCTL_MCLKCFG` reader"]
pub type R = crate::R<SysctlMclkcfgSpec>;
#[doc = "Register `SYSCTL_MCLKCFG` writer"]
pub type W = crate::W<SysctlMclkcfgSpec>;
#[doc = "Field `MDIV` reader - MDIV may be used to divide the MCLK frequency when MCLK is sourced from SYSOSC. MDIV=0h corresponds to /1 (no divider). MDIV=1h corresponds to /2 (divide-by-2). MDIV=Fh corresponds to /16 (divide-by-16). MDIV may be set between /1 and /16 on an integer basis."]
pub type MdivR = crate::FieldReader;
#[doc = "Field `MDIV` writer - MDIV may be used to divide the MCLK frequency when MCLK is sourced from SYSOSC. MDIV=0h corresponds to /1 (no divider). MDIV=1h corresponds to /2 (divide-by-2). MDIV=Fh corresponds to /16 (divide-by-16). MDIV may be set between /1 and /16 on an integer basis."]
pub type MdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "FLASHWAIT specifies the number of flash wait states when MCLK is sourced from HSCLK. FLASHWAIT has no effect when MCLK is sourced from SYSOSC or LFCLK.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashwait {
    #[doc = "0: No flash wait states are applied"]
    Wait0 = 0,
    #[doc = "1: One flash wait state is applied"]
    Wait1 = 1,
    #[doc = "2: 2 flash wait states are applied"]
    Wait2 = 2,
}
impl From<Flashwait> for u8 {
    #[inline(always)]
    fn from(variant: Flashwait) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashwait {
    type Ux = u8;
}
impl crate::IsEnum for Flashwait {}
#[doc = "Field `FLASHWAIT` reader - FLASHWAIT specifies the number of flash wait states when MCLK is sourced from HSCLK. FLASHWAIT has no effect when MCLK is sourced from SYSOSC or LFCLK."]
pub type FlashwaitR = crate::FieldReader<Flashwait>;
impl FlashwaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flashwait> {
        match self.bits {
            0 => Some(Flashwait::Wait0),
            1 => Some(Flashwait::Wait1),
            2 => Some(Flashwait::Wait2),
            _ => None,
        }
    }
    #[doc = "No flash wait states are applied"]
    #[inline(always)]
    pub fn is_wait0(&self) -> bool {
        *self == Flashwait::Wait0
    }
    #[doc = "One flash wait state is applied"]
    #[inline(always)]
    pub fn is_wait1(&self) -> bool {
        *self == Flashwait::Wait1
    }
    #[doc = "2 flash wait states are applied"]
    #[inline(always)]
    pub fn is_wait2(&self) -> bool {
        *self == Flashwait::Wait2
    }
}
#[doc = "Field `FLASHWAIT` writer - FLASHWAIT specifies the number of flash wait states when MCLK is sourced from HSCLK. FLASHWAIT has no effect when MCLK is sourced from SYSOSC or LFCLK."]
pub type FlashwaitW<'a, REG> = crate::FieldWriter<'a, REG, 4, Flashwait>;
impl<'a, REG> FlashwaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No flash wait states are applied"]
    #[inline(always)]
    pub fn wait0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashwait::Wait0)
    }
    #[doc = "One flash wait state is applied"]
    #[inline(always)]
    pub fn wait1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashwait::Wait1)
    }
    #[doc = "2 flash wait states are applied"]
    #[inline(always)]
    pub fn wait2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashwait::Wait2)
    }
}
#[doc = "USEMFTICK specifies whether the 4MHz constant-rate clock (MFCLK) to peripherals is enabled or disabled. When enabled, MDIV must be disabled (set to 0h=/1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usemftick {
    #[doc = "0: The 4MHz rate MFCLK to peripherals is enabled"]
    Disable = 0,
    #[doc = "1: The 4MHz rate MFCLK to peripherals is enabled."]
    Enable = 1,
}
impl From<Usemftick> for bool {
    #[inline(always)]
    fn from(variant: Usemftick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEMFTICK` reader - USEMFTICK specifies whether the 4MHz constant-rate clock (MFCLK) to peripherals is enabled or disabled. When enabled, MDIV must be disabled (set to 0h=/1)."]
pub type UsemftickR = crate::BitReader<Usemftick>;
impl UsemftickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usemftick {
        match self.bits {
            false => Usemftick::Disable,
            true => Usemftick::Enable,
        }
    }
    #[doc = "The 4MHz rate MFCLK to peripherals is enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usemftick::Disable
    }
    #[doc = "The 4MHz rate MFCLK to peripherals is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usemftick::Enable
    }
}
#[doc = "Field `USEMFTICK` writer - USEMFTICK specifies whether the 4MHz constant-rate clock (MFCLK) to peripherals is enabled or disabled. When enabled, MDIV must be disabled (set to 0h=/1)."]
pub type UsemftickW<'a, REG> = crate::BitWriter<'a, REG, Usemftick>;
impl<'a, REG> UsemftickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 4MHz rate MFCLK to peripherals is enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usemftick::Disable)
    }
    #[doc = "The 4MHz rate MFCLK to peripherals is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usemftick::Enable)
    }
}
#[doc = "USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usehsclk {
    #[doc = "0: MCLK will not use the high speed clock (HSCLK)"]
    Disable = 0,
    #[doc = "1: MCLK will use the high speed clock (HSCLK) in RUN and SLEEP mode"]
    Enable = 1,
}
impl From<Usehsclk> for bool {
    #[inline(always)]
    fn from(variant: Usehsclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEHSCLK` reader - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
pub type UsehsclkR = crate::BitReader<Usehsclk>;
impl UsehsclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usehsclk {
        match self.bits {
            false => Usehsclk::Disable,
            true => Usehsclk::Enable,
        }
    }
    #[doc = "MCLK will not use the high speed clock (HSCLK)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usehsclk::Disable
    }
    #[doc = "MCLK will use the high speed clock (HSCLK) in RUN and SLEEP mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usehsclk::Enable
    }
}
#[doc = "Field `USEHSCLK` writer - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
pub type UsehsclkW<'a, REG> = crate::BitWriter<'a, REG, Usehsclk>;
impl<'a, REG> UsehsclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK will not use the high speed clock (HSCLK)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usehsclk::Disable)
    }
    #[doc = "MCLK will use the high speed clock (HSCLK) in RUN and SLEEP mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usehsclk::Enable)
    }
}
#[doc = "USELFCLK sets the MCLK source policy. Set USELFCLK to use LFCLK as the MCLK source. Note that setting USELFCLK does not disable SYSOSC, and SYSOSC remains available for direct use by analog modules.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselfclk {
    #[doc = "0: MCLK will not use the low frequency clock (LFCLK)"]
    Disable = 0,
    #[doc = "1: MCLK will use the low frequency clock (LFCLK)"]
    Enable = 1,
}
impl From<Uselfclk> for bool {
    #[inline(always)]
    fn from(variant: Uselfclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USELFCLK` reader - USELFCLK sets the MCLK source policy. Set USELFCLK to use LFCLK as the MCLK source. Note that setting USELFCLK does not disable SYSOSC, and SYSOSC remains available for direct use by analog modules."]
pub type UselfclkR = crate::BitReader<Uselfclk>;
impl UselfclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uselfclk {
        match self.bits {
            false => Uselfclk::Disable,
            true => Uselfclk::Enable,
        }
    }
    #[doc = "MCLK will not use the low frequency clock (LFCLK)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uselfclk::Disable
    }
    #[doc = "MCLK will use the low frequency clock (LFCLK)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uselfclk::Enable
    }
}
#[doc = "Field `USELFCLK` writer - USELFCLK sets the MCLK source policy. Set USELFCLK to use LFCLK as the MCLK source. Note that setting USELFCLK does not disable SYSOSC, and SYSOSC remains available for direct use by analog modules."]
pub type UselfclkW<'a, REG> = crate::BitWriter<'a, REG, Uselfclk>;
impl<'a, REG> UselfclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK will not use the low frequency clock (LFCLK)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uselfclk::Disable)
    }
    #[doc = "MCLK will use the low frequency clock (LFCLK)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uselfclk::Enable)
    }
}
#[doc = "STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopclkstby {
    #[doc = "0: ULPCLK/LFCLK runs to all PD0 peripherals in STANDBY mode"]
    Disable = 0,
    #[doc = "1: ULPCLK/LFCLK is disabled to all peripherals in STANDBY mode except TIMG0 and TIMG1"]
    Enable = 1,
}
impl From<Stopclkstby> for bool {
    #[inline(always)]
    fn from(variant: Stopclkstby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPCLKSTBY` reader - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
pub type StopclkstbyR = crate::BitReader<Stopclkstby>;
impl StopclkstbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopclkstby {
        match self.bits {
            false => Stopclkstby::Disable,
            true => Stopclkstby::Enable,
        }
    }
    #[doc = "ULPCLK/LFCLK runs to all PD0 peripherals in STANDBY mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stopclkstby::Disable
    }
    #[doc = "ULPCLK/LFCLK is disabled to all peripherals in STANDBY mode except TIMG0 and TIMG1"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stopclkstby::Enable
    }
}
#[doc = "Field `STOPCLKSTBY` writer - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
pub type StopclkstbyW<'a, REG> = crate::BitWriter<'a, REG, Stopclkstby>;
impl<'a, REG> StopclkstbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ULPCLK/LFCLK runs to all PD0 peripherals in STANDBY mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopclkstby::Disable)
    }
    #[doc = "ULPCLK/LFCLK is disabled to all peripherals in STANDBY mode except TIMG0 and TIMG1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopclkstby::Enable)
    }
}
#[doc = "MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkdeadchk {
    #[doc = "0: The MCLK dead check monitor is disabled"]
    Disable = 0,
    #[doc = "1: The MCLK dead check monitor is enabled"]
    Enable = 1,
}
impl From<Mclkdeadchk> for bool {
    #[inline(always)]
    fn from(variant: Mclkdeadchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKDEADCHK` reader - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
pub type MclkdeadchkR = crate::BitReader<Mclkdeadchk>;
impl MclkdeadchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkdeadchk {
        match self.bits {
            false => Mclkdeadchk::Disable,
            true => Mclkdeadchk::Enable,
        }
    }
    #[doc = "The MCLK dead check monitor is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mclkdeadchk::Disable
    }
    #[doc = "The MCLK dead check monitor is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mclkdeadchk::Enable
    }
}
#[doc = "Field `MCLKDEADCHK` writer - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
pub type MclkdeadchkW<'a, REG> = crate::BitWriter<'a, REG, Mclkdeadchk>;
impl<'a, REG> MclkdeadchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MCLK dead check monitor is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdeadchk::Disable)
    }
    #[doc = "The MCLK dead check monitor is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdeadchk::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - MDIV may be used to divide the MCLK frequency when MCLK is sourced from SYSOSC. MDIV=0h corresponds to /1 (no divider). MDIV=1h corresponds to /2 (divide-by-2). MDIV=Fh corresponds to /16 (divide-by-16). MDIV may be set between /1 and /16 on an integer basis."]
    #[inline(always)]
    pub fn mdiv(&self) -> MdivR {
        MdivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - FLASHWAIT specifies the number of flash wait states when MCLK is sourced from HSCLK. FLASHWAIT has no effect when MCLK is sourced from SYSOSC or LFCLK."]
    #[inline(always)]
    pub fn flashwait(&self) -> FlashwaitR {
        FlashwaitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - USEMFTICK specifies whether the 4MHz constant-rate clock (MFCLK) to peripherals is enabled or disabled. When enabled, MDIV must be disabled (set to 0h=/1)."]
    #[inline(always)]
    pub fn usemftick(&self) -> UsemftickR {
        UsemftickR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
    #[inline(always)]
    pub fn usehsclk(&self) -> UsehsclkR {
        UsehsclkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - USELFCLK sets the MCLK source policy. Set USELFCLK to use LFCLK as the MCLK source. Note that setting USELFCLK does not disable SYSOSC, and SYSOSC remains available for direct use by analog modules."]
    #[inline(always)]
    pub fn uselfclk(&self) -> UselfclkR {
        UselfclkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
    #[inline(always)]
    pub fn stopclkstby(&self) -> StopclkstbyR {
        StopclkstbyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
    #[inline(always)]
    pub fn mclkdeadchk(&self) -> MclkdeadchkR {
        MclkdeadchkR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MDIV may be used to divide the MCLK frequency when MCLK is sourced from SYSOSC. MDIV=0h corresponds to /1 (no divider). MDIV=1h corresponds to /2 (divide-by-2). MDIV=Fh corresponds to /16 (divide-by-16). MDIV may be set between /1 and /16 on an integer basis."]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MdivW<'_, SysctlMclkcfgSpec> {
        MdivW::new(self, 0)
    }
    #[doc = "Bits 8:11 - FLASHWAIT specifies the number of flash wait states when MCLK is sourced from HSCLK. FLASHWAIT has no effect when MCLK is sourced from SYSOSC or LFCLK."]
    #[inline(always)]
    pub fn flashwait(&mut self) -> FlashwaitW<'_, SysctlMclkcfgSpec> {
        FlashwaitW::new(self, 8)
    }
    #[doc = "Bit 12 - USEMFTICK specifies whether the 4MHz constant-rate clock (MFCLK) to peripherals is enabled or disabled. When enabled, MDIV must be disabled (set to 0h=/1)."]
    #[inline(always)]
    pub fn usemftick(&mut self) -> UsemftickW<'_, SysctlMclkcfgSpec> {
        UsemftickW::new(self, 12)
    }
    #[doc = "Bit 16 - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
    #[inline(always)]
    pub fn usehsclk(&mut self) -> UsehsclkW<'_, SysctlMclkcfgSpec> {
        UsehsclkW::new(self, 16)
    }
    #[doc = "Bit 20 - USELFCLK sets the MCLK source policy. Set USELFCLK to use LFCLK as the MCLK source. Note that setting USELFCLK does not disable SYSOSC, and SYSOSC remains available for direct use by analog modules."]
    #[inline(always)]
    pub fn uselfclk(&mut self) -> UselfclkW<'_, SysctlMclkcfgSpec> {
        UselfclkW::new(self, 20)
    }
    #[doc = "Bit 21 - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
    #[inline(always)]
    pub fn stopclkstby(&mut self) -> StopclkstbyW<'_, SysctlMclkcfgSpec> {
        StopclkstbyW::new(self, 21)
    }
    #[doc = "Bit 22 - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
    #[inline(always)]
    pub fn mclkdeadchk(&mut self) -> MclkdeadchkW<'_, SysctlMclkcfgSpec> {
        MclkdeadchkW::new(self, 22)
    }
}
#[doc = "Main clock (MCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlMclkcfgSpec;
impl crate::RegisterSpec for SysctlMclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_mclkcfg::R`](R) reader structure"]
impl crate::Readable for SysctlMclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_mclkcfg::W`](W) writer structure"]
impl crate::Writable for SysctlMclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_MCLKCFG to value 0x0200"]
impl crate::Resettable for SysctlMclkcfgSpec {
    const RESET_VALUE: u32 = 0x0200;
}
