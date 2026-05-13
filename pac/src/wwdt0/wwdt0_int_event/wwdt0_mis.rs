#[doc = "Register `WWDT0_MIS` reader"]
pub type R = crate::R<Wwdt0MisSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inttim {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Inttim> for bool {
    #[inline(always)]
    fn from(variant: Inttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTTIM` reader - Interval Timer Interrupt."]
pub type InttimR = crate::BitReader<Inttim>;
impl InttimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inttim {
        match self.bits {
            false => Inttim::Clr,
            true => Inttim::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Inttim::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Inttim::Set
    }
}
impl R {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn inttim(&self) -> InttimR {
        InttimR::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0MisSpec;
impl crate::RegisterSpec for Wwdt0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_mis::R`](R) reader structure"]
impl crate::Readable for Wwdt0MisSpec {}
#[doc = "`reset()` method sets WWDT0_MIS to value 0"]
impl crate::Resettable for Wwdt0MisSpec {}
