#[doc = "Register `CPUSS_INT_GROUP1_IMASK` reader"]
pub type R = crate::R<CpussIntGroup1ImaskSpec>;
#[doc = "Masks the corresponding interrupt\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Int {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
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
#[doc = "Field `INT` reader - Masks the corresponding interrupt"]
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
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Int::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Int::Set
    }
}
impl R {
    #[doc = "Bits 0:7 - Masks the corresponding interrupt"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_imask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussIntGroup1ImaskSpec;
impl crate::RegisterSpec for CpussIntGroup1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_int_group1_imask::R`](R) reader structure"]
impl crate::Readable for CpussIntGroup1ImaskSpec {}
#[doc = "`reset()` method sets CPUSS_INT_GROUP1_IMASK to value 0xff"]
impl crate::Resettable for CpussIntGroup1ImaskSpec {
    const RESET_VALUE: u32 = 0xff;
}
