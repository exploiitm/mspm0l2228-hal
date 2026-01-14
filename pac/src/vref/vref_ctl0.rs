#[doc = "Register `VREF_CTL0` reader"]
pub type R = crate::R<VrefCtl0Spec>;
#[doc = "Register `VREF_CTL0` writer"]
pub type W = crate::W<VrefCtl0Spec>;
#[doc = "This bit enables the VREF module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: VREF is disabled"]
    Disable = 0,
    #[doc = "1: VREF is enabled"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - This bit enables the VREF module."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "VREF is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "VREF is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - This bit enables the VREF module."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREF is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "VREF is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "Comparator Vref Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompVrefEnable {
    #[doc = "0: COMP VREF is disabled"]
    Disable = 0,
    #[doc = "1: COMP VREF is enabled"]
    Enable = 1,
}
impl From<CompVrefEnable> for bool {
    #[inline(always)]
    fn from(variant: CompVrefEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP_VREF_ENABLE` reader - Comparator Vref Enable"]
pub type CompVrefEnableR = crate::BitReader<CompVrefEnable>;
impl CompVrefEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CompVrefEnable {
        match self.bits {
            false => CompVrefEnable::Disable,
            true => CompVrefEnable::Enable,
        }
    }
    #[doc = "COMP VREF is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CompVrefEnable::Disable
    }
    #[doc = "COMP VREF is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CompVrefEnable::Enable
    }
}
#[doc = "Field `COMP_VREF_ENABLE` writer - Comparator Vref Enable"]
pub type CompVrefEnableW<'a, REG> = crate::BitWriter<'a, REG, CompVrefEnable>;
impl<'a, REG> CompVrefEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP VREF is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CompVrefEnable::Disable)
    }
    #[doc = "COMP VREF is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CompVrefEnable::Enable)
    }
}
#[doc = "These bits configure output buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufconfig {
    #[doc = "0: Configure Output Buffer to 2.5v"]
    Output2p5v = 0,
    #[doc = "1: Configure Output Buffer to 1.4v"]
    Output1p4v = 1,
}
impl From<Bufconfig> for bool {
    #[inline(always)]
    fn from(variant: Bufconfig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFCONFIG` reader - These bits configure output buffer."]
pub type BufconfigR = crate::BitReader<Bufconfig>;
impl BufconfigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufconfig {
        match self.bits {
            false => Bufconfig::Output2p5v,
            true => Bufconfig::Output1p4v,
        }
    }
    #[doc = "Configure Output Buffer to 2.5v"]
    #[inline(always)]
    pub fn is_output2p5v(&self) -> bool {
        *self == Bufconfig::Output2p5v
    }
    #[doc = "Configure Output Buffer to 1.4v"]
    #[inline(always)]
    pub fn is_output1p4v(&self) -> bool {
        *self == Bufconfig::Output1p4v
    }
}
#[doc = "Field `BUFCONFIG` writer - These bits configure output buffer."]
pub type BufconfigW<'a, REG> = crate::BitWriter<'a, REG, Bufconfig>;
impl<'a, REG> BufconfigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure Output Buffer to 2.5v"]
    #[inline(always)]
    pub fn output2p5v(self) -> &'a mut crate::W<REG> {
        self.variant(Bufconfig::Output2p5v)
    }
    #[doc = "Configure Output Buffer to 1.4v"]
    #[inline(always)]
    pub fn output1p4v(self) -> &'a mut crate::W<REG> {
        self.variant(Bufconfig::Output1p4v)
    }
}
#[doc = "This bit enable sample and hold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shmode {
    #[doc = "0: Sample and hold mode is disable"]
    Disable = 0,
    #[doc = "1: Sample and hold mode is enable"]
    Enable = 1,
}
impl From<Shmode> for bool {
    #[inline(always)]
    fn from(variant: Shmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHMODE` reader - This bit enable sample and hold mode"]
pub type ShmodeR = crate::BitReader<Shmode>;
impl ShmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shmode {
        match self.bits {
            false => Shmode::Disable,
            true => Shmode::Enable,
        }
    }
    #[doc = "Sample and hold mode is disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Shmode::Disable
    }
    #[doc = "Sample and hold mode is enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Shmode::Enable
    }
}
#[doc = "Field `SHMODE` writer - This bit enable sample and hold mode"]
pub type ShmodeW<'a, REG> = crate::BitWriter<'a, REG, Shmode>;
impl<'a, REG> ShmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample and hold mode is disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Shmode::Disable)
    }
    #[doc = "Sample and hold mode is enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Shmode::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the VREF module."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator Vref Enable"]
    #[inline(always)]
    pub fn comp_vref_enable(&self) -> CompVrefEnableR {
        CompVrefEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - These bits configure output buffer."]
    #[inline(always)]
    pub fn bufconfig(&self) -> BufconfigR {
        BufconfigR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit enable sample and hold mode"]
    #[inline(always)]
    pub fn shmode(&self) -> ShmodeR {
        ShmodeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the VREF module."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, VrefCtl0Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator Vref Enable"]
    #[inline(always)]
    pub fn comp_vref_enable(&mut self) -> CompVrefEnableW<'_, VrefCtl0Spec> {
        CompVrefEnableW::new(self, 1)
    }
    #[doc = "Bit 7 - These bits configure output buffer."]
    #[inline(always)]
    pub fn bufconfig(&mut self) -> BufconfigW<'_, VrefCtl0Spec> {
        BufconfigW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit enable sample and hold mode"]
    #[inline(always)]
    pub fn shmode(&mut self) -> ShmodeW<'_, VrefCtl0Spec> {
        ShmodeW::new(self, 8)
    }
}
#[doc = "Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefCtl0Spec;
impl crate::RegisterSpec for VrefCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_ctl0::R`](R) reader structure"]
impl crate::Readable for VrefCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_ctl0::W`](W) writer structure"]
impl crate::Writable for VrefCtl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREF_CTL0 to value 0"]
impl crate::Resettable for VrefCtl0Spec {}
