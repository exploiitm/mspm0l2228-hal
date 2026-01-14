#[doc = "Register `LFSS_TSMON` reader"]
pub type R = crate::R<LfssTsmonSpec>;
#[doc = "Field `MONBIN` reader - Time Stamp Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MonbinR = crate::FieldReader;
#[doc = "Field `MONLOWBCD` reader - Time Stamp Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonlowbcdR = crate::FieldReader;
#[doc = "Field `MONHIGHBCD` reader - Time Stamp Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonhighbcdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Time Stamp Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monbin(&self) -> MonbinR {
        MonbinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Time Stamp Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monlowbcd(&self) -> MonlowbcdR {
        MonlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Time Stamp Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monhighbcd(&self) -> MonhighbcdR {
        MonhighbcdR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Time Stamp Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsmonSpec;
impl crate::RegisterSpec for LfssTsmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tsmon::R`](R) reader structure"]
impl crate::Readable for LfssTsmonSpec {}
#[doc = "`reset()` method sets LFSS_TSMON to value 0"]
impl crate::Resettable for LfssTsmonSpec {}
