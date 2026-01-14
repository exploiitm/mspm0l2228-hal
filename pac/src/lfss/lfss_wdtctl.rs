#[doc = "Register `LFSS_WDTCTL` reader"]
pub type R = crate::R<LfssWdtctlSpec>;
#[doc = "Register `LFSS_WDTCTL` writer"]
pub type W = crate::W<LfssWdtctlSpec>;
#[doc = "Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: Minimum value"]
    Min = 0,
    #[doc = "7: Maximum value"]
    Max = 7,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible."]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv> {
        match self.bits {
            0 => Some(Clkdiv::Min),
            7 => Some(Clkdiv::Max),
            _ => None,
        }
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Clkdiv::Min
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Clkdiv::Max
    }
}
#[doc = "Field `CLKDIV` writer - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkdiv>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Min)
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Max)
    }
}
#[doc = "Timer Period of the WDT. These bits select the total watchdog timer count.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Per {
    #[doc = "0: Total timer count is 2^25"]
    PerEn25 = 0,
    #[doc = "1: Total timer count is 2^21"]
    PerEn21 = 1,
    #[doc = "2: Total timer count is 2^18"]
    PerEn18 = 2,
    #[doc = "3: Total timer count is 2^15"]
    PerEn15 = 3,
    #[doc = "4: Total timer count is 2^12 (default)"]
    PerEn12 = 4,
    #[doc = "5: Total timer count is 2^10"]
    PerEn10 = 5,
    #[doc = "6: Total timer count is 2^8"]
    PerEn8 = 6,
    #[doc = "7: Total timer count is 2^6"]
    PerEn6 = 7,
}
impl From<Per> for u8 {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Per {
    type Ux = u8;
}
impl crate::IsEnum for Per {}
#[doc = "Field `PER` reader - Timer Period of the WDT. These bits select the total watchdog timer count."]
pub type PerR = crate::FieldReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            0 => Per::PerEn25,
            1 => Per::PerEn21,
            2 => Per::PerEn18,
            3 => Per::PerEn15,
            4 => Per::PerEn12,
            5 => Per::PerEn10,
            6 => Per::PerEn8,
            7 => Per::PerEn6,
            _ => unreachable!(),
        }
    }
    #[doc = "Total timer count is 2^25"]
    #[inline(always)]
    pub fn is_per_en_25(&self) -> bool {
        *self == Per::PerEn25
    }
    #[doc = "Total timer count is 2^21"]
    #[inline(always)]
    pub fn is_per_en_21(&self) -> bool {
        *self == Per::PerEn21
    }
    #[doc = "Total timer count is 2^18"]
    #[inline(always)]
    pub fn is_per_en_18(&self) -> bool {
        *self == Per::PerEn18
    }
    #[doc = "Total timer count is 2^15"]
    #[inline(always)]
    pub fn is_per_en_15(&self) -> bool {
        *self == Per::PerEn15
    }
    #[doc = "Total timer count is 2^12 (default)"]
    #[inline(always)]
    pub fn is_per_en_12(&self) -> bool {
        *self == Per::PerEn12
    }
    #[doc = "Total timer count is 2^10"]
    #[inline(always)]
    pub fn is_per_en_10(&self) -> bool {
        *self == Per::PerEn10
    }
    #[doc = "Total timer count is 2^8"]
    #[inline(always)]
    pub fn is_per_en_8(&self) -> bool {
        *self == Per::PerEn8
    }
    #[doc = "Total timer count is 2^6"]
    #[inline(always)]
    pub fn is_per_en_6(&self) -> bool {
        *self == Per::PerEn6
    }
}
#[doc = "Field `PER` writer - Timer Period of the WDT. These bits select the total watchdog timer count."]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Per, crate::Safe>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Total timer count is 2^25"]
    #[inline(always)]
    pub fn per_en_25(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn25)
    }
    #[doc = "Total timer count is 2^21"]
    #[inline(always)]
    pub fn per_en_21(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn21)
    }
    #[doc = "Total timer count is 2^18"]
    #[inline(always)]
    pub fn per_en_18(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn18)
    }
    #[doc = "Total timer count is 2^15"]
    #[inline(always)]
    pub fn per_en_15(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn15)
    }
    #[doc = "Total timer count is 2^12 (default)"]
    #[inline(always)]
    pub fn per_en_12(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn12)
    }
    #[doc = "Total timer count is 2^10"]
    #[inline(always)]
    pub fn per_en_10(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn10)
    }
    #[doc = "Total timer count is 2^8"]
    #[inline(always)]
    pub fn per_en_8(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn8)
    }
    #[doc = "Total timer count is 2^6"]
    #[inline(always)]
    pub fn per_en_6(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PerEn6)
    }
}
impl R {
    #[doc = "Bits 0:2 - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Timer Period of the WDT. These bits select the total watchdog timer count."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, LfssWdtctlSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timer Period of the WDT. These bits select the total watchdog timer count."]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, LfssWdtctlSpec> {
        PerW::new(self, 4)
    }
}
#[doc = "Watchdog Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssWdtctlSpec;
impl crate::RegisterSpec for LfssWdtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_wdtctl::R`](R) reader structure"]
impl crate::Readable for LfssWdtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_wdtctl::W`](W) writer structure"]
impl crate::Writable for LfssWdtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_WDTCTL to value 0x43"]
impl crate::Resettable for LfssWdtctlSpec {
    const RESET_VALUE: u32 = 0x43;
}
