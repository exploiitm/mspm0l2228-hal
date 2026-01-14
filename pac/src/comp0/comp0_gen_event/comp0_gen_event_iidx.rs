#[doc = "Register `COMP0_GEN_EVENT_IIDX` reader"]
pub type R = crate::R<Comp0GenEventIidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Statr {
    #[doc = "0: No pending interrupt"]
    NoIntr = 0,
    #[doc = "1: Comparator output interrupt"]
    Compifg = 1,
    #[doc = "2: Comparator output inverted interrupt"]
    Compinvifg = 2,
    #[doc = "3: Comparator output ready interrupt"]
    Outrdyifg = 3,
}
impl From<Statr> for u8 {
    #[inline(always)]
    fn from(variant: Statr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Statr {
    type Ux = u8;
}
impl crate::IsEnum for Statr {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Statr>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Statr {
        match self.bits {
            0 => Statr::NoIntr,
            1 => Statr::Compifg,
            2 => Statr::Compinvifg,
            3 => Statr::Outrdyifg,
            _ => unreachable!(),
        }
    }
    #[doc = "No pending interrupt"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Statr::NoIntr
    }
    #[doc = "Comparator output interrupt"]
    #[inline(always)]
    pub fn is_compifg(&self) -> bool {
        *self == Statr::Compifg
    }
    #[doc = "Comparator output inverted interrupt"]
    #[inline(always)]
    pub fn is_compinvifg(&self) -> bool {
        *self == Statr::Compinvifg
    }
    #[doc = "Comparator output ready interrupt"]
    #[inline(always)]
    pub fn is_outrdyifg(&self) -> bool {
        *self == Statr::Outrdyifg
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 3) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_gen_event_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0GenEventIidxSpec;
impl crate::RegisterSpec for Comp0GenEventIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_gen_event_iidx::R`](R) reader structure"]
impl crate::Readable for Comp0GenEventIidxSpec {}
#[doc = "`reset()` method sets COMP0_GEN_EVENT_IIDX to value 0"]
impl crate::Resettable for Comp0GenEventIidxSpec {}
