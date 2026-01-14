#[doc = "Register `EVENTLP_DIAGSTAT` reader"]
pub type R = crate::R<EventlpDiagstatSpec>;
#[doc = "Current diagnostic state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: No failures and no diagnostic"]
    None = 0,
    #[doc = "1: Functional Failure"]
    FuncFail = 1,
    #[doc = "2: Diagnostic Failure"]
    DiagFail = 2,
    #[doc = "4: Diagnostic Pass"]
    DiagPass = 4,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - Current diagnostic state"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::None),
            1 => Some(State::FuncFail),
            2 => Some(State::DiagFail),
            4 => Some(State::DiagPass),
            _ => None,
        }
    }
    #[doc = "No failures and no diagnostic"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == State::None
    }
    #[doc = "Functional Failure"]
    #[inline(always)]
    pub fn is_func_fail(&self) -> bool {
        *self == State::FuncFail
    }
    #[doc = "Diagnostic Failure"]
    #[inline(always)]
    pub fn is_diag_fail(&self) -> bool {
        *self == State::DiagFail
    }
    #[doc = "Diagnostic Pass"]
    #[inline(always)]
    pub fn is_diag_pass(&self) -> bool {
        *self == State::DiagPass
    }
}
#[doc = "Field `NUMDIAG` reader - This is a hardware constant that indicates how many DIAGPAR registers are included in this SFTYDIAG sub-region."]
pub type NumdiagR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - Current diagnostic state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:25 - This is a hardware constant that indicates how many DIAGPAR registers are included in this SFTYDIAG sub-region."]
    #[inline(always)]
    pub fn numdiag(&self) -> NumdiagR {
        NumdiagR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "Diagnostic Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_diagstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpDiagstatSpec;
impl crate::RegisterSpec for EventlpDiagstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_diagstat::R`](R) reader structure"]
impl crate::Readable for EventlpDiagstatSpec {}
#[doc = "`reset()` method sets EVENTLP_DIAGSTAT to value 0x0301_0000"]
impl crate::Resettable for EventlpDiagstatSpec {
    const RESET_VALUE: u32 = 0x0301_0000;
}
