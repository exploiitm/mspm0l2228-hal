#[doc = "Register `LFSS_CTL` reader"]
pub type R = crate::R<LfssCtlSpec>;
#[doc = "Register `LFSS_CTL` writer"]
pub type W = crate::W<LfssCtlSpec>;
#[doc = "Real-time clock time event 0x0 = Minute changed 0x1 = Hour changed 0x2 = Every day at midnight (00:00) 0x3 = Every day at noon (12:00)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtctevtx {
    #[doc = "0: Generate RTC event every minute."]
    Minute = 0,
    #[doc = "1: Generate RTC event every hour."]
    Hour = 1,
    #[doc = "2: Generate RTC event at midnight."]
    Midnight = 2,
    #[doc = "3: Generate RTC event at noon."]
    Noon = 3,
}
impl From<Rtctevtx> for u8 {
    #[inline(always)]
    fn from(variant: Rtctevtx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtctevtx {
    type Ux = u8;
}
impl crate::IsEnum for Rtctevtx {}
#[doc = "Field `RTCTEVTX` reader - Real-time clock time event 0x0 = Minute changed 0x1 = Hour changed 0x2 = Every day at midnight (00:00) 0x3 = Every day at noon (12:00)"]
pub type RtctevtxR = crate::FieldReader<Rtctevtx>;
impl RtctevtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctevtx {
        match self.bits {
            0 => Rtctevtx::Minute,
            1 => Rtctevtx::Hour,
            2 => Rtctevtx::Midnight,
            3 => Rtctevtx::Noon,
            _ => unreachable!(),
        }
    }
    #[doc = "Generate RTC event every minute."]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == Rtctevtx::Minute
    }
    #[doc = "Generate RTC event every hour."]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == Rtctevtx::Hour
    }
    #[doc = "Generate RTC event at midnight."]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == Rtctevtx::Midnight
    }
    #[doc = "Generate RTC event at noon."]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == Rtctevtx::Noon
    }
}
#[doc = "Field `RTCTEVTX` writer - Real-time clock time event 0x0 = Minute changed 0x1 = Hour changed 0x2 = Every day at midnight (00:00) 0x3 = Every day at noon (12:00)"]
pub type RtctevtxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtctevtx, crate::Safe>;
impl<'a, REG> RtctevtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generate RTC event every minute."]
    #[inline(always)]
    pub fn minute(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevtx::Minute)
    }
    #[doc = "Generate RTC event every hour."]
    #[inline(always)]
    pub fn hour(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevtx::Hour)
    }
    #[doc = "Generate RTC event at midnight."]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevtx::Midnight)
    }
    #[doc = "Generate RTC event at noon."]
    #[inline(always)]
    pub fn noon(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevtx::Noon)
    }
}
#[doc = "Real-time clock BCD select. Selects BCD counting for real-time clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcbcd {
    #[doc = "0: Binary code selected"]
    Binary = 0,
    #[doc = "1: Binary coded decimal (BCD) code selected"]
    Bcd = 1,
}
impl From<Rtcbcd> for bool {
    #[inline(always)]
    fn from(variant: Rtcbcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCBCD` reader - Real-time clock BCD select. Selects BCD counting for real-time clock."]
pub type RtcbcdR = crate::BitReader<Rtcbcd>;
impl RtcbcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcbcd {
        match self.bits {
            false => Rtcbcd::Binary,
            true => Rtcbcd::Bcd,
        }
    }
    #[doc = "Binary code selected"]
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == Rtcbcd::Binary
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == Rtcbcd::Bcd
    }
}
#[doc = "Field `RTCBCD` writer - Real-time clock BCD select. Selects BCD counting for real-time clock."]
pub type RtcbcdW<'a, REG> = crate::BitWriter<'a, REG, Rtcbcd>;
impl<'a, REG> RtcbcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Binary code selected"]
    #[inline(always)]
    pub fn binary(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcbcd::Binary)
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn bcd(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcbcd::Bcd)
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-time clock time event 0x0 = Minute changed 0x1 = Hour changed 0x2 = Every day at midnight (00:00) 0x3 = Every day at noon (12:00)"]
    #[inline(always)]
    pub fn rtctevtx(&self) -> RtctevtxR {
        RtctevtxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Real-time clock BCD select. Selects BCD counting for real-time clock."]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RtcbcdR {
        RtcbcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-time clock time event 0x0 = Minute changed 0x1 = Hour changed 0x2 = Every day at midnight (00:00) 0x3 = Every day at noon (12:00)"]
    #[inline(always)]
    pub fn rtctevtx(&mut self) -> RtctevtxW<'_, LfssCtlSpec> {
        RtctevtxW::new(self, 0)
    }
    #[doc = "Bit 7 - Real-time clock BCD select. Selects BCD counting for real-time clock."]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RtcbcdW<'_, LfssCtlSpec> {
        RtcbcdW::new(self, 7)
    }
}
#[doc = "RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssCtlSpec;
impl crate::RegisterSpec for LfssCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_ctl::R`](R) reader structure"]
impl crate::Readable for LfssCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_ctl::W`](W) writer structure"]
impl crate::Writable for LfssCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_CTL to value 0"]
impl crate::Resettable for LfssCtlSpec {}
