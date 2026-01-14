#[doc = "Register `COMP0_CTL2` reader"]
pub type R = crate::R<Comp0Ctl2Spec>;
#[doc = "Register `COMP0_CTL2` writer"]
pub type W = crate::W<Comp0Ctl2Spec>;
#[doc = "This bit requests ULP_REF bandgap operation in fast mode(static) or low power mode (sampled). The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Fast mode operation offers higher accuracy but consumes higher current. Low power operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL > 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refmode {
    #[doc = "0: ULP_REF bandgap, local reference buffer and 8-bit DAC inside comparator operate in static mode."]
    Static = 0,
    #[doc = "1: ULP_REF bandgap, local reference buffer and 8-bit DAC inside comparator operate in sampled mode."]
    Sampled = 1,
}
impl From<Refmode> for bool {
    #[inline(always)]
    fn from(variant: Refmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFMODE` reader - This bit requests ULP_REF bandgap operation in fast mode(static) or low power mode (sampled). The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Fast mode operation offers higher accuracy but consumes higher current. Low power operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL > 0."]
pub type RefmodeR = crate::BitReader<Refmode>;
impl RefmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refmode {
        match self.bits {
            false => Refmode::Static,
            true => Refmode::Sampled,
        }
    }
    #[doc = "ULP_REF bandgap, local reference buffer and 8-bit DAC inside comparator operate in static mode."]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Refmode::Static
    }
    #[doc = "ULP_REF bandgap, local reference buffer and 8-bit DAC inside comparator operate in sampled mode."]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == Refmode::Sampled
    }
}
#[doc = "Field `REFMODE` writer - This bit requests ULP_REF bandgap operation in fast mode(static) or low power mode (sampled). The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Fast mode operation offers higher accuracy but consumes higher current. Low power operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL > 0."]
pub type RefmodeW<'a, REG> = crate::BitWriter<'a, REG, Refmode>;
impl<'a, REG> RefmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ULP_REF bandgap, local reference buffer and 8-bit DAC inside comparator operate in static mode."]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(Refmode::Static)
    }
    #[doc = "ULP_REF bandgap, local reference buffer and 8-bit DAC inside comparator operate in sampled mode."]
    #[inline(always)]
    pub fn sampled(self) -> &'a mut crate::W<REG> {
        self.variant(Refmode::Sampled)
    }
}
#[doc = "These bits select the reference source for the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsrc {
    #[doc = "0: Reference voltage generator is disabled (local reference buffer as well as DAC)."]
    Off = 0,
    #[doc = "1: VDDA selected as the reference source to DAC and DAC output applied as reference to comparator."]
    VddaDac = 1,
    #[doc = "2: VREF selected as reference to DAC and DAC output applied as reference to comparator."]
    VrefDac = 2,
    #[doc = "3: In devices where internal VREF is buffered and hookedup to extrernal VREF pin, VREF applied as reference to comparator. DAC is switched off. Note: In LEGO_A3, DAC is turned off in this selection, in other deviced DAC is kept on."]
    Vref = 3,
    #[doc = "5: VDDA is used as comparator reference. Note: In LEGO_A3, DAC is turned off in this selection, in other deviced DAC is kept on."]
    Vdda = 5,
    #[doc = "6: Internal reference selected as the reference source to DAC and DAC output applied as reference to comparator."]
    IntvrefDac = 6,
    #[doc = "7: Internal VREF is used as the source of comparator. Not all devices will have this option."]
    Intvref = 7,
}
impl From<Refsrc> for u8 {
    #[inline(always)]
    fn from(variant: Refsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsrc {
    type Ux = u8;
}
impl crate::IsEnum for Refsrc {}
#[doc = "Field `REFSRC` reader - These bits select the reference source for the comparator."]
pub type RefsrcR = crate::FieldReader<Refsrc>;
impl RefsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refsrc> {
        match self.bits {
            0 => Some(Refsrc::Off),
            1 => Some(Refsrc::VddaDac),
            2 => Some(Refsrc::VrefDac),
            3 => Some(Refsrc::Vref),
            5 => Some(Refsrc::Vdda),
            6 => Some(Refsrc::IntvrefDac),
            7 => Some(Refsrc::Intvref),
            _ => None,
        }
    }
    #[doc = "Reference voltage generator is disabled (local reference buffer as well as DAC)."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Refsrc::Off
    }
    #[doc = "VDDA selected as the reference source to DAC and DAC output applied as reference to comparator."]
    #[inline(always)]
    pub fn is_vdda_dac(&self) -> bool {
        *self == Refsrc::VddaDac
    }
    #[doc = "VREF selected as reference to DAC and DAC output applied as reference to comparator."]
    #[inline(always)]
    pub fn is_vref_dac(&self) -> bool {
        *self == Refsrc::VrefDac
    }
    #[doc = "In devices where internal VREF is buffered and hookedup to extrernal VREF pin, VREF applied as reference to comparator. DAC is switched off. Note: In LEGO_A3, DAC is turned off in this selection, in other deviced DAC is kept on."]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Refsrc::Vref
    }
    #[doc = "VDDA is used as comparator reference. Note: In LEGO_A3, DAC is turned off in this selection, in other deviced DAC is kept on."]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == Refsrc::Vdda
    }
    #[doc = "Internal reference selected as the reference source to DAC and DAC output applied as reference to comparator."]
    #[inline(always)]
    pub fn is_intvref_dac(&self) -> bool {
        *self == Refsrc::IntvrefDac
    }
    #[doc = "Internal VREF is used as the source of comparator. Not all devices will have this option."]
    #[inline(always)]
    pub fn is_intvref(&self) -> bool {
        *self == Refsrc::Intvref
    }
}
#[doc = "Field `REFSRC` writer - These bits select the reference source for the comparator."]
pub type RefsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refsrc>;
impl<'a, REG> RefsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference voltage generator is disabled (local reference buffer as well as DAC)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::Off)
    }
    #[doc = "VDDA selected as the reference source to DAC and DAC output applied as reference to comparator."]
    #[inline(always)]
    pub fn vdda_dac(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::VddaDac)
    }
    #[doc = "VREF selected as reference to DAC and DAC output applied as reference to comparator."]
    #[inline(always)]
    pub fn vref_dac(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::VrefDac)
    }
    #[doc = "In devices where internal VREF is buffered and hookedup to extrernal VREF pin, VREF applied as reference to comparator. DAC is switched off. Note: In LEGO_A3, DAC is turned off in this selection, in other deviced DAC is kept on."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::Vref)
    }
    #[doc = "VDDA is used as comparator reference. Note: In LEGO_A3, DAC is turned off in this selection, in other deviced DAC is kept on."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::Vdda)
    }
    #[doc = "Internal reference selected as the reference source to DAC and DAC output applied as reference to comparator."]
    #[inline(always)]
    pub fn intvref_dac(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::IntvrefDac)
    }
    #[doc = "Internal VREF is used as the source of comparator. Not all devices will have this option."]
    #[inline(always)]
    pub fn intvref(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::Intvref)
    }
}
#[doc = "This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refsel {
    #[doc = "0: If EXCH bit is 0, the selected reference is applied to positive terminal. If EXCH bit is 1, the selected reference is applied to negative terminal."]
    Positive = 0,
    #[doc = "1: If EXCH bit is 0, the selected reference is applied to negative terminal. If EXCH bit is 1, the selected reference is applied to positive terminal."]
    Negative = 1,
}
impl From<Refsel> for bool {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFSEL` reader - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
pub type RefselR = crate::BitReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            false => Refsel::Positive,
            true => Refsel::Negative,
        }
    }
    #[doc = "If EXCH bit is 0, the selected reference is applied to positive terminal. If EXCH bit is 1, the selected reference is applied to negative terminal."]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == Refsel::Positive
    }
    #[doc = "If EXCH bit is 0, the selected reference is applied to negative terminal. If EXCH bit is 1, the selected reference is applied to positive terminal."]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == Refsel::Negative
    }
}
#[doc = "Field `REFSEL` writer - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
pub type RefselW<'a, REG> = crate::BitWriter<'a, REG, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If EXCH bit is 0, the selected reference is applied to positive terminal. If EXCH bit is 1, the selected reference is applied to negative terminal."]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Positive)
    }
    #[doc = "If EXCH bit is 0, the selected reference is applied to negative terminal. If EXCH bit is 1, the selected reference is applied to positive terminal."]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Negative)
    }
}
#[doc = "These bits select the blanking source for the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blanksrc {
    #[doc = "0: Blanking source disabled"]
    Disable = 0,
    #[doc = "1: Select Blanking Source 1"]
    Blanksrc1 = 1,
    #[doc = "2: Select Blanking Source 2"]
    Blanksrc2 = 2,
    #[doc = "3: Select Blanking Source 3"]
    Blanksrc3 = 3,
    #[doc = "4: Select Blanking Source 4"]
    Blanksrc4 = 4,
    #[doc = "5: Select Blanking Source 5"]
    Blanksrc5 = 5,
    #[doc = "6: Select Blanking Source 6"]
    Blanksrc6 = 6,
}
impl From<Blanksrc> for u8 {
    #[inline(always)]
    fn from(variant: Blanksrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blanksrc {
    type Ux = u8;
}
impl crate::IsEnum for Blanksrc {}
#[doc = "Field `BLANKSRC` reader - These bits select the blanking source for the comparator."]
pub type BlanksrcR = crate::FieldReader<Blanksrc>;
impl BlanksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blanksrc> {
        match self.bits {
            0 => Some(Blanksrc::Disable),
            1 => Some(Blanksrc::Blanksrc1),
            2 => Some(Blanksrc::Blanksrc2),
            3 => Some(Blanksrc::Blanksrc3),
            4 => Some(Blanksrc::Blanksrc4),
            5 => Some(Blanksrc::Blanksrc5),
            6 => Some(Blanksrc::Blanksrc6),
            _ => None,
        }
    }
    #[doc = "Blanking source disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blanksrc::Disable
    }
    #[doc = "Select Blanking Source 1"]
    #[inline(always)]
    pub fn is_blanksrc1(&self) -> bool {
        *self == Blanksrc::Blanksrc1
    }
    #[doc = "Select Blanking Source 2"]
    #[inline(always)]
    pub fn is_blanksrc2(&self) -> bool {
        *self == Blanksrc::Blanksrc2
    }
    #[doc = "Select Blanking Source 3"]
    #[inline(always)]
    pub fn is_blanksrc3(&self) -> bool {
        *self == Blanksrc::Blanksrc3
    }
    #[doc = "Select Blanking Source 4"]
    #[inline(always)]
    pub fn is_blanksrc4(&self) -> bool {
        *self == Blanksrc::Blanksrc4
    }
    #[doc = "Select Blanking Source 5"]
    #[inline(always)]
    pub fn is_blanksrc5(&self) -> bool {
        *self == Blanksrc::Blanksrc5
    }
    #[doc = "Select Blanking Source 6"]
    #[inline(always)]
    pub fn is_blanksrc6(&self) -> bool {
        *self == Blanksrc::Blanksrc6
    }
}
#[doc = "Field `BLANKSRC` writer - These bits select the blanking source for the comparator."]
pub type BlanksrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Blanksrc>;
impl<'a, REG> BlanksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blanking source disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Disable)
    }
    #[doc = "Select Blanking Source 1"]
    #[inline(always)]
    pub fn blanksrc1(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Blanksrc1)
    }
    #[doc = "Select Blanking Source 2"]
    #[inline(always)]
    pub fn blanksrc2(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Blanksrc2)
    }
    #[doc = "Select Blanking Source 3"]
    #[inline(always)]
    pub fn blanksrc3(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Blanksrc3)
    }
    #[doc = "Select Blanking Source 4"]
    #[inline(always)]
    pub fn blanksrc4(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Blanksrc4)
    }
    #[doc = "Select Blanking Source 5"]
    #[inline(always)]
    pub fn blanksrc5(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Blanksrc5)
    }
    #[doc = "Select Blanking Source 6"]
    #[inline(always)]
    pub fn blanksrc6(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksrc::Blanksrc6)
    }
}
#[doc = "This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacctl {
    #[doc = "0: Comparator output controls selection between DACCODE0 and DACCODE1"]
    CompoutSel = 0,
    #[doc = "1: DACSW bit controls selection between DACCODE0 and DACCODE1"]
    DacswSel = 1,
}
impl From<Dacctl> for bool {
    #[inline(always)]
    fn from(variant: Dacctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCTL` reader - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
pub type DacctlR = crate::BitReader<Dacctl>;
impl DacctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacctl {
        match self.bits {
            false => Dacctl::CompoutSel,
            true => Dacctl::DacswSel,
        }
    }
    #[doc = "Comparator output controls selection between DACCODE0 and DACCODE1"]
    #[inline(always)]
    pub fn is_compout_sel(&self) -> bool {
        *self == Dacctl::CompoutSel
    }
    #[doc = "DACSW bit controls selection between DACCODE0 and DACCODE1"]
    #[inline(always)]
    pub fn is_dacsw_sel(&self) -> bool {
        *self == Dacctl::DacswSel
    }
}
#[doc = "Field `DACCTL` writer - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
pub type DacctlW<'a, REG> = crate::BitWriter<'a, REG, Dacctl>;
impl<'a, REG> DacctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output controls selection between DACCODE0 and DACCODE1"]
    #[inline(always)]
    pub fn compout_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Dacctl::CompoutSel)
    }
    #[doc = "DACSW bit controls selection between DACCODE0 and DACCODE1"]
    #[inline(always)]
    pub fn dacsw_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Dacctl::DacswSel)
    }
}
#[doc = "This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacsw {
    #[doc = "0: DACCODE0 selected for 8-bit DAC"]
    Daccode0Sel = 0,
    #[doc = "1: DACCODE1 selected for 8-bit DAC"]
    Daccode1Sel = 1,
}
impl From<Dacsw> for bool {
    #[inline(always)]
    fn from(variant: Dacsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSW` reader - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
pub type DacswR = crate::BitReader<Dacsw>;
impl DacswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacsw {
        match self.bits {
            false => Dacsw::Daccode0Sel,
            true => Dacsw::Daccode1Sel,
        }
    }
    #[doc = "DACCODE0 selected for 8-bit DAC"]
    #[inline(always)]
    pub fn is_daccode0_sel(&self) -> bool {
        *self == Dacsw::Daccode0Sel
    }
    #[doc = "DACCODE1 selected for 8-bit DAC"]
    #[inline(always)]
    pub fn is_daccode1_sel(&self) -> bool {
        *self == Dacsw::Daccode1Sel
    }
}
#[doc = "Field `DACSW` writer - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
pub type DacswW<'a, REG> = crate::BitWriter<'a, REG, Dacsw>;
impl<'a, REG> DacswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DACCODE0 selected for 8-bit DAC"]
    #[inline(always)]
    pub fn daccode0_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Dacsw::Daccode0Sel)
    }
    #[doc = "DACCODE1 selected for 8-bit DAC"]
    #[inline(always)]
    pub fn daccode1_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Dacsw::Daccode1Sel)
    }
}
#[doc = "Enable sampled mode of comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampmode {
    #[doc = "0: Sampled mode disabled"]
    Disable = 0,
    #[doc = "1: Sampled mode enabled"]
    Enable = 1,
}
impl From<Sampmode> for bool {
    #[inline(always)]
    fn from(variant: Sampmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPMODE` reader - Enable sampled mode of comparator."]
pub type SampmodeR = crate::BitReader<Sampmode>;
impl SampmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampmode {
        match self.bits {
            false => Sampmode::Disable,
            true => Sampmode::Enable,
        }
    }
    #[doc = "Sampled mode disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sampmode::Disable
    }
    #[doc = "Sampled mode enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sampmode::Enable
    }
}
#[doc = "Field `SAMPMODE` writer - Enable sampled mode of comparator."]
pub type SampmodeW<'a, REG> = crate::BitWriter<'a, REG, Sampmode>;
impl<'a, REG> SampmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampled mode disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Disable)
    }
    #[doc = "Sampled mode enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit requests ULP_REF bandgap operation in fast mode(static) or low power mode (sampled). The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Fast mode operation offers higher accuracy but consumes higher current. Low power operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL > 0."]
    #[inline(always)]
    pub fn refmode(&self) -> RefmodeR {
        RefmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:5 - These bits select the reference source for the comparator."]
    #[inline(always)]
    pub fn refsrc(&self) -> RefsrcR {
        RefsrcR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - These bits select the blanking source for the comparator."]
    #[inline(always)]
    pub fn blanksrc(&self) -> BlanksrcR {
        BlanksrcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
    #[inline(always)]
    pub fn dacctl(&self) -> DacctlR {
        DacctlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
    #[inline(always)]
    pub fn dacsw(&self) -> DacswR {
        DacswR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable sampled mode of comparator."]
    #[inline(always)]
    pub fn sampmode(&self) -> SampmodeR {
        SampmodeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit requests ULP_REF bandgap operation in fast mode(static) or low power mode (sampled). The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Fast mode operation offers higher accuracy but consumes higher current. Low power operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL > 0."]
    #[inline(always)]
    pub fn refmode(&mut self) -> RefmodeW<'_, Comp0Ctl2Spec> {
        RefmodeW::new(self, 0)
    }
    #[doc = "Bits 3:5 - These bits select the reference source for the comparator."]
    #[inline(always)]
    pub fn refsrc(&mut self) -> RefsrcW<'_, Comp0Ctl2Spec> {
        RefsrcW::new(self, 3)
    }
    #[doc = "Bit 7 - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, Comp0Ctl2Spec> {
        RefselW::new(self, 7)
    }
    #[doc = "Bits 8:10 - These bits select the blanking source for the comparator."]
    #[inline(always)]
    pub fn blanksrc(&mut self) -> BlanksrcW<'_, Comp0Ctl2Spec> {
        BlanksrcW::new(self, 8)
    }
    #[doc = "Bit 16 - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
    #[inline(always)]
    pub fn dacctl(&mut self) -> DacctlW<'_, Comp0Ctl2Spec> {
        DacctlW::new(self, 16)
    }
    #[doc = "Bit 17 - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
    #[inline(always)]
    pub fn dacsw(&mut self) -> DacswW<'_, Comp0Ctl2Spec> {
        DacswW::new(self, 17)
    }
    #[doc = "Bit 24 - Enable sampled mode of comparator."]
    #[inline(always)]
    pub fn sampmode(&mut self) -> SampmodeW<'_, Comp0Ctl2Spec> {
        SampmodeW::new(self, 24)
    }
}
#[doc = "Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0Ctl2Spec;
impl crate::RegisterSpec for Comp0Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_ctl2::R`](R) reader structure"]
impl crate::Readable for Comp0Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`comp0_ctl2::W`](W) writer structure"]
impl crate::Writable for Comp0Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CTL2 to value 0"]
impl crate::Resettable for Comp0Ctl2Spec {}
