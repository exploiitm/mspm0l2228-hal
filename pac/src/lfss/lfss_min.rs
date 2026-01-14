#[doc = "Register `LFSS_MIN` reader"]
pub type R = crate::R<LfssMinSpec>;
#[doc = "Register `LFSS_MIN` writer"]
pub type W = crate::W<LfssMinSpec>;
#[doc = "Field `MINBIN` reader - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MinbinR = crate::FieldReader;
#[doc = "Field `MINBIN` writer - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MinbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MINLOWBCD` reader - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinlowbcdR = crate::FieldReader;
#[doc = "Field `MINLOWBCD` writer - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINHIGHBCD` reader - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinhighbcdR = crate::FieldReader;
#[doc = "Field `MINHIGHBCD` writer - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minbin(&self) -> MinbinR {
        MinbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minlowbcd(&self) -> MinlowbcdR {
        MinlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minhighbcd(&self) -> MinhighbcdR {
        MinhighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minbin(&mut self) -> MinbinW<'_, LfssMinSpec> {
        MinbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minlowbcd(&mut self) -> MinlowbcdW<'_, LfssMinSpec> {
        MinlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn minhighbcd(&mut self) -> MinhighbcdW<'_, LfssMinSpec> {
        MinhighbcdW::new(self, 12)
    }
}
#[doc = "RTC Minutes Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssMinSpec;
impl crate::RegisterSpec for LfssMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_min::R`](R) reader structure"]
impl crate::Readable for LfssMinSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_min::W`](W) writer structure"]
impl crate::Writable for LfssMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_MIN to value 0"]
impl crate::Resettable for LfssMinSpec {}
