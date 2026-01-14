#[doc = "Register `AESADV_INT_EVENT2_IIDX` reader"]
pub type R = crate::R<AesadvIntEvent2IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: AES DMA Trigger 1 (Data Output trigger)"]
    Trig1 = 1,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Trig1),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "AES DMA Trigger 1 (Data Output trigger)"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == Stat::Trig1
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent2IidxSpec;
impl crate::RegisterSpec for AesadvIntEvent2IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_int_event2_iidx::R`](R) reader structure"]
impl crate::Readable for AesadvIntEvent2IidxSpec {}
#[doc = "`reset()` method sets AESADV_INT_EVENT2_IIDX to value 0"]
impl crate::Resettable for AesadvIntEvent2IidxSpec {}
