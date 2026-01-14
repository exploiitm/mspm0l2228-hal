#[doc = "Register `LFSS_SEC` reader"]
pub type R = crate::R<LfssSecSpec>;
#[doc = "Register `LFSS_SEC` writer"]
pub type W = crate::W<LfssSecSpec>;
#[doc = "Field `SECBIN` reader - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type SecbinR = crate::FieldReader;
#[doc = "Field `SECBIN` writer - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type SecbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SECLOWBCD` reader - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SeclowbcdR = crate::FieldReader;
#[doc = "Field `SECLOWBCD` writer - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SeclowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECHIGHBCD` reader - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SechighbcdR = crate::FieldReader;
#[doc = "Field `SECHIGHBCD` writer - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SechighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn secbin(&self) -> SecbinR {
        SecbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn seclowbcd(&self) -> SeclowbcdR {
        SeclowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sechighbcd(&self) -> SechighbcdR {
        SechighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn secbin(&mut self) -> SecbinW<'_, LfssSecSpec> {
        SecbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn seclowbcd(&mut self) -> SeclowbcdW<'_, LfssSecSpec> {
        SeclowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sechighbcd(&mut self) -> SechighbcdW<'_, LfssSecSpec> {
        SechighbcdW::new(self, 12)
    }
}
#[doc = "RTC Seconds Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSecSpec;
impl crate::RegisterSpec for LfssSecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_sec::R`](R) reader structure"]
impl crate::Readable for LfssSecSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_sec::W`](W) writer structure"]
impl crate::Writable for LfssSecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SEC to value 0"]
impl crate::Resettable for LfssSecSpec {}
