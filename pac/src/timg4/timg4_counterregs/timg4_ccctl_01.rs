#[doc = "Register `TIMG4_CCCTL_01[%s]` reader"]
pub type R = crate::R<Timg4Ccctl01Spec>;
#[doc = "Register `TIMG4_CCCTL_01[%s]` writer"]
pub type W = crate::W<Timg4Ccctl01Spec>;
#[doc = "Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccond {
    #[doc = "0: None (never captures)"]
    Nocapture = 0,
    #[doc = "1: Rising edge of CCP or trigger assertion edge"]
    CcTrigRise = 1,
    #[doc = "2: Falling edge of CCP or trigger de-assertion edge"]
    CcTrigFall = 2,
    #[doc = "3: Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    CcTrigEdge = 3,
}
impl From<Ccond> for u8 {
    #[inline(always)]
    fn from(variant: Ccond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccond {
    type Ux = u8;
}
impl crate::IsEnum for Ccond {}
#[doc = "Field `CCOND` reader - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
pub type CcondR = crate::FieldReader<Ccond>;
impl CcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccond> {
        match self.bits {
            0 => Some(Ccond::Nocapture),
            1 => Some(Ccond::CcTrigRise),
            2 => Some(Ccond::CcTrigFall),
            3 => Some(Ccond::CcTrigEdge),
            _ => None,
        }
    }
    #[doc = "None (never captures)"]
    #[inline(always)]
    pub fn is_nocapture(&self) -> bool {
        *self == Ccond::Nocapture
    }
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_rise(&self) -> bool {
        *self == Ccond::CcTrigRise
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_fall(&self) -> bool {
        *self == Ccond::CcTrigFall
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn is_cc_trig_edge(&self) -> bool {
        *self == Ccond::CcTrigEdge
    }
}
#[doc = "Field `CCOND` writer - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
pub type CcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccond>;
impl<'a, REG> CcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None (never captures)"]
    #[inline(always)]
    pub fn nocapture(self) -> &'a mut crate::W<REG> {
        self.variant(Ccond::Nocapture)
    }
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccond::CcTrigRise)
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccond::CcTrigFall)
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccond::CcTrigEdge)
    }
}
#[doc = "Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acond {
    #[doc = "0: Each TIMCLK"]
    Timclk = 0,
    #[doc = "1: Rising edge of CCP or trigger assertion edge"]
    CcTrigRise = 1,
    #[doc = "2: Falling edge of CCP or trigger de-assertion edge"]
    CcTrigFall = 2,
    #[doc = "3: Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    CcTrigEdge = 3,
    #[doc = "5: CCP High or Trigger assertion (level)"]
    CcTrigHigh = 5,
}
impl From<Acond> for u8 {
    #[inline(always)]
    fn from(variant: Acond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acond {
    type Ux = u8;
}
impl crate::IsEnum for Acond {}
#[doc = "Field `ACOND` reader - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
pub type AcondR = crate::FieldReader<Acond>;
impl AcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acond> {
        match self.bits {
            0 => Some(Acond::Timclk),
            1 => Some(Acond::CcTrigRise),
            2 => Some(Acond::CcTrigFall),
            3 => Some(Acond::CcTrigEdge),
            5 => Some(Acond::CcTrigHigh),
            _ => None,
        }
    }
    #[doc = "Each TIMCLK"]
    #[inline(always)]
    pub fn is_timclk(&self) -> bool {
        *self == Acond::Timclk
    }
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_rise(&self) -> bool {
        *self == Acond::CcTrigRise
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_fall(&self) -> bool {
        *self == Acond::CcTrigFall
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn is_cc_trig_edge(&self) -> bool {
        *self == Acond::CcTrigEdge
    }
    #[doc = "CCP High or Trigger assertion (level)"]
    #[inline(always)]
    pub fn is_cc_trig_high(&self) -> bool {
        *self == Acond::CcTrigHigh
    }
}
#[doc = "Field `ACOND` writer - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
pub type AcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Acond>;
impl<'a, REG> AcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each TIMCLK"]
    #[inline(always)]
    pub fn timclk(self) -> &'a mut crate::W<REG> {
        self.variant(Acond::Timclk)
    }
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Acond::CcTrigRise)
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Acond::CcTrigFall)
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Acond::CcTrigEdge)
    }
    #[doc = "CCP High or Trigger assertion (level)"]
    #[inline(always)]
    pub fn cc_trig_high(self) -> &'a mut crate::W<REG> {
        self.variant(Acond::CcTrigHigh)
    }
}
#[doc = "Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcond {
    #[doc = "1: Rising edge of CCP or trigger assertion edge"]
    CcTrigRise = 1,
    #[doc = "2: Falling edge of CCP or trigger de-assertion edge"]
    CcTrigFall = 2,
    #[doc = "3: Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    CcTrigEdge = 3,
}
impl From<Lcond> for u8 {
    #[inline(always)]
    fn from(variant: Lcond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcond {
    type Ux = u8;
}
impl crate::IsEnum for Lcond {}
#[doc = "Field `LCOND` reader - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
pub type LcondR = crate::FieldReader<Lcond>;
impl LcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lcond> {
        match self.bits {
            1 => Some(Lcond::CcTrigRise),
            2 => Some(Lcond::CcTrigFall),
            3 => Some(Lcond::CcTrigEdge),
            _ => None,
        }
    }
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_rise(&self) -> bool {
        *self == Lcond::CcTrigRise
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_fall(&self) -> bool {
        *self == Lcond::CcTrigFall
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn is_cc_trig_edge(&self) -> bool {
        *self == Lcond::CcTrigEdge
    }
}
#[doc = "Field `LCOND` writer - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
pub type LcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcond>;
impl<'a, REG> LcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Lcond::CcTrigRise)
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Lcond::CcTrigFall)
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Lcond::CcTrigEdge)
    }
}
#[doc = "Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zcond {
    #[doc = "1: Rising edge of CCP or trigger assertion edge"]
    CcTrigRise = 1,
    #[doc = "2: Falling edge of CCP or trigger de-assertion edge"]
    CcTrigFall = 2,
    #[doc = "3: Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    CcTrigEdge = 3,
}
impl From<Zcond> for u8 {
    #[inline(always)]
    fn from(variant: Zcond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Zcond {
    type Ux = u8;
}
impl crate::IsEnum for Zcond {}
#[doc = "Field `ZCOND` reader - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
pub type ZcondR = crate::FieldReader<Zcond>;
impl ZcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Zcond> {
        match self.bits {
            1 => Some(Zcond::CcTrigRise),
            2 => Some(Zcond::CcTrigFall),
            3 => Some(Zcond::CcTrigEdge),
            _ => None,
        }
    }
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_rise(&self) -> bool {
        *self == Zcond::CcTrigRise
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn is_cc_trig_fall(&self) -> bool {
        *self == Zcond::CcTrigFall
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn is_cc_trig_edge(&self) -> bool {
        *self == Zcond::CcTrigEdge
    }
}
#[doc = "Field `ZCOND` writer - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
pub type ZcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Zcond>;
impl<'a, REG> ZcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge of CCP or trigger assertion edge"]
    #[inline(always)]
    pub fn cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Zcond::CcTrigRise)
    }
    #[doc = "Falling edge of CCP or trigger de-assertion edge"]
    #[inline(always)]
    pub fn cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Zcond::CcTrigFall)
    }
    #[doc = "Either edge of CCP or trigger change (assertion/de-assertion edge)"]
    #[inline(always)]
    pub fn cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Zcond::CcTrigEdge)
    }
}
#[doc = "Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coc {
    #[doc = "0: Compare"]
    Compare = 0,
    #[doc = "1: Capture"]
    Capture = 1,
}
impl From<Coc> for bool {
    #[inline(always)]
    fn from(variant: Coc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COC` reader - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
pub type CocR = crate::BitReader<Coc>;
impl CocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Coc {
        match self.bits {
            false => Coc::Compare,
            true => Coc::Capture,
        }
    }
    #[doc = "Compare"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == Coc::Compare
    }
    #[doc = "Capture"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Coc::Capture
    }
}
#[doc = "Field `COC` writer - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
pub type CocW<'a, REG> = crate::BitWriter<'a, REG, Coc>;
impl<'a, REG> CocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(Coc::Compare)
    }
    #[doc = "Capture"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Coc::Capture)
    }
}
#[doc = "Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccupd {
    #[doc = "0: Writes to the CCx_y register is written to the register directly and has immediate effect."]
    Immediately = 0,
    #[doc = "1: Following a zero event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0."]
    ZeroEvt = 1,
    #[doc = "2: Following a compare (down) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    CompareDownEvt = 2,
    #[doc = "3: Following a compare (up) event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    CompareUpEvt = 3,
    #[doc = "4: Following a zero or load event Writes to the CCx_y register are stored in shadow register and transferred to ECCx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LD. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
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
    #[doc = "Following a zero event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0."]
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
    #[doc = "Following a zero or load event Writes to the CCx_y register are stored in shadow register and transferred to ECCx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LD. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
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
    #[doc = "Following a zero event Writes to the CCx_y register are stored in shadow register and transferred to CCx_y in the TIMCLK cycle following CTR equals 0."]
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
    #[doc = "Following a zero or load event Writes to the CCx_y register are stored in shadow register and transferred to ECCx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LD. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
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
#[doc = "Selects the source second CCU event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2selu {
    #[doc = "0: Selects CCU from CC0."]
    SelCcu0 = 0,
    #[doc = "1: Selects CCU from CC1."]
    SelCcu1 = 1,
    #[doc = "2: Selects CCU from CC2."]
    SelCcu2 = 2,
    #[doc = "3: Selects CCU from CC3."]
    SelCcu3 = 3,
    #[doc = "4: Selects CCU from CC4."]
    SelCcu4 = 4,
    #[doc = "5: Selects CCU from CC5."]
    SelCcu5 = 5,
}
impl From<Cc2selu> for u8 {
    #[inline(always)]
    fn from(variant: Cc2selu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2selu {
    type Ux = u8;
}
impl crate::IsEnum for Cc2selu {}
#[doc = "Field `CC2SELU` reader - Selects the source second CCU event."]
pub type Cc2seluR = crate::FieldReader<Cc2selu>;
impl Cc2seluR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cc2selu> {
        match self.bits {
            0 => Some(Cc2selu::SelCcu0),
            1 => Some(Cc2selu::SelCcu1),
            2 => Some(Cc2selu::SelCcu2),
            3 => Some(Cc2selu::SelCcu3),
            4 => Some(Cc2selu::SelCcu4),
            5 => Some(Cc2selu::SelCcu5),
            _ => None,
        }
    }
    #[doc = "Selects CCU from CC0."]
    #[inline(always)]
    pub fn is_sel_ccu0(&self) -> bool {
        *self == Cc2selu::SelCcu0
    }
    #[doc = "Selects CCU from CC1."]
    #[inline(always)]
    pub fn is_sel_ccu1(&self) -> bool {
        *self == Cc2selu::SelCcu1
    }
    #[doc = "Selects CCU from CC2."]
    #[inline(always)]
    pub fn is_sel_ccu2(&self) -> bool {
        *self == Cc2selu::SelCcu2
    }
    #[doc = "Selects CCU from CC3."]
    #[inline(always)]
    pub fn is_sel_ccu3(&self) -> bool {
        *self == Cc2selu::SelCcu3
    }
    #[doc = "Selects CCU from CC4."]
    #[inline(always)]
    pub fn is_sel_ccu4(&self) -> bool {
        *self == Cc2selu::SelCcu4
    }
    #[doc = "Selects CCU from CC5."]
    #[inline(always)]
    pub fn is_sel_ccu5(&self) -> bool {
        *self == Cc2selu::SelCcu5
    }
}
#[doc = "Field `CC2SELU` writer - Selects the source second CCU event."]
pub type Cc2seluW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cc2selu>;
impl<'a, REG> Cc2seluW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects CCU from CC0."]
    #[inline(always)]
    pub fn sel_ccu0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2selu::SelCcu0)
    }
    #[doc = "Selects CCU from CC1."]
    #[inline(always)]
    pub fn sel_ccu1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2selu::SelCcu1)
    }
    #[doc = "Selects CCU from CC2."]
    #[inline(always)]
    pub fn sel_ccu2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2selu::SelCcu2)
    }
    #[doc = "Selects CCU from CC3."]
    #[inline(always)]
    pub fn sel_ccu3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2selu::SelCcu3)
    }
    #[doc = "Selects CCU from CC4."]
    #[inline(always)]
    pub fn sel_ccu4(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2selu::SelCcu4)
    }
    #[doc = "Selects CCU from CC5."]
    #[inline(always)]
    pub fn sel_ccu5(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2selu::SelCcu5)
    }
}
#[doc = "CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccactupd {
    #[doc = "0: Value written to the CCACT register has immediate effect."]
    Immediately = 0,
    #[doc = "1: Following a zero event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0."]
    ZeroEvt = 1,
    #[doc = "2: Following a compare (down) event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    CompareDownEvt = 2,
    #[doc = "3: Following a compare (up) event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    CompareUpEvt = 3,
    #[doc = "4: Following a zero or load event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LDn. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
    ZeroLoadEvt = 4,
    #[doc = "5: Following a zero event with repeat count also zero. Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0 and if RC equal 0."]
    ZeroRcZeroEvt = 5,
    #[doc = "6: On a TRIG pulse, the value stored in CCACTx_y shadow register is loaded into CCACTx_y active register."]
    Trig = 6,
}
impl From<Ccactupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccactupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccactupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccactupd {}
#[doc = "Field `CCACTUPD` reader - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
pub type CcactupdR = crate::FieldReader<Ccactupd>;
impl CcactupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccactupd> {
        match self.bits {
            0 => Some(Ccactupd::Immediately),
            1 => Some(Ccactupd::ZeroEvt),
            2 => Some(Ccactupd::CompareDownEvt),
            3 => Some(Ccactupd::CompareUpEvt),
            4 => Some(Ccactupd::ZeroLoadEvt),
            5 => Some(Ccactupd::ZeroRcZeroEvt),
            6 => Some(Ccactupd::Trig),
            _ => None,
        }
    }
    #[doc = "Value written to the CCACT register has immediate effect."]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == Ccactupd::Immediately
    }
    #[doc = "Following a zero event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0."]
    #[inline(always)]
    pub fn is_zero_evt(&self) -> bool {
        *self == Ccactupd::ZeroEvt
    }
    #[doc = "Following a compare (down) event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn is_compare_down_evt(&self) -> bool {
        *self == Ccactupd::CompareDownEvt
    }
    #[doc = "Following a compare (up) event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn is_compare_up_evt(&self) -> bool {
        *self == Ccactupd::CompareUpEvt
    }
    #[doc = "Following a zero or load event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LDn. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
    #[inline(always)]
    pub fn is_zero_load_evt(&self) -> bool {
        *self == Ccactupd::ZeroLoadEvt
    }
    #[doc = "Following a zero event with repeat count also zero. Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0 and if RC equal 0."]
    #[inline(always)]
    pub fn is_zero_rc_zero_evt(&self) -> bool {
        *self == Ccactupd::ZeroRcZeroEvt
    }
    #[doc = "On a TRIG pulse, the value stored in CCACTx_y shadow register is loaded into CCACTx_y active register."]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Ccactupd::Trig
    }
}
#[doc = "Field `CCACTUPD` writer - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
pub type CcactupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccactupd>;
impl<'a, REG> CcactupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value written to the CCACT register has immediate effect."]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::Immediately)
    }
    #[doc = "Following a zero event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0."]
    #[inline(always)]
    pub fn zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::ZeroEvt)
    }
    #[doc = "Following a compare (down) event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::CompareDownEvt)
    }
    #[doc = "Following a compare (up) event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals the CCx_y register value."]
    #[inline(always)]
    pub fn compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::CompareUpEvt)
    }
    #[doc = "Following a zero or load event Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0 or CTR. Equals LDn. Note this update mechanism is defined for use only in configurations using up/down counting. This mode is not intended for use in down count configurations."]
    #[inline(always)]
    pub fn zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::ZeroLoadEvt)
    }
    #[doc = "Following a zero event with repeat count also zero. Writes to the CCACTx_y register are stored in shadow register and transferred to CCACTx_y in the TIMCLK cycle following CTR equals 0 and if RC equal 0."]
    #[inline(always)]
    pub fn zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::ZeroRcZeroEvt)
    }
    #[doc = "On a TRIG pulse, the value stored in CCACTx_y shadow register is loaded into CCACTx_y active register."]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccactupd::Trig)
    }
}
#[doc = "Selects the source second CCD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2seld {
    #[doc = "0: Selects CCD from CC0."]
    SelCcd0 = 0,
    #[doc = "1: Selects CCD from CC1."]
    SelCcd1 = 1,
    #[doc = "2: Selects CCD from CC2."]
    SelCcd2 = 2,
    #[doc = "3: Selects CCD from CC3."]
    SelCcd3 = 3,
    #[doc = "4: Selects CCD from CC4."]
    SelCcd4 = 4,
    #[doc = "5: Selects CCD from CC5."]
    SelCcd5 = 5,
}
impl From<Cc2seld> for u8 {
    #[inline(always)]
    fn from(variant: Cc2seld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2seld {
    type Ux = u8;
}
impl crate::IsEnum for Cc2seld {}
#[doc = "Field `CC2SELD` reader - Selects the source second CCD event."]
pub type Cc2seldR = crate::FieldReader<Cc2seld>;
impl Cc2seldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cc2seld> {
        match self.bits {
            0 => Some(Cc2seld::SelCcd0),
            1 => Some(Cc2seld::SelCcd1),
            2 => Some(Cc2seld::SelCcd2),
            3 => Some(Cc2seld::SelCcd3),
            4 => Some(Cc2seld::SelCcd4),
            5 => Some(Cc2seld::SelCcd5),
            _ => None,
        }
    }
    #[doc = "Selects CCD from CC0."]
    #[inline(always)]
    pub fn is_sel_ccd0(&self) -> bool {
        *self == Cc2seld::SelCcd0
    }
    #[doc = "Selects CCD from CC1."]
    #[inline(always)]
    pub fn is_sel_ccd1(&self) -> bool {
        *self == Cc2seld::SelCcd1
    }
    #[doc = "Selects CCD from CC2."]
    #[inline(always)]
    pub fn is_sel_ccd2(&self) -> bool {
        *self == Cc2seld::SelCcd2
    }
    #[doc = "Selects CCD from CC3."]
    #[inline(always)]
    pub fn is_sel_ccd3(&self) -> bool {
        *self == Cc2seld::SelCcd3
    }
    #[doc = "Selects CCD from CC4."]
    #[inline(always)]
    pub fn is_sel_ccd4(&self) -> bool {
        *self == Cc2seld::SelCcd4
    }
    #[doc = "Selects CCD from CC5."]
    #[inline(always)]
    pub fn is_sel_ccd5(&self) -> bool {
        *self == Cc2seld::SelCcd5
    }
}
#[doc = "Field `CC2SELD` writer - Selects the source second CCD event."]
pub type Cc2seldW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cc2seld>;
impl<'a, REG> Cc2seldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects CCD from CC0."]
    #[inline(always)]
    pub fn sel_ccd0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2seld::SelCcd0)
    }
    #[doc = "Selects CCD from CC1."]
    #[inline(always)]
    pub fn sel_ccd1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2seld::SelCcd1)
    }
    #[doc = "Selects CCD from CC2."]
    #[inline(always)]
    pub fn sel_ccd2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2seld::SelCcd2)
    }
    #[doc = "Selects CCD from CC3."]
    #[inline(always)]
    pub fn sel_ccd3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2seld::SelCcd3)
    }
    #[doc = "Selects CCD from CC4."]
    #[inline(always)]
    pub fn sel_ccd4(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2seld::SelCcd4)
    }
    #[doc = "Selects CCD from CC5."]
    #[inline(always)]
    pub fn sel_ccd5(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2seld::SelCcd5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccond(&self) -> CcondR {
        CcondR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
    #[inline(always)]
    pub fn acond(&self) -> AcondR {
        AcondR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn lcond(&self) -> LcondR {
        LcondR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn zcond(&self) -> ZcondR {
        ZcondR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
    #[inline(always)]
    pub fn coc(&self) -> CocR {
        CocR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccupd(&self) -> CcupdR {
        CcupdR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Selects the source second CCU event."]
    #[inline(always)]
    pub fn cc2selu(&self) -> Cc2seluR {
        Cc2seluR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 26:28 - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
    #[inline(always)]
    pub fn ccactupd(&self) -> CcactupdR {
        CcactupdR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Selects the source second CCD event."]
    #[inline(always)]
    pub fn cc2seld(&self) -> Cc2seldR {
        Cc2seldR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccond(&mut self) -> CcondW<'_, Timg4Ccctl01Spec> {
        CcondW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
    #[inline(always)]
    pub fn acond(&mut self) -> AcondW<'_, Timg4Ccctl01Spec> {
        AcondW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn lcond(&mut self) -> LcondW<'_, Timg4Ccctl01Spec> {
        LcondW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn zcond(&mut self) -> ZcondW<'_, Timg4Ccctl01Spec> {
        ZcondW::new(self, 12)
    }
    #[doc = "Bit 17 - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
    #[inline(always)]
    pub fn coc(&mut self) -> CocW<'_, Timg4Ccctl01Spec> {
        CocW::new(self, 17)
    }
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccupd(&mut self) -> CcupdW<'_, Timg4Ccctl01Spec> {
        CcupdW::new(self, 18)
    }
    #[doc = "Bits 22:24 - Selects the source second CCU event."]
    #[inline(always)]
    pub fn cc2selu(&mut self) -> Cc2seluW<'_, Timg4Ccctl01Spec> {
        Cc2seluW::new(self, 22)
    }
    #[doc = "Bits 26:28 - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
    #[inline(always)]
    pub fn ccactupd(&mut self) -> CcactupdW<'_, Timg4Ccctl01Spec> {
        CcactupdW::new(self, 26)
    }
    #[doc = "Bits 29:31 - Selects the source second CCD event."]
    #[inline(always)]
    pub fn cc2seld(&mut self) -> Cc2seldW<'_, Timg4Ccctl01Spec> {
        Cc2seldW::new(self, 29)
    }
}
#[doc = "Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`timg4_ccctl_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_ccctl_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg4Ccctl01Spec;
impl crate::RegisterSpec for Timg4Ccctl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg4_ccctl_01::R`](R) reader structure"]
impl crate::Readable for Timg4Ccctl01Spec {}
#[doc = "`write(|w| ..)` method takes [`timg4_ccctl_01::W`](W) writer structure"]
impl crate::Writable for Timg4Ccctl01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG4_CCCTL_01[%s] to value 0"]
impl crate::Resettable for Timg4Ccctl01Spec {}
