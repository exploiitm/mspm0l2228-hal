#[doc = "Register `LFSS_TCMP` reader"]
pub type R = crate::R<LfssTcmpSpec>;
#[doc = "Register `LFSS_TCMP` writer"]
pub type W = crate::W<LfssTcmpSpec>;
#[doc = "Field `RTCTCMPX` reader - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
pub type RtctcmpxR = crate::FieldReader;
#[doc = "Field `RTCTCMPX` writer - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
pub type RtctcmpxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctcmps {
    #[doc = "0: Down calibration. Frequency adjusted down."]
    Down = 0,
    #[doc = "1: Up calibration. Frequency adjusted up."]
    Up = 1,
}
impl From<Rtctcmps> for bool {
    #[inline(always)]
    fn from(variant: Rtctcmps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCMPS` reader - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
pub type RtctcmpsR = crate::BitReader<Rtctcmps>;
impl RtctcmpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctcmps {
        match self.bits {
            false => Rtctcmps::Down,
            true => Rtctcmps::Up,
        }
    }
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Rtctcmps::Down
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Rtctcmps::Up
    }
}
#[doc = "Field `RTCTCMPS` writer - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
pub type RtctcmpsW<'a, REG> = crate::BitWriter<'a, REG, Rtctcmps>;
impl<'a, REG> RtctcmpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctcmps::Down)
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctcmps::Up)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
    #[inline(always)]
    pub fn rtctcmpx(&self) -> RtctcmpxR {
        RtctcmpxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RtctcmpsR {
        RtctcmpsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
    #[inline(always)]
    pub fn rtctcmpx(&mut self) -> RtctcmpxW<'_, LfssTcmpSpec> {
        RtctcmpxW::new(self, 0)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RtctcmpsW<'_, LfssTcmpSpec> {
        RtctcmpsW::new(self, 15)
    }
}
#[doc = "RTC Temperature Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTcmpSpec;
impl crate::RegisterSpec for LfssTcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tcmp::R`](R) reader structure"]
impl crate::Readable for LfssTcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tcmp::W`](W) writer structure"]
impl crate::Writable for LfssTcmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TCMP to value 0"]
impl crate::Resettable for LfssTcmpSpec {}
