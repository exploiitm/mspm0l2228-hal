#[doc = "Register `COMP0_CPU_INT_RIS` reader"]
pub type R = crate::R<Comp0CpuIntRisSpec>;
#[doc = "Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compifg {
    #[doc = "0: No interrupt pending"]
    Clr = 0,
    #[doc = "1: Interrupt pending"]
    Set = 1,
}
impl From<Compifg> for bool {
    #[inline(always)]
    fn from(variant: Compifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPIFG` reader - Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
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
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Compifg::Clr
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Compifg::Set
    }
}
#[doc = "Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compinvifg {
    #[doc = "0: No interrupt pending"]
    Clr = 0,
    #[doc = "1: Interrupt pending"]
    Set = 1,
}
impl From<Compinvifg> for bool {
    #[inline(always)]
    fn from(variant: Compinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPINVIFG` reader - Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
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
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Compinvifg::Clr
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Compinvifg::Set
    }
}
#[doc = "Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outrdyifg {
    #[doc = "0: No interrupt pending"]
    Clr = 0,
    #[doc = "1: Interrupt pending"]
    Set = 1,
}
impl From<Outrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Outrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTRDYIFG` reader - Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid."]
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
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Outrdyifg::Clr
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Outrdyifg::Set
    }
}
impl R {
    #[doc = "Bit 1 - Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
    #[inline(always)]
    pub fn compifg(&self) -> CompifgR {
        CompifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
    #[inline(always)]
    pub fn compinvifg(&self) -> CompinvifgR {
        CompinvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid."]
    #[inline(always)]
    pub fn outrdyifg(&self) -> OutrdyifgR {
        OutrdyifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0CpuIntRisSpec;
impl crate::RegisterSpec for Comp0CpuIntRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_cpu_int_ris::R`](R) reader structure"]
impl crate::Readable for Comp0CpuIntRisSpec {}
#[doc = "`reset()` method sets COMP0_CPU_INT_RIS to value 0"]
impl crate::Resettable for Comp0CpuIntRisSpec {}
