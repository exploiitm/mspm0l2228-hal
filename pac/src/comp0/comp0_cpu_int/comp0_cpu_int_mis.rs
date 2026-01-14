#[doc = "Register `COMP0_CPU_INT_MIS` reader"]
pub type R = crate::R<Comp0CpuIntMisSpec>;
#[doc = "Masked interrupt status for COMPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compifg {
    #[doc = "0: COMPIFG does not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: COMPIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Compifg> for bool {
    #[inline(always)]
    fn from(variant: Compifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPIFG` reader - Masked interrupt status for COMPIFG"]
pub type CompifgR = crate::BitReader<Compifg>;
impl CompifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compifg {
        match self.bits {
            false => Compifg::Clr,
            true => Compifg::Set,
        }
    }
    #[doc = "COMPIFG does not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Compifg::Clr
    }
    #[doc = "COMPIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Compifg::Set
    }
}
#[doc = "Masked interrupt status for COMPINVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compinvifg {
    #[doc = "0: COMPINVIFG does not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: COMPINVIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Compinvifg> for bool {
    #[inline(always)]
    fn from(variant: Compinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPINVIFG` reader - Masked interrupt status for COMPINVIFG"]
pub type CompinvifgR = crate::BitReader<Compinvifg>;
impl CompinvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compinvifg {
        match self.bits {
            false => Compinvifg::Clr,
            true => Compinvifg::Set,
        }
    }
    #[doc = "COMPINVIFG does not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Compinvifg::Clr
    }
    #[doc = "COMPINVIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Compinvifg::Set
    }
}
#[doc = "Masked interrupt status for OUTRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outrdyifg {
    #[doc = "0: OUTRDYIFG does not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: OUTRDYIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Outrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Outrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTRDYIFG` reader - Masked interrupt status for OUTRDYIFG"]
pub type OutrdyifgR = crate::BitReader<Outrdyifg>;
impl OutrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outrdyifg {
        match self.bits {
            false => Outrdyifg::Clr,
            true => Outrdyifg::Set,
        }
    }
    #[doc = "OUTRDYIFG does not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Outrdyifg::Clr
    }
    #[doc = "OUTRDYIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Outrdyifg::Set
    }
}
impl R {
    #[doc = "Bit 1 - Masked interrupt status for COMPIFG"]
    #[inline(always)]
    pub fn compifg(&self) -> CompifgR {
        CompifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt status for COMPINVIFG"]
    #[inline(always)]
    pub fn compinvifg(&self) -> CompinvifgR {
        CompinvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt status for OUTRDYIFG"]
    #[inline(always)]
    pub fn outrdyifg(&self) -> OutrdyifgR {
        OutrdyifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0CpuIntMisSpec;
impl crate::RegisterSpec for Comp0CpuIntMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_cpu_int_mis::R`](R) reader structure"]
impl crate::Readable for Comp0CpuIntMisSpec {}
#[doc = "`reset()` method sets COMP0_CPU_INT_MIS to value 0"]
impl crate::Resettable for Comp0CpuIntMisSpec {}
