#[doc = "Register `LFSS_TSDAY` reader"]
pub type R = crate::R<LfssTsdaySpec>;
#[doc = "Field `DOW` reader - Time Stamp Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOMBIN` reader - Time Stamp Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type DombinR = crate::FieldReader;
#[doc = "Field `DOMLOWBCD` reader - Time Stamp Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DomlowbcdR = crate::FieldReader;
#[doc = "Field `DOMHIGHBCD` reader - Time Stamp Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DomhighbcdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Time Stamp Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Time Stamp Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn dombin(&self) -> DombinR {
        DombinR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn domlowbcd(&self) -> DomlowbcdR {
        DomlowbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Time Stamp Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn domhighbcd(&self) -> DomhighbcdR {
        DomhighbcdR::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "Time Stamp Day Of Week / MonthRegister - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsday::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsdaySpec;
impl crate::RegisterSpec for LfssTsdaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tsday::R`](R) reader structure"]
impl crate::Readable for LfssTsdaySpec {}
#[doc = "`reset()` method sets LFSS_TSDAY to value 0"]
impl crate::Resettable for LfssTsdaySpec {}
