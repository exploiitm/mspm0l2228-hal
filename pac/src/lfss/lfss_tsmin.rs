#[doc = "Register `LFSS_TSMIN` reader"]
pub type R = crate::R<LfssTsminSpec>;
#[doc = "Field `MINBIN` reader - Time Stamp Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MinbinR = crate::FieldReader;
#[doc = "Field `MINLOWBCD` reader - Time Stamp Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinlowbcdR = crate::FieldReader;
#[doc = "Field `MINHIGHBCD` reader - Time Stamp Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinhighbcdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Time Stamp Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minbin(&self) -> MinbinR {
        MinbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Time Stamp Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minlowbcd(&self) -> MinlowbcdR {
        MinlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Stamp Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minhighbcd(&self) -> MinhighbcdR {
        MinhighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
}
#[doc = "Time Stamp Minutes Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsmin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsminSpec;
impl crate::RegisterSpec for LfssTsminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tsmin::R`](R) reader structure"]
impl crate::Readable for LfssTsminSpec {}
#[doc = "`reset()` method sets LFSS_TSMIN to value 0"]
impl crate::Resettable for LfssTsminSpec {}
