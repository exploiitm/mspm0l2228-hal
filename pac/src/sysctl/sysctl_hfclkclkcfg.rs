#[doc = "Register `SYSCTL_HFCLKCLKCFG` reader"]
pub type R = crate::R<SysctlHfclkclkcfgSpec>;
#[doc = "Register `SYSCTL_HFCLKCLKCFG` writer"]
pub type W = crate::W<SysctlHfclkclkcfgSpec>;
#[doc = "HFXTTIME specifies the HFXT startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), HFXT will be checked after this time expires.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxttime {
    #[doc = "0: Minimum startup time (approximatly zero)"]
    Minstarttime = 0,
    #[doc = "255: Maximum startup time (approximatly 16.32ms)"]
    Maxstarttime = 255,
}
impl From<Hfxttime> for u8 {
    #[inline(always)]
    fn from(variant: Hfxttime) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxttime {
    type Ux = u8;
}
impl crate::IsEnum for Hfxttime {}
#[doc = "Field `HFXTTIME` reader - HFXTTIME specifies the HFXT startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), HFXT will be checked after this time expires."]
pub type HfxttimeR = crate::FieldReader<Hfxttime>;
impl HfxttimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfxttime> {
        match self.bits {
            0 => Some(Hfxttime::Minstarttime),
            255 => Some(Hfxttime::Maxstarttime),
            _ => None,
        }
    }
    #[doc = "Minimum startup time (approximatly zero)"]
    #[inline(always)]
    pub fn is_minstarttime(&self) -> bool {
        *self == Hfxttime::Minstarttime
    }
    #[doc = "Maximum startup time (approximatly 16.32ms)"]
    #[inline(always)]
    pub fn is_maxstarttime(&self) -> bool {
        *self == Hfxttime::Maxstarttime
    }
}
#[doc = "Field `HFXTTIME` writer - HFXTTIME specifies the HFXT startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), HFXT will be checked after this time expires."]
pub type HfxttimeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Hfxttime>;
impl<'a, REG> HfxttimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum startup time (approximatly zero)"]
    #[inline(always)]
    pub fn minstarttime(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxttime::Minstarttime)
    }
    #[doc = "Maximum startup time (approximatly 16.32ms)"]
    #[inline(always)]
    pub fn maxstarttime(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxttime::Maxstarttime)
    }
}
#[doc = "HFXT Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxtrsel {
    #[doc = "0: 4MHz &lt;= HFXT frequency &lt;= 8MHz"]
    Range4to8 = 0,
    #[doc = "1: 8MHz &lt; HFXT frequency &lt;= 16MHz"]
    Range8to16 = 1,
    #[doc = "2: 16MHz &lt; HFXT frequency &lt;= 32MHz"]
    Range16to32 = 2,
    #[doc = "3: 32MHz &lt; HFXT frequency &lt;= 48MHz"]
    Range32to48 = 3,
}
impl From<Hfxtrsel> for u8 {
    #[inline(always)]
    fn from(variant: Hfxtrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxtrsel {
    type Ux = u8;
}
impl crate::IsEnum for Hfxtrsel {}
#[doc = "Field `HFXTRSEL` reader - HFXT Range Select"]
pub type HfxtrselR = crate::FieldReader<Hfxtrsel>;
impl HfxtrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtrsel {
        match self.bits {
            0 => Hfxtrsel::Range4to8,
            1 => Hfxtrsel::Range8to16,
            2 => Hfxtrsel::Range16to32,
            3 => Hfxtrsel::Range32to48,
            _ => unreachable!(),
        }
    }
    #[doc = "4MHz &lt;= HFXT frequency &lt;= 8MHz"]
    #[inline(always)]
    pub fn is_range4to8(&self) -> bool {
        *self == Hfxtrsel::Range4to8
    }
    #[doc = "8MHz &lt; HFXT frequency &lt;= 16MHz"]
    #[inline(always)]
    pub fn is_range8to16(&self) -> bool {
        *self == Hfxtrsel::Range8to16
    }
    #[doc = "16MHz &lt; HFXT frequency &lt;= 32MHz"]
    #[inline(always)]
    pub fn is_range16to32(&self) -> bool {
        *self == Hfxtrsel::Range16to32
    }
    #[doc = "32MHz &lt; HFXT frequency &lt;= 48MHz"]
    #[inline(always)]
    pub fn is_range32to48(&self) -> bool {
        *self == Hfxtrsel::Range32to48
    }
}
#[doc = "Field `HFXTRSEL` writer - HFXT Range Select"]
pub type HfxtrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hfxtrsel, crate::Safe>;
impl<'a, REG> HfxtrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4MHz &lt;= HFXT frequency &lt;= 8MHz"]
    #[inline(always)]
    pub fn range4to8(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtrsel::Range4to8)
    }
    #[doc = "8MHz &lt; HFXT frequency &lt;= 16MHz"]
    #[inline(always)]
    pub fn range8to16(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtrsel::Range8to16)
    }
    #[doc = "16MHz &lt; HFXT frequency &lt;= 32MHz"]
    #[inline(always)]
    pub fn range16to32(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtrsel::Range16to32)
    }
    #[doc = "32MHz &lt; HFXT frequency &lt;= 48MHz"]
    #[inline(always)]
    pub fn range32to48(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtrsel::Range32to48)
    }
}
#[doc = "HFCLKFLTCHK enables or disables the HFCLK startup monitor.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkfltchk {
    #[doc = "0: HFCLK startup is not checked"]
    Disable = 0,
    #[doc = "1: HFCLK startup is checked"]
    Enable = 1,
}
impl From<Hfclkfltchk> for bool {
    #[inline(always)]
    fn from(variant: Hfclkfltchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKFLTCHK` reader - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
pub type HfclkfltchkR = crate::BitReader<Hfclkfltchk>;
impl HfclkfltchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkfltchk {
        match self.bits {
            false => Hfclkfltchk::Disable,
            true => Hfclkfltchk::Enable,
        }
    }
    #[doc = "HFCLK startup is not checked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hfclkfltchk::Disable
    }
    #[doc = "HFCLK startup is checked"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hfclkfltchk::Enable
    }
}
#[doc = "Field `HFCLKFLTCHK` writer - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
pub type HfclkfltchkW<'a, REG> = crate::BitWriter<'a, REG, Hfclkfltchk>;
impl<'a, REG> HfclkfltchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFCLK startup is not checked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkfltchk::Disable)
    }
    #[doc = "HFCLK startup is checked"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkfltchk::Enable)
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXTTIME specifies the HFXT startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), HFXT will be checked after this time expires."]
    #[inline(always)]
    pub fn hfxttime(&self) -> HfxttimeR {
        HfxttimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:13 - HFXT Range Select"]
    #[inline(always)]
    pub fn hfxtrsel(&self) -> HfxtrselR {
        HfxtrselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 28 - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
    #[inline(always)]
    pub fn hfclkfltchk(&self) -> HfclkfltchkR {
        HfclkfltchkR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXTTIME specifies the HFXT startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), HFXT will be checked after this time expires."]
    #[inline(always)]
    pub fn hfxttime(&mut self) -> HfxttimeW<'_, SysctlHfclkclkcfgSpec> {
        HfxttimeW::new(self, 0)
    }
    #[doc = "Bits 12:13 - HFXT Range Select"]
    #[inline(always)]
    pub fn hfxtrsel(&mut self) -> HfxtrselW<'_, SysctlHfclkclkcfgSpec> {
        HfxtrselW::new(self, 12)
    }
    #[doc = "Bit 28 - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
    #[inline(always)]
    pub fn hfclkfltchk(&mut self) -> HfclkfltchkW<'_, SysctlHfclkclkcfgSpec> {
        HfclkfltchkW::new(self, 28)
    }
}
#[doc = "High-frequency clock (HFCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_hfclkclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_hfclkclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlHfclkclkcfgSpec;
impl crate::RegisterSpec for SysctlHfclkclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_hfclkclkcfg::R`](R) reader structure"]
impl crate::Readable for SysctlHfclkclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_hfclkclkcfg::W`](W) writer structure"]
impl crate::Writable for SysctlHfclkclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_HFCLKCLKCFG to value 0x1000_0000"]
impl crate::Resettable for SysctlHfclkclkcfgSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
