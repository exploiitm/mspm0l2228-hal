#[doc = "Register `SYSCTL_GENCLKEN` reader"]
pub type R = crate::R<SysctlGenclkenSpec>;
#[doc = "Register `SYSCTL_GENCLKEN` writer"]
pub type W = crate::W<SysctlGenclkenSpec>;
#[doc = "EXCLKEN enables the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclken {
    #[doc = "0: CLK_OUT block is disabled"]
    Disable = 0,
    #[doc = "1: CLK_OUT block is enabled"]
    Enable = 1,
}
impl From<Exclken> for bool {
    #[inline(always)]
    fn from(variant: Exclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLKEN` reader - EXCLKEN enables the CLK_OUT external clock output block."]
pub type ExclkenR = crate::BitReader<Exclken>;
impl ExclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclken {
        match self.bits {
            false => Exclken::Disable,
            true => Exclken::Enable,
        }
    }
    #[doc = "CLK_OUT block is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exclken::Disable
    }
    #[doc = "CLK_OUT block is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exclken::Enable
    }
}
#[doc = "Field `EXCLKEN` writer - EXCLKEN enables the CLK_OUT external clock output block."]
pub type ExclkenW<'a, REG> = crate::BitWriter<'a, REG, Exclken>;
impl<'a, REG> ExclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK_OUT block is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exclken::Disable)
    }
    #[doc = "CLK_OUT block is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exclken::Enable)
    }
}
#[doc = "MFPCLKEN enables the middle frequency precision clock (MFPCLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mfpclken {
    #[doc = "0: MFPCLK is disabled"]
    Disable = 0,
    #[doc = "1: MFPCLK is enabled"]
    Enable = 1,
}
impl From<Mfpclken> for bool {
    #[inline(always)]
    fn from(variant: Mfpclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFPCLKEN` reader - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
pub type MfpclkenR = crate::BitReader<Mfpclken>;
impl MfpclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mfpclken {
        match self.bits {
            false => Mfpclken::Disable,
            true => Mfpclken::Enable,
        }
    }
    #[doc = "MFPCLK is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mfpclken::Disable
    }
    #[doc = "MFPCLK is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mfpclken::Enable
    }
}
#[doc = "Field `MFPCLKEN` writer - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
pub type MfpclkenW<'a, REG> = crate::BitWriter<'a, REG, Mfpclken>;
impl<'a, REG> MfpclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MFPCLK is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mfpclken::Disable)
    }
    #[doc = "MFPCLK is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mfpclken::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - EXCLKEN enables the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclken(&self) -> ExclkenR {
        ExclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
    #[inline(always)]
    pub fn mfpclken(&self) -> MfpclkenR {
        MfpclkenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXCLKEN enables the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclken(&mut self) -> ExclkenW<'_, SysctlGenclkenSpec> {
        ExclkenW::new(self, 0)
    }
    #[doc = "Bit 4 - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
    #[inline(always)]
    pub fn mfpclken(&mut self) -> MfpclkenW<'_, SysctlGenclkenSpec> {
        MfpclkenW::new(self, 4)
    }
}
#[doc = "General clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_genclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_genclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlGenclkenSpec;
impl crate::RegisterSpec for SysctlGenclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_genclken::R`](R) reader structure"]
impl crate::Readable for SysctlGenclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_genclken::W`](W) writer structure"]
impl crate::Writable for SysctlGenclkenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_GENCLKEN to value 0"]
impl crate::Resettable for SysctlGenclkenSpec {}
