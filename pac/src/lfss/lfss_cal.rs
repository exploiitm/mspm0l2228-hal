#[doc = "Register `LFSS_CAL` reader"]
pub type R = crate::R<LfssCalSpec>;
#[doc = "Register `LFSS_CAL` writer"]
pub type W = crate::W<LfssCalSpec>;
#[doc = "Field `RTCOCALX` reader - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
pub type RtcocalxR = crate::FieldReader;
#[doc = "Field `RTCOCALX` writer - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
pub type RtcocalxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcocals {
    #[doc = "0: Down calibration. Frequency adjusted down."]
    Down = 0,
    #[doc = "1: Up calibration. Frequency adjusted up."]
    Up = 1,
}
impl From<Rtcocals> for bool {
    #[inline(always)]
    fn from(variant: Rtcocals) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOCALS` reader - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
pub type RtcocalsR = crate::BitReader<Rtcocals>;
impl RtcocalsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcocals {
        match self.bits {
            false => Rtcocals::Down,
            true => Rtcocals::Up,
        }
    }
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Rtcocals::Down
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Rtcocals::Up
    }
}
#[doc = "Field `RTCOCALS` writer - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
pub type RtcocalsW<'a, REG> = crate::BitWriter<'a, REG, Rtcocals>;
impl<'a, REG> RtcocalsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcocals::Down)
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcocals::Up)
    }
}
#[doc = "Real-time clock calibration frequency. Selects frequency output to RTCCLK pin for calibration measurement. The corresponding port must be configured for the peripheral module function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtccalfx {
    #[doc = "0: 32 kHz"]
    F32khz = 0,
    #[doc = "1: 512 Hz"]
    F512hz = 1,
    #[doc = "2: 256 Hz"]
    F256hz = 2,
    #[doc = "3: 1 Hz"]
    F1hz = 3,
}
impl From<Rtccalfx> for u8 {
    #[inline(always)]
    fn from(variant: Rtccalfx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtccalfx {
    type Ux = u8;
}
impl crate::IsEnum for Rtccalfx {}
#[doc = "Field `RTCCALFX` reader - Real-time clock calibration frequency. Selects frequency output to RTCCLK pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
pub type RtccalfxR = crate::FieldReader<Rtccalfx>;
impl RtccalfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtccalfx {
        match self.bits {
            0 => Rtccalfx::F32khz,
            1 => Rtccalfx::F512hz,
            2 => Rtccalfx::F256hz,
            3 => Rtccalfx::F1hz,
            _ => unreachable!(),
        }
    }
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn is_f32khz(&self) -> bool {
        *self == Rtccalfx::F32khz
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn is_f512hz(&self) -> bool {
        *self == Rtccalfx::F512hz
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn is_f256hz(&self) -> bool {
        *self == Rtccalfx::F256hz
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn is_f1hz(&self) -> bool {
        *self == Rtccalfx::F1hz
    }
}
#[doc = "Field `RTCCALFX` writer - Real-time clock calibration frequency. Selects frequency output to RTCCLK pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
pub type RtccalfxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtccalfx, crate::Safe>;
impl<'a, REG> RtccalfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn f32khz(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalfx::F32khz)
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn f512hz(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalfx::F512hz)
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn f256hz(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalfx::F256hz)
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn f1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalfx::F1hz)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
    #[inline(always)]
    pub fn rtcocalx(&self) -> RtcocalxR {
        RtcocalxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
    #[inline(always)]
    pub fn rtcocals(&self) -> RtcocalsR {
        RtcocalsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Real-time clock calibration frequency. Selects frequency output to RTCCLK pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
    #[inline(always)]
    pub fn rtccalfx(&self) -> RtccalfxR {
        RtccalfxR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
    #[inline(always)]
    pub fn rtcocalx(&mut self) -> RtcocalxW<'_, LfssCalSpec> {
        RtcocalxW::new(self, 0)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RtcocalsW<'_, LfssCalSpec> {
        RtcocalsW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Real-time clock calibration frequency. Selects frequency output to RTCCLK pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
    #[inline(always)]
    pub fn rtccalfx(&mut self) -> RtccalfxW<'_, LfssCalSpec> {
        RtccalfxW::new(self, 16)
    }
}
#[doc = "RTC Clock Offset Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssCalSpec;
impl crate::RegisterSpec for LfssCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_cal::R`](R) reader structure"]
impl crate::Readable for LfssCalSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_cal::W`](W) writer structure"]
impl crate::Writable for LfssCalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_CAL to value 0"]
impl crate::Resettable for LfssCalSpec {}
