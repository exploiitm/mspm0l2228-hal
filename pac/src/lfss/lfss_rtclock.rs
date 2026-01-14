#[doc = "Register `LFSS_RTCLOCK` reader"]
pub type R = crate::R<LfssRtclockSpec>;
#[doc = "Register `LFSS_RTCLOCK` writer"]
pub type W = crate::W<LfssRtclockSpec>;
#[doc = "If set, the register bit will protect the CLKCTL, SEC, MIN, HOUR, DAY, MON, YEAR and LFSSRST from accidental writes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: RTC counter is writable."]
    Clr = 0,
    #[doc = "1: RTC counter is read only access."]
    Set = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTECT` reader - If set, the register bit will protect the CLKCTL, SEC, MIN, HOUR, DAY, MON, YEAR and LFSSRST from accidental writes."]
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::Clr,
            true => Protect::Set,
        }
    }
    #[doc = "RTC counter is writable."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Protect::Clr
    }
    #[doc = "RTC counter is read only access."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Protect::Set
    }
}
#[doc = "Field `PROTECT` writer - If set, the register bit will protect the CLKCTL, SEC, MIN, HOUR, DAY, MON, YEAR and LFSSRST from accidental writes."]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC counter is writable."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Clr)
    }
    #[doc = "RTC counter is read only access."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Set)
    }
}
impl R {
    #[doc = "Bit 0 - If set, the register bit will protect the CLKCTL, SEC, MIN, HOUR, DAY, MON, YEAR and LFSSRST from accidental writes."]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, the register bit will protect the CLKCTL, SEC, MIN, HOUR, DAY, MON, YEAR and LFSSRST from accidental writes."]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<'_, LfssRtclockSpec> {
        ProtectW::new(self, 0)
    }
}
#[doc = "Real time clock lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_rtclock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_rtclock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssRtclockSpec;
impl crate::RegisterSpec for LfssRtclockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_rtclock::R`](R) reader structure"]
impl crate::Readable for LfssRtclockSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_rtclock::W`](W) writer structure"]
impl crate::Writable for LfssRtclockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_RTCLOCK to value 0"]
impl crate::Resettable for LfssRtclockSpec {}
