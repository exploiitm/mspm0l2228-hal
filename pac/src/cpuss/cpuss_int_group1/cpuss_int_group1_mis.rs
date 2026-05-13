#[doc = "Register `CPUSS_INT_GROUP1_MIS` reader"]
pub type R = crate::R<CpussIntGroup1MisSpec>;
#[doc = "Masked interrupt status for INT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: INT does not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: INT requests an interrupt service routine"]
    Set = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Masked interrupt status for INT0"]
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
    #[doc = "INT does not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Int::Clr
    }
    #[doc = "INT requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Int::Set
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt status for INT0"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussIntGroup1MisSpec;
impl crate::RegisterSpec for CpussIntGroup1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_int_group1_mis::R`](R) reader structure"]
impl crate::Readable for CpussIntGroup1MisSpec {}
#[doc = "`reset()` method sets CPUSS_INT_GROUP1_MIS to value 0"]
impl crate::Resettable for CpussIntGroup1MisSpec {}
