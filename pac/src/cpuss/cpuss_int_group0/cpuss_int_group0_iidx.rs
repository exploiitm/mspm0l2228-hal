#[doc = "Register `CPUSS_INT_GROUP0_IIDX` reader"]
pub type R = crate::R<CpussIntGroup0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No pending interrupt"]
    NoIntr = 0,
    #[doc = "1: Interrupt 0"]
    Int0 = 1,
    #[doc = "2: Interrupt 1"]
    Int1 = 2,
    #[doc = "3: Interrupt 2"]
    Int2 = 3,
    #[doc = "4: Interrupt 3"]
    Int3 = 4,
    #[doc = "5: Interrupt 4"]
    Int4 = 5,
    #[doc = "6: Interrupt 5"]
    Int5 = 6,
    #[doc = "7: Interrupt 6"]
    Int6 = 7,
    #[doc = "8: Interrupt 7"]
    Int7 = 8,
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
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Int0),
            2 => Some(Stat::Int1),
            3 => Some(Stat::Int2),
            4 => Some(Stat::Int3),
            5 => Some(Stat::Int4),
            6 => Some(Stat::Int5),
            7 => Some(Stat::Int6),
            8 => Some(Stat::Int7),
            _ => None,
        }
    }
    #[doc = "No pending interrupt"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn is_int0(&self) -> bool {
        *self == Stat::Int0
    }
    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn is_int1(&self) -> bool {
        *self == Stat::Int1
    }
    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn is_int2(&self) -> bool {
        *self == Stat::Int2
    }
    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn is_int3(&self) -> bool {
        *self == Stat::Int3
    }
    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn is_int4(&self) -> bool {
        *self == Stat::Int4
    }
    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn is_int5(&self) -> bool {
        *self == Stat::Int5
    }
    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn is_int6(&self) -> bool {
        *self == Stat::Int6
    }
    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn is_int7(&self) -> bool {
        *self == Stat::Int7
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_int_group0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussIntGroup0IidxSpec;
impl crate::RegisterSpec for CpussIntGroup0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_int_group0_iidx::R`](R) reader structure"]
impl crate::Readable for CpussIntGroup0IidxSpec {}
#[doc = "`reset()` method sets CPUSS_INT_GROUP0_IIDX to value 0"]
impl crate::Resettable for CpussIntGroup0IidxSpec {}
