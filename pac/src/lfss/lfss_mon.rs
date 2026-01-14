#[doc = "Register `LFSS_MON` reader"]
pub type R = crate::R<LfssMonSpec>;
#[doc = "Register `LFSS_MON` writer"]
pub type W = crate::W<LfssMonSpec>;
#[doc = "Field `MONBIN` reader - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MonbinR = crate::FieldReader;
#[doc = "Field `MONBIN` writer - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MonbinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONLOWBCD` reader - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonlowbcdR = crate::FieldReader;
#[doc = "Field `MONLOWBCD` writer - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONHIGHBCD` reader - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonhighbcdR = crate::BitReader;
#[doc = "Field `MONHIGHBCD` writer - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonhighbcdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monbin(&self) -> MonbinR {
        MonbinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monlowbcd(&self) -> MonlowbcdR {
        MonlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monhighbcd(&self) -> MonhighbcdR {
        MonhighbcdR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monbin(&mut self) -> MonbinW<'_, LfssMonSpec> {
        MonbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monlowbcd(&mut self) -> MonlowbcdW<'_, LfssMonSpec> {
        MonlowbcdW::new(self, 8)
    }
    #[doc = "Bit 12 - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn monhighbcd(&mut self) -> MonhighbcdW<'_, LfssMonSpec> {
        MonhighbcdW::new(self, 12)
    }
}
#[doc = "RTC Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssMonSpec;
impl crate::RegisterSpec for LfssMonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_mon::R`](R) reader structure"]
impl crate::Readable for LfssMonSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_mon::W`](W) writer structure"]
impl crate::Writable for LfssMonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_MON to value 0"]
impl crate::Resettable for LfssMonSpec {}
