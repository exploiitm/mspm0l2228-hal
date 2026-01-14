#[doc = "Register `SYSCTL_MGMT_ADC12B1MSPS0_CLKCFG` reader"]
pub type R = crate::R<SysctlMgmtAdc12b1msps0ClkcfgSpec>;
#[doc = "Register `SYSCTL_MGMT_ADC12B1MSPS0_CLKCFG` writer"]
pub type W = crate::W<SysctlMgmtAdc12b1msps0ClkcfgSpec>;
#[doc = "Sample Window Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampclk {
    #[doc = "0: `0`"]
    Ulpclk = 0,
    #[doc = "1: `1`"]
    Sysosc = 1,
    #[doc = "2: `10`"]
    Hfclk = 2,
}
impl From<Sampclk> for u8 {
    #[inline(always)]
    fn from(variant: Sampclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampclk {
    type Ux = u8;
}
impl crate::IsEnum for Sampclk {}
#[doc = "Field `SAMPCLK` reader - Sample Window Clock"]
pub type SampclkR = crate::FieldReader<Sampclk>;
impl SampclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sampclk> {
        match self.bits {
            0 => Some(Sampclk::Ulpclk),
            1 => Some(Sampclk::Sysosc),
            2 => Some(Sampclk::Hfclk),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ulpclk(&self) -> bool {
        *self == Sampclk::Ulpclk
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Sampclk::Sysosc
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Sampclk::Hfclk
    }
}
#[doc = "Field `SAMPCLK` writer - Sample Window Clock"]
pub type SampclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sampclk>;
impl<'a, REG> SampclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ulpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sampclk::Ulpclk)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Sampclk::Sysosc)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sampclk::Hfclk)
    }
}
#[doc = "Conversion Clock is ON during RUN Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cconrun {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Cconrun> for bool {
    #[inline(always)]
    fn from(variant: Cconrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCONRUN` reader - Conversion Clock is ON during RUN Mode"]
pub type CconrunR = crate::BitReader<Cconrun>;
impl CconrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cconrun {
        match self.bits {
            false => Cconrun::Disable,
            true => Cconrun::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cconrun::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cconrun::Enable
    }
}
#[doc = "Field `CCONRUN` writer - Conversion Clock is ON during RUN Mode"]
pub type CconrunW<'a, REG> = crate::BitWriter<'a, REG, Cconrun>;
impl<'a, REG> CconrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconrun::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconrun::Enable)
    }
}
#[doc = "Conversion Clock is ON during STOP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cconstop {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Cconstop> for bool {
    #[inline(always)]
    fn from(variant: Cconstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCONSTOP` reader - Conversion Clock is ON during STOP Mode"]
pub type CconstopR = crate::BitReader<Cconstop>;
impl CconstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cconstop {
        match self.bits {
            false => Cconstop::Disable,
            true => Cconstop::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cconstop::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cconstop::Enable
    }
}
#[doc = "Field `CCONSTOP` writer - Conversion Clock is ON during STOP Mode"]
pub type CconstopW<'a, REG> = crate::BitWriter<'a, REG, Cconstop>;
impl<'a, REG> CconstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconstop::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconstop::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sample Window Clock"]
    #[inline(always)]
    pub fn sampclk(&self) -> SampclkR {
        SampclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Conversion Clock is ON during RUN Mode"]
    #[inline(always)]
    pub fn cconrun(&self) -> CconrunR {
        CconrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Conversion Clock is ON during STOP Mode"]
    #[inline(always)]
    pub fn cconstop(&self) -> CconstopR {
        CconstopR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample Window Clock"]
    #[inline(always)]
    pub fn sampclk(&mut self) -> SampclkW<'_, SysctlMgmtAdc12b1msps0ClkcfgSpec> {
        SampclkW::new(self, 0)
    }
    #[doc = "Bit 4 - Conversion Clock is ON during RUN Mode"]
    #[inline(always)]
    pub fn cconrun(&mut self) -> CconrunW<'_, SysctlMgmtAdc12b1msps0ClkcfgSpec> {
        CconrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Conversion Clock is ON during STOP Mode"]
    #[inline(always)]
    pub fn cconstop(&mut self) -> CconstopW<'_, SysctlMgmtAdc12b1msps0ClkcfgSpec> {
        CconstopW::new(self, 5)
    }
}
#[doc = "IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_adc12b1msps0_clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_adc12b1msps0_clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlMgmtAdc12b1msps0ClkcfgSpec;
impl crate::RegisterSpec for SysctlMgmtAdc12b1msps0ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_mgmt_adc12b1msps0_clkcfg::R`](R) reader structure"]
impl crate::Readable for SysctlMgmtAdc12b1msps0ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_mgmt_adc12b1msps0_clkcfg::W`](W) writer structure"]
impl crate::Writable for SysctlMgmtAdc12b1msps0ClkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_MGMT_ADC12B1MSPS0_CLKCFG to value 0"]
impl crate::Resettable for SysctlMgmtAdc12b1msps0ClkcfgSpec {}
