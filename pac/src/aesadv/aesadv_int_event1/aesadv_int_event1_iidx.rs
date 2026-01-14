#[doc = "Register `AESADV_INT_EVENT1_IIDX` reader"]
pub type R = crate::R<AesadvIntEvent1IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: AES trigger 0 DMA (Data Input trigger)"]
    Trig0 = 1,
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
            1 => Some(Stat::Trig0),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "AES trigger 0 DMA (Data Input trigger)"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == Stat::Trig0
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent1IidxSpec;
impl crate::RegisterSpec for AesadvIntEvent1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_int_event1_iidx::R`](R) reader structure"]
impl crate::Readable for AesadvIntEvent1IidxSpec {}
#[doc = "`reset()` method sets AESADV_INT_EVENT1_IIDX to value 0"]
impl crate::Resettable for AesadvIntEvent1IidxSpec {}
