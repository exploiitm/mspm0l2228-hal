#[doc = "Register `LFSS_A2DAY` reader"]
pub type R = crate::R<LfssA2daySpec>;
#[doc = "Register `LFSS_A2DAY` writer"]
pub type W = crate::W<LfssA2daySpec>;
#[doc = "Field `ADOW` reader - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type AdowR = crate::FieldReader;
#[doc = "Field `ADOW` writer - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type AdowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adowae {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Adowae> for bool {
    #[inline(always)]
    fn from(variant: Adowae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOWAE` reader - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
pub type AdowaeR = crate::BitReader<Adowae>;
impl AdowaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adowae {
        match self.bits {
            false => Adowae::Disable,
            true => Adowae::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adowae::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adowae::Enable
    }
}
#[doc = "Field `ADOWAE` writer - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
pub type AdowaeW<'a, REG> = crate::BitWriter<'a, REG, Adowae>;
impl<'a, REG> AdowaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adowae::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adowae::Enable)
    }
}
#[doc = "Field `ADOMBIN` reader - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type AdombinR = crate::FieldReader;
#[doc = "Field `ADOMBIN` writer - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type AdombinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adomaebin {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Adomaebin> for bool {
    #[inline(always)]
    fn from(variant: Adomaebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOMAEBIN` reader - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AdomaebinR = crate::BitReader<Adomaebin>;
impl AdomaebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adomaebin {
        match self.bits {
            false => Adomaebin::Disable,
            true => Adomaebin::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adomaebin::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adomaebin::Enable
    }
}
#[doc = "Field `ADOMAEBIN` writer - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AdomaebinW<'a, REG> = crate::BitWriter<'a, REG, Adomaebin>;
impl<'a, REG> AdomaebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adomaebin::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adomaebin::Enable)
    }
}
#[doc = "Field `ADOMLOWBCD` reader - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AdomlowbcdR = crate::FieldReader;
#[doc = "Field `ADOMLOWBCD` writer - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AdomlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADOMHIGHBCD` reader - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AdomhighbcdR = crate::FieldReader;
#[doc = "Field `ADOMHIGHBCD` writer - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AdomhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adomaebcd {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Adomaebcd> for bool {
    #[inline(always)]
    fn from(variant: Adomaebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOMAEBCD` reader - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AdomaebcdR = crate::BitReader<Adomaebcd>;
impl AdomaebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adomaebcd {
        match self.bits {
            false => Adomaebcd::Disable,
            true => Adomaebcd::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adomaebcd::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adomaebcd::Enable
    }
}
#[doc = "Field `ADOMAEBCD` writer - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AdomaebcdW<'a, REG> = crate::BitWriter<'a, REG, Adomaebcd>;
impl<'a, REG> AdomaebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adomaebcd::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adomaebcd::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn adow(&self) -> AdowR {
        AdowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn adowae(&self) -> AdowaeR {
        AdowaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn adombin(&self) -> AdombinR {
        AdombinR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn adomaebin(&self) -> AdomaebinR {
        AdomaebinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn adomlowbcd(&self) -> AdomlowbcdR {
        AdomlowbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn adomhighbcd(&self) -> AdomhighbcdR {
        AdomhighbcdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn adomaebcd(&self) -> AdomaebcdR {
        AdomaebcdR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn adow(&mut self) -> AdowW<'_, LfssA2daySpec> {
        AdowW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn adowae(&mut self) -> AdowaeW<'_, LfssA2daySpec> {
        AdowaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn adombin(&mut self) -> AdombinW<'_, LfssA2daySpec> {
        AdombinW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn adomaebin(&mut self) -> AdomaebinW<'_, LfssA2daySpec> {
        AdomaebinW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn adomlowbcd(&mut self) -> AdomlowbcdW<'_, LfssA2daySpec> {
        AdomlowbcdW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn adomhighbcd(&mut self) -> AdomhighbcdW<'_, LfssA2daySpec> {
        AdomhighbcdW::new(self, 20)
    }
    #[doc = "Bit 23 - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn adomaebcd(&mut self) -> AdomaebcdW<'_, LfssA2daySpec> {
        AdomaebcdW::new(self, 23)
    }
}
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a2day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a2day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssA2daySpec;
impl crate::RegisterSpec for LfssA2daySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_a2day::R`](R) reader structure"]
impl crate::Readable for LfssA2daySpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_a2day::W`](W) writer structure"]
impl crate::Writable for LfssA2daySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_A2DAY to value 0"]
impl crate::Resettable for LfssA2daySpec {}
