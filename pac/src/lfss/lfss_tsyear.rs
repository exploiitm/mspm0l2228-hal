#[doc = "Register `LFSS_TSYEAR` reader"]
pub type R = crate::R<LfssTsyearSpec>;
#[doc = "Field `YEARLOWBIN` reader - Time Stamp Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearlowbinR = crate::FieldReader;
#[doc = "Field `YEARHIGHBIN` reader - Time Stamp Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearhighbinR = crate::FieldReader;
#[doc = "Field `YERARLOWESTBCD` reader - Time Stamp Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YerarlowestbcdR = crate::FieldReader;
#[doc = "Field `DECADEBCD` reader - Time Stamp Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DecadebcdR = crate::FieldReader;
#[doc = "Field `CENTLOWBCD` reader - Time Stamp Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type CentlowbcdR = crate::FieldReader;
#[doc = "Field `CENTHIGHBCD` reader - Time Stamp Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type CenthighbcdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearlowbin(&self) -> YearlowbinR {
        YearlowbinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Time Stamp Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yearhighbin(&self) -> YearhighbinR {
        YearhighbinR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn yerarlowestbcd(&self) -> YerarlowestbcdR {
        YerarlowestbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Time Stamp Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn decadebcd(&self) -> DecadebcdR {
        DecadebcdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Time Stamp Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn centlowbcd(&self) -> CentlowbcdR {
        CentlowbcdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Time Stamp Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn centhighbcd(&self) -> CenthighbcdR {
        CenthighbcdR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Time Stamp Years Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsyear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsyearSpec;
impl crate::RegisterSpec for LfssTsyearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tsyear::R`](R) reader structure"]
impl crate::Readable for LfssTsyearSpec {}
#[doc = "`reset()` method sets LFSS_TSYEAR to value 0"]
impl crate::Resettable for LfssTsyearSpec {}
