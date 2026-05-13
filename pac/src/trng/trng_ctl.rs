#[doc = "Register `TRNG_CTL` reader"]
pub type R = crate::R<TrngCtlSpec>;
#[doc = "Register `TRNG_CTL` writer"]
pub type W = crate::W<TrngCtlSpec>;
#[doc = "Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "0: Turns the power off of the analog source and clocks the digital interface"]
    PwrOff = 0,
    #[doc = "1: Initiates the powerup test sequence for the digital components. This verifies that the digital components are properly working. IRQ_CMD_DONE indicates that the test is done. The results of this test are in bits 0:6 in TEST_RESULTS register"]
    PwrupDig = 1,
    #[doc = "2: Initiates the powerup test sequence for the analog TRNG. This verifies that the analog component is generating sequences with enough entropy. IRQ_CMD_DONE indicates that the test is done. The results of this test are in bit 7 in TEST_RESULTS register"]
    PwrupAna = 2,
    #[doc = "3: Normal operating mode for TRNG. All components are turned on."]
    NormFunc = 3,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` reader - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC"]
pub type CmdR = crate::FieldReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd {
        match self.bits {
            0 => Cmd::PwrOff,
            1 => Cmd::PwrupDig,
            2 => Cmd::PwrupAna,
            3 => Cmd::NormFunc,
            _ => unreachable!(),
        }
    }
    #[doc = "Turns the power off of the analog source and clocks the digital interface"]
    #[inline(always)]
    pub fn is_pwr_off(&self) -> bool {
        *self == Cmd::PwrOff
    }
    #[doc = "Initiates the powerup test sequence for the digital components. This verifies that the digital components are properly working. IRQ_CMD_DONE indicates that the test is done. The results of this test are in bits 0:6 in TEST_RESULTS register"]
    #[inline(always)]
    pub fn is_pwrup_dig(&self) -> bool {
        *self == Cmd::PwrupDig
    }
    #[doc = "Initiates the powerup test sequence for the analog TRNG. This verifies that the analog component is generating sequences with enough entropy. IRQ_CMD_DONE indicates that the test is done. The results of this test are in bit 7 in TEST_RESULTS register"]
    #[inline(always)]
    pub fn is_pwrup_ana(&self) -> bool {
        *self == Cmd::PwrupAna
    }
    #[doc = "Normal operating mode for TRNG. All components are turned on."]
    #[inline(always)]
    pub fn is_norm_func(&self) -> bool {
        *self == Cmd::NormFunc
    }
}
#[doc = "Field `CMD` writer - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd, crate::Safe>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Turns the power off of the analog source and clocks the digital interface"]
    #[inline(always)]
    pub fn pwr_off(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::PwrOff)
    }
    #[doc = "Initiates the powerup test sequence for the digital components. This verifies that the digital components are properly working. IRQ_CMD_DONE indicates that the test is done. The results of this test are in bits 0:6 in TEST_RESULTS register"]
    #[inline(always)]
    pub fn pwrup_dig(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::PwrupDig)
    }
    #[doc = "Initiates the powerup test sequence for the analog TRNG. This verifies that the analog component is generating sequences with enough entropy. IRQ_CMD_DONE indicates that the test is done. The results of this test are in bit 7 in TEST_RESULTS register"]
    #[inline(always)]
    pub fn pwrup_ana(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::PwrupAna)
    }
    #[doc = "Normal operating mode for TRNG. All components are turned on."]
    #[inline(always)]
    pub fn norm_func(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::NormFunc)
    }
}
#[doc = "Field `DECIM_RATE` reader - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
pub type DecimRateR = crate::FieldReader;
#[doc = "Field `DECIM_RATE` writer - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
pub type DecimRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PWRUP_CLKDIV` reader - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
pub type PwrupClkdivR = crate::BitReader;
#[doc = "Field `PWRUP_CLKDIV` writer - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
pub type PwrupClkdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUP_PCHRG_CFG` reader - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
pub type PwrupPchrgCfgR = crate::FieldReader;
#[doc = "Field `PWRUP_PCHRG_CFG` writer - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
pub type PwrupPchrgCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRUP_PSTART_CFG` reader - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
pub type PwrupPstartCfgR = crate::FieldReader;
#[doc = "Field `PWRUP_PSTART_CFG` writer - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
pub type PwrupPstartCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
    #[inline(always)]
    pub fn decim_rate(&self) -> DecimRateR {
        DecimRateR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
    #[inline(always)]
    pub fn pwrup_clkdiv(&self) -> PwrupClkdivR {
        PwrupClkdivR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
    #[inline(always)]
    pub fn pwrup_pchrg_cfg(&self) -> PwrupPchrgCfgR {
        PwrupPchrgCfgR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
    #[inline(always)]
    pub fn pwrup_pstart_cfg(&self) -> PwrupPstartCfgR {
        PwrupPstartCfgR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --> OFF 01 --> PWRUP_DIG 10 --> PWRUP_ANA 11 --> NORM_FUNC"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, TrngCtlSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
    #[inline(always)]
    pub fn decim_rate(&mut self) -> DecimRateW<'_, TrngCtlSpec> {
        DecimRateW::new(self, 8)
    }
    #[doc = "Bit 16 - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
    #[inline(always)]
    pub fn pwrup_clkdiv(&mut self) -> PwrupClkdivW<'_, TrngCtlSpec> {
        PwrupClkdivW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
    #[inline(always)]
    pub fn pwrup_pchrg_cfg(&mut self) -> PwrupPchrgCfgW<'_, TrngCtlSpec> {
        PwrupPchrgCfgW::new(self, 17)
    }
    #[doc = "Bits 19:20 - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
    #[inline(always)]
    pub fn pwrup_pstart_cfg(&mut self) -> PwrupPstartCfgW<'_, TrngCtlSpec> {
        PwrupPstartCfgW::new(self, 19)
    }
}
#[doc = "Controls the command and decimation rate\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngCtlSpec;
impl crate::RegisterSpec for TrngCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_ctl::R`](R) reader structure"]
impl crate::Readable for TrngCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`trng_ctl::W`](W) writer structure"]
impl crate::Writable for TrngCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRNG_CTL to value 0x0014_0000"]
impl crate::Resettable for TrngCtlSpec {
    const RESET_VALUE: u32 = 0x0014_0000;
}
