#[doc = "Register `COMP0_STAT` reader"]
pub type R = crate::R<Comp0StatSpec>;
#[doc = "This bit reflects the value of the comparator output. Writing to this bit has no effect on the comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    #[doc = "0: Comparator output is low"]
    Low = 0,
    #[doc = "1: Comparator output is high"]
    High = 1,
}
impl From<Out> for bool {
    #[inline(always)]
    fn from(variant: Out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT` reader - This bit reflects the value of the comparator output. Writing to this bit has no effect on the comparator output."]
pub type OutR = crate::BitReader<Out>;
impl OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out {
        match self.bits {
            false => Out::Low,
            true => Out::High,
        }
    }
    #[doc = "Comparator output is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Out::Low
    }
    #[doc = "Comparator output is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Out::High
    }
}
impl R {
    #[doc = "Bit 0 - This bit reflects the value of the comparator output. Writing to this bit has no effect on the comparator output."]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0StatSpec;
impl crate::RegisterSpec for Comp0StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_stat::R`](R) reader structure"]
impl crate::Readable for Comp0StatSpec {}
#[doc = "`reset()` method sets COMP0_STAT to value 0"]
impl crate::Resettable for Comp0StatSpec {}
