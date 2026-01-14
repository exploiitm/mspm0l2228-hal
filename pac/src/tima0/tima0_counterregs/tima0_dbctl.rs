#[doc = "Register `TIMA0_DBCTL` reader"]
pub type R = crate::R<Tima0DbctlSpec>;
#[doc = "Register `TIMA0_DBCTL` writer"]
pub type W = crate::W<Tima0DbctlSpec>;
#[doc = "Field `RISEDELAY` reader - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
pub type RisedelayR = crate::FieldReader<u16>;
#[doc = "Field `RISEDELAY` writer - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
pub type RisedelayW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Dead Band Mode 1 Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M1Enable {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<M1Enable> for bool {
    #[inline(always)]
    fn from(variant: M1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M1_ENABLE` reader - Dead Band Mode 1 Enable."]
pub type M1EnableR = crate::BitReader<M1Enable>;
impl M1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M1Enable {
        match self.bits {
            false => M1Enable::Disabled,
            true => M1Enable::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M1Enable::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M1Enable::Enabled
    }
}
#[doc = "Field `M1_ENABLE` writer - Dead Band Mode 1 Enable."]
pub type M1EnableW<'a, REG> = crate::BitWriter<'a, REG, M1Enable>;
impl<'a, REG> M1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M1Enable::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M1Enable::Enabled)
    }
}
#[doc = "Field `FALLDELAY` reader - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
pub type FalldelayR = crate::FieldReader<u16>;
#[doc = "Field `FALLDELAY` writer - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
pub type FalldelayW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
    #[inline(always)]
    pub fn risedelay(&self) -> RisedelayR {
        RisedelayR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Dead Band Mode 1 Enable."]
    #[inline(always)]
    pub fn m1_enable(&self) -> M1EnableR {
        M1EnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
    #[inline(always)]
    pub fn falldelay(&self) -> FalldelayR {
        FalldelayR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
    #[inline(always)]
    pub fn risedelay(&mut self) -> RisedelayW<'_, Tima0DbctlSpec> {
        RisedelayW::new(self, 0)
    }
    #[doc = "Bit 12 - Dead Band Mode 1 Enable."]
    #[inline(always)]
    pub fn m1_enable(&mut self) -> M1EnableW<'_, Tima0DbctlSpec> {
        M1EnableW::new(self, 12)
    }
    #[doc = "Bits 16:27 - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
    #[inline(always)]
    pub fn falldelay(&mut self) -> FalldelayW<'_, Tima0DbctlSpec> {
        FalldelayW::new(self, 16)
    }
}
#[doc = "Dead Band insertion control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_dbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_dbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0DbctlSpec;
impl crate::RegisterSpec for Tima0DbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_dbctl::R`](R) reader structure"]
impl crate::Readable for Tima0DbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_dbctl::W`](W) writer structure"]
impl crate::Writable for Tima0DbctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_DBCTL to value 0"]
impl crate::Resettable for Tima0DbctlSpec {}
