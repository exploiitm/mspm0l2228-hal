#[doc = "Register `LFSS_A1MIN` reader"]
pub type R = crate::R<LfssA1minSpec>;
#[doc = "Register `LFSS_A1MIN` writer"]
pub type W = crate::W<LfssA1minSpec>;
#[doc = "Field `AMINBIN` reader - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type AminbinR = crate::FieldReader;
#[doc = "Field `AMINBIN` writer - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type AminbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aminaebin {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Aminaebin> for bool {
    #[inline(always)]
    fn from(variant: Aminaebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMINAEBIN` reader - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AminaebinR = crate::BitReader<Aminaebin>;
impl AminaebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aminaebin {
        match self.bits {
            false => Aminaebin::Disable,
            true => Aminaebin::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Aminaebin::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Aminaebin::Enable
    }
}
#[doc = "Field `AMINAEBIN` writer - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AminaebinW<'a, REG> = crate::BitWriter<'a, REG, Aminaebin>;
impl<'a, REG> AminaebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Aminaebin::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aminaebin::Enable)
    }
}
#[doc = "Field `AMINLOWBCD` reader - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AminlowbcdR = crate::FieldReader;
#[doc = "Field `AMINLOWBCD` writer - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AminlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AMINHIGHBCD` reader - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AminhighbcdR = crate::FieldReader;
#[doc = "Field `AMINHIGHBCD` writer - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AminhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aminaebcd {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Aminaebcd> for bool {
    #[inline(always)]
    fn from(variant: Aminaebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMINAEBCD` reader - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AminaebcdR = crate::BitReader<Aminaebcd>;
impl AminaebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aminaebcd {
        match self.bits {
            false => Aminaebcd::Disable,
            true => Aminaebcd::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Aminaebcd::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Aminaebcd::Enable
    }
}
#[doc = "Field `AMINAEBCD` writer - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AminaebcdW<'a, REG> = crate::BitWriter<'a, REG, Aminaebcd>;
impl<'a, REG> AminaebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Aminaebcd::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aminaebcd::Enable)
    }
}
impl R {
    #[doc = "Bits 0:5 - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn aminbin(&self) -> AminbinR {
        AminbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn aminaebin(&self) -> AminaebinR {
        AminaebinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn aminlowbcd(&self) -> AminlowbcdR {
        AminlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn aminhighbcd(&self) -> AminhighbcdR {
        AminhighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn aminaebcd(&self) -> AminaebcdR {
        AminaebcdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn aminbin(&mut self) -> AminbinW<'_, LfssA1minSpec> {
        AminbinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn aminaebin(&mut self) -> AminaebinW<'_, LfssA1minSpec> {
        AminaebinW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn aminlowbcd(&mut self) -> AminlowbcdW<'_, LfssA1minSpec> {
        AminlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn aminhighbcd(&mut self) -> AminhighbcdW<'_, LfssA1minSpec> {
        AminhighbcdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn aminaebcd(&mut self) -> AminaebcdW<'_, LfssA1minSpec> {
        AminaebcdW::new(self, 15)
    }
}
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a1min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a1min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssA1minSpec;
impl crate::RegisterSpec for LfssA1minSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_a1min::R`](R) reader structure"]
impl crate::Readable for LfssA1minSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_a1min::W`](W) writer structure"]
impl crate::Writable for LfssA1minSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_A1MIN to value 0"]
impl crate::Resettable for LfssA1minSpec {}
