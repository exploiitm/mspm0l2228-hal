#[doc = "Register `LCD_LCDVCTL` reader"]
pub type R = crate::R<LcdLcdvctlSpec>;
#[doc = "Register `LCD_LCDVCTL` writer"]
pub type W = crate::W<LcdLcdvctlSpec>;
#[doc = "Selects whether R13 voltage is switched or in static mode 0b = Static mode 1b = Switched mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdrefmode {
    #[doc = "0: Static mode"]
    StaticMode = 0,
    #[doc = "1: Switched mode"]
    SwitchedMode = 1,
}
impl From<Lcdrefmode> for bool {
    #[inline(always)]
    fn from(variant: Lcdrefmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDREFMODE` reader - Selects whether R13 voltage is switched or in static mode 0b = Static mode 1b = Switched mode"]
pub type LcdrefmodeR = crate::BitReader<Lcdrefmode>;
impl LcdrefmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdrefmode {
        match self.bits {
            false => Lcdrefmode::StaticMode,
            true => Lcdrefmode::SwitchedMode,
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn is_static_mode(&self) -> bool {
        *self == Lcdrefmode::StaticMode
    }
    #[doc = "Switched mode"]
    #[inline(always)]
    pub fn is_switched_mode(&self) -> bool {
        *self == Lcdrefmode::SwitchedMode
    }
}
#[doc = "Field `LCDREFMODE` writer - Selects whether R13 voltage is switched or in static mode 0b = Static mode 1b = Switched mode"]
pub type LcdrefmodeW<'a, REG> = crate::BitWriter<'a, REG, Lcdrefmode>;
impl<'a, REG> LcdrefmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn static_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrefmode::StaticMode)
    }
    #[doc = "Switched mode"]
    #[inline(always)]
    pub fn switched_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrefmode::SwitchedMode)
    }
}
#[doc = "Bias select. LCDBIASSEL is ignored in static mode as well as for 2-mux, 3-mux and 4-mux LCD modes. For 5-mux to 8-mux modes: 0b = 1/3 bias 1b = 1/4 bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdbiassel {
    #[doc = "0: 1/3 bias"]
    OneBy3Bias = 0,
    #[doc = "1: 1/4 bias"]
    OneBy4Bias = 1,
}
impl From<Lcdbiassel> for bool {
    #[inline(always)]
    fn from(variant: Lcdbiassel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDBIASSEL` reader - Bias select. LCDBIASSEL is ignored in static mode as well as for 2-mux, 3-mux and 4-mux LCD modes. For 5-mux to 8-mux modes: 0b = 1/3 bias 1b = 1/4 bias"]
pub type LcdbiasselR = crate::BitReader<Lcdbiassel>;
impl LcdbiasselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdbiassel {
        match self.bits {
            false => Lcdbiassel::OneBy3Bias,
            true => Lcdbiassel::OneBy4Bias,
        }
    }
    #[doc = "1/3 bias"]
    #[inline(always)]
    pub fn is_one_by_3_bias(&self) -> bool {
        *self == Lcdbiassel::OneBy3Bias
    }
    #[doc = "1/4 bias"]
    #[inline(always)]
    pub fn is_one_by_4_bias(&self) -> bool {
        *self == Lcdbiassel::OneBy4Bias
    }
}
#[doc = "Field `LCDBIASSEL` writer - Bias select. LCDBIASSEL is ignored in static mode as well as for 2-mux, 3-mux and 4-mux LCD modes. For 5-mux to 8-mux modes: 0b = 1/3 bias 1b = 1/4 bias"]
pub type LcdbiasselW<'a, REG> = crate::BitWriter<'a, REG, Lcdbiassel>;
impl<'a, REG> LcdbiasselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1/3 bias"]
    #[inline(always)]
    pub fn one_by_3_bias(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdbiassel::OneBy3Bias)
    }
    #[doc = "1/4 bias"]
    #[inline(always)]
    pub fn one_by_4_bias(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdbiassel::OneBy4Bias)
    }
}
#[doc = "Enables the internal bias voltage resistor divider. The actual voltage source used for the resistor divider is selected by the VLCDSEL_VDD_R33 bit configuration. 0b = Internal bias voltage resistor divider is disabled 1b = Internal bias voltage resistor divider is enabledDD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdintbiasen {
    #[doc = "0: Internal bias voltage resistor divider is disabled"]
    IntBiasDisable = 0,
    #[doc = "1: Internal bias voltage resistor divider is enabled"]
    IntBiasEnable = 1,
}
impl From<Lcdintbiasen> for bool {
    #[inline(always)]
    fn from(variant: Lcdintbiasen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDINTBIASEN` reader - Enables the internal bias voltage resistor divider. The actual voltage source used for the resistor divider is selected by the VLCDSEL_VDD_R33 bit configuration. 0b = Internal bias voltage resistor divider is disabled 1b = Internal bias voltage resistor divider is enabledDD"]
pub type LcdintbiasenR = crate::BitReader<Lcdintbiasen>;
impl LcdintbiasenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdintbiasen {
        match self.bits {
            false => Lcdintbiasen::IntBiasDisable,
            true => Lcdintbiasen::IntBiasEnable,
        }
    }
    #[doc = "Internal bias voltage resistor divider is disabled"]
    #[inline(always)]
    pub fn is_int_bias_disable(&self) -> bool {
        *self == Lcdintbiasen::IntBiasDisable
    }
    #[doc = "Internal bias voltage resistor divider is enabled"]
    #[inline(always)]
    pub fn is_int_bias_enable(&self) -> bool {
        *self == Lcdintbiasen::IntBiasEnable
    }
}
#[doc = "Field `LCDINTBIASEN` writer - Enables the internal bias voltage resistor divider. The actual voltage source used for the resistor divider is selected by the VLCDSEL_VDD_R33 bit configuration. 0b = Internal bias voltage resistor divider is disabled 1b = Internal bias voltage resistor divider is enabledDD"]
pub type LcdintbiasenW<'a, REG> = crate::BitWriter<'a, REG, Lcdintbiasen>;
impl<'a, REG> LcdintbiasenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal bias voltage resistor divider is disabled"]
    #[inline(always)]
    pub fn int_bias_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdintbiasen::IntBiasDisable)
    }
    #[doc = "Internal bias voltage resistor divider is enabled"]
    #[inline(always)]
    pub fn int_bias_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdintbiasen::IntBiasEnable)
    }
}
#[doc = "Selects if the LCD bias voltage V1 is sourced from the R33 pin or from the internal supply voltage AVDD This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1 0b = V1 is sourced from R33 1b = V1 is sourced internally from AVDD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VlcdselVddR33 {
    #[doc = "0: V1 is sourced from R33"]
    SelR33 = 0,
    #[doc = "1: V1 is sourced internally from AVDD"]
    SelAvdd = 1,
}
impl From<VlcdselVddR33> for bool {
    #[inline(always)]
    fn from(variant: VlcdselVddR33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLCDSEL_VDD_R33` reader - Selects if the LCD bias voltage V1 is sourced from the R33 pin or from the internal supply voltage AVDD This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1 0b = V1 is sourced from R33 1b = V1 is sourced internally from AVDD"]
pub type VlcdselVddR33R = crate::BitReader<VlcdselVddR33>;
impl VlcdselVddR33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VlcdselVddR33 {
        match self.bits {
            false => VlcdselVddR33::SelR33,
            true => VlcdselVddR33::SelAvdd,
        }
    }
    #[doc = "V1 is sourced from R33"]
    #[inline(always)]
    pub fn is_sel_r33(&self) -> bool {
        *self == VlcdselVddR33::SelR33
    }
    #[doc = "V1 is sourced internally from AVDD"]
    #[inline(always)]
    pub fn is_sel_avdd(&self) -> bool {
        *self == VlcdselVddR33::SelAvdd
    }
}
#[doc = "Field `VLCDSEL_VDD_R33` writer - Selects if the LCD bias voltage V1 is sourced from the R33 pin or from the internal supply voltage AVDD This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1 0b = V1 is sourced from R33 1b = V1 is sourced internally from AVDD"]
pub type VlcdselVddR33W<'a, REG> = crate::BitWriter<'a, REG, VlcdselVddR33>;
impl<'a, REG> VlcdselVddR33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V1 is sourced from R33"]
    #[inline(always)]
    pub fn sel_r33(self) -> &'a mut crate::W<REG> {
        self.variant(VlcdselVddR33::SelR33)
    }
    #[doc = "V1 is sourced internally from AVDD"]
    #[inline(always)]
    pub fn sel_avdd(self) -> &'a mut crate::W<REG> {
        self.variant(VlcdselVddR33::SelAvdd)
    }
}
#[doc = "High-power or Low-power LCD. This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1. It selects the resistor ladder that is used to generate the bias voltages for the LCD. 0b = Low-power LCD is used 1b = Higher-power LCD is used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcdHpLp {
    #[doc = "0: Low-power LCD is used"]
    LpMode = 0,
    #[doc = "1: Higher-power LCD is used"]
    HpMode = 1,
}
impl From<LcdHpLp> for bool {
    #[inline(always)]
    fn from(variant: LcdHpLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCD_HP_LP` reader - High-power or Low-power LCD. This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1. It selects the resistor ladder that is used to generate the bias voltages for the LCD. 0b = Low-power LCD is used 1b = Higher-power LCD is used"]
pub type LcdHpLpR = crate::BitReader<LcdHpLp>;
impl LcdHpLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcdHpLp {
        match self.bits {
            false => LcdHpLp::LpMode,
            true => LcdHpLp::HpMode,
        }
    }
    #[doc = "Low-power LCD is used"]
    #[inline(always)]
    pub fn is_lp_mode(&self) -> bool {
        *self == LcdHpLp::LpMode
    }
    #[doc = "Higher-power LCD is used"]
    #[inline(always)]
    pub fn is_hp_mode(&self) -> bool {
        *self == LcdHpLp::HpMode
    }
}
#[doc = "Field `LCD_HP_LP` writer - High-power or Low-power LCD. This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1. It selects the resistor ladder that is used to generate the bias voltages for the LCD. 0b = Low-power LCD is used 1b = Higher-power LCD is used"]
pub type LcdHpLpW<'a, REG> = crate::BitWriter<'a, REG, LcdHpLp>;
impl<'a, REG> LcdHpLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-power LCD is used"]
    #[inline(always)]
    pub fn lp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LcdHpLp::LpMode)
    }
    #[doc = "Higher-power LCD is used"]
    #[inline(always)]
    pub fn hp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LcdHpLp::HpMode)
    }
}
#[doc = "Selects if R33 is supplied either from AVDD internally or from charge pump 0b = R33 connected to external supply 1b = R33 internally connected to AVDD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdselvdd {
    #[doc = "0: R33 connected to external supply"]
    SelExtSupply = 0,
    #[doc = "1: R33 internally connected to AVDD"]
    SelAvdd = 1,
}
impl From<Lcdselvdd> for bool {
    #[inline(always)]
    fn from(variant: Lcdselvdd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSELVDD` reader - Selects if R33 is supplied either from AVDD internally or from charge pump 0b = R33 connected to external supply 1b = R33 internally connected to AVDD"]
pub type LcdselvddR = crate::BitReader<Lcdselvdd>;
impl LcdselvddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdselvdd {
        match self.bits {
            false => Lcdselvdd::SelExtSupply,
            true => Lcdselvdd::SelAvdd,
        }
    }
    #[doc = "R33 connected to external supply"]
    #[inline(always)]
    pub fn is_sel_ext_supply(&self) -> bool {
        *self == Lcdselvdd::SelExtSupply
    }
    #[doc = "R33 internally connected to AVDD"]
    #[inline(always)]
    pub fn is_sel_avdd(&self) -> bool {
        *self == Lcdselvdd::SelAvdd
    }
}
#[doc = "Field `LCDSELVDD` writer - Selects if R33 is supplied either from AVDD internally or from charge pump 0b = R33 connected to external supply 1b = R33 internally connected to AVDD"]
pub type LcdselvddW<'a, REG> = crate::BitWriter<'a, REG, Lcdselvdd>;
impl<'a, REG> LcdselvddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "R33 connected to external supply"]
    #[inline(always)]
    pub fn sel_ext_supply(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdselvdd::SelExtSupply)
    }
    #[doc = "R33 internally connected to AVDD"]
    #[inline(always)]
    pub fn sel_avdd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdselvdd::SelAvdd)
    }
}
#[doc = "Internal reference voltage enable on R13 0b = Internal reference voltage disabled 1b = Internal reference voltage enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdrefen {
    #[doc = "0: Internal reference voltage disabled"]
    IntRefDisable = 0,
    #[doc = "1: Internal reference voltage enabled"]
    IntRefEnable = 1,
}
impl From<Lcdrefen> for bool {
    #[inline(always)]
    fn from(variant: Lcdrefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDREFEN` reader - Internal reference voltage enable on R13 0b = Internal reference voltage disabled 1b = Internal reference voltage enabled"]
pub type LcdrefenR = crate::BitReader<Lcdrefen>;
impl LcdrefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdrefen {
        match self.bits {
            false => Lcdrefen::IntRefDisable,
            true => Lcdrefen::IntRefEnable,
        }
    }
    #[doc = "Internal reference voltage disabled"]
    #[inline(always)]
    pub fn is_int_ref_disable(&self) -> bool {
        *self == Lcdrefen::IntRefDisable
    }
    #[doc = "Internal reference voltage enabled"]
    #[inline(always)]
    pub fn is_int_ref_enable(&self) -> bool {
        *self == Lcdrefen::IntRefEnable
    }
}
#[doc = "Field `LCDREFEN` writer - Internal reference voltage enable on R13 0b = Internal reference voltage disabled 1b = Internal reference voltage enabled"]
pub type LcdrefenW<'a, REG> = crate::BitWriter<'a, REG, Lcdrefen>;
impl<'a, REG> LcdrefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference voltage disabled"]
    #[inline(always)]
    pub fn int_ref_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrefen::IntRefDisable)
    }
    #[doc = "Internal reference voltage enabled"]
    #[inline(always)]
    pub fn int_ref_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrefen::IntRefEnable)
    }
}
#[doc = "Charge pump enable 0b = Charge pump disabled(1) 1b = Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcpen {
    #[doc = "0: Charge pump disabled(1)"]
    CpDisable = 0,
    #[doc = "1: Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
    CpEnable = 1,
}
impl From<Lcdcpen> for bool {
    #[inline(always)]
    fn from(variant: Lcdcpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCPEN` reader - Charge pump enable 0b = Charge pump disabled(1) 1b = Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
pub type LcdcpenR = crate::BitReader<Lcdcpen>;
impl LcdcpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcpen {
        match self.bits {
            false => Lcdcpen::CpDisable,
            true => Lcdcpen::CpEnable,
        }
    }
    #[doc = "Charge pump disabled(1)"]
    #[inline(always)]
    pub fn is_cp_disable(&self) -> bool {
        *self == Lcdcpen::CpDisable
    }
    #[doc = "Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
    #[inline(always)]
    pub fn is_cp_enable(&self) -> bool {
        *self == Lcdcpen::CpEnable
    }
}
#[doc = "Field `LCDCPEN` writer - Charge pump enable 0b = Charge pump disabled(1) 1b = Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
pub type LcdcpenW<'a, REG> = crate::BitWriter<'a, REG, Lcdcpen>;
impl<'a, REG> LcdcpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge pump disabled(1)"]
    #[inline(always)]
    pub fn cp_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpen::CpDisable)
    }
    #[doc = "Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
    #[inline(always)]
    pub fn cp_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpen::CpEnable)
    }
}
#[doc = "Internal reference voltage select on R13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vlcdx {
    #[doc = "0: 2.60 V"]
    Sel2p60v = 0,
    #[doc = "1: 2.66 V"]
    Sel2p66v = 1,
    #[doc = "2: 2.72 V"]
    Sel2p72v = 2,
    #[doc = "3: 2.78 V"]
    Sel2p78v = 3,
    #[doc = "4: 2.84 V"]
    Sel2p84v = 4,
    #[doc = "5: 2.90 V"]
    Sel2p90v = 5,
    #[doc = "6: 2.96 V"]
    Sel2p96v = 6,
    #[doc = "7: 3.02 V"]
    Sel3p02v = 7,
    #[doc = "8: 3.08 V"]
    Sel3p08v = 8,
    #[doc = "9: 3.14 V"]
    Sel3p14v = 9,
    #[doc = "10: 3.20 V"]
    Sel3p20v = 10,
    #[doc = "11: 3.26 V"]
    Sel3p26v = 11,
    #[doc = "12: 3.32 V"]
    Sel3p32v = 12,
    #[doc = "13: 3.38 V"]
    Sel3p38v = 13,
    #[doc = "14: 3.44 V"]
    Sel3p44v = 14,
    #[doc = "15: 3.50 V"]
    Sel3p50v = 15,
}
impl From<Vlcdx> for u8 {
    #[inline(always)]
    fn from(variant: Vlcdx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vlcdx {
    type Ux = u8;
}
impl crate::IsEnum for Vlcdx {}
#[doc = "Field `VLCDx` reader - Internal reference voltage select on R13."]
pub type VlcdxR = crate::FieldReader<Vlcdx>;
impl VlcdxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlcdx {
        match self.bits {
            0 => Vlcdx::Sel2p60v,
            1 => Vlcdx::Sel2p66v,
            2 => Vlcdx::Sel2p72v,
            3 => Vlcdx::Sel2p78v,
            4 => Vlcdx::Sel2p84v,
            5 => Vlcdx::Sel2p90v,
            6 => Vlcdx::Sel2p96v,
            7 => Vlcdx::Sel3p02v,
            8 => Vlcdx::Sel3p08v,
            9 => Vlcdx::Sel3p14v,
            10 => Vlcdx::Sel3p20v,
            11 => Vlcdx::Sel3p26v,
            12 => Vlcdx::Sel3p32v,
            13 => Vlcdx::Sel3p38v,
            14 => Vlcdx::Sel3p44v,
            15 => Vlcdx::Sel3p50v,
            _ => unreachable!(),
        }
    }
    #[doc = "2.60 V"]
    #[inline(always)]
    pub fn is_sel_2p60v(&self) -> bool {
        *self == Vlcdx::Sel2p60v
    }
    #[doc = "2.66 V"]
    #[inline(always)]
    pub fn is_sel_2p66v(&self) -> bool {
        *self == Vlcdx::Sel2p66v
    }
    #[doc = "2.72 V"]
    #[inline(always)]
    pub fn is_sel_2p72v(&self) -> bool {
        *self == Vlcdx::Sel2p72v
    }
    #[doc = "2.78 V"]
    #[inline(always)]
    pub fn is_sel_2p78v(&self) -> bool {
        *self == Vlcdx::Sel2p78v
    }
    #[doc = "2.84 V"]
    #[inline(always)]
    pub fn is_sel_2p84v(&self) -> bool {
        *self == Vlcdx::Sel2p84v
    }
    #[doc = "2.90 V"]
    #[inline(always)]
    pub fn is_sel_2p90v(&self) -> bool {
        *self == Vlcdx::Sel2p90v
    }
    #[doc = "2.96 V"]
    #[inline(always)]
    pub fn is_sel_2p96v(&self) -> bool {
        *self == Vlcdx::Sel2p96v
    }
    #[doc = "3.02 V"]
    #[inline(always)]
    pub fn is_sel_3p02v(&self) -> bool {
        *self == Vlcdx::Sel3p02v
    }
    #[doc = "3.08 V"]
    #[inline(always)]
    pub fn is_sel_3p08v(&self) -> bool {
        *self == Vlcdx::Sel3p08v
    }
    #[doc = "3.14 V"]
    #[inline(always)]
    pub fn is_sel_3p14v(&self) -> bool {
        *self == Vlcdx::Sel3p14v
    }
    #[doc = "3.20 V"]
    #[inline(always)]
    pub fn is_sel_3p20v(&self) -> bool {
        *self == Vlcdx::Sel3p20v
    }
    #[doc = "3.26 V"]
    #[inline(always)]
    pub fn is_sel_3p26v(&self) -> bool {
        *self == Vlcdx::Sel3p26v
    }
    #[doc = "3.32 V"]
    #[inline(always)]
    pub fn is_sel_3p32v(&self) -> bool {
        *self == Vlcdx::Sel3p32v
    }
    #[doc = "3.38 V"]
    #[inline(always)]
    pub fn is_sel_3p38v(&self) -> bool {
        *self == Vlcdx::Sel3p38v
    }
    #[doc = "3.44 V"]
    #[inline(always)]
    pub fn is_sel_3p44v(&self) -> bool {
        *self == Vlcdx::Sel3p44v
    }
    #[doc = "3.50 V"]
    #[inline(always)]
    pub fn is_sel_3p50v(&self) -> bool {
        *self == Vlcdx::Sel3p50v
    }
}
#[doc = "Field `VLCDx` writer - Internal reference voltage select on R13."]
pub type VlcdxW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vlcdx, crate::Safe>;
impl<'a, REG> VlcdxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.60 V"]
    #[inline(always)]
    pub fn sel_2p60v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p60v)
    }
    #[doc = "2.66 V"]
    #[inline(always)]
    pub fn sel_2p66v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p66v)
    }
    #[doc = "2.72 V"]
    #[inline(always)]
    pub fn sel_2p72v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p72v)
    }
    #[doc = "2.78 V"]
    #[inline(always)]
    pub fn sel_2p78v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p78v)
    }
    #[doc = "2.84 V"]
    #[inline(always)]
    pub fn sel_2p84v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p84v)
    }
    #[doc = "2.90 V"]
    #[inline(always)]
    pub fn sel_2p90v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p90v)
    }
    #[doc = "2.96 V"]
    #[inline(always)]
    pub fn sel_2p96v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel2p96v)
    }
    #[doc = "3.02 V"]
    #[inline(always)]
    pub fn sel_3p02v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p02v)
    }
    #[doc = "3.08 V"]
    #[inline(always)]
    pub fn sel_3p08v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p08v)
    }
    #[doc = "3.14 V"]
    #[inline(always)]
    pub fn sel_3p14v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p14v)
    }
    #[doc = "3.20 V"]
    #[inline(always)]
    pub fn sel_3p20v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p20v)
    }
    #[doc = "3.26 V"]
    #[inline(always)]
    pub fn sel_3p26v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p26v)
    }
    #[doc = "3.32 V"]
    #[inline(always)]
    pub fn sel_3p32v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p32v)
    }
    #[doc = "3.38 V"]
    #[inline(always)]
    pub fn sel_3p38v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p38v)
    }
    #[doc = "3.44 V"]
    #[inline(always)]
    pub fn sel_3p44v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p44v)
    }
    #[doc = "3.50 V"]
    #[inline(always)]
    pub fn sel_3p50v(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdx::Sel3p50v)
    }
}
#[doc = "Charge pump frequency selection. 0000b = 32.768 kHz / 1 / 8 = 4.096 kHz 0001b = 32.768 kHz / 2 / 8 = 2.048 kHz 0010b = 32.768 kHz / 3 / 8 = 1.365 kHz 0011b = 32.768 kHz / 4 / 8 = 1.024 kHz 0100b = 32.768 kHz / 5 / 8 = 819 Hz 0101b = 32.768 kHz / 6 / 8 = 682 Hz 0110b = 32.768 kHz / 7 / 8 = 585 Hz 0111b = 32.768 kHz / 8 / 8 = 512 Hz 1000b = 32.768 kHz / 9 / 8 = 455 Hz 1001b = 32.768 kHz / 10 / 8 = 409 Hz 1010b = 32.768 kHz / 11 / 8 = 372 Hz 1011b = 32.768 kHz / 12 / 8 = 341 Hz 1100b = 32.768 kHz / 13 / 8 = 315 Hz 1101b = 32.768 kHz / 14 / 8 = 292 Hz 1110b = 32.768 kHz / 15 / 8 = 273 Hz 1111b = 32.768 kHz / 16 / 8 = 256 Hz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdcpfselx {
    #[doc = "0: 32.768 kHz / 1 / 8 = 4.096 kHz"]
    DivBy1By8 = 0,
    #[doc = "1: 32.768 kHz / 2 / 8 = 2.048 kHz"]
    DivBy2By8 = 1,
    #[doc = "2: 32.768 kHz / 3 / 8 = 1.365 kHz"]
    DivBy3By8 = 2,
    #[doc = "3: 32.768 kHz / 4 / 8 = 1.024 kHz"]
    DivBy4By8 = 3,
    #[doc = "4: 32.768 kHz / 5 / 8 = 819 Hz"]
    DivBy5By8 = 4,
    #[doc = "5: 32.768 kHz / 6 / 8 = 682 Hz"]
    DivBy6By8 = 5,
    #[doc = "6: 32.768 kHz / 7 / 8 = 585 Hz"]
    DivBy7By8 = 6,
    #[doc = "7: 32.768 kHz / 8 / 8 = 512 Hz"]
    DivBy8By8 = 7,
    #[doc = "8: 32.768 kHz / 9 / 8 = 455 Hz"]
    DivBy9By8 = 8,
    #[doc = "9: 32.768 kHz / 10 / 8 = 409 Hz"]
    DivBy10By8 = 9,
    #[doc = "10: 32.768 kHz / 11 / 8 = 372 Hz"]
    DivBy11By8 = 10,
    #[doc = "11: 32.768 kHz / 12 / 8 = 341 Hz"]
    DivBy12By8 = 11,
    #[doc = "12: 32.768 kHz / 13 / 8 = 315 Hz"]
    DivBy13By8 = 12,
    #[doc = "13: 32.768 kHz / 14 / 8 = 292 Hz"]
    DivBy14By8 = 13,
    #[doc = "14: 32.768 kHz / 15 / 8 = 273 Hz"]
    DivBy15By8 = 14,
    #[doc = "15: 32.768 kHz / 16 / 8 = 256 Hz"]
    DivBy16By8 = 15,
}
impl From<Lcdcpfselx> for u8 {
    #[inline(always)]
    fn from(variant: Lcdcpfselx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdcpfselx {
    type Ux = u8;
}
impl crate::IsEnum for Lcdcpfselx {}
#[doc = "Field `LCDCPFSELx` reader - Charge pump frequency selection. 0000b = 32.768 kHz / 1 / 8 = 4.096 kHz 0001b = 32.768 kHz / 2 / 8 = 2.048 kHz 0010b = 32.768 kHz / 3 / 8 = 1.365 kHz 0011b = 32.768 kHz / 4 / 8 = 1.024 kHz 0100b = 32.768 kHz / 5 / 8 = 819 Hz 0101b = 32.768 kHz / 6 / 8 = 682 Hz 0110b = 32.768 kHz / 7 / 8 = 585 Hz 0111b = 32.768 kHz / 8 / 8 = 512 Hz 1000b = 32.768 kHz / 9 / 8 = 455 Hz 1001b = 32.768 kHz / 10 / 8 = 409 Hz 1010b = 32.768 kHz / 11 / 8 = 372 Hz 1011b = 32.768 kHz / 12 / 8 = 341 Hz 1100b = 32.768 kHz / 13 / 8 = 315 Hz 1101b = 32.768 kHz / 14 / 8 = 292 Hz 1110b = 32.768 kHz / 15 / 8 = 273 Hz 1111b = 32.768 kHz / 16 / 8 = 256 Hz"]
pub type LcdcpfselxR = crate::FieldReader<Lcdcpfselx>;
impl LcdcpfselxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcpfselx {
        match self.bits {
            0 => Lcdcpfselx::DivBy1By8,
            1 => Lcdcpfselx::DivBy2By8,
            2 => Lcdcpfselx::DivBy3By8,
            3 => Lcdcpfselx::DivBy4By8,
            4 => Lcdcpfselx::DivBy5By8,
            5 => Lcdcpfselx::DivBy6By8,
            6 => Lcdcpfselx::DivBy7By8,
            7 => Lcdcpfselx::DivBy8By8,
            8 => Lcdcpfselx::DivBy9By8,
            9 => Lcdcpfselx::DivBy10By8,
            10 => Lcdcpfselx::DivBy11By8,
            11 => Lcdcpfselx::DivBy12By8,
            12 => Lcdcpfselx::DivBy13By8,
            13 => Lcdcpfselx::DivBy14By8,
            14 => Lcdcpfselx::DivBy15By8,
            15 => Lcdcpfselx::DivBy16By8,
            _ => unreachable!(),
        }
    }
    #[doc = "32.768 kHz / 1 / 8 = 4.096 kHz"]
    #[inline(always)]
    pub fn is_div_by_1_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy1By8
    }
    #[doc = "32.768 kHz / 2 / 8 = 2.048 kHz"]
    #[inline(always)]
    pub fn is_div_by_2_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy2By8
    }
    #[doc = "32.768 kHz / 3 / 8 = 1.365 kHz"]
    #[inline(always)]
    pub fn is_div_by_3_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy3By8
    }
    #[doc = "32.768 kHz / 4 / 8 = 1.024 kHz"]
    #[inline(always)]
    pub fn is_div_by_4_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy4By8
    }
    #[doc = "32.768 kHz / 5 / 8 = 819 Hz"]
    #[inline(always)]
    pub fn is_div_by_5_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy5By8
    }
    #[doc = "32.768 kHz / 6 / 8 = 682 Hz"]
    #[inline(always)]
    pub fn is_div_by_6_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy6By8
    }
    #[doc = "32.768 kHz / 7 / 8 = 585 Hz"]
    #[inline(always)]
    pub fn is_div_by_7_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy7By8
    }
    #[doc = "32.768 kHz / 8 / 8 = 512 Hz"]
    #[inline(always)]
    pub fn is_div_by_8_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy8By8
    }
    #[doc = "32.768 kHz / 9 / 8 = 455 Hz"]
    #[inline(always)]
    pub fn is_div_by_9_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy9By8
    }
    #[doc = "32.768 kHz / 10 / 8 = 409 Hz"]
    #[inline(always)]
    pub fn is_div_by_10_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy10By8
    }
    #[doc = "32.768 kHz / 11 / 8 = 372 Hz"]
    #[inline(always)]
    pub fn is_div_by_11_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy11By8
    }
    #[doc = "32.768 kHz / 12 / 8 = 341 Hz"]
    #[inline(always)]
    pub fn is_div_by_12_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy12By8
    }
    #[doc = "32.768 kHz / 13 / 8 = 315 Hz"]
    #[inline(always)]
    pub fn is_div_by_13_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy13By8
    }
    #[doc = "32.768 kHz / 14 / 8 = 292 Hz"]
    #[inline(always)]
    pub fn is_div_by_14_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy14By8
    }
    #[doc = "32.768 kHz / 15 / 8 = 273 Hz"]
    #[inline(always)]
    pub fn is_div_by_15_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy15By8
    }
    #[doc = "32.768 kHz / 16 / 8 = 256 Hz"]
    #[inline(always)]
    pub fn is_div_by_16_by_8(&self) -> bool {
        *self == Lcdcpfselx::DivBy16By8
    }
}
#[doc = "Field `LCDCPFSELx` writer - Charge pump frequency selection. 0000b = 32.768 kHz / 1 / 8 = 4.096 kHz 0001b = 32.768 kHz / 2 / 8 = 2.048 kHz 0010b = 32.768 kHz / 3 / 8 = 1.365 kHz 0011b = 32.768 kHz / 4 / 8 = 1.024 kHz 0100b = 32.768 kHz / 5 / 8 = 819 Hz 0101b = 32.768 kHz / 6 / 8 = 682 Hz 0110b = 32.768 kHz / 7 / 8 = 585 Hz 0111b = 32.768 kHz / 8 / 8 = 512 Hz 1000b = 32.768 kHz / 9 / 8 = 455 Hz 1001b = 32.768 kHz / 10 / 8 = 409 Hz 1010b = 32.768 kHz / 11 / 8 = 372 Hz 1011b = 32.768 kHz / 12 / 8 = 341 Hz 1100b = 32.768 kHz / 13 / 8 = 315 Hz 1101b = 32.768 kHz / 14 / 8 = 292 Hz 1110b = 32.768 kHz / 15 / 8 = 273 Hz 1111b = 32.768 kHz / 16 / 8 = 256 Hz"]
pub type LcdcpfselxW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lcdcpfselx, crate::Safe>;
impl<'a, REG> LcdcpfselxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768 kHz / 1 / 8 = 4.096 kHz"]
    #[inline(always)]
    pub fn div_by_1_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy1By8)
    }
    #[doc = "32.768 kHz / 2 / 8 = 2.048 kHz"]
    #[inline(always)]
    pub fn div_by_2_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy2By8)
    }
    #[doc = "32.768 kHz / 3 / 8 = 1.365 kHz"]
    #[inline(always)]
    pub fn div_by_3_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy3By8)
    }
    #[doc = "32.768 kHz / 4 / 8 = 1.024 kHz"]
    #[inline(always)]
    pub fn div_by_4_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy4By8)
    }
    #[doc = "32.768 kHz / 5 / 8 = 819 Hz"]
    #[inline(always)]
    pub fn div_by_5_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy5By8)
    }
    #[doc = "32.768 kHz / 6 / 8 = 682 Hz"]
    #[inline(always)]
    pub fn div_by_6_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy6By8)
    }
    #[doc = "32.768 kHz / 7 / 8 = 585 Hz"]
    #[inline(always)]
    pub fn div_by_7_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy7By8)
    }
    #[doc = "32.768 kHz / 8 / 8 = 512 Hz"]
    #[inline(always)]
    pub fn div_by_8_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy8By8)
    }
    #[doc = "32.768 kHz / 9 / 8 = 455 Hz"]
    #[inline(always)]
    pub fn div_by_9_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy9By8)
    }
    #[doc = "32.768 kHz / 10 / 8 = 409 Hz"]
    #[inline(always)]
    pub fn div_by_10_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy10By8)
    }
    #[doc = "32.768 kHz / 11 / 8 = 372 Hz"]
    #[inline(always)]
    pub fn div_by_11_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy11By8)
    }
    #[doc = "32.768 kHz / 12 / 8 = 341 Hz"]
    #[inline(always)]
    pub fn div_by_12_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy12By8)
    }
    #[doc = "32.768 kHz / 13 / 8 = 315 Hz"]
    #[inline(always)]
    pub fn div_by_13_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy13By8)
    }
    #[doc = "32.768 kHz / 14 / 8 = 292 Hz"]
    #[inline(always)]
    pub fn div_by_14_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy14By8)
    }
    #[doc = "32.768 kHz / 15 / 8 = 273 Hz"]
    #[inline(always)]
    pub fn div_by_15_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy15By8)
    }
    #[doc = "32.768 kHz / 16 / 8 = 256 Hz"]
    #[inline(always)]
    pub fn div_by_16_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpfselx::DivBy16By8)
    }
}
#[doc = "Enables the voltage boost circuitry which provides a boosted VDDA voltage. This boosted voltage is to be used in the switch controls, when the VDDA supply is less than 1.6V.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdvbsten {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: Enable."]
    Enable = 1,
}
impl From<Lcdvbsten> for bool {
    #[inline(always)]
    fn from(variant: Lcdvbsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDVBSTEN` reader - Enables the voltage boost circuitry which provides a boosted VDDA voltage. This boosted voltage is to be used in the switch controls, when the VDDA supply is less than 1.6V."]
pub type LcdvbstenR = crate::BitReader<Lcdvbsten>;
impl LcdvbstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdvbsten {
        match self.bits {
            false => Lcdvbsten::Disable,
            true => Lcdvbsten::Enable,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lcdvbsten::Disable
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lcdvbsten::Enable
    }
}
#[doc = "Field `LCDVBSTEN` writer - Enables the voltage boost circuitry which provides a boosted VDDA voltage. This boosted voltage is to be used in the switch controls, when the VDDA supply is less than 1.6V."]
pub type LcdvbstenW<'a, REG> = crate::BitWriter<'a, REG, Lcdvbsten>;
impl<'a, REG> LcdvbstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdvbsten::Disable)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdvbsten::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Selects whether R13 voltage is switched or in static mode 0b = Static mode 1b = Switched mode"]
    #[inline(always)]
    pub fn lcdrefmode(&self) -> LcdrefmodeR {
        LcdrefmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bias select. LCDBIASSEL is ignored in static mode as well as for 2-mux, 3-mux and 4-mux LCD modes. For 5-mux to 8-mux modes: 0b = 1/3 bias 1b = 1/4 bias"]
    #[inline(always)]
    pub fn lcdbiassel(&self) -> LcdbiasselR {
        LcdbiasselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the internal bias voltage resistor divider. The actual voltage source used for the resistor divider is selected by the VLCDSEL_VDD_R33 bit configuration. 0b = Internal bias voltage resistor divider is disabled 1b = Internal bias voltage resistor divider is enabledDD"]
    #[inline(always)]
    pub fn lcdintbiasen(&self) -> LcdintbiasenR {
        LcdintbiasenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects if the LCD bias voltage V1 is sourced from the R33 pin or from the internal supply voltage AVDD This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1 0b = V1 is sourced from R33 1b = V1 is sourced internally from AVDD"]
    #[inline(always)]
    pub fn vlcdsel_vdd_r33(&self) -> VlcdselVddR33R {
        VlcdselVddR33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High-power or Low-power LCD. This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1. It selects the resistor ladder that is used to generate the bias voltages for the LCD. 0b = Low-power LCD is used 1b = Higher-power LCD is used"]
    #[inline(always)]
    pub fn lcd_hp_lp(&self) -> LcdHpLpR {
        LcdHpLpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects if R33 is supplied either from AVDD internally or from charge pump 0b = R33 connected to external supply 1b = R33 internally connected to AVDD"]
    #[inline(always)]
    pub fn lcdselvdd(&self) -> LcdselvddR {
        LcdselvddR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal reference voltage enable on R13 0b = Internal reference voltage disabled 1b = Internal reference voltage enabled"]
    #[inline(always)]
    pub fn lcdrefen(&self) -> LcdrefenR {
        LcdrefenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Charge pump enable 0b = Charge pump disabled(1) 1b = Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
    #[inline(always)]
    pub fn lcdcpen(&self) -> LcdcpenR {
        LcdcpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Internal reference voltage select on R13."]
    #[inline(always)]
    pub fn vlcdx(&self) -> VlcdxR {
        VlcdxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Charge pump frequency selection. 0000b = 32.768 kHz / 1 / 8 = 4.096 kHz 0001b = 32.768 kHz / 2 / 8 = 2.048 kHz 0010b = 32.768 kHz / 3 / 8 = 1.365 kHz 0011b = 32.768 kHz / 4 / 8 = 1.024 kHz 0100b = 32.768 kHz / 5 / 8 = 819 Hz 0101b = 32.768 kHz / 6 / 8 = 682 Hz 0110b = 32.768 kHz / 7 / 8 = 585 Hz 0111b = 32.768 kHz / 8 / 8 = 512 Hz 1000b = 32.768 kHz / 9 / 8 = 455 Hz 1001b = 32.768 kHz / 10 / 8 = 409 Hz 1010b = 32.768 kHz / 11 / 8 = 372 Hz 1011b = 32.768 kHz / 12 / 8 = 341 Hz 1100b = 32.768 kHz / 13 / 8 = 315 Hz 1101b = 32.768 kHz / 14 / 8 = 292 Hz 1110b = 32.768 kHz / 15 / 8 = 273 Hz 1111b = 32.768 kHz / 16 / 8 = 256 Hz"]
    #[inline(always)]
    pub fn lcdcpfselx(&self) -> LcdcpfselxR {
        LcdcpfselxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enables the voltage boost circuitry which provides a boosted VDDA voltage. This boosted voltage is to be used in the switch controls, when the VDDA supply is less than 1.6V."]
    #[inline(always)]
    pub fn lcdvbsten(&self) -> LcdvbstenR {
        LcdvbstenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects whether R13 voltage is switched or in static mode 0b = Static mode 1b = Switched mode"]
    #[inline(always)]
    pub fn lcdrefmode(&mut self) -> LcdrefmodeW<'_, LcdLcdvctlSpec> {
        LcdrefmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - Bias select. LCDBIASSEL is ignored in static mode as well as for 2-mux, 3-mux and 4-mux LCD modes. For 5-mux to 8-mux modes: 0b = 1/3 bias 1b = 1/4 bias"]
    #[inline(always)]
    pub fn lcdbiassel(&mut self) -> LcdbiasselW<'_, LcdLcdvctlSpec> {
        LcdbiasselW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the internal bias voltage resistor divider. The actual voltage source used for the resistor divider is selected by the VLCDSEL_VDD_R33 bit configuration. 0b = Internal bias voltage resistor divider is disabled 1b = Internal bias voltage resistor divider is enabledDD"]
    #[inline(always)]
    pub fn lcdintbiasen(&mut self) -> LcdintbiasenW<'_, LcdLcdvctlSpec> {
        LcdintbiasenW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects if the LCD bias voltage V1 is sourced from the R33 pin or from the internal supply voltage AVDD This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1 0b = V1 is sourced from R33 1b = V1 is sourced internally from AVDD"]
    #[inline(always)]
    pub fn vlcdsel_vdd_r33(&mut self) -> VlcdselVddR33W<'_, LcdLcdvctlSpec> {
        VlcdselVddR33W::new(self, 3)
    }
    #[doc = "Bit 4 - High-power or Low-power LCD. This bit is only effective when the internal bias voltage resistor divider is used, that is, when LCDINTBIASEN = 1. It selects the resistor ladder that is used to generate the bias voltages for the LCD. 0b = Low-power LCD is used 1b = Higher-power LCD is used"]
    #[inline(always)]
    pub fn lcd_hp_lp(&mut self) -> LcdHpLpW<'_, LcdLcdvctlSpec> {
        LcdHpLpW::new(self, 4)
    }
    #[doc = "Bit 5 - Selects if R33 is supplied either from AVDD internally or from charge pump 0b = R33 connected to external supply 1b = R33 internally connected to AVDD"]
    #[inline(always)]
    pub fn lcdselvdd(&mut self) -> LcdselvddW<'_, LcdLcdvctlSpec> {
        LcdselvddW::new(self, 5)
    }
    #[doc = "Bit 6 - Internal reference voltage enable on R13 0b = Internal reference voltage disabled 1b = Internal reference voltage enabled"]
    #[inline(always)]
    pub fn lcdrefen(&mut self) -> LcdrefenW<'_, LcdLcdvctlSpec> {
        LcdrefenW::new(self, 6)
    }
    #[doc = "Bit 7 - Charge pump enable 0b = Charge pump disabled(1) 1b = Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCDx > 0 or VLCDREFx > 0."]
    #[inline(always)]
    pub fn lcdcpen(&mut self) -> LcdcpenW<'_, LcdLcdvctlSpec> {
        LcdcpenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Internal reference voltage select on R13."]
    #[inline(always)]
    pub fn vlcdx(&mut self) -> VlcdxW<'_, LcdLcdvctlSpec> {
        VlcdxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Charge pump frequency selection. 0000b = 32.768 kHz / 1 / 8 = 4.096 kHz 0001b = 32.768 kHz / 2 / 8 = 2.048 kHz 0010b = 32.768 kHz / 3 / 8 = 1.365 kHz 0011b = 32.768 kHz / 4 / 8 = 1.024 kHz 0100b = 32.768 kHz / 5 / 8 = 819 Hz 0101b = 32.768 kHz / 6 / 8 = 682 Hz 0110b = 32.768 kHz / 7 / 8 = 585 Hz 0111b = 32.768 kHz / 8 / 8 = 512 Hz 1000b = 32.768 kHz / 9 / 8 = 455 Hz 1001b = 32.768 kHz / 10 / 8 = 409 Hz 1010b = 32.768 kHz / 11 / 8 = 372 Hz 1011b = 32.768 kHz / 12 / 8 = 341 Hz 1100b = 32.768 kHz / 13 / 8 = 315 Hz 1101b = 32.768 kHz / 14 / 8 = 292 Hz 1110b = 32.768 kHz / 15 / 8 = 273 Hz 1111b = 32.768 kHz / 16 / 8 = 256 Hz"]
    #[inline(always)]
    pub fn lcdcpfselx(&mut self) -> LcdcpfselxW<'_, LcdLcdvctlSpec> {
        LcdcpfselxW::new(self, 12)
    }
    #[doc = "Bit 24 - Enables the voltage boost circuitry which provides a boosted VDDA voltage. This boosted voltage is to be used in the switch controls, when the VDDA supply is less than 1.6V."]
    #[inline(always)]
    pub fn lcdvbsten(&mut self) -> LcdvbstenW<'_, LcdLcdvctlSpec> {
        LcdvbstenW::new(self, 24)
    }
}
#[doc = "LCD voltage control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdvctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdvctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdvctlSpec;
impl crate::RegisterSpec for LcdLcdvctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdvctl::R`](R) reader structure"]
impl crate::Readable for LcdLcdvctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdvctl::W`](W) writer structure"]
impl crate::Writable for LcdLcdvctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDVCTL to value 0"]
impl crate::Resettable for LcdLcdvctlSpec {}
