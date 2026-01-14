#[doc = "Register `SYSCTL_NMIICLR` writer"]
pub type W = crate::W<SysctlNmiiclrSpec>;
#[doc = "Clr the BORLVL NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borlvl {
    #[doc = "0: Writing 0h hs no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Borlvl> for bool {
    #[inline(always)]
    fn from(variant: Borlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORLVL` writer - Clr the BORLVL NMI"]
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
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Borlvl::Clr)
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0 {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Clr = 1,
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::Clr)
    }
}
#[doc = "LFXT-EXLF Monitor Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkfail {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Clr = 1,
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lfclkfail::Clr)
    }
}
#[doc = "Flash Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashded {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Clr = 1,
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Flashded::Clr)
    }
}
#[doc = "SRAM Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramded {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Clr = 1,
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Sramded::Clr)
    }
}
#[doc = "VBAT Power Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatdn {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Clr = 1,
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatdn::Clr)
    }
}
#[doc = "VBAT Power On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatup {
    #[doc = "0: `0`"]
    NoEffect = 0,
    #[doc = "1: `1`"]
    Clr = 1,
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatup::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clr the BORLVL NMI"]
    #[inline(always)]
    pub fn borlvl(&mut self) -> BorlvlW<'_, SysctlNmiiclrSpec> {
        BorlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn wwdt0(&mut self) -> Wwdt0W<'_, SysctlNmiiclrSpec> {
        Wwdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - LFXT-EXLF Monitor Fail"]
    #[inline(always)]
    pub fn lfclkfail(&mut self) -> LfclkfailW<'_, SysctlNmiiclrSpec> {
        LfclkfailW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn flashded(&mut self) -> FlashdedW<'_, SysctlNmiiclrSpec> {
        FlashdedW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM Double Error Detect"]
    #[inline(always)]
    pub fn sramded(&mut self) -> SramdedW<'_, SysctlNmiiclrSpec> {
        SramdedW::new(self, 4)
    }
    #[doc = "Bit 5 - VBAT Power Off"]
    #[inline(always)]
    pub fn vbatdn(&mut self) -> VbatdnW<'_, SysctlNmiiclrSpec> {
        VbatdnW::new(self, 5)
    }
    #[doc = "Bit 6 - VBAT Power On"]
    #[inline(always)]
    pub fn vbatup(&mut self) -> VbatupW<'_, SysctlNmiiclrSpec> {
        VbatupW::new(self, 6)
    }
}
#[doc = "NMI interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_nmiiclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlNmiiclrSpec;
impl crate::RegisterSpec for SysctlNmiiclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysctl_nmiiclr::W`](W) writer structure"]
impl crate::Writable for SysctlNmiiclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_NMIICLR to value 0"]
impl crate::Resettable for SysctlNmiiclrSpec {}
