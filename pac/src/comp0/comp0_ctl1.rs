#[doc = "Register `COMP0_CTL1` reader"]
pub type R = crate::R<Comp0Ctl1Spec>;
#[doc = "Register `COMP0_CTL1` writer"]
pub type W = crate::W<Comp0Ctl1Spec>;
#[doc = "This bit turns on the comparator. When the comparator is turned off it consumes no power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Comparator is off"]
    Off = 0,
    #[doc = "1: Comparator is on"]
    On = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Off,
            true => Enable::On,
        }
    }
    #[doc = "Comparator is off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Enable::Off
    }
    #[doc = "Comparator is on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Enable::On
    }
}
#[doc = "Field `ENABLE` writer - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator is off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Off)
    }
    #[doc = "Comparator is on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::On)
    }
}
#[doc = "This bit selects the comparator operating mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Comparator is in fast mode"]
    Fast = 0,
    #[doc = "1: Comparator is in ultra-low power mode"]
    Ulp = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - This bit selects the comparator operating mode."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Fast,
            true => Mode::Ulp,
        }
    }
    #[doc = "Comparator is in fast mode"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Mode::Fast
    }
    #[doc = "Comparator is in ultra-low power mode"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Mode::Ulp
    }
}
#[doc = "Field `MODE` writer - This bit selects the comparator operating mode."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator is in fast mode"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Fast)
    }
    #[doc = "Comparator is in ultra-low power mode"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ulp)
    }
}
#[doc = "This bit exchanges the comparator inputs and inverts the comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exch {
    #[doc = "0: Comparator inputs not exchanged and output not inverted"]
    NoExc = 0,
    #[doc = "1: Comparator inputs exchanged and output inverted"]
    Exc = 1,
}
impl From<Exch> for bool {
    #[inline(always)]
    fn from(variant: Exch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCH` reader - This bit exchanges the comparator inputs and inverts the comparator output."]
pub type ExchR = crate::BitReader<Exch>;
impl ExchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exch {
        match self.bits {
            false => Exch::NoExc,
            true => Exch::Exc,
        }
    }
    #[doc = "Comparator inputs not exchanged and output not inverted"]
    #[inline(always)]
    pub fn is_no_exc(&self) -> bool {
        *self == Exch::NoExc
    }
    #[doc = "Comparator inputs exchanged and output inverted"]
    #[inline(always)]
    pub fn is_exc(&self) -> bool {
        *self == Exch::Exc
    }
}
#[doc = "Field `EXCH` writer - This bit exchanges the comparator inputs and inverts the comparator output."]
pub type ExchW<'a, REG> = crate::BitWriter<'a, REG, Exch>;
impl<'a, REG> ExchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator inputs not exchanged and output not inverted"]
    #[inline(always)]
    pub fn no_exc(self) -> &'a mut crate::W<REG> {
        self.variant(Exch::NoExc)
    }
    #[doc = "Comparator inputs exchanged and output inverted"]
    #[inline(always)]
    pub fn exc(self) -> &'a mut crate::W<REG> {
        self.variant(Exch::Exc)
    }
}
#[doc = "This bit shorts the positive and negative input terminals of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Short {
    #[doc = "0: Comparator positive and negative input terminals are not shorted"]
    NoSht = 0,
    #[doc = "1: Comparator positive and negative input terminals are shorted"]
    Sht = 1,
}
impl From<Short> for bool {
    #[inline(always)]
    fn from(variant: Short) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHORT` reader - This bit shorts the positive and negative input terminals of the comparator."]
pub type ShortR = crate::BitReader<Short>;
impl ShortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Short {
        match self.bits {
            false => Short::NoSht,
            true => Short::Sht,
        }
    }
    #[doc = "Comparator positive and negative input terminals are not shorted"]
    #[inline(always)]
    pub fn is_no_sht(&self) -> bool {
        *self == Short::NoSht
    }
    #[doc = "Comparator positive and negative input terminals are shorted"]
    #[inline(always)]
    pub fn is_sht(&self) -> bool {
        *self == Short::Sht
    }
}
#[doc = "Field `SHORT` writer - This bit shorts the positive and negative input terminals of the comparator."]
pub type ShortW<'a, REG> = crate::BitWriter<'a, REG, Short>;
impl<'a, REG> ShortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator positive and negative input terminals are not shorted"]
    #[inline(always)]
    pub fn no_sht(self) -> &'a mut crate::W<REG> {
        self.variant(Short::NoSht)
    }
    #[doc = "Comparator positive and negative input terminals are shorted"]
    #[inline(always)]
    pub fn sht(self) -> &'a mut crate::W<REG> {
        self.variant(Short::Sht)
    }
}
#[doc = "This bit selected the interrupt edge for COMPIFG and COMPINVIFG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ies {
    #[doc = "0: Rising edge sets COMPIFG and falling edge sets COMPINVIFG"]
    Rising = 0,
    #[doc = "1: Falling edge sets COMPIFG and rising edge sets COMPINVIFG"]
    Falling = 1,
}
impl From<Ies> for bool {
    #[inline(always)]
    fn from(variant: Ies) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IES` reader - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
pub type IesR = crate::BitReader<Ies>;
impl IesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ies {
        match self.bits {
            false => Ies::Rising,
            true => Ies::Falling,
        }
    }
    #[doc = "Rising edge sets COMPIFG and falling edge sets COMPINVIFG"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Ies::Rising
    }
    #[doc = "Falling edge sets COMPIFG and rising edge sets COMPINVIFG"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Ies::Falling
    }
}
#[doc = "Field `IES` writer - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
pub type IesW<'a, REG> = crate::BitWriter<'a, REG, Ies>;
impl<'a, REG> IesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge sets COMPIFG and falling edge sets COMPINVIFG"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ies::Rising)
    }
    #[doc = "Falling edge sets COMPIFG and rising edge sets COMPINVIFG"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ies::Falling)
    }
}
#[doc = "These bits select the hysteresis setting of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hyst {
    #[doc = "0: No hysteresis"]
    NoHys = 0,
    #[doc = "1: Low hysteresis, typical 10mV"]
    LowHys = 1,
    #[doc = "2: Medium hysteresis, typical 20mV"]
    MedHys = 2,
    #[doc = "3: High hysteresis, typical 30mV"]
    HighHys = 3,
}
impl From<Hyst> for u8 {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hyst {
    type Ux = u8;
}
impl crate::IsEnum for Hyst {}
#[doc = "Field `HYST` reader - These bits select the hysteresis setting of the comparator."]
pub type HystR = crate::FieldReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            0 => Hyst::NoHys,
            1 => Hyst::LowHys,
            2 => Hyst::MedHys,
            3 => Hyst::HighHys,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hys(&self) -> bool {
        *self == Hyst::NoHys
    }
    #[doc = "Low hysteresis, typical 10mV"]
    #[inline(always)]
    pub fn is_low_hys(&self) -> bool {
        *self == Hyst::LowHys
    }
    #[doc = "Medium hysteresis, typical 20mV"]
    #[inline(always)]
    pub fn is_med_hys(&self) -> bool {
        *self == Hyst::MedHys
    }
    #[doc = "High hysteresis, typical 30mV"]
    #[inline(always)]
    pub fn is_high_hys(&self) -> bool {
        *self == Hyst::HighHys
    }
}
#[doc = "Field `HYST` writer - These bits select the hysteresis setting of the comparator."]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hyst, crate::Safe>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::NoHys)
    }
    #[doc = "Low hysteresis, typical 10mV"]
    #[inline(always)]
    pub fn low_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::LowHys)
    }
    #[doc = "Medium hysteresis, typical 20mV"]
    #[inline(always)]
    pub fn med_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::MedHys)
    }
    #[doc = "High hysteresis, typical 30mV"]
    #[inline(always)]
    pub fn high_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::HighHys)
    }
}
#[doc = "This bit selects the comparator output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outpol {
    #[doc = "0: Comparator output is non-inverted"]
    NonInv = 0,
    #[doc = "1: Comparator output is inverted"]
    Inv = 1,
}
impl From<Outpol> for bool {
    #[inline(always)]
    fn from(variant: Outpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPOL` reader - This bit selects the comparator output polarity."]
pub type OutpolR = crate::BitReader<Outpol>;
impl OutpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outpol {
        match self.bits {
            false => Outpol::NonInv,
            true => Outpol::Inv,
        }
    }
    #[doc = "Comparator output is non-inverted"]
    #[inline(always)]
    pub fn is_non_inv(&self) -> bool {
        *self == Outpol::NonInv
    }
    #[doc = "Comparator output is inverted"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Outpol::Inv
    }
}
#[doc = "Field `OUTPOL` writer - This bit selects the comparator output polarity."]
pub type OutpolW<'a, REG> = crate::BitWriter<'a, REG, Outpol>;
impl<'a, REG> OutpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output is non-inverted"]
    #[inline(always)]
    pub fn non_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Outpol::NonInv)
    }
    #[doc = "Comparator output is inverted"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Outpol::Inv)
    }
}
#[doc = "This bit enables the analog filter at comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flten {
    #[doc = "0: Comparator output filter is disabled"]
    Disable = 0,
    #[doc = "1: Comparator output filter is enabled"]
    Enable = 1,
}
impl From<Flten> for bool {
    #[inline(always)]
    fn from(variant: Flten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTEN` reader - This bit enables the analog filter at comparator output."]
pub type FltenR = crate::BitReader<Flten>;
impl FltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flten {
        match self.bits {
            false => Flten::Disable,
            true => Flten::Enable,
        }
    }
    #[doc = "Comparator output filter is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Flten::Disable
    }
    #[doc = "Comparator output filter is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Flten::Enable
    }
}
#[doc = "Field `FLTEN` writer - This bit enables the analog filter at comparator output."]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG, Flten>;
impl<'a, REG> FltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output filter is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Flten::Disable)
    }
    #[doc = "Comparator output filter is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Flten::Enable)
    }
}
#[doc = "These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fltdly {
    #[doc = "0: Typical filter delay of 70 ns"]
    Dly0 = 0,
    #[doc = "1: Typical filter delay of 500 ns"]
    Dly1 = 1,
    #[doc = "2: Typical filter delay of 1200 ns"]
    Dly2 = 2,
    #[doc = "3: Typical filter delay of 2700 ns"]
    Dly3 = 3,
}
impl From<Fltdly> for u8 {
    #[inline(always)]
    fn from(variant: Fltdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fltdly {
    type Ux = u8;
}
impl crate::IsEnum for Fltdly {}
#[doc = "Field `FLTDLY` reader - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
pub type FltdlyR = crate::FieldReader<Fltdly>;
impl FltdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fltdly {
        match self.bits {
            0 => Fltdly::Dly0,
            1 => Fltdly::Dly1,
            2 => Fltdly::Dly2,
            3 => Fltdly::Dly3,
            _ => unreachable!(),
        }
    }
    #[doc = "Typical filter delay of 70 ns"]
    #[inline(always)]
    pub fn is_dly_0(&self) -> bool {
        *self == Fltdly::Dly0
    }
    #[doc = "Typical filter delay of 500 ns"]
    #[inline(always)]
    pub fn is_dly_1(&self) -> bool {
        *self == Fltdly::Dly1
    }
    #[doc = "Typical filter delay of 1200 ns"]
    #[inline(always)]
    pub fn is_dly_2(&self) -> bool {
        *self == Fltdly::Dly2
    }
    #[doc = "Typical filter delay of 2700 ns"]
    #[inline(always)]
    pub fn is_dly_3(&self) -> bool {
        *self == Fltdly::Dly3
    }
}
#[doc = "Field `FLTDLY` writer - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
pub type FltdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fltdly, crate::Safe>;
impl<'a, REG> FltdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Typical filter delay of 70 ns"]
    #[inline(always)]
    pub fn dly_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fltdly::Dly0)
    }
    #[doc = "Typical filter delay of 500 ns"]
    #[inline(always)]
    pub fn dly_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fltdly::Dly1)
    }
    #[doc = "Typical filter delay of 1200 ns"]
    #[inline(always)]
    pub fn dly_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fltdly::Dly2)
    }
    #[doc = "Typical filter delay of 2700 ns"]
    #[inline(always)]
    pub fn dly_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fltdly::Dly3)
    }
}
#[doc = "This bit enables window comparator operation of comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wincompen {
    #[doc = "0: window comparator is disable"]
    Off = 0,
    #[doc = "1: window comparator is enable"]
    On = 1,
}
impl From<Wincompen> for bool {
    #[inline(always)]
    fn from(variant: Wincompen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINCOMPEN` reader - This bit enables window comparator operation of comparator."]
pub type WincompenR = crate::BitReader<Wincompen>;
impl WincompenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wincompen {
        match self.bits {
            false => Wincompen::Off,
            true => Wincompen::On,
        }
    }
    #[doc = "window comparator is disable"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Wincompen::Off
    }
    #[doc = "window comparator is enable"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Wincompen::On
    }
}
#[doc = "Field `WINCOMPEN` writer - This bit enables window comparator operation of comparator."]
pub type WincompenW<'a, REG> = crate::BitWriter<'a, REG, Wincompen>;
impl<'a, REG> WincompenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "window comparator is disable"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Wincompen::Off)
    }
    #[doc = "window comparator is enable"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Wincompen::On)
    }
}
impl R {
    #[doc = "Bit 0 - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit selects the comparator operating mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit exchanges the comparator inputs and inverts the comparator output."]
    #[inline(always)]
    pub fn exch(&self) -> ExchR {
        ExchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit shorts the positive and negative input terminals of the comparator."]
    #[inline(always)]
    pub fn short(&self) -> ShortR {
        ShortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
    #[inline(always)]
    pub fn ies(&self) -> IesR {
        IesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - These bits select the hysteresis setting of the comparator."]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - This bit selects the comparator output polarity."]
    #[inline(always)]
    pub fn outpol(&self) -> OutpolR {
        OutpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit enables the analog filter at comparator output."]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
    #[inline(always)]
    pub fn fltdly(&self) -> FltdlyR {
        FltdlyR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - This bit enables window comparator operation of comparator."]
    #[inline(always)]
    pub fn wincompen(&self) -> WincompenR {
        WincompenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Comp0Ctl1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit selects the comparator operating mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Comp0Ctl1Spec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit exchanges the comparator inputs and inverts the comparator output."]
    #[inline(always)]
    pub fn exch(&mut self) -> ExchW<'_, Comp0Ctl1Spec> {
        ExchW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit shorts the positive and negative input terminals of the comparator."]
    #[inline(always)]
    pub fn short(&mut self) -> ShortW<'_, Comp0Ctl1Spec> {
        ShortW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
    #[inline(always)]
    pub fn ies(&mut self) -> IesW<'_, Comp0Ctl1Spec> {
        IesW::new(self, 4)
    }
    #[doc = "Bits 5:6 - These bits select the hysteresis setting of the comparator."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, Comp0Ctl1Spec> {
        HystW::new(self, 5)
    }
    #[doc = "Bit 7 - This bit selects the comparator output polarity."]
    #[inline(always)]
    pub fn outpol(&mut self) -> OutpolW<'_, Comp0Ctl1Spec> {
        OutpolW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit enables the analog filter at comparator output."]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, Comp0Ctl1Spec> {
        FltenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
    #[inline(always)]
    pub fn fltdly(&mut self) -> FltdlyW<'_, Comp0Ctl1Spec> {
        FltdlyW::new(self, 9)
    }
    #[doc = "Bit 12 - This bit enables window comparator operation of comparator."]
    #[inline(always)]
    pub fn wincompen(&mut self) -> WincompenW<'_, Comp0Ctl1Spec> {
        WincompenW::new(self, 12)
    }
}
#[doc = "Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0Ctl1Spec;
impl crate::RegisterSpec for Comp0Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_ctl1::R`](R) reader structure"]
impl crate::Readable for Comp0Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`comp0_ctl1::W`](W) writer structure"]
impl crate::Writable for Comp0Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CTL1 to value 0"]
impl crate::Resettable for Comp0Ctl1Spec {}
