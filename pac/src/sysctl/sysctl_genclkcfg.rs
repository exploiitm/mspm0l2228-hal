#[doc = "Register `SYSCTL_GENCLKCFG` reader"]
pub type R = crate::R<SysctlGenclkcfgSpec>;
#[doc = "Register `SYSCTL_GENCLKCFG` writer"]
pub type W = crate::W<SysctlGenclkcfgSpec>;
#[doc = "EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exclksrc {
    #[doc = "0: CLK_OUT is SYSOSC"]
    Sysosc = 0,
    #[doc = "1: CLK_OUT is ULPCLK (EXCLKDIVEN must be enabled)"]
    Ulpclk = 1,
    #[doc = "2: CLK_OUT is LFCLK"]
    Lfclk = 2,
    #[doc = "3: CLK_OUT is MFPCLK (EXCLKDIVEN must be enabled)"]
    Mfpclk = 3,
    #[doc = "4: CLK_OUT is HFCLK"]
    Hfclk = 4,
}
impl From<Exclksrc> for u8 {
    #[inline(always)]
    fn from(variant: Exclksrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exclksrc {
    type Ux = u8;
}
impl crate::IsEnum for Exclksrc {}
#[doc = "Field `EXCLKSRC` reader - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
pub type ExclksrcR = crate::FieldReader<Exclksrc>;
impl ExclksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exclksrc> {
        match self.bits {
            0 => Some(Exclksrc::Sysosc),
            1 => Some(Exclksrc::Ulpclk),
            2 => Some(Exclksrc::Lfclk),
            3 => Some(Exclksrc::Mfpclk),
            4 => Some(Exclksrc::Hfclk),
            _ => None,
        }
    }
    #[doc = "CLK_OUT is SYSOSC"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Exclksrc::Sysosc
    }
    #[doc = "CLK_OUT is ULPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn is_ulpclk(&self) -> bool {
        *self == Exclksrc::Ulpclk
    }
    #[doc = "CLK_OUT is LFCLK"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == Exclksrc::Lfclk
    }
    #[doc = "CLK_OUT is MFPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn is_mfpclk(&self) -> bool {
        *self == Exclksrc::Mfpclk
    }
    #[doc = "CLK_OUT is HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Exclksrc::Hfclk
    }
}
#[doc = "Field `EXCLKSRC` writer - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
pub type ExclksrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Exclksrc>;
impl<'a, REG> ExclksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_OUT is SYSOSC"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Sysosc)
    }
    #[doc = "CLK_OUT is ULPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn ulpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Ulpclk)
    }
    #[doc = "CLK_OUT is LFCLK"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Lfclk)
    }
    #[doc = "CLK_OUT is MFPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn mfpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Mfpclk)
    }
    #[doc = "CLK_OUT is HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Hfclk)
    }
}
#[doc = "EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exclkdivval {
    #[doc = "0: CLK_OUT source is divided by 2"]
    Div2 = 0,
    #[doc = "1: CLK_OUT source is divided by 4"]
    Div4 = 1,
    #[doc = "2: CLK_OUT source is divided by 6"]
    Div6 = 2,
    #[doc = "3: CLK_OUT source is divided by 8"]
    Div8 = 3,
    #[doc = "4: CLK_OUT source is divided by 10"]
    Div10 = 4,
    #[doc = "5: CLK_OUT source is divided by 12"]
    Div12 = 5,
    #[doc = "6: CLK_OUT source is divided by 14"]
    Div14 = 6,
    #[doc = "7: CLK_OUT source is divided by 16"]
    Div16 = 7,
}
impl From<Exclkdivval> for u8 {
    #[inline(always)]
    fn from(variant: Exclkdivval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exclkdivval {
    type Ux = u8;
}
impl crate::IsEnum for Exclkdivval {}
#[doc = "Field `EXCLKDIVVAL` reader - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
pub type ExclkdivvalR = crate::FieldReader<Exclkdivval>;
impl ExclkdivvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclkdivval {
        match self.bits {
            0 => Exclkdivval::Div2,
            1 => Exclkdivval::Div4,
            2 => Exclkdivval::Div6,
            3 => Exclkdivval::Div8,
            4 => Exclkdivval::Div10,
            5 => Exclkdivval::Div12,
            6 => Exclkdivval::Div14,
            7 => Exclkdivval::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Exclkdivval::Div2
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Exclkdivval::Div4
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Exclkdivval::Div6
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Exclkdivval::Div8
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Exclkdivval::Div10
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Exclkdivval::Div12
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Exclkdivval::Div14
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Exclkdivval::Div16
    }
}
#[doc = "Field `EXCLKDIVVAL` writer - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
pub type ExclkdivvalW<'a, REG> = crate::FieldWriter<'a, REG, 3, Exclkdivval, crate::Safe>;
impl<'a, REG> ExclkdivvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div2)
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div4)
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div6)
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div8)
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div10)
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div12)
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div14)
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div16)
    }
}
#[doc = "EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclkdiven {
    #[doc = "0: CLock divider is disabled (passthrough, EXCLKDIVVAL is not applied)"]
    Passthru = 0,
    #[doc = "1: Clock divider is enabled (EXCLKDIVVAL is applied)"]
    Enable = 1,
}
impl From<Exclkdiven> for bool {
    #[inline(always)]
    fn from(variant: Exclkdiven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLKDIVEN` reader - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
pub type ExclkdivenR = crate::BitReader<Exclkdiven>;
impl ExclkdivenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclkdiven {
        match self.bits {
            false => Exclkdiven::Passthru,
            true => Exclkdiven::Enable,
        }
    }
    #[doc = "CLock divider is disabled (passthrough, EXCLKDIVVAL is not applied)"]
    #[inline(always)]
    pub fn is_passthru(&self) -> bool {
        *self == Exclkdiven::Passthru
    }
    #[doc = "Clock divider is enabled (EXCLKDIVVAL is applied)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exclkdiven::Enable
    }
}
#[doc = "Field `EXCLKDIVEN` writer - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
pub type ExclkdivenW<'a, REG> = crate::BitWriter<'a, REG, Exclkdiven>;
impl<'a, REG> ExclkdivenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLock divider is disabled (passthrough, EXCLKDIVVAL is not applied)"]
    #[inline(always)]
    pub fn passthru(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdiven::Passthru)
    }
    #[doc = "Clock divider is enabled (EXCLKDIVVAL is applied)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdiven::Enable)
    }
}
#[doc = "MFPCLKSRC selects the MFPCLK (middle frequency precision clock) source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mfpclksrc {
    #[doc = "0: MFPCLK is sourced from SYSOSC"]
    Sysosc = 0,
    #[doc = "1: MFPCLK is sourced from HFCLK"]
    Hfclk = 1,
}
impl From<Mfpclksrc> for bool {
    #[inline(always)]
    fn from(variant: Mfpclksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFPCLKSRC` reader - MFPCLKSRC selects the MFPCLK (middle frequency precision clock) source."]
pub type MfpclksrcR = crate::BitReader<Mfpclksrc>;
impl MfpclksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mfpclksrc {
        match self.bits {
            false => Mfpclksrc::Sysosc,
            true => Mfpclksrc::Hfclk,
        }
    }
    #[doc = "MFPCLK is sourced from SYSOSC"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Mfpclksrc::Sysosc
    }
    #[doc = "MFPCLK is sourced from HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Mfpclksrc::Hfclk
    }
}
#[doc = "Field `MFPCLKSRC` writer - MFPCLKSRC selects the MFPCLK (middle frequency precision clock) source."]
pub type MfpclksrcW<'a, REG> = crate::BitWriter<'a, REG, Mfpclksrc>;
impl<'a, REG> MfpclksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MFPCLK is sourced from SYSOSC"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Mfpclksrc::Sysosc)
    }
    #[doc = "MFPCLK is sourced from HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mfpclksrc::Hfclk)
    }
}
#[doc = "HFCLK4MFPCLKDIV selects the divider applied to HFCLK when HFCLK is used as the MFPCLK source. Integer dividers from /1 to /16 may be selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfclk4mfpclkdiv {
    #[doc = "0: HFCLK is not divided before being used for MFPCLK"]
    Div1 = 0,
    #[doc = "1: HFCLK is divided by 2 before being used for MFPCLK"]
    Div2 = 1,
    #[doc = "2: HFCLK is divided by 3 before being used for MFPCLK"]
    Div3 = 2,
    #[doc = "3: HFCLK is divided by 4 before being used for MFPCLK"]
    Div4 = 3,
    #[doc = "4: HFCLK is divided by 5 before being used for MFPCLK"]
    Div5 = 4,
    #[doc = "5: HFCLK is divided by 6 before being used for MFPCLK"]
    Div6 = 5,
    #[doc = "6: HFCLK is divided by 7 before being used for MFPCLK"]
    Div7 = 6,
    #[doc = "7: HFCLK is divided by 8 before being used for MFPCLK"]
    Div8 = 7,
    #[doc = "8: HFCLK is divided by 9 before being used for MFPCLK"]
    Div9 = 8,
    #[doc = "9: HFCLK is divided by 10 before being used for MFPCLK"]
    Div10 = 9,
    #[doc = "10: HFCLK is divided by 11 before being used for MFPCLK"]
    Div11 = 10,
    #[doc = "11: HFCLK is divided by 12 before being used for MFPCLK"]
    Div12 = 11,
    #[doc = "12: HFCLK is divided by 13 before being used for MFPCLK"]
    Div13 = 12,
    #[doc = "13: HFCLK is divided by 14 before being used for MFPCLK"]
    Div14 = 13,
    #[doc = "14: HFCLK is divided by 15 before being used for MFPCLK"]
    Div15 = 14,
    #[doc = "15: HFCLK is divided by 16 before being used for MFPCLK"]
    Div16 = 15,
}
impl From<Hfclk4mfpclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Hfclk4mfpclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfclk4mfpclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Hfclk4mfpclkdiv {}
#[doc = "Field `HFCLK4MFPCLKDIV` reader - HFCLK4MFPCLKDIV selects the divider applied to HFCLK when HFCLK is used as the MFPCLK source. Integer dividers from /1 to /16 may be selected."]
pub type Hfclk4mfpclkdivR = crate::FieldReader<Hfclk4mfpclkdiv>;
impl Hfclk4mfpclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclk4mfpclkdiv {
        match self.bits {
            0 => Hfclk4mfpclkdiv::Div1,
            1 => Hfclk4mfpclkdiv::Div2,
            2 => Hfclk4mfpclkdiv::Div3,
            3 => Hfclk4mfpclkdiv::Div4,
            4 => Hfclk4mfpclkdiv::Div5,
            5 => Hfclk4mfpclkdiv::Div6,
            6 => Hfclk4mfpclkdiv::Div7,
            7 => Hfclk4mfpclkdiv::Div8,
            8 => Hfclk4mfpclkdiv::Div9,
            9 => Hfclk4mfpclkdiv::Div10,
            10 => Hfclk4mfpclkdiv::Div11,
            11 => Hfclk4mfpclkdiv::Div12,
            12 => Hfclk4mfpclkdiv::Div13,
            13 => Hfclk4mfpclkdiv::Div14,
            14 => Hfclk4mfpclkdiv::Div15,
            15 => Hfclk4mfpclkdiv::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "HFCLK is not divided before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div1
    }
    #[doc = "HFCLK is divided by 2 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div2
    }
    #[doc = "HFCLK is divided by 3 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div3
    }
    #[doc = "HFCLK is divided by 4 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div4
    }
    #[doc = "HFCLK is divided by 5 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div5
    }
    #[doc = "HFCLK is divided by 6 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div6
    }
    #[doc = "HFCLK is divided by 7 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div7
    }
    #[doc = "HFCLK is divided by 8 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div8
    }
    #[doc = "HFCLK is divided by 9 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div9
    }
    #[doc = "HFCLK is divided by 10 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div10
    }
    #[doc = "HFCLK is divided by 11 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div11
    }
    #[doc = "HFCLK is divided by 12 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div12
    }
    #[doc = "HFCLK is divided by 13 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div13
    }
    #[doc = "HFCLK is divided by 14 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div14
    }
    #[doc = "HFCLK is divided by 15 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div15
    }
    #[doc = "HFCLK is divided by 16 before being used for MFPCLK"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Hfclk4mfpclkdiv::Div16
    }
}
#[doc = "Field `HFCLK4MFPCLKDIV` writer - HFCLK4MFPCLKDIV selects the divider applied to HFCLK when HFCLK is used as the MFPCLK source. Integer dividers from /1 to /16 may be selected."]
pub type Hfclk4mfpclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hfclk4mfpclkdiv, crate::Safe>;
impl<'a, REG> Hfclk4mfpclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFCLK is not divided before being used for MFPCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div1)
    }
    #[doc = "HFCLK is divided by 2 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div2)
    }
    #[doc = "HFCLK is divided by 3 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div3)
    }
    #[doc = "HFCLK is divided by 4 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div4)
    }
    #[doc = "HFCLK is divided by 5 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div5)
    }
    #[doc = "HFCLK is divided by 6 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div6)
    }
    #[doc = "HFCLK is divided by 7 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div7)
    }
    #[doc = "HFCLK is divided by 8 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div8)
    }
    #[doc = "HFCLK is divided by 9 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div9)
    }
    #[doc = "HFCLK is divided by 10 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div10)
    }
    #[doc = "HFCLK is divided by 11 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div11)
    }
    #[doc = "HFCLK is divided by 12 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div12)
    }
    #[doc = "HFCLK is divided by 13 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div13)
    }
    #[doc = "HFCLK is divided by 14 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div14)
    }
    #[doc = "HFCLK is divided by 15 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div15)
    }
    #[doc = "HFCLK is divided by 16 before being used for MFPCLK"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk4mfpclkdiv::Div16)
    }
}
#[doc = "FCCSELCLK selectes the frequency clock counter (FCC) clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fccselclk {
    #[doc = "0: FCC clock is MCLK"]
    Mclk = 0,
    #[doc = "1: FCC clock is SYSOSC"]
    Sysosc = 1,
    #[doc = "2: FCC clock is HFCLK"]
    Hfclk = 2,
    #[doc = "3: FCC clock is the CLK_OUT selection"]
    Extclk = 3,
    #[doc = "7: FCC clock is the FCCIN external input"]
    Fccin = 7,
}
impl From<Fccselclk> for u8 {
    #[inline(always)]
    fn from(variant: Fccselclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fccselclk {
    type Ux = u8;
}
impl crate::IsEnum for Fccselclk {}
#[doc = "Field `FCCSELCLK` reader - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
pub type FccselclkR = crate::FieldReader<Fccselclk>;
impl FccselclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fccselclk> {
        match self.bits {
            0 => Some(Fccselclk::Mclk),
            1 => Some(Fccselclk::Sysosc),
            2 => Some(Fccselclk::Hfclk),
            3 => Some(Fccselclk::Extclk),
            7 => Some(Fccselclk::Fccin),
            _ => None,
        }
    }
    #[doc = "FCC clock is MCLK"]
    #[inline(always)]
    pub fn is_mclk(&self) -> bool {
        *self == Fccselclk::Mclk
    }
    #[doc = "FCC clock is SYSOSC"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Fccselclk::Sysosc
    }
    #[doc = "FCC clock is HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Fccselclk::Hfclk
    }
    #[doc = "FCC clock is the CLK_OUT selection"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Fccselclk::Extclk
    }
    #[doc = "FCC clock is the FCCIN external input"]
    #[inline(always)]
    pub fn is_fccin(&self) -> bool {
        *self == Fccselclk::Fccin
    }
}
#[doc = "Field `FCCSELCLK` writer - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
pub type FccselclkW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fccselclk>;
impl<'a, REG> FccselclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FCC clock is MCLK"]
    #[inline(always)]
    pub fn mclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Mclk)
    }
    #[doc = "FCC clock is SYSOSC"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Sysosc)
    }
    #[doc = "FCC clock is HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Hfclk)
    }
    #[doc = "FCC clock is the CLK_OUT selection"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Extclk)
    }
    #[doc = "FCC clock is the FCCIN external input"]
    #[inline(always)]
    pub fn fccin(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Fccin)
    }
}
#[doc = "FCCTRIGSRC selects the frequency clock counter (FCC) trigger source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcctrigsrc {
    #[doc = "0: FCC trigger is the external pin"]
    Extpin = 0,
    #[doc = "1: FCC trigger is the LFCLK"]
    Lfclk = 1,
}
impl From<Fcctrigsrc> for bool {
    #[inline(always)]
    fn from(variant: Fcctrigsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCTRIGSRC` reader - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
pub type FcctrigsrcR = crate::BitReader<Fcctrigsrc>;
impl FcctrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcctrigsrc {
        match self.bits {
            false => Fcctrigsrc::Extpin,
            true => Fcctrigsrc::Lfclk,
        }
    }
    #[doc = "FCC trigger is the external pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Fcctrigsrc::Extpin
    }
    #[doc = "FCC trigger is the LFCLK"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == Fcctrigsrc::Lfclk
    }
}
#[doc = "Field `FCCTRIGSRC` writer - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
pub type FcctrigsrcW<'a, REG> = crate::BitWriter<'a, REG, Fcctrigsrc>;
impl<'a, REG> FcctrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FCC trigger is the external pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Fcctrigsrc::Extpin)
    }
    #[doc = "FCC trigger is the LFCLK"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fcctrigsrc::Lfclk)
    }
}
#[doc = "FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcclvltrig {
    #[doc = "0: Rising edge to rising edge triggered"]
    Rise2rise = 0,
    #[doc = "1: Level triggered"]
    Level = 1,
}
impl From<Fcclvltrig> for bool {
    #[inline(always)]
    fn from(variant: Fcclvltrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCLVLTRIG` reader - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
pub type FcclvltrigR = crate::BitReader<Fcclvltrig>;
impl FcclvltrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcclvltrig {
        match self.bits {
            false => Fcclvltrig::Rise2rise,
            true => Fcclvltrig::Level,
        }
    }
    #[doc = "Rising edge to rising edge triggered"]
    #[inline(always)]
    pub fn is_rise2rise(&self) -> bool {
        *self == Fcclvltrig::Rise2rise
    }
    #[doc = "Level triggered"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Fcclvltrig::Level
    }
}
#[doc = "Field `FCCLVLTRIG` writer - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
pub type FcclvltrigW<'a, REG> = crate::BitWriter<'a, REG, Fcclvltrig>;
impl<'a, REG> FcclvltrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge to rising edge triggered"]
    #[inline(always)]
    pub fn rise2rise(self) -> &'a mut crate::W<REG> {
        self.variant(Fcclvltrig::Rise2rise)
    }
    #[doc = "Level triggered"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Fcclvltrig::Level)
    }
}
#[doc = "ANACPUMPCFG selects the analog mux charge pump (VBOOST) enable method.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anacpumpcfg {
    #[doc = "0: VBOOST is enabled on request from a COMP, GPAMP, or OPA"]
    Ondemand = 0,
    #[doc = "1: VBOOST is enabled when the device is in RUN or SLEEP mode, or when a COMP/GPAMP/OPA is enabled"]
    Onactive = 1,
    #[doc = "2: VBOOST is always enabled"]
    Onalways = 2,
}
impl From<Anacpumpcfg> for u8 {
    #[inline(always)]
    fn from(variant: Anacpumpcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anacpumpcfg {
    type Ux = u8;
}
impl crate::IsEnum for Anacpumpcfg {}
#[doc = "Field `ANACPUMPCFG` reader - ANACPUMPCFG selects the analog mux charge pump (VBOOST) enable method."]
pub type AnacpumpcfgR = crate::FieldReader<Anacpumpcfg>;
impl AnacpumpcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anacpumpcfg> {
        match self.bits {
            0 => Some(Anacpumpcfg::Ondemand),
            1 => Some(Anacpumpcfg::Onactive),
            2 => Some(Anacpumpcfg::Onalways),
            _ => None,
        }
    }
    #[doc = "VBOOST is enabled on request from a COMP, GPAMP, or OPA"]
    #[inline(always)]
    pub fn is_ondemand(&self) -> bool {
        *self == Anacpumpcfg::Ondemand
    }
    #[doc = "VBOOST is enabled when the device is in RUN or SLEEP mode, or when a COMP/GPAMP/OPA is enabled"]
    #[inline(always)]
    pub fn is_onactive(&self) -> bool {
        *self == Anacpumpcfg::Onactive
    }
    #[doc = "VBOOST is always enabled"]
    #[inline(always)]
    pub fn is_onalways(&self) -> bool {
        *self == Anacpumpcfg::Onalways
    }
}
#[doc = "Field `ANACPUMPCFG` writer - ANACPUMPCFG selects the analog mux charge pump (VBOOST) enable method."]
pub type AnacpumpcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anacpumpcfg>;
impl<'a, REG> AnacpumpcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VBOOST is enabled on request from a COMP, GPAMP, or OPA"]
    #[inline(always)]
    pub fn ondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Anacpumpcfg::Ondemand)
    }
    #[doc = "VBOOST is enabled when the device is in RUN or SLEEP mode, or when a COMP/GPAMP/OPA is enabled"]
    #[inline(always)]
    pub fn onactive(self) -> &'a mut crate::W<REG> {
        self.variant(Anacpumpcfg::Onactive)
    }
    #[doc = "VBOOST is always enabled"]
    #[inline(always)]
    pub fn onalways(self) -> &'a mut crate::W<REG> {
        self.variant(Anacpumpcfg::Onalways)
    }
}
#[doc = "Field `FCCTRIGCNT` reader - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
pub type FcctrigcntR = crate::FieldReader;
#[doc = "Field `FCCTRIGCNT` writer - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
pub type FcctrigcntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
    #[inline(always)]
    pub fn exclksrc(&self) -> ExclksrcR {
        ExclksrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdivval(&self) -> ExclkdivvalR {
        ExclkdivvalR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdiven(&self) -> ExclkdivenR {
        ExclkdivenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - MFPCLKSRC selects the MFPCLK (middle frequency precision clock) source."]
    #[inline(always)]
    pub fn mfpclksrc(&self) -> MfpclksrcR {
        MfpclksrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - HFCLK4MFPCLKDIV selects the divider applied to HFCLK when HFCLK is used as the MFPCLK source. Integer dividers from /1 to /16 may be selected."]
    #[inline(always)]
    pub fn hfclk4mfpclkdiv(&self) -> Hfclk4mfpclkdivR {
        Hfclk4mfpclkdivR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
    #[inline(always)]
    pub fn fccselclk(&self) -> FccselclkR {
        FccselclkR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
    #[inline(always)]
    pub fn fcctrigsrc(&self) -> FcctrigsrcR {
        FcctrigsrcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
    #[inline(always)]
    pub fn fcclvltrig(&self) -> FcclvltrigR {
        FcclvltrigR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - ANACPUMPCFG selects the analog mux charge pump (VBOOST) enable method."]
    #[inline(always)]
    pub fn anacpumpcfg(&self) -> AnacpumpcfgR {
        AnacpumpcfgR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:28 - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
    #[inline(always)]
    pub fn fcctrigcnt(&self) -> FcctrigcntR {
        FcctrigcntR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
    #[inline(always)]
    pub fn exclksrc(&mut self) -> ExclksrcW<'_, SysctlGenclkcfgSpec> {
        ExclksrcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdivval(&mut self) -> ExclkdivvalW<'_, SysctlGenclkcfgSpec> {
        ExclkdivvalW::new(self, 4)
    }
    #[doc = "Bit 7 - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdiven(&mut self) -> ExclkdivenW<'_, SysctlGenclkcfgSpec> {
        ExclkdivenW::new(self, 7)
    }
    #[doc = "Bit 9 - MFPCLKSRC selects the MFPCLK (middle frequency precision clock) source."]
    #[inline(always)]
    pub fn mfpclksrc(&mut self) -> MfpclksrcW<'_, SysctlGenclkcfgSpec> {
        MfpclksrcW::new(self, 9)
    }
    #[doc = "Bits 12:15 - HFCLK4MFPCLKDIV selects the divider applied to HFCLK when HFCLK is used as the MFPCLK source. Integer dividers from /1 to /16 may be selected."]
    #[inline(always)]
    pub fn hfclk4mfpclkdiv(&mut self) -> Hfclk4mfpclkdivW<'_, SysctlGenclkcfgSpec> {
        Hfclk4mfpclkdivW::new(self, 12)
    }
    #[doc = "Bits 16:19 - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
    #[inline(always)]
    pub fn fccselclk(&mut self) -> FccselclkW<'_, SysctlGenclkcfgSpec> {
        FccselclkW::new(self, 16)
    }
    #[doc = "Bit 20 - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
    #[inline(always)]
    pub fn fcctrigsrc(&mut self) -> FcctrigsrcW<'_, SysctlGenclkcfgSpec> {
        FcctrigsrcW::new(self, 20)
    }
    #[doc = "Bit 21 - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
    #[inline(always)]
    pub fn fcclvltrig(&mut self) -> FcclvltrigW<'_, SysctlGenclkcfgSpec> {
        FcclvltrigW::new(self, 21)
    }
    #[doc = "Bits 22:23 - ANACPUMPCFG selects the analog mux charge pump (VBOOST) enable method."]
    #[inline(always)]
    pub fn anacpumpcfg(&mut self) -> AnacpumpcfgW<'_, SysctlGenclkcfgSpec> {
        AnacpumpcfgW::new(self, 22)
    }
    #[doc = "Bits 24:28 - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
    #[inline(always)]
    pub fn fcctrigcnt(&mut self) -> FcctrigcntW<'_, SysctlGenclkcfgSpec> {
        FcctrigcntW::new(self, 24)
    }
}
#[doc = "General clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_genclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_genclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlGenclkcfgSpec;
impl crate::RegisterSpec for SysctlGenclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_genclkcfg::R`](R) reader structure"]
impl crate::Readable for SysctlGenclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_genclkcfg::W`](W) writer structure"]
impl crate::Writable for SysctlGenclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_GENCLKCFG to value 0"]
impl crate::Resettable for SysctlGenclkcfgSpec {}
