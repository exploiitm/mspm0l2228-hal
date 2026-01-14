#[doc = "Register `LFSS_TSHOUR` reader"]
pub type R = crate::R<LfssTshourSpec>;
#[doc = "Field `HOURBIN` reader - Time Stamp Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type HourbinR = crate::FieldReader;
#[doc = "Field `HOURLOWBCD` reader - Time Stamp Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourlowbcdR = crate::FieldReader;
#[doc = "Field `HOURHIGHBCD` reader - Time Stamp Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourhighbcdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Time Stamp Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourbin(&self) -> HourbinR {
        HourbinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Time Stamp Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourlowbcd(&self) -> HourlowbcdR {
        HourlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Time Stamp Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hourhighbcd(&self) -> HourhighbcdR {
        HourhighbcdR::new(((self.bits >> 12) & 3) as u8)
    }
}
#[doc = "Time Stamp Hours Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tshour::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTshourSpec;
impl crate::RegisterSpec for LfssTshourSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tshour::R`](R) reader structure"]
impl crate::Readable for LfssTshourSpec {}
#[doc = "`reset()` method sets LFSS_TSHOUR to value 0"]
impl crate::Resettable for LfssTshourSpec {}
