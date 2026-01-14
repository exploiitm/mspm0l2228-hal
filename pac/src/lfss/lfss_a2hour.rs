#[doc = "Register `LFSS_A2HOUR` reader"]
pub type R = crate::R<LfssA2hourSpec>;
#[doc = "Register `LFSS_A2HOUR` writer"]
pub type W = crate::W<LfssA2hourSpec>;
#[doc = "Field `AHOURBIN` reader - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type AhourbinR = crate::FieldReader;
#[doc = "Field `AHOURBIN` writer - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type AhourbinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahouraebin {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Ahouraebin> for bool {
    #[inline(always)]
    fn from(variant: Ahouraebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHOURAEBIN` reader - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AhouraebinR = crate::BitReader<Ahouraebin>;
impl AhouraebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahouraebin {
        match self.bits {
            false => Ahouraebin::Disable,
            true => Ahouraebin::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ahouraebin::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ahouraebin::Enable
    }
}
#[doc = "Field `AHOURAEBIN` writer - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AhouraebinW<'a, REG> = crate::BitWriter<'a, REG, Ahouraebin>;
impl<'a, REG> AhouraebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ahouraebin::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ahouraebin::Enable)
    }
}
#[doc = "Field `AHOURLOWBCD` reader - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AhourlowbcdR = crate::FieldReader;
#[doc = "Field `AHOURLOWBCD` writer - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type AhourlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHOURHIGHBCD` reader - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
pub type AhourhighbcdR = crate::FieldReader;
#[doc = "Field `AHOURHIGHBCD` writer - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
pub type AhourhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahouraebcd {
    #[doc = "0: No alarm"]
    Disable = 0,
    #[doc = "1: Alarm enabled"]
    Enable = 1,
}
impl From<Ahouraebcd> for bool {
    #[inline(always)]
    fn from(variant: Ahouraebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHOURAEBCD` reader - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AhouraebcdR = crate::BitReader<Ahouraebcd>;
impl AhouraebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahouraebcd {
        match self.bits {
            false => Ahouraebcd::Disable,
            true => Ahouraebcd::Enable,
        }
    }
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ahouraebcd::Disable
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ahouraebcd::Enable
    }
}
#[doc = "Field `AHOURAEBCD` writer - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
pub type AhouraebcdW<'a, REG> = crate::BitWriter<'a, REG, Ahouraebcd>;
impl<'a, REG> AhouraebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ahouraebcd::Disable)
    }
    #[doc = "Alarm enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ahouraebcd::Enable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn ahourbin(&self) -> AhourbinR {
        AhourbinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn ahouraebin(&self) -> AhouraebinR {
        AhouraebinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn ahourlowbcd(&self) -> AhourlowbcdR {
        AhourlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
    #[inline(always)]
    pub fn ahourhighbcd(&self) -> AhourhighbcdR {
        AhourhighbcdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn ahouraebcd(&self) -> AhouraebcdR {
        AhouraebcdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn ahourbin(&mut self) -> AhourbinW<'_, LfssA2hourSpec> {
        AhourbinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn ahouraebin(&mut self) -> AhouraebinW<'_, LfssA2hourSpec> {
        AhouraebinW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn ahourlowbcd(&mut self) -> AhourlowbcdW<'_, LfssA2hourSpec> {
        AhourlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
    #[inline(always)]
    pub fn ahourhighbcd(&mut self) -> AhourhighbcdW<'_, LfssA2hourSpec> {
        AhourhighbcdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored. 0x0= Alarm disabled. 0x1= Alarm enabled."]
    #[inline(always)]
    pub fn ahouraebcd(&mut self) -> AhouraebcdW<'_, LfssA2hourSpec> {
        AhouraebcdW::new(self, 15)
    }
}
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a2hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a2hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssA2hourSpec;
impl crate::RegisterSpec for LfssA2hourSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_a2hour::R`](R) reader structure"]
impl crate::Readable for LfssA2hourSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_a2hour::W`](W) writer structure"]
impl crate::Writable for LfssA2hourSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_A2HOUR to value 0"]
impl crate::Resettable for LfssA2hourSpec {}
