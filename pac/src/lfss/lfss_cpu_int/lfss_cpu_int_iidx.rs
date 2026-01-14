#[doc = "Register `LFSS_CPU_INT_IIDX` reader"]
pub type R = crate::R<LfssCpuIntIidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: RTC ready"]
    Rtcrdy = 1,
    #[doc = "2: RTC time event"]
    Rtctev = 2,
    #[doc = "3: RTC alarm 1"]
    Rtca1 = 3,
    #[doc = "4: RTC alarm 2"]
    Rtca2 = 4,
    #[doc = "5: RTC prescale timer 0"]
    Rt0ps = 5,
    #[doc = "6: RTC prescale timer 1"]
    Rt1ps = 6,
    #[doc = "7: RTC prescale timer 2"]
    Rt2ps = 7,
    #[doc = "8: Time stamp event"]
    Tsevt = 8,
    #[doc = "9: Tamper I/O 0 event"]
    Tio0 = 9,
    #[doc = "10: Tamper I/O 1 event"]
    Tio1 = 10,
    #[doc = "11: Tamper I/O 2 event"]
    Tio2 = 11,
    #[doc = "12: Tamper I/O 3 event"]
    Tio3 = 12,
    #[doc = "13: Tamper I/O 4 event"]
    Tio4 = 13,
    #[doc = "14: Tamper I/O 5 event"]
    Tio5 = 14,
    #[doc = "15: Tamper I/O 6 event"]
    Tio6 = 15,
    #[doc = "16: Tamper I/O 7 event"]
    Tio7 = 16,
    #[doc = "17: Tamper I/O 8 event"]
    Tio8 = 17,
    #[doc = "18: Tamper I/O 9 event"]
    Tio9 = 18,
    #[doc = "19: Tamper I/O 10 event"]
    Tio10 = 19,
    #[doc = "20: Tamper I/O 11 event"]
    Tio11 = 20,
    #[doc = "21: Tamper I/O 12 event"]
    Tio12 = 21,
    #[doc = "22: Tamper I/O 13 event"]
    Tio13 = 22,
    #[doc = "23: Tamper I/O 14 event"]
    Tio14 = 23,
    #[doc = "24: Tamper I/O 15 event"]
    Tio15 = 24,
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
            1 => Some(Stat::Rtcrdy),
            2 => Some(Stat::Rtctev),
            3 => Some(Stat::Rtca1),
            4 => Some(Stat::Rtca2),
            5 => Some(Stat::Rt0ps),
            6 => Some(Stat::Rt1ps),
            7 => Some(Stat::Rt2ps),
            8 => Some(Stat::Tsevt),
            9 => Some(Stat::Tio0),
            10 => Some(Stat::Tio1),
            11 => Some(Stat::Tio2),
            12 => Some(Stat::Tio3),
            13 => Some(Stat::Tio4),
            14 => Some(Stat::Tio5),
            15 => Some(Stat::Tio6),
            16 => Some(Stat::Tio7),
            17 => Some(Stat::Tio8),
            18 => Some(Stat::Tio9),
            19 => Some(Stat::Tio10),
            20 => Some(Stat::Tio11),
            21 => Some(Stat::Tio12),
            22 => Some(Stat::Tio13),
            23 => Some(Stat::Tio14),
            24 => Some(Stat::Tio15),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "RTC ready"]
    #[inline(always)]
    pub fn is_rtcrdy(&self) -> bool {
        *self == Stat::Rtcrdy
    }
    #[doc = "RTC time event"]
    #[inline(always)]
    pub fn is_rtctev(&self) -> bool {
        *self == Stat::Rtctev
    }
    #[doc = "RTC alarm 1"]
    #[inline(always)]
    pub fn is_rtca1(&self) -> bool {
        *self == Stat::Rtca1
    }
    #[doc = "RTC alarm 2"]
    #[inline(always)]
    pub fn is_rtca2(&self) -> bool {
        *self == Stat::Rtca2
    }
    #[doc = "RTC prescale timer 0"]
    #[inline(always)]
    pub fn is_rt0ps(&self) -> bool {
        *self == Stat::Rt0ps
    }
    #[doc = "RTC prescale timer 1"]
    #[inline(always)]
    pub fn is_rt1ps(&self) -> bool {
        *self == Stat::Rt1ps
    }
    #[doc = "RTC prescale timer 2"]
    #[inline(always)]
    pub fn is_rt2ps(&self) -> bool {
        *self == Stat::Rt2ps
    }
    #[doc = "Time stamp event"]
    #[inline(always)]
    pub fn is_tsevt(&self) -> bool {
        *self == Stat::Tsevt
    }
    #[doc = "Tamper I/O 0 event"]
    #[inline(always)]
    pub fn is_tio0(&self) -> bool {
        *self == Stat::Tio0
    }
    #[doc = "Tamper I/O 1 event"]
    #[inline(always)]
    pub fn is_tio1(&self) -> bool {
        *self == Stat::Tio1
    }
    #[doc = "Tamper I/O 2 event"]
    #[inline(always)]
    pub fn is_tio2(&self) -> bool {
        *self == Stat::Tio2
    }
    #[doc = "Tamper I/O 3 event"]
    #[inline(always)]
    pub fn is_tio3(&self) -> bool {
        *self == Stat::Tio3
    }
    #[doc = "Tamper I/O 4 event"]
    #[inline(always)]
    pub fn is_tio4(&self) -> bool {
        *self == Stat::Tio4
    }
    #[doc = "Tamper I/O 5 event"]
    #[inline(always)]
    pub fn is_tio5(&self) -> bool {
        *self == Stat::Tio5
    }
    #[doc = "Tamper I/O 6 event"]
    #[inline(always)]
    pub fn is_tio6(&self) -> bool {
        *self == Stat::Tio6
    }
    #[doc = "Tamper I/O 7 event"]
    #[inline(always)]
    pub fn is_tio7(&self) -> bool {
        *self == Stat::Tio7
    }
    #[doc = "Tamper I/O 8 event"]
    #[inline(always)]
    pub fn is_tio8(&self) -> bool {
        *self == Stat::Tio8
    }
    #[doc = "Tamper I/O 9 event"]
    #[inline(always)]
    pub fn is_tio9(&self) -> bool {
        *self == Stat::Tio9
    }
    #[doc = "Tamper I/O 10 event"]
    #[inline(always)]
    pub fn is_tio10(&self) -> bool {
        *self == Stat::Tio10
    }
    #[doc = "Tamper I/O 11 event"]
    #[inline(always)]
    pub fn is_tio11(&self) -> bool {
        *self == Stat::Tio11
    }
    #[doc = "Tamper I/O 12 event"]
    #[inline(always)]
    pub fn is_tio12(&self) -> bool {
        *self == Stat::Tio12
    }
    #[doc = "Tamper I/O 13 event"]
    #[inline(always)]
    pub fn is_tio13(&self) -> bool {
        *self == Stat::Tio13
    }
    #[doc = "Tamper I/O 14 event"]
    #[inline(always)]
    pub fn is_tio14(&self) -> bool {
        *self == Stat::Tio14
    }
    #[doc = "Tamper I/O 15 event"]
    #[inline(always)]
    pub fn is_tio15(&self) -> bool {
        *self == Stat::Tio15
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cpu_int_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssCpuIntIidxSpec;
impl crate::RegisterSpec for LfssCpuIntIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_cpu_int_iidx::R`](R) reader structure"]
impl crate::Readable for LfssCpuIntIidxSpec {}
#[doc = "`reset()` method sets LFSS_CPU_INT_IIDX to value 0"]
impl crate::Resettable for LfssCpuIntIidxSpec {}
