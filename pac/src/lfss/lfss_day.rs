#[doc = "Register `LFSS_DAY` reader"]
pub type R = crate::R<LfssDaySpec>;
#[doc = "Register `LFSS_DAY` writer"]
pub type W = crate::W<LfssDaySpec>;
#[doc = "Field `DOW` reader - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOW` writer - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DOMBIN` reader - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type DombinR = crate::FieldReader;
#[doc = "Field `DOMBIN` writer - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type DombinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DOMLOWBCD` reader - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DomlowbcdR = crate::FieldReader;
#[doc = "Field `DOMLOWBCD` writer - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DomlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DOMHIGHBCD` reader - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DomhighbcdR = crate::FieldReader;
#[doc = "Field `DOMHIGHBCD` writer - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DomhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn dombin(&self) -> DombinR {
        DombinR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn domlowbcd(&self) -> DomlowbcdR {
        DomlowbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn domhighbcd(&self) -> DomhighbcdR {
        DomhighbcdR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn dow(&mut self) -> DowW<'_, LfssDaySpec> {
        DowW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn dombin(&mut self) -> DombinW<'_, LfssDaySpec> {
        DombinW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn domlowbcd(&mut self) -> DomlowbcdW<'_, LfssDaySpec> {
        DomlowbcdW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn domhighbcd(&mut self) -> DomhighbcdW<'_, LfssDaySpec> {
        DomhighbcdW::new(self, 20)
    }
}
#[doc = "RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssDaySpec;
impl crate::RegisterSpec for LfssDaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_day::R`](R) reader structure"]
impl crate::Readable for LfssDaySpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_day::W`](W) writer structure"]
impl crate::Writable for LfssDaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_DAY to value 0"]
impl crate::Resettable for LfssDaySpec {}
