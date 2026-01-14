#[doc = "Register `TIMA0_CCCTL_45[%s]` reader"]
pub type R = crate::R<Tima0Ccctl45Spec>;
#[doc = "Register `TIMA0_CCCTL_45[%s]` writer"]
pub type W = crate::W<Tima0Ccctl45Spec>;
#[doc = "Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccupd {
    #[doc = "0: Writes to the CCx_y register is written to the register directly and has immediate effect."]
    Immediately = 0,
    #[doc = "1: Following a zero event Writes to the CCx_y register are stored in shadow register and transferred to ECCx_y in the TIMCLK cycle following CTR equals 0."]
    ZeroEvt = 1,
    #[doc = "2: Following a compare (down) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    CompareDownEvt = 2,
    #[doc = "3: Following a compare (up) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    CompareUpEvt = 3,
    #[doc = "4: Following a zero or load event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LD. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
    ZeroLoadEvt = 4,
    #[doc = "5: Following a zero event with repeat count also zero. Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0 and if RC equal 0."]
    ZeroRcZeroEvt = 5,
    #[doc = "6: Following a TRIG pulse. Writes to the CCx_y register are stored in shadow register and transferred to CCx_y #xD; 0."]
    Trig = 6,
}
impl From<Ccupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccupd {}
#[doc = "Field `CCUPD` reader - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type CcupdR = crate::FieldReader<Ccupd>;
impl CcupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccupd> {
        match self.bits {
            0 => Some(Ccupd::Immediately),
            1 => Some(Ccupd::ZeroEvt),
            2 => Some(Ccupd::CompareDownEvt),
            3 => Some(Ccupd::CompareUpEvt),
            4 => Some(Ccupd::ZeroLoadEvt),
            5 => Some(Ccupd::ZeroRcZeroEvt),
            6 => Some(Ccupd::Trig),
            _ => None,
        }
    }
    #[doc = "Writes to the CCx_y register is written to the register directly and has immediate effect."]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == Ccupd::Immediately
    }
    #[doc = "Following a zero event Writes to the CCx_y register are stored in shadow register and transferred to ECCx_y in the TIMCLK cycle following CTR equals 0."]
    #[inline(always)]
    pub fn is_zero_evt(&self) -> bool {
        *self == Ccupd::ZeroEvt
    }
    #[doc = "Following a compare (down) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn is_compare_down_evt(&self) -> bool {
        *self == Ccupd::CompareDownEvt
    }
    #[doc = "Following a compare (up) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn is_compare_up_evt(&self) -> bool {
        *self == Ccupd::CompareUpEvt
    }
    #[doc = "Following a zero or load event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LD. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
    #[inline(always)]
    pub fn is_zero_load_evt(&self) -> bool {
        *self == Ccupd::ZeroLoadEvt
    }
    #[doc = "Following a zero event with repeat count also zero. Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0 and if RC equal 0."]
    #[inline(always)]
    pub fn is_zero_rc_zero_evt(&self) -> bool {
        *self == Ccupd::ZeroRcZeroEvt
    }
    #[doc = "Following a TRIG pulse. Writes to the CCx_y register are stored in shadow register and transferred to CCx_y #xD; 0."]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Ccupd::Trig
    }
}
#[doc = "Field `CCUPD` writer - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type CcupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccupd>;
impl<'a, REG> CcupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writes to the CCx_y register is written to the register directly and has immediate effect."]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::Immediately)
    }
    #[doc = "Following a zero event Writes to the CCx_y register are stored in shadow register and transferred to ECCx_y in the TIMCLK cycle following CTR equals 0."]
    #[inline(always)]
    pub fn zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::ZeroEvt)
    }
    #[doc = "Following a compare (down) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::CompareDownEvt)
    }
    #[doc = "Following a compare (up) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::CompareUpEvt)
    }
    #[doc = "Following a zero or load event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LD. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
    #[inline(always)]
    pub fn zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::ZeroLoadEvt)
    }
    #[doc = "Following a zero event with repeat count also zero. Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0 and if RC equal 0."]
    #[inline(always)]
    pub fn zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::ZeroRcZeroEvt)
    }
    #[doc = "Following a TRIG pulse. Writes to the CCx_y register are stored in shadow register and transferred to CCx_y #xD; 0."]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccupd::Trig)
    }
}
#[doc = "Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scercnez {
    #[doc = "0: CCD, CCU and RC events are always generated from the counter when their conditions are generated."]
    Disabled = 0,
    #[doc = "1: CCD, CCU and RC events are generated from the counter when their conditions are generated and the RC register value is 0."]
    Enabled = 1,
}
impl From<Scercnez> for bool {
    #[inline(always)]
    fn from(variant: Scercnez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCERCNEZ` reader - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
pub type ScercnezR = crate::BitReader<Scercnez>;
impl ScercnezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scercnez {
        match self.bits {
            false => Scercnez::Disabled,
            true => Scercnez::Enabled,
        }
    }
    #[doc = "CCD, CCU and RC events are always generated from the counter when their conditions are generated."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scercnez::Disabled
    }
    #[doc = "CCD, CCU and RC events are generated from the counter when their conditions are generated and the RC register value is 0."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scercnez::Enabled
    }
}
#[doc = "Field `SCERCNEZ` writer - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
pub type ScercnezW<'a, REG> = crate::BitWriter<'a, REG, Scercnez>;
impl<'a, REG> ScercnezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCD, CCU and RC events are always generated from the counter when their conditions are generated."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scercnez::Disabled)
    }
    #[doc = "CCD, CCU and RC events are generated from the counter when their conditions are generated and the RC register value is 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scercnez::Enabled)
    }
}
impl R {
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccupd(&self) -> CcupdR {
        CcupdR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn scercnez(&self) -> ScercnezR {
        ScercnezR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccupd(&mut self) -> CcupdW<'_, Tima0Ccctl45Spec> {
        CcupdW::new(self, 18)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn scercnez(&mut self) -> ScercnezW<'_, Tima0Ccctl45Spec> {
        ScercnezW::new(self, 25)
    }
}
#[doc = "Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccctl_45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccctl_45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0Ccctl45Spec;
impl crate::RegisterSpec for Tima0Ccctl45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_ccctl_45::R`](R) reader structure"]
impl crate::Readable for Tima0Ccctl45Spec {}
#[doc = "`write(|w| ..)` method takes [`tima0_ccctl_45::W`](W) writer structure"]
impl crate::Writable for Tima0Ccctl45Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_CCCTL_45[%s] to value 0"]
impl crate::Resettable for Tima0Ccctl45Spec {}
