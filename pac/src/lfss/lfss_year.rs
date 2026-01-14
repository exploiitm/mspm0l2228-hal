#[doc = "Register `LFSS_YEAR` reader"]
pub type R = crate::R<LfssYearSpec>;
#[doc = "Register `LFSS_YEAR` writer"]
pub type W = crate::W<LfssYearSpec>;
#[doc = "Field `YEARLOWBIN` reader - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearlowbinR = crate::FieldReader;
#[doc = "Field `YEARLOWBIN` writer - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearlowbinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEARHIGHBIN` reader - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearhighbinR = crate::FieldReader;
#[doc = "Field `YEARHIGHBIN` writer - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearhighbinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEARLOWESTBCD` reader - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearlowestbcdR = crate::FieldReader;
#[doc = "Field `YEARLOWESTBCD` writer - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearlowestbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DECADEBCD` reader - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DecadebcdR = crate::FieldReader;
#[doc = "Field `DECADEBCD` writer - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DecadebcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CENTLOWBCD` reader - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type CentlowbcdR = crate::FieldReader;
#[doc = "Field `CENTLOWBCD` writer - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type CentlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CENTHIGHBCD` reader - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type CenthighbcdR = crate::FieldReader;
#[doc = "Field `CENTHIGHBCD` writer - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type CenthighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearlowbin(&self) -> YearlowbinR {
        YearlowbinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearhighbin(&self) -> YearhighbinR {
        YearhighbinR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearlowestbcd(&self) -> YearlowestbcdR {
        YearlowestbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn decadebcd(&self) -> DecadebcdR {
        DecadebcdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn centlowbcd(&self) -> CentlowbcdR {
        CentlowbcdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn centhighbcd(&self) -> CenthighbcdR {
        CenthighbcdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearlowbin(&mut self) -> YearlowbinW<'_, LfssYearSpec> {
        YearlowbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearhighbin(&mut self) -> YearhighbinW<'_, LfssYearSpec> {
        YearhighbinW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearlowestbcd(&mut self) -> YearlowestbcdW<'_, LfssYearSpec> {
        YearlowestbcdW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn decadebcd(&mut self) -> DecadebcdW<'_, LfssYearSpec> {
        DecadebcdW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn centlowbcd(&mut self) -> CentlowbcdW<'_, LfssYearSpec> {
        CentlowbcdW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn centhighbcd(&mut self) -> CenthighbcdW<'_, LfssYearSpec> {
        CenthighbcdW::new(self, 28)
    }
}
#[doc = "RTC Year Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_year::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_year::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssYearSpec;
impl crate::RegisterSpec for LfssYearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_year::R`](R) reader structure"]
impl crate::Readable for LfssYearSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_year::W`](W) writer structure"]
impl crate::Writable for LfssYearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_YEAR to value 0"]
impl crate::Resettable for LfssYearSpec {}
