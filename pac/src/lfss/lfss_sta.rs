#[doc = "Register `LFSS_STA` reader"]
pub type R = crate::R<LfssStaSpec>;
#[doc = "Real-time clock ready. This bit indicates when the real-time clock time values are safe for reading.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdy {
    #[doc = "0: RTC time values in transition"]
    NotReady = 0,
    #[doc = "1: RTC time values safe for reading."]
    Ready = 1,
}
impl From<Rtcrdy> for bool {
    #[inline(always)]
    fn from(variant: Rtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDY` reader - Real-time clock ready. This bit indicates when the real-time clock time values are safe for reading."]
pub type RtcrdyR = crate::BitReader<Rtcrdy>;
impl RtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcrdy {
        match self.bits {
            false => Rtcrdy::NotReady,
            true => Rtcrdy::Ready,
        }
    }
    #[doc = "RTC time values in transition"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Rtcrdy::NotReady
    }
    #[doc = "RTC time values safe for reading."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Rtcrdy::Ready
    }
}
#[doc = "Real-time clock temperature compensation ready. This is a read only bit that indicates when the RTCTCMPx can be written. Write to RTCTCMPx should be avoided when RTCTCRDY is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctcrdy {
    #[doc = "0: RTC temperature compensation in transition"]
    NotReady = 0,
    #[doc = "1: RTC temperature compensation ready"]
    Ready = 1,
}
impl From<Rtctcrdy> for bool {
    #[inline(always)]
    fn from(variant: Rtctcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCRDY` reader - Real-time clock temperature compensation ready. This is a read only bit that indicates when the RTCTCMPx can be written. Write to RTCTCMPx should be avoided when RTCTCRDY is reset."]
pub type RtctcrdyR = crate::BitReader<Rtctcrdy>;
impl RtctcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctcrdy {
        match self.bits {
            false => Rtctcrdy::NotReady,
            true => Rtctcrdy::Ready,
        }
    }
    #[doc = "RTC temperature compensation in transition"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Rtctcrdy::NotReady
    }
    #[doc = "RTC temperature compensation ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Rtctcrdy::Ready
    }
}
#[doc = "Real-time clock temperature compensation write OK. This is a read-only bit that indicates if the write to RTCTCMP is successful or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctcok {
    #[doc = "0: Write to RTCTCMPx is unsuccessful"]
    NotOk = 0,
    #[doc = "1: Write to RTCTCMPx is successful"]
    Ok = 1,
}
impl From<Rtctcok> for bool {
    #[inline(always)]
    fn from(variant: Rtctcok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCOK` reader - Real-time clock temperature compensation write OK. This is a read-only bit that indicates if the write to RTCTCMP is successful or not."]
pub type RtctcokR = crate::BitReader<Rtctcok>;
impl RtctcokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctcok {
        match self.bits {
            false => Rtctcok::NotOk,
            true => Rtctcok::Ok,
        }
    }
    #[doc = "Write to RTCTCMPx is unsuccessful"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == Rtctcok::NotOk
    }
    #[doc = "Write to RTCTCMPx is successful"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == Rtctcok::Ok
    }
}
impl R {
    #[doc = "Bit 0 - Real-time clock ready. This bit indicates when the real-time clock time values are safe for reading."]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RtcrdyR {
        RtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time clock temperature compensation ready. This is a read only bit that indicates when the RTCTCMPx can be written. Write to RTCTCMPx should be avoided when RTCTCRDY is reset."]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RtctcrdyR {
        RtctcrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time clock temperature compensation write OK. This is a read-only bit that indicates if the write to RTCTCMP is successful or not."]
    #[inline(always)]
    pub fn rtctcok(&self) -> RtctcokR {
        RtctcokR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "RTC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssStaSpec;
impl crate::RegisterSpec for LfssStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_sta::R`](R) reader structure"]
impl crate::Readable for LfssStaSpec {}
#[doc = "`reset()` method sets LFSS_STA to value 0"]
impl crate::Resettable for LfssStaSpec {}
