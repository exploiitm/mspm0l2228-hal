#[doc = "Register `FLASHCTL_CFGPCNT` reader"]
pub type R = crate::R<FlashctlCfgpcntSpec>;
#[doc = "Register `FLASHCTL_CFGPCNT` writer"]
pub type W = crate::W<FlashctlCfgpcntSpec>;
#[doc = "Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxpcntovr {
    #[doc = "0: Use hard-wired (default) value for maximum pulse count"]
    Default = 0,
    #[doc = "1: Use value from MAXPCNTVAL field as maximum puse count"]
    Override = 1,
}
impl From<Maxpcntovr> for bool {
    #[inline(always)]
    fn from(variant: Maxpcntovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXPCNTOVR` reader - Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MaxpcntovrR = crate::BitReader<Maxpcntovr>;
impl MaxpcntovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxpcntovr {
        match self.bits {
            false => Maxpcntovr::Default,
            true => Maxpcntovr::Override,
        }
    }
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Maxpcntovr::Default
    }
    #[doc = "Use value from MAXPCNTVAL field as maximum puse count"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Maxpcntovr::Override
    }
}
#[doc = "Field `MAXPCNTOVR` writer - Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MaxpcntovrW<'a, REG> = crate::BitWriter<'a, REG, Maxpcntovr>;
impl<'a, REG> MaxpcntovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Maxpcntovr::Default)
    }
    #[doc = "Use value from MAXPCNTVAL field as maximum puse count"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Maxpcntovr::Override)
    }
}
#[doc = "Field `MAXPCNTVAL` reader - Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
pub type MaxpcntvalR = crate::FieldReader;
#[doc = "Field `MAXPCNTVAL` writer - Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
pub type MaxpcntvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Override hard-wired maximum pulse count for erase. If set, then the value in MAXERSPCNTVAL will be used as the max pulse count for erase operations. By default, this bit is 0, and a hard-wired max pulse count is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxerspcntovr {
    #[doc = "0: Use hard-wired (default) value for maximum pulse count"]
    Default = 0,
    #[doc = "1: Use value from MAXERSPCNTVAL field as maximum erase pulse count"]
    Override = 1,
}
impl From<Maxerspcntovr> for bool {
    #[inline(always)]
    fn from(variant: Maxerspcntovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXERSPCNTOVR` reader - Override hard-wired maximum pulse count for erase. If set, then the value in MAXERSPCNTVAL will be used as the max pulse count for erase operations. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MaxerspcntovrR = crate::BitReader<Maxerspcntovr>;
impl MaxerspcntovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxerspcntovr {
        match self.bits {
            false => Maxerspcntovr::Default,
            true => Maxerspcntovr::Override,
        }
    }
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Maxerspcntovr::Default
    }
    #[doc = "Use value from MAXERSPCNTVAL field as maximum erase pulse count"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Maxerspcntovr::Override
    }
}
#[doc = "Field `MAXERSPCNTOVR` writer - Override hard-wired maximum pulse count for erase. If set, then the value in MAXERSPCNTVAL will be used as the max pulse count for erase operations. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MaxerspcntovrW<'a, REG> = crate::BitWriter<'a, REG, Maxerspcntovr>;
impl<'a, REG> MaxerspcntovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Maxerspcntovr::Default)
    }
    #[doc = "Use value from MAXERSPCNTVAL field as maximum erase pulse count"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Maxerspcntovr::Override)
    }
}
#[doc = "Field `MAXERSPCNTVAL` reader - Override maximum pulse count for erase with this value. If MAXERSPCNTOVR = 0, then this field is ignored. If MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for erase."]
pub type MaxerspcntvalR = crate::FieldReader<u16>;
#[doc = "Field `MAXERSPCNTVAL` writer - Override maximum pulse count for erase with this value. If MAXERSPCNTOVR = 0, then this field is ignored. If MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for erase."]
pub type MaxerspcntvalW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    pub fn maxpcntovr(&self) -> MaxpcntovrR {
        MaxpcntovrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11 - Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
    #[inline(always)]
    pub fn maxpcntval(&self) -> MaxpcntvalR {
        MaxpcntvalR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Override hard-wired maximum pulse count for erase. If set, then the value in MAXERSPCNTVAL will be used as the max pulse count for erase operations. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    pub fn maxerspcntovr(&self) -> MaxerspcntovrR {
        MaxerspcntovrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Override maximum pulse count for erase with this value. If MAXERSPCNTOVR = 0, then this field is ignored. If MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for erase."]
    #[inline(always)]
    pub fn maxerspcntval(&self) -> MaxerspcntvalR {
        MaxerspcntvalR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    pub fn maxpcntovr(&mut self) -> MaxpcntovrW<'_, FlashctlCfgpcntSpec> {
        MaxpcntovrW::new(self, 0)
    }
    #[doc = "Bits 4:11 - Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
    #[inline(always)]
    pub fn maxpcntval(&mut self) -> MaxpcntvalW<'_, FlashctlCfgpcntSpec> {
        MaxpcntvalW::new(self, 4)
    }
    #[doc = "Bit 16 - Override hard-wired maximum pulse count for erase. If set, then the value in MAXERSPCNTVAL will be used as the max pulse count for erase operations. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    pub fn maxerspcntovr(&mut self) -> MaxerspcntovrW<'_, FlashctlCfgpcntSpec> {
        MaxerspcntovrW::new(self, 16)
    }
    #[doc = "Bits 20:31 - Override maximum pulse count for erase with this value. If MAXERSPCNTOVR = 0, then this field is ignored. If MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for erase."]
    #[inline(always)]
    pub fn maxerspcntval(&mut self) -> MaxerspcntvalW<'_, FlashctlCfgpcntSpec> {
        MaxerspcntvalW::new(self, 20)
    }
}
#[doc = "Pulse Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cfgpcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cfgpcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCfgpcntSpec;
impl crate::RegisterSpec for FlashctlCfgpcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cfgpcnt::R`](R) reader structure"]
impl crate::Readable for FlashctlCfgpcntSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cfgpcnt::W`](W) writer structure"]
impl crate::Writable for FlashctlCfgpcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CFGPCNT to value 0"]
impl crate::Resettable for FlashctlCfgpcntSpec {}
