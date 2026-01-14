#[doc = "Register `SYSCTL_LFCLKCFG` reader"]
pub type R = crate::R<SysctlLfclkcfgSpec>;
#[doc = "Register `SYSCTL_LFCLKCFG` writer"]
pub type W = crate::W<SysctlLfclkcfgSpec>;
#[doc = "XT1DRIVE selects the low frequency crystal oscillator (LFXT) drive strength.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xt1drive {
    #[doc = "0: Lowest drive and current"]
    Lowestdrv = 0,
    #[doc = "1: Lower drive and current"]
    Lowerdrv = 1,
    #[doc = "2: Higher drive and current"]
    Higherdrv = 2,
    #[doc = "3: Highest drive and current"]
    Highestdrv = 3,
}
impl From<Xt1drive> for u8 {
    #[inline(always)]
    fn from(variant: Xt1drive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xt1drive {
    type Ux = u8;
}
impl crate::IsEnum for Xt1drive {}
#[doc = "Field `XT1DRIVE` reader - XT1DRIVE selects the low frequency crystal oscillator (LFXT) drive strength."]
pub type Xt1driveR = crate::FieldReader<Xt1drive>;
impl Xt1driveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1drive {
        match self.bits {
            0 => Xt1drive::Lowestdrv,
            1 => Xt1drive::Lowerdrv,
            2 => Xt1drive::Higherdrv,
            3 => Xt1drive::Highestdrv,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest drive and current"]
    #[inline(always)]
    pub fn is_lowestdrv(&self) -> bool {
        *self == Xt1drive::Lowestdrv
    }
    #[doc = "Lower drive and current"]
    #[inline(always)]
    pub fn is_lowerdrv(&self) -> bool {
        *self == Xt1drive::Lowerdrv
    }
    #[doc = "Higher drive and current"]
    #[inline(always)]
    pub fn is_higherdrv(&self) -> bool {
        *self == Xt1drive::Higherdrv
    }
    #[doc = "Highest drive and current"]
    #[inline(always)]
    pub fn is_highestdrv(&self) -> bool {
        *self == Xt1drive::Highestdrv
    }
}
#[doc = "Field `XT1DRIVE` writer - XT1DRIVE selects the low frequency crystal oscillator (LFXT) drive strength."]
pub type Xt1driveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xt1drive, crate::Safe>;
impl<'a, REG> Xt1driveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest drive and current"]
    #[inline(always)]
    pub fn lowestdrv(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Lowestdrv)
    }
    #[doc = "Lower drive and current"]
    #[inline(always)]
    pub fn lowerdrv(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Lowerdrv)
    }
    #[doc = "Higher drive and current"]
    #[inline(always)]
    pub fn higherdrv(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Higherdrv)
    }
    #[doc = "Highest drive and current"]
    #[inline(always)]
    pub fn highestdrv(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Highestdrv)
    }
}
#[doc = "MONITOR enables or disables the LFCLK monitor, which continuously checks LFXT or LFCLK_IN for a clock stuck fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monitor {
    #[doc = "0: Clock monitor is disabled"]
    Disable = 0,
    #[doc = "1: Clock monitor is enabled"]
    Enable = 1,
}
impl From<Monitor> for bool {
    #[inline(always)]
    fn from(variant: Monitor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONITOR` reader - MONITOR enables or disables the LFCLK monitor, which continuously checks LFXT or LFCLK_IN for a clock stuck fault."]
pub type MonitorR = crate::BitReader<Monitor>;
impl MonitorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monitor {
        match self.bits {
            false => Monitor::Disable,
            true => Monitor::Enable,
        }
    }
    #[doc = "Clock monitor is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Monitor::Disable
    }
    #[doc = "Clock monitor is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Monitor::Enable
    }
}
#[doc = "Field `MONITOR` writer - MONITOR enables or disables the LFCLK monitor, which continuously checks LFXT or LFCLK_IN for a clock stuck fault."]
pub type MonitorW<'a, REG> = crate::BitWriter<'a, REG, Monitor>;
impl<'a, REG> MonitorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock monitor is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Monitor::Disable)
    }
    #[doc = "Clock monitor is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Monitor::Enable)
    }
}
#[doc = "LOWCAP controls the low-power LFXT mode. When the LFXT load capacitance is less than 3pf, LOWCAP may be set for reduced power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowcap {
    #[doc = "0: LFXT low capacitance mode is disabled"]
    Disable = 0,
    #[doc = "1: LFXT low capacitance mode is enabled"]
    Enable = 1,
}
impl From<Lowcap> for bool {
    #[inline(always)]
    fn from(variant: Lowcap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWCAP` reader - LOWCAP controls the low-power LFXT mode. When the LFXT load capacitance is less than 3pf, LOWCAP may be set for reduced power consumption."]
pub type LowcapR = crate::BitReader<Lowcap>;
impl LowcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowcap {
        match self.bits {
            false => Lowcap::Disable,
            true => Lowcap::Enable,
        }
    }
    #[doc = "LFXT low capacitance mode is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lowcap::Disable
    }
    #[doc = "LFXT low capacitance mode is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lowcap::Enable
    }
}
#[doc = "Field `LOWCAP` writer - LOWCAP controls the low-power LFXT mode. When the LFXT load capacitance is less than 3pf, LOWCAP may be set for reduced power consumption."]
pub type LowcapW<'a, REG> = crate::BitWriter<'a, REG, Lowcap>;
impl<'a, REG> LowcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXT low capacitance mode is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lowcap::Disable)
    }
    #[doc = "LFXT low capacitance mode is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lowcap::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - XT1DRIVE selects the low frequency crystal oscillator (LFXT) drive strength."]
    #[inline(always)]
    pub fn xt1drive(&self) -> Xt1driveR {
        Xt1driveR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - MONITOR enables or disables the LFCLK monitor, which continuously checks LFXT or LFCLK_IN for a clock stuck fault."]
    #[inline(always)]
    pub fn monitor(&self) -> MonitorR {
        MonitorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LOWCAP controls the low-power LFXT mode. When the LFXT load capacitance is less than 3pf, LOWCAP may be set for reduced power consumption."]
    #[inline(always)]
    pub fn lowcap(&self) -> LowcapR {
        LowcapR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - XT1DRIVE selects the low frequency crystal oscillator (LFXT) drive strength."]
    #[inline(always)]
    pub fn xt1drive(&mut self) -> Xt1driveW<'_, SysctlLfclkcfgSpec> {
        Xt1driveW::new(self, 0)
    }
    #[doc = "Bit 4 - MONITOR enables or disables the LFCLK monitor, which continuously checks LFXT or LFCLK_IN for a clock stuck fault."]
    #[inline(always)]
    pub fn monitor(&mut self) -> MonitorW<'_, SysctlLfclkcfgSpec> {
        MonitorW::new(self, 4)
    }
    #[doc = "Bit 8 - LOWCAP controls the low-power LFXT mode. When the LFXT load capacitance is less than 3pf, LOWCAP may be set for reduced power consumption."]
    #[inline(always)]
    pub fn lowcap(&mut self) -> LowcapW<'_, SysctlLfclkcfgSpec> {
        LowcapW::new(self, 8)
    }
}
#[doc = "Low frequency crystal oscillator (LFXT) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_lfclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_lfclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlLfclkcfgSpec;
impl crate::RegisterSpec for SysctlLfclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_lfclkcfg::R`](R) reader structure"]
impl crate::Readable for SysctlLfclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_lfclkcfg::W`](W) writer structure"]
impl crate::Writable for SysctlLfclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_LFCLKCFG to value 0x03"]
impl crate::Resettable for SysctlLfclkcfgSpec {
    const RESET_VALUE: u32 = 0x03;
}
