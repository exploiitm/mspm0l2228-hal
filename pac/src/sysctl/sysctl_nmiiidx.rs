#[doc = "Register `SYSCTL_NMIIIDX` reader"]
pub type R = crate::R<SysctlNmiiidxSpec>;
#[doc = "The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No NMI pending"]
    NoIntr = 0,
    #[doc = "1: BOR Threshold NMI pending"]
    Borlvl = 1,
    #[doc = "2: `10`"]
    Wwdt0 = 2,
    #[doc = "3: `11`"]
    Lfclkfail = 3,
    #[doc = "4: `100`"]
    Flashded = 4,
    #[doc = "5: `101`"]
    Sramded = 5,
    #[doc = "6: `110`"]
    Vbatdn = 6,
    #[doc = "7: `111`"]
    Vbatup = 7,
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
#[doc = "Field `STAT` reader - The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register."]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stat {
        match self.bits {
            0 => Stat::NoIntr,
            1 => Stat::Borlvl,
            2 => Stat::Wwdt0,
            3 => Stat::Lfclkfail,
            4 => Stat::Flashded,
            5 => Stat::Sramded,
            6 => Stat::Vbatdn,
            7 => Stat::Vbatup,
            _ => unreachable!(),
        }
    }
    #[doc = "No NMI pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "BOR Threshold NMI pending"]
    #[inline(always)]
    pub fn is_borlvl(&self) -> bool {
        *self == Stat::Borlvl
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_wwdt0(&self) -> bool {
        *self == Stat::Wwdt0
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_lfclkfail(&self) -> bool {
        *self == Stat::Lfclkfail
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_flashded(&self) -> bool {
        *self == Stat::Flashded
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_sramded(&self) -> bool {
        *self == Stat::Sramded
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_vbatdn(&self) -> bool {
        *self == Stat::Vbatdn
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_vbatup(&self) -> bool {
        *self == Stat::Vbatup
    }
}
impl R {
    #[doc = "Bits 0:2 - The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 7) as u8)
    }
}
#[doc = "NMI interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_nmiiidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlNmiiidxSpec;
impl crate::RegisterSpec for SysctlNmiiidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_nmiiidx::R`](R) reader structure"]
impl crate::Readable for SysctlNmiiidxSpec {}
#[doc = "`reset()` method sets SYSCTL_NMIIIDX to value 0"]
impl crate::Resettable for SysctlNmiiidxSpec {}
