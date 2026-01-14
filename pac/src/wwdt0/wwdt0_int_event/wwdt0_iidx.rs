#[doc = "Register `WWDT0_IIDX` reader"]
pub type R = crate::R<Wwdt0IidxSpec>;
#[doc = "Module Interrupt Vector Value. This register provides the highest priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Interval Timer Interrupt; Interrupt Flag: INTTIM; Interrupt Priority: Highest"]
    Inttim = 1,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Module Interrupt Vector Value. This register provides the highest priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC."]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Inttim),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Interval Timer Interrupt; Interrupt Flag: INTTIM; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_inttim(&self) -> bool {
        *self == Stat::Inttim
    }
}
impl R {
    #[doc = "Bits 0:4 - Module Interrupt Vector Value. This register provides the highest priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0IidxSpec;
impl crate::RegisterSpec for Wwdt0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_iidx::R`](R) reader structure"]
impl crate::Readable for Wwdt0IidxSpec {}
#[doc = "`reset()` method sets WWDT0_IIDX to value 0"]
impl crate::Resettable for Wwdt0IidxSpec {}
