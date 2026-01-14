#[doc = "Register `SYSCTL_ISET` writer"]
pub type W = crate::W<SysctlIsetSpec>;
#[doc = "Set the LFOSCGOOD interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfoscgood {
    #[doc = "0: Writing 0h hs no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Lfoscgood> for bool {
    #[inline(always)]
    fn from(variant: Lfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFOSCGOOD` writer - Set the LFOSCGOOD interrupt."]
pub type LfoscgoodW<'a, REG> = crate::BitWriter<'a, REG, Lfoscgood>;
impl<'a, REG> LfoscgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0h hs no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lfoscgood::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lfoscgood::Set)
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anaclkerr {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Anaclkerr> for bool {
    #[inline(always)]
    fn from(variant: Anaclkerr) -> Self {
        variant as u8 != 0
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
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Anaclkerr::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Anaclkerr::Set)
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashsec {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Flashsec> for bool {
    #[inline(always)]
    fn from(variant: Flashsec) -> Self {
        variant as u8 != 0
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
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsec::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsec::Set)
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsec {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Sramsec> for bool {
    #[inline(always)]
    fn from(variant: Sramsec) -> Self {
        variant as u8 != 0
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
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsec::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsec::Set)
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtgood {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Lfxtgood> for bool {
    #[inline(always)]
    fn from(variant: Lfxtgood) -> Self {
        variant as u8 != 0
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
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtgood::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtgood::Set)
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkgood {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Hfclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hfclkgood) -> Self {
        variant as u8 != 0
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
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkgood::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkgood::Set)
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkgood {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Hsclkgood> for bool {
    #[inline(always)]
    fn from(variant: Hsclkgood) -> Self {
        variant as u8 != 0
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
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkgood::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkgood::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Set the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn lfoscgood(&mut self) -> LfoscgoodW<'_, SysctlIsetSpec> {
        LfoscgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn anaclkerr(&mut self) -> AnaclkerrW<'_, SysctlIsetSpec> {
        AnaclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn flashsec(&mut self) -> FlashsecW<'_, SysctlIsetSpec> {
        FlashsecW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn sramsec(&mut self) -> SramsecW<'_, SysctlIsetSpec> {
        SramsecW::new(self, 3)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn lfxtgood(&mut self) -> LfxtgoodW<'_, SysctlIsetSpec> {
        LfxtgoodW::new(self, 4)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn hfclkgood(&mut self) -> HfclkgoodW<'_, SysctlIsetSpec> {
        HfclkgoodW::new(self, 5)
    }
    #[doc = "Bit 6 - HSCLK GOOD"]
    #[inline(always)]
    pub fn hsclkgood(&mut self) -> HsclkgoodW<'_, SysctlIsetSpec> {
        HsclkgoodW::new(self, 6)
    }
}
#[doc = "SYSCTL interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlIsetSpec;
impl crate::RegisterSpec for SysctlIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_iset::W`](W) writer structure"]
impl crate::Writable for SysctlIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_ISET to value 0"]
impl crate::Resettable for SysctlIsetSpec {}
