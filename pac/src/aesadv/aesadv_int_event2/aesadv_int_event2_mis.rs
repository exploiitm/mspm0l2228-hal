#[doc = "Register `AESADV_INT_EVENT2_MIS` reader"]
pub type R = crate::R<AesadvIntEvent2MisSpec>;
#[doc = "TRIG1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig1 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Trig1> for bool {
    #[inline(always)]
    fn from(variant: Trig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG1` reader - TRIG1 event"]
pub type Trig1R = crate::BitReader<Trig1>;
impl Trig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig1 {
        match self.bits {
            false => Trig1::Clr,
            true => Trig1::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trig1::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trig1::Set
    }
}
impl R {
    #[doc = "Bit 0 - TRIG1 event"]
    #[inline(always)]
    pub fn trig1(&self) -> Trig1R {
        Trig1R::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent2MisSpec;
impl crate::RegisterSpec for AesadvIntEvent2MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_int_event2_mis::R`](R) reader structure"]
impl crate::Readable for AesadvIntEvent2MisSpec {}
#[doc = "`reset()` method sets AESADV_INT_EVENT2_MIS to value 0"]
impl crate::Resettable for AesadvIntEvent2MisSpec {}
