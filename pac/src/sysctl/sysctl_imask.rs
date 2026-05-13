#[doc = "Register `SYSCTL_IMASK` reader"]
pub type R = crate::R<SysctlImaskSpec>;
#[doc = "Register `SYSCTL_IMASK` writer"]
pub type W = crate::W<SysctlImaskSpec>;
#[doc = "Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfoscgood {
    #[doc = "0: Interrupt disabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled"]
    Enable = 1,
}
impl From<Lfoscgood> for bool {
    #[inline(always)]
    fn from(variant: Lfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFOSCGOOD` reader - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
pub type LfoscgoodR = crate::BitReader<Lfoscgood>;
impl LfoscgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfoscgood {
        match self.bits {
            false => Lfoscgood::Disable,
            true => Lfoscgood::Enable,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lfoscgood::Disable
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lfoscgood::Enable
    }
}
#[doc = "Field `LFOSCGOOD` writer - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
pub type LfoscgoodW<'a, REG> = crate::BitWriter<'a, REG, Lfoscgood>;
impl<'a, REG> LfoscgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lfoscgood::Disable)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lfoscgood::Enable)
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anaclkerr {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Anaclkerr> for bool {
    #[inline(always)]
    fn from(variant: Anaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACLKERR` reader - Analog Clocking Consistency Error"]
pub type AnaclkerrR = crate::BitReader<Anaclkerr>;
impl AnaclkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anaclkerr {
        match self.bits {
            false => Anaclkerr::Disable,
            true => Anaclkerr::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Anaclkerr::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Anaclkerr::Enable
    }
}
#[doc = "Field `ANACLKERR` writer - Analog Clocking Consistency Error"]
pub type AnaclkerrW<'a, REG> = crate::BitWriter<'a, REG, Anaclkerr>;
impl<'a, REG> AnaclkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Anaclkerr::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Anaclkerr::Enable)
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashsec {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Flashsec> for bool {
    #[inline(always)]
    fn from(variant: Flashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHSEC` reader - Flash Single Error Correct"]
pub type FlashsecR = crate::BitReader<Flashsec>;
impl FlashsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashsec {
        match self.bits {
            false => Flashsec::Disable,
            true => Flashsec::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Flashsec::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Flashsec::Enable
    }
}
#[doc = "Field `FLASHSEC` writer - Flash Single Error Correct"]
pub type FlashsecW<'a, REG> = crate::BitWriter<'a, REG, Flashsec>;
impl<'a, REG> FlashsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsec::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsec::Enable)
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsec {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Sramsec> for bool {
    #[inline(always)]
    fn from(variant: Sramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMSEC` reader - SRAM Single Error Correct"]
pub type SramsecR = crate::BitReader<Sramsec>;
impl SramsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsec {
        match self.bits {
            false => Sramsec::Disable,
            true => Sramsec::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sramsec::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sramsec::Enable
    }
}
#[doc = "Field `SRAMSEC` writer - SRAM Single Error Correct"]
pub type SramsecW<'a, REG> = crate::BitWriter<'a, REG, Sramsec>;
impl<'a, REG> SramsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsec::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsec::Enable)
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtgood {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Lfxtgood> for bool {
    #[inline(always)]
    fn from(variant: Lfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTGOOD` reader - LFXT GOOD"]
pub type LfxtgoodR = crate::BitReader<Lfxtgood>;
impl LfxtgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtgood {
        match self.bits {
            false => Lfxtgood::Disable,
            true => Lfxtgood::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lfxtgood::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lfxtgood::Enable
    }
}
#[doc = "Field `LFXTGOOD` writer - LFXT GOOD"]
pub type LfxtgoodW<'a, REG> = crate::BitWriter<'a, REG, Lfxtgood>;
impl<'a, REG> LfxtgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtgood::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtgood::Enable)
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkgood {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Hfclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKGOOD` reader - HFCLK GOOD"]
pub type HfclkgoodR = crate::BitReader<Hfclkgood>;
impl HfclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkgood {
        match self.bits {
            false => Hfclkgood::Disable,
            true => Hfclkgood::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hfclkgood::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hfclkgood::Enable
    }
}
#[doc = "Field `HFCLKGOOD` writer - HFCLK GOOD"]
pub type HfclkgoodW<'a, REG> = crate::BitWriter<'a, REG, Hfclkgood>;
impl<'a, REG> HfclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkgood::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkgood::Enable)
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkgood {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Hsclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKGOOD` reader - HSCLK GOOD"]
pub type HsclkgoodR = crate::BitReader<Hsclkgood>;
impl HsclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclkgood {
        match self.bits {
            false => Hsclkgood::Disable,
            true => Hsclkgood::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hsclkgood::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hsclkgood::Enable
    }
}
#[doc = "Field `HSCLKGOOD` writer - HSCLK GOOD"]
pub type HsclkgoodW<'a, REG> = crate::BitWriter<'a, REG, Hsclkgood>;
impl<'a, REG> HsclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkgood::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkgood::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
    #[inline(always)]
    pub fn lfoscgood(&self) -> LfoscgoodR {
        LfoscgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn anaclkerr(&self) -> AnaclkerrR {
        AnaclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn flashsec(&self) -> FlashsecR {
        FlashsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn sramsec(&self) -> SramsecR {
        SramsecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn lfxtgood(&self) -> LfxtgoodR {
        LfxtgoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn hfclkgood(&self) -> HfclkgoodR {
        HfclkgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSCLK GOOD"]
    #[inline(always)]
    pub fn hsclkgood(&self) -> HsclkgoodR {
        HsclkgoodR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
    #[inline(always)]
    pub fn lfoscgood(&mut self) -> LfoscgoodW<'_, SysctlImaskSpec> {
        LfoscgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn anaclkerr(&mut self) -> AnaclkerrW<'_, SysctlImaskSpec> {
        AnaclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn flashsec(&mut self) -> FlashsecW<'_, SysctlImaskSpec> {
        FlashsecW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn sramsec(&mut self) -> SramsecW<'_, SysctlImaskSpec> {
        SramsecW::new(self, 3)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn lfxtgood(&mut self) -> LfxtgoodW<'_, SysctlImaskSpec> {
        LfxtgoodW::new(self, 4)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn hfclkgood(&mut self) -> HfclkgoodW<'_, SysctlImaskSpec> {
        HfclkgoodW::new(self, 5)
    }
    #[doc = "Bit 6 - HSCLK GOOD"]
    #[inline(always)]
    pub fn hsclkgood(&mut self) -> HsclkgoodW<'_, SysctlImaskSpec> {
        HsclkgoodW::new(self, 6)
    }
}
#[doc = "SYSCTL interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlImaskSpec;
impl crate::RegisterSpec for SysctlImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_imask::R`](R) reader structure"]
impl crate::Readable for SysctlImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_imask::W`](W) writer structure"]
impl crate::Writable for SysctlImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_IMASK to value 0"]
impl crate::Resettable for SysctlImaskSpec {}
