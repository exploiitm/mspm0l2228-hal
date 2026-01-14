#[doc = "Register `CPUSS_INT_GROUP1_RIS` reader"]
pub type R = crate::R<CpussIntGroup1RisSpec>;
#[doc = "Raw interrupt status for INT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: INT0 did not occur."]
    Clr = 0,
    #[doc = "1: INT0 occurred."]
    Set = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Raw interrupt status for INT"]
pub type IntR = crate::BitReader<Int>;
impl IntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int {
        match self.bits {
            false => Int::Clr,
            true => Int::Set,
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
    #[doc = "Bit 0 - Raw interrupt status for INT"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussIntGroup1RisSpec;
impl crate::RegisterSpec for CpussIntGroup1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_int_group1_ris::R`](R) reader structure"]
impl crate::Readable for CpussIntGroup1RisSpec {}
#[doc = "`reset()` method sets CPUSS_INT_GROUP1_RIS to value 0"]
impl crate::Resettable for CpussIntGroup1RisSpec {}
