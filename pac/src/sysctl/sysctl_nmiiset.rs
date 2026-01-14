#[doc = "Register `SYSCTL_NMIISET` writer"]
pub type W = crate::W<SysctlNmiisetSpec>;
#[doc = "Set the BORLVL NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borlvl {
    #[doc = "0: Writing 0h hs no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Borlvl> for bool {
    #[inline(always)]
    fn from(variant: Borlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORLVL` writer - Set the BORLVL NMI"]
pub type BorlvlW<'a, REG> = crate::BitWriter<'a, REG, Borlvl>;
impl<'a, REG> BorlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0h hs no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Borlvl::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Borlvl::Set)
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0 {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Wwdt0> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0` writer - Watch Dog 0 Fault"]
pub type Wwdt0W<'a, REG> = crate::BitWriter<'a, REG, Wwdt0>;
impl<'a, REG> Wwdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::Set)
    }
}
#[doc = "LFXT-EXLF Monitor Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkfail {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Lfclkfail> for bool {
    #[inline(always)]
    fn from(variant: Lfclkfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKFAIL` writer - LFXT-EXLF Monitor Fail"]
pub type LfclkfailW<'a, REG> = crate::BitWriter<'a, REG, Lfclkfail>;
impl<'a, REG> LfclkfailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lfclkfail::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lfclkfail::Set)
    }
}
#[doc = "Flash Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashded {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Flashded> for bool {
    #[inline(always)]
    fn from(variant: Flashded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHDED` writer - Flash Double Error Detect"]
pub type FlashdedW<'a, REG> = crate::BitWriter<'a, REG, Flashded>;
impl<'a, REG> FlashdedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flashded::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Flashded::Set)
    }
}
#[doc = "SRAM Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramded {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Sramded> for bool {
    #[inline(always)]
    fn from(variant: Sramded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMDED` writer - SRAM Double Error Detect"]
pub type SramdedW<'a, REG> = crate::BitWriter<'a, REG, Sramded>;
impl<'a, REG> SramdedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sramded::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Sramded::Set)
    }
}
#[doc = "VBAT Power Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatdn {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Vbatdn> for bool {
    #[inline(always)]
    fn from(variant: Vbatdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATDN` writer - VBAT Power Off"]
pub type VbatdnW<'a, REG> = crate::BitWriter<'a, REG, Vbatdn>;
impl<'a, REG> VbatdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatdn::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatdn::Set)
    }
}
#[doc = "VBAT Power On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatup {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Vbatup> for bool {
    #[inline(always)]
    fn from(variant: Vbatup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATUP` writer - VBAT Power On"]
pub type VbatupW<'a, REG> = crate::BitWriter<'a, REG, Vbatup>;
impl<'a, REG> VbatupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatup::NoEffect)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatup::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Set the BORLVL NMI"]
    #[inline(always)]
    pub fn borlvl(&mut self) -> BorlvlW<'_, SysctlNmiisetSpec> {
        BorlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn wwdt0(&mut self) -> Wwdt0W<'_, SysctlNmiisetSpec> {
        Wwdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - LFXT-EXLF Monitor Fail"]
    #[inline(always)]
    pub fn lfclkfail(&mut self) -> LfclkfailW<'_, SysctlNmiisetSpec> {
        LfclkfailW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn flashded(&mut self) -> FlashdedW<'_, SysctlNmiisetSpec> {
        FlashdedW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM Double Error Detect"]
    #[inline(always)]
    pub fn sramded(&mut self) -> SramdedW<'_, SysctlNmiisetSpec> {
        SramdedW::new(self, 4)
    }
    #[doc = "Bit 5 - VBAT Power Off"]
    #[inline(always)]
    pub fn vbatdn(&mut self) -> VbatdnW<'_, SysctlNmiisetSpec> {
        VbatdnW::new(self, 5)
    }
    #[doc = "Bit 6 - VBAT Power On"]
    #[inline(always)]
    pub fn vbatup(&mut self) -> VbatupW<'_, SysctlNmiisetSpec> {
        VbatupW::new(self, 6)
    }
}
#[doc = "NMI interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_nmiiset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlNmiisetSpec;
impl crate::RegisterSpec for SysctlNmiisetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_nmiiset::W`](W) writer structure"]
impl crate::Writable for SysctlNmiisetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_NMIISET to value 0"]
impl crate::Resettable for SysctlNmiisetSpec {}
