#[doc = "Register `CPUSS_INT_GROUP1_ISET` writer"]
pub type W = crate::W<CpussIntGroup1IsetSpec>;
#[doc = "Sets INT in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to INT is set"]
    Set = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` writer - Sets INT in RIS register"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG, Int>;
impl<'a, REG> IntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Int::NoEffect)
    }
    #[doc = "RIS bit corresponding to INT is set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Int::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Sets INT in RIS register"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, CpussIntGroup1IsetSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_int_group1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussIntGroup1IsetSpec;
impl crate::RegisterSpec for CpussIntGroup1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cpuss_int_group1_iset::W`](W) writer structure"]
impl crate::Writable for CpussIntGroup1IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUSS_INT_GROUP1_ISET to value 0"]
impl crate::Resettable for CpussIntGroup1IsetSpec {}
