#[doc = "Register `LFSS_TSSEC` reader"]
pub type R = crate::R<LfssTssecSpec>;
#[doc = "Field `SECBIN` reader - Time Stamp Second Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type SecbinR = crate::FieldReader;
#[doc = "Field `SECLOWBCD` reader - Time Stamp Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SeclowbcdR = crate::FieldReader;
#[doc = "Field `SECHIGHBCD` reader - Time Stamp Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SechighbcdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Time Stamp Second Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn secbin(&self) -> SecbinR {
        SecbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Time Stamp Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn seclowbcd(&self) -> SeclowbcdR {
        SeclowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Stamp Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sechighbcd(&self) -> SechighbcdR {
        SechighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
}
#[doc = "Time Stamp Seconds Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tssec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTssecSpec;
impl crate::RegisterSpec for LfssTssecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tssec::R`](R) reader structure"]
impl crate::Readable for LfssTssecSpec {}
#[doc = "`reset()` method sets LFSS_TSSEC to value 0"]
impl crate::Resettable for LfssTssecSpec {}
