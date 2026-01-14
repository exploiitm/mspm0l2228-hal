#[doc = "Register `LFSS_HOUR` reader"]
pub type R = crate::R<LfssHourSpec>;
#[doc = "Register `LFSS_HOUR` writer"]
pub type W = crate::W<LfssHourSpec>;
#[doc = "Field `HOURBIN` reader - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type HourbinR = crate::FieldReader;
#[doc = "Field `HOURBIN` writer - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type HourbinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOURLOWBCD` reader - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourlowbcdR = crate::FieldReader;
#[doc = "Field `HOURLOWBCD` writer - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURHIGHBCD` reader - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourhighbcdR = crate::FieldReader;
#[doc = "Field `HOURHIGHBCD` writer - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourbin(&self) -> HourbinR {
        HourbinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourlowbcd(&self) -> HourlowbcdR {
        HourlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourhighbcd(&self) -> HourhighbcdR {
        HourhighbcdR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourbin(&mut self) -> HourbinW<'_, LfssHourSpec> {
        HourbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourlowbcd(&mut self) -> HourlowbcdW<'_, LfssHourSpec> {
        HourlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourhighbcd(&mut self) -> HourhighbcdW<'_, LfssHourSpec> {
        HourhighbcdW::new(self, 12)
    }
}
#[doc = "RTC Hours Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssHourSpec;
impl crate::RegisterSpec for LfssHourSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_hour::R`](R) reader structure"]
impl crate::Readable for LfssHourSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_hour::W`](W) writer structure"]
impl crate::Writable for LfssHourSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_HOUR to value 0"]
impl crate::Resettable for LfssHourSpec {}
