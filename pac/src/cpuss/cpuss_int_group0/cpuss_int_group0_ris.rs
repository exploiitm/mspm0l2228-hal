#[doc = "Register `CPUSS_INT_GROUP0_RIS` reader"]
pub type R = crate::R<CpussIntGroup0RisSpec>;
#[doc = "Raw interrupt status for INT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Int {
    #[doc = "0: INT0 did not occur."]
    Clr = 0,
    #[doc = "1: INT0 occurred."]
    Set = 1,
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Int {
    type Ux = u8;
}
impl crate::IsEnum for Int {}
#[doc = "Field `INT` reader - Raw interrupt status for INT"]
pub type IntR = crate::FieldReader<Int>;
impl IntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Int> {
        match self.bits {
            0 => Some(Int::Clr),
            1 => Some(Int::Set),
            _ => None,
        }
    }
    #[doc = "INT0 did not occur."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Int::Clr
    }
    #[doc = "INT0 occurred."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Int::Set
    }
}
impl R {
    #[doc = "Bits 0:7 - Raw interrupt status for INT"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussIntGroup0RisSpec;
impl crate::RegisterSpec for CpussIntGroup0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_int_group0_ris::R`](R) reader structure"]
impl crate::Readable for CpussIntGroup0RisSpec {}
#[doc = "`reset()` method sets CPUSS_INT_GROUP0_RIS to value 0"]
impl crate::Resettable for CpussIntGroup0RisSpec {}
