#[doc = "Register `TRNG_STAT` reader"]
pub type R = crate::R<TrngStatSpec>;
#[doc = "Field `ADAP_FAIL` reader - Indicates that the Adaptive Proportion Test (1,2,3, or 4-bit counters) failed by having too many or too few counted samples in the last 1024 bit window."]
pub type AdapFailR = crate::BitReader;
#[doc = "Field `REP_FAIL` reader - Indicates that the repetition counter test caused the most recent failure. Thus, the health count numbers are most likely not for a complete 1024-bit window."]
pub type RepFailR = crate::BitReader;
#[doc = "Field `ISSUED_CMD` reader - Indicates the last accepted command that is issued to the TRNG interface. Upon writing a valid command, this register will update and the command will be in progress until CMD_DONE_IRQ is set. CMD_DONE_IRQ indicates that the state is in PWROFF, NORM_FUNC, or ERROR. These states will accept new commands. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC"]
pub type IssuedCmdR = crate::FieldReader;
#[doc = "Field `FSM_STATE` reader - Current state of the front end FSM (behind a clock domain crossing). 2 reads are REQUIRED as there is a chance of metastability when reading this States: 0000: OFF 0001: PWRUP_ES 0011: NORM_FUNC 0111: TEST_DIG 1011: TEST_ANA 1010: ERROR 0010: PWRDOWN_ES"]
pub type FsmStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates that the Adaptive Proportion Test (1,2,3, or 4-bit counters) failed by having too many or too few counted samples in the last 1024 bit window."]
    #[inline(always)]
    pub fn adap_fail(&self) -> AdapFailR {
        AdapFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the repetition counter test caused the most recent failure. Thus, the health count numbers are most likely not for a complete 1024-bit window."]
    #[inline(always)]
    pub fn rep_fail(&self) -> RepFailR {
        RepFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates the last accepted command that is issued to the TRNG interface. Upon writing a valid command, this register will update and the command will be in progress until CMD_DONE_IRQ is set. CMD_DONE_IRQ indicates that the state is in PWROFF, NORM_FUNC, or ERROR. These states will accept new commands. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC"]
    #[inline(always)]
    pub fn issued_cmd(&self) -> IssuedCmdR {
        IssuedCmdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Current state of the front end FSM (behind a clock domain crossing). 2 reads are REQUIRED as there is a chance of metastability when reading this States: 0000: OFF 0001: PWRUP_ES 0011: NORM_FUNC 0111: TEST_DIG 1011: TEST_ANA 1010: ERROR 0010: PWRDOWN_ES"]
    #[inline(always)]
    pub fn fsm_state(&self) -> FsmStateR {
        FsmStateR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Status register that informs health test results and last issued command\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngStatSpec;
impl crate::RegisterSpec for TrngStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_stat::R`](R) reader structure"]
impl crate::Readable for TrngStatSpec {}
#[doc = "`reset()` method sets TRNG_STAT to value 0"]
impl crate::Resettable for TrngStatSpec {}
