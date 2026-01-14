#[doc = "Register `LCD_LCDCTL0` reader"]
pub type R = crate::R<LcdLcdctl0Spec>;
#[doc = "Register `LCD_LCDCTL0` writer"]
pub type W = crate::W<LcdLcdctl0Spec>;
#[doc = "LCD on. This bit turns the LCD module on or off. 0b = LCD module off 1b = LCD module on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdon {
    #[doc = "0: LCD module off"]
    LcdModDisable = 0,
    #[doc = "1: LCD module on"]
    LcdModEnable = 1,
}
impl From<Lcdon> for bool {
    #[inline(always)]
    fn from(variant: Lcdon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDON` reader - LCD on. This bit turns the LCD module on or off. 0b = LCD module off 1b = LCD module on"]
pub type LcdonR = crate::BitReader<Lcdon>;
impl LcdonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdon {
        match self.bits {
            false => Lcdon::LcdModDisable,
            true => Lcdon::LcdModEnable,
        }
    }
    #[doc = "LCD module off"]
    #[inline(always)]
    pub fn is_lcd_mod_disable(&self) -> bool {
        *self == Lcdon::LcdModDisable
    }
    #[doc = "LCD module on"]
    #[inline(always)]
    pub fn is_lcd_mod_enable(&self) -> bool {
        *self == Lcdon::LcdModEnable
    }
}
#[doc = "Field `LCDON` writer - LCD on. This bit turns the LCD module on or off. 0b = LCD module off 1b = LCD module on"]
pub type LcdonW<'a, REG> = crate::BitWriter<'a, REG, Lcdon>;
impl<'a, REG> LcdonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD module off"]
    #[inline(always)]
    pub fn lcd_mod_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdon::LcdModDisable)
    }
    #[doc = "LCD module on"]
    #[inline(always)]
    pub fn lcd_mod_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdon::LcdModEnable)
    }
}
#[doc = "LCD low-power waveform. This bit is only applicable for 1/3 bias mode, that is, for LCDBIASSEL = 0. 0b = Standard LCD waveforms on segment and common lines selected. 1b = Low-power LCD waveforms on segment and common lines selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdlp {
    #[doc = "0: Standard LCD waveforms"]
    StdLcd = 0,
    #[doc = "1: Low power LCD waveforms"]
    LpLcd = 1,
}
impl From<Lcdlp> for bool {
    #[inline(always)]
    fn from(variant: Lcdlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDLP` reader - LCD low-power waveform. This bit is only applicable for 1/3 bias mode, that is, for LCDBIASSEL = 0. 0b = Standard LCD waveforms on segment and common lines selected. 1b = Low-power LCD waveforms on segment and common lines selected."]
pub type LcdlpR = crate::BitReader<Lcdlp>;
impl LcdlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdlp {
        match self.bits {
            false => Lcdlp::StdLcd,
            true => Lcdlp::LpLcd,
        }
    }
    #[doc = "Standard LCD waveforms"]
    #[inline(always)]
    pub fn is_std_lcd(&self) -> bool {
        *self == Lcdlp::StdLcd
    }
    #[doc = "Low power LCD waveforms"]
    #[inline(always)]
    pub fn is_lp_lcd(&self) -> bool {
        *self == Lcdlp::LpLcd
    }
}
#[doc = "Field `LCDLP` writer - LCD low-power waveform. This bit is only applicable for 1/3 bias mode, that is, for LCDBIASSEL = 0. 0b = Standard LCD waveforms on segment and common lines selected. 1b = Low-power LCD waveforms on segment and common lines selected."]
pub type LcdlpW<'a, REG> = crate::BitWriter<'a, REG, Lcdlp>;
impl<'a, REG> LcdlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard LCD waveforms"]
    #[inline(always)]
    pub fn std_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdlp::StdLcd)
    }
    #[doc = "Low power LCD waveforms"]
    #[inline(always)]
    pub fn lp_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdlp::LpLcd)
    }
}
#[doc = "LCD segments on. This bit supports flashing LCD applications by turning off all segment lines, while leaving the LCD timing generator and R33 enabled. 0b = All LCD segments are off. 1b = All LCD segments are enabled and on or off according to their corresponding memory location.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdson {
    #[doc = "0: All LCD segments are off."]
    LcdSegOff = 0,
    #[doc = "1: All LCD segments are enabled and on or off according to their corresponding memory location."]
    LcdSegOn = 1,
}
impl From<Lcdson> for bool {
    #[inline(always)]
    fn from(variant: Lcdson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSON` reader - LCD segments on. This bit supports flashing LCD applications by turning off all segment lines, while leaving the LCD timing generator and R33 enabled. 0b = All LCD segments are off. 1b = All LCD segments are enabled and on or off according to their corresponding memory location."]
pub type LcdsonR = crate::BitReader<Lcdson>;
impl LcdsonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdson {
        match self.bits {
            false => Lcdson::LcdSegOff,
            true => Lcdson::LcdSegOn,
        }
    }
    #[doc = "All LCD segments are off."]
    #[inline(always)]
    pub fn is_lcd_seg_off(&self) -> bool {
        *self == Lcdson::LcdSegOff
    }
    #[doc = "All LCD segments are enabled and on or off according to their corresponding memory location."]
    #[inline(always)]
    pub fn is_lcd_seg_on(&self) -> bool {
        *self == Lcdson::LcdSegOn
    }
}
#[doc = "Field `LCDSON` writer - LCD segments on. This bit supports flashing LCD applications by turning off all segment lines, while leaving the LCD timing generator and R33 enabled. 0b = All LCD segments are off. 1b = All LCD segments are enabled and on or off according to their corresponding memory location."]
pub type LcdsonW<'a, REG> = crate::BitWriter<'a, REG, Lcdson>;
impl<'a, REG> LcdsonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All LCD segments are off."]
    #[inline(always)]
    pub fn lcd_seg_off(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdson::LcdSegOff)
    }
    #[doc = "All LCD segments are enabled and on or off according to their corresponding memory location."]
    #[inline(always)]
    pub fn lcd_seg_on(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdson::LcdSegOn)
    }
}
#[doc = "LCD mux rate. These bits select the LCD mode. Change only while LCDON = 0. 000b = Static 001b = 2-mux 010b = 3-mux 011b = 4-mux 100b = 5-mux 101b = 6-mux 110b = 7-mux 111b = 8-mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdmxx {
    #[doc = "0: Static"]
    MxStatic = 0,
    #[doc = "1: 2-mux"]
    Mx2 = 1,
    #[doc = "2: 3-mux"]
    Mx3 = 2,
    #[doc = "3: 4-mux"]
    Mx4 = 3,
    #[doc = "4: 5-mux"]
    Mx5 = 4,
    #[doc = "5: 6-mux"]
    Mx6 = 5,
    #[doc = "6: 7-mux"]
    Mx7 = 6,
    #[doc = "7: 8-mux"]
    Mx8 = 7,
}
impl From<Lcdmxx> for u8 {
    #[inline(always)]
    fn from(variant: Lcdmxx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdmxx {
    type Ux = u8;
}
impl crate::IsEnum for Lcdmxx {}
#[doc = "Field `LCDMXx` reader - LCD mux rate. These bits select the LCD mode. Change only while LCDON = 0. 000b = Static 001b = 2-mux 010b = 3-mux 011b = 4-mux 100b = 5-mux 101b = 6-mux 110b = 7-mux 111b = 8-mux"]
pub type LcdmxxR = crate::FieldReader<Lcdmxx>;
impl LcdmxxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdmxx {
        match self.bits {
            0 => Lcdmxx::MxStatic,
            1 => Lcdmxx::Mx2,
            2 => Lcdmxx::Mx3,
            3 => Lcdmxx::Mx4,
            4 => Lcdmxx::Mx5,
            5 => Lcdmxx::Mx6,
            6 => Lcdmxx::Mx7,
            7 => Lcdmxx::Mx8,
            _ => unreachable!(),
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_mx_static(&self) -> bool {
        *self == Lcdmxx::MxStatic
    }
    #[doc = "2-mux"]
    #[inline(always)]
    pub fn is_mx_2(&self) -> bool {
        *self == Lcdmxx::Mx2
    }
    #[doc = "3-mux"]
    #[inline(always)]
    pub fn is_mx_3(&self) -> bool {
        *self == Lcdmxx::Mx3
    }
    #[doc = "4-mux"]
    #[inline(always)]
    pub fn is_mx_4(&self) -> bool {
        *self == Lcdmxx::Mx4
    }
    #[doc = "5-mux"]
    #[inline(always)]
    pub fn is_mx_5(&self) -> bool {
        *self == Lcdmxx::Mx5
    }
    #[doc = "6-mux"]
    #[inline(always)]
    pub fn is_mx_6(&self) -> bool {
        *self == Lcdmxx::Mx6
    }
    #[doc = "7-mux"]
    #[inline(always)]
    pub fn is_mx_7(&self) -> bool {
        *self == Lcdmxx::Mx7
    }
    #[doc = "8-mux"]
    #[inline(always)]
    pub fn is_mx_8(&self) -> bool {
        *self == Lcdmxx::Mx8
    }
}
#[doc = "Field `LCDMXx` writer - LCD mux rate. These bits select the LCD mode. Change only while LCDON = 0. 000b = Static 001b = 2-mux 010b = 3-mux 011b = 4-mux 100b = 5-mux 101b = 6-mux 110b = 7-mux 111b = 8-mux"]
pub type LcdmxxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdmxx, crate::Safe>;
impl<'a, REG> LcdmxxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn mx_static(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::MxStatic)
    }
    #[doc = "2-mux"]
    #[inline(always)]
    pub fn mx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx2)
    }
    #[doc = "3-mux"]
    #[inline(always)]
    pub fn mx_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx3)
    }
    #[doc = "4-mux"]
    #[inline(always)]
    pub fn mx_4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx4)
    }
    #[doc = "5-mux"]
    #[inline(always)]
    pub fn mx_5(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx5)
    }
    #[doc = "6-mux"]
    #[inline(always)]
    pub fn mx_6(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx6)
    }
    #[doc = "7-mux"]
    #[inline(always)]
    pub fn mx_7(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx7)
    }
    #[doc = "8-mux"]
    #[inline(always)]
    pub fn mx_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmxx::Mx8)
    }
}
#[doc = "LCD frequency divider. Together with LCDMXx, the LCD frequency fLCD is calculated as fLCD = fSOURCE / ((LCDDIVx + 1) * Value\\[LCDMXx\\]). Change only while LCDON = 0. 00000b = Divide by 1 00001b = Divide by 2 . 11110b = Divide by 31 11111b = Divide by 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcddivx {
    #[doc = "0: Divide by 1"]
    DivBy1 = 0,
    #[doc = "1: Divide by 2"]
    DivBy2 = 1,
    #[doc = "2: Divide by 3"]
    DivBy3 = 2,
    #[doc = "3: Divide by 4"]
    DivBy4 = 3,
    #[doc = "4: Divide by 5"]
    DivBy5 = 4,
    #[doc = "5: Divide by 6"]
    DivBy6 = 5,
    #[doc = "6: Divide by 7"]
    DivBy7 = 6,
    #[doc = "7: Divide by 8"]
    DivBy8 = 7,
    #[doc = "8: Divide by 9"]
    DivBy9 = 8,
    #[doc = "9: Divide by 10"]
    DivBy10 = 9,
    #[doc = "10: Divide by 11"]
    DivBy11 = 10,
    #[doc = "11: Divide by 12"]
    DivBy12 = 11,
    #[doc = "12: Divide by 13"]
    DivBy13 = 12,
    #[doc = "13: Divide by 14"]
    DivBy14 = 13,
    #[doc = "14: Divide by 15"]
    DivBy15 = 14,
    #[doc = "15: Divide by 16"]
    DivBy16 = 15,
    #[doc = "16: Divide by 17"]
    DivBy17 = 16,
    #[doc = "17: Divide by 18"]
    DivBy18 = 17,
    #[doc = "18: Divide by 19"]
    DivBy19 = 18,
    #[doc = "19: Divide by 20"]
    DivBy20 = 19,
    #[doc = "20: Divide by 21"]
    DivBy21 = 20,
    #[doc = "21: Divide by 22"]
    DivBy22 = 21,
    #[doc = "22: Divide by 23"]
    DivBy23 = 22,
    #[doc = "23: Divide by 24"]
    DivBy24 = 23,
    #[doc = "24: Divide by 25"]
    DivBy25 = 24,
    #[doc = "25: Divide by 26"]
    DivBy26 = 25,
    #[doc = "26: Divide by 27"]
    DivBy27 = 26,
    #[doc = "27: Divide by 28"]
    DivBy28 = 27,
    #[doc = "28: Divide by 29"]
    DivBy29 = 28,
    #[doc = "29: Divide by 30"]
    DivBy30 = 29,
    #[doc = "30: Divide by 31"]
    DivBy31 = 30,
    #[doc = "31: Divide by 32"]
    DivBy32 = 31,
}
impl From<Lcddivx> for u8 {
    #[inline(always)]
    fn from(variant: Lcddivx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcddivx {
    type Ux = u8;
}
impl crate::IsEnum for Lcddivx {}
#[doc = "Field `LCDDIVx` reader - LCD frequency divider. Together with LCDMXx, the LCD frequency fLCD is calculated as fLCD = fSOURCE / ((LCDDIVx + 1) * Value\\[LCDMXx\\]). Change only while LCDON = 0. 00000b = Divide by 1 00001b = Divide by 2 . 11110b = Divide by 31 11111b = Divide by 32"]
pub type LcddivxR = crate::FieldReader<Lcddivx>;
impl LcddivxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcddivx {
        match self.bits {
            0 => Lcddivx::DivBy1,
            1 => Lcddivx::DivBy2,
            2 => Lcddivx::DivBy3,
            3 => Lcddivx::DivBy4,
            4 => Lcddivx::DivBy5,
            5 => Lcddivx::DivBy6,
            6 => Lcddivx::DivBy7,
            7 => Lcddivx::DivBy8,
            8 => Lcddivx::DivBy9,
            9 => Lcddivx::DivBy10,
            10 => Lcddivx::DivBy11,
            11 => Lcddivx::DivBy12,
            12 => Lcddivx::DivBy13,
            13 => Lcddivx::DivBy14,
            14 => Lcddivx::DivBy15,
            15 => Lcddivx::DivBy16,
            16 => Lcddivx::DivBy17,
            17 => Lcddivx::DivBy18,
            18 => Lcddivx::DivBy19,
            19 => Lcddivx::DivBy20,
            20 => Lcddivx::DivBy21,
            21 => Lcddivx::DivBy22,
            22 => Lcddivx::DivBy23,
            23 => Lcddivx::DivBy24,
            24 => Lcddivx::DivBy25,
            25 => Lcddivx::DivBy26,
            26 => Lcddivx::DivBy27,
            27 => Lcddivx::DivBy28,
            28 => Lcddivx::DivBy29,
            29 => Lcddivx::DivBy30,
            30 => Lcddivx::DivBy31,
            31 => Lcddivx::DivBy32,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == Lcddivx::DivBy1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Lcddivx::DivBy2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_div_by_3(&self) -> bool {
        *self == Lcddivx::DivBy3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Lcddivx::DivBy4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_div_by_5(&self) -> bool {
        *self == Lcddivx::DivBy5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_div_by_6(&self) -> bool {
        *self == Lcddivx::DivBy6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_div_by_7(&self) -> bool {
        *self == Lcddivx::DivBy7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Lcddivx::DivBy8
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn is_div_by_9(&self) -> bool {
        *self == Lcddivx::DivBy9
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_div_by_10(&self) -> bool {
        *self == Lcddivx::DivBy10
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn is_div_by_11(&self) -> bool {
        *self == Lcddivx::DivBy11
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn is_div_by_12(&self) -> bool {
        *self == Lcddivx::DivBy12
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn is_div_by_13(&self) -> bool {
        *self == Lcddivx::DivBy13
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn is_div_by_14(&self) -> bool {
        *self == Lcddivx::DivBy14
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn is_div_by_15(&self) -> bool {
        *self == Lcddivx::DivBy15
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == Lcddivx::DivBy16
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn is_div_by_17(&self) -> bool {
        *self == Lcddivx::DivBy17
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn is_div_by_18(&self) -> bool {
        *self == Lcddivx::DivBy18
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn is_div_by_19(&self) -> bool {
        *self == Lcddivx::DivBy19
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn is_div_by_20(&self) -> bool {
        *self == Lcddivx::DivBy20
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn is_div_by_21(&self) -> bool {
        *self == Lcddivx::DivBy21
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn is_div_by_22(&self) -> bool {
        *self == Lcddivx::DivBy22
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn is_div_by_23(&self) -> bool {
        *self == Lcddivx::DivBy23
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn is_div_by_24(&self) -> bool {
        *self == Lcddivx::DivBy24
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn is_div_by_25(&self) -> bool {
        *self == Lcddivx::DivBy25
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn is_div_by_26(&self) -> bool {
        *self == Lcddivx::DivBy26
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn is_div_by_27(&self) -> bool {
        *self == Lcddivx::DivBy27
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn is_div_by_28(&self) -> bool {
        *self == Lcddivx::DivBy28
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn is_div_by_29(&self) -> bool {
        *self == Lcddivx::DivBy29
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn is_div_by_30(&self) -> bool {
        *self == Lcddivx::DivBy30
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn is_div_by_31(&self) -> bool {
        *self == Lcddivx::DivBy31
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == Lcddivx::DivBy32
    }
}
#[doc = "Field `LCDDIVx` writer - LCD frequency divider. Together with LCDMXx, the LCD frequency fLCD is calculated as fLCD = fSOURCE / ((LCDDIVx + 1) * Value\\[LCDMXx\\]). Change only while LCDON = 0. 00000b = Divide by 1 00001b = Divide by 2 . 11110b = Divide by 31 11111b = Divide by 32"]
pub type LcddivxW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lcddivx, crate::Safe>;
impl<'a, REG> LcddivxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn div_by_5(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn div_by_7(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn div_by_9(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn div_by_10(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn div_by_11(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div_by_12(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn div_by_13(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn div_by_14(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn div_by_15(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy16)
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn div_by_17(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy17)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn div_by_18(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy18)
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn div_by_19(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy19)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn div_by_20(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy20)
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn div_by_21(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy21)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn div_by_22(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy22)
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn div_by_23(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy23)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn div_by_24(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy24)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn div_by_25(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy25)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn div_by_26(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy26)
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn div_by_27(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy27)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn div_by_28(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy28)
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn div_by_29(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy29)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn div_by_30(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy30)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn div_by_31(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy31)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddivx::DivBy32)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdsyncext {
    #[doc = "0: External eynchronization off."]
    LcdExtSyncOff = 0,
    #[doc = "1: External synchronization on."]
    LcdExtSyncOn = 1,
}
impl From<Lcdsyncext> for bool {
    #[inline(always)]
    fn from(variant: Lcdsyncext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSYNCEXT` reader - "]
pub type LcdsyncextR = crate::BitReader<Lcdsyncext>;
impl LcdsyncextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdsyncext {
        match self.bits {
            false => Lcdsyncext::LcdExtSyncOff,
            true => Lcdsyncext::LcdExtSyncOn,
        }
    }
    #[doc = "External eynchronization off."]
    #[inline(always)]
    pub fn is_lcd_ext_sync_off(&self) -> bool {
        *self == Lcdsyncext::LcdExtSyncOff
    }
    #[doc = "External synchronization on."]
    #[inline(always)]
    pub fn is_lcd_ext_sync_on(&self) -> bool {
        *self == Lcdsyncext::LcdExtSyncOn
    }
}
#[doc = "Field `LCDSYNCEXT` writer - "]
pub type LcdsyncextW<'a, REG> = crate::BitWriter<'a, REG, Lcdsyncext>;
impl<'a, REG> LcdsyncextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External eynchronization off."]
    #[inline(always)]
    pub fn lcd_ext_sync_off(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdsyncext::LcdExtSyncOff)
    }
    #[doc = "External synchronization on."]
    #[inline(always)]
    pub fn lcd_ext_sync_on(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdsyncext::LcdExtSyncOn)
    }
}
impl R {
    #[doc = "Bit 0 - LCD on. This bit turns the LCD module on or off. 0b = LCD module off 1b = LCD module on"]
    #[inline(always)]
    pub fn lcdon(&self) -> LcdonR {
        LcdonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD low-power waveform. This bit is only applicable for 1/3 bias mode, that is, for LCDBIASSEL = 0. 0b = Standard LCD waveforms on segment and common lines selected. 1b = Low-power LCD waveforms on segment and common lines selected."]
    #[inline(always)]
    pub fn lcdlp(&self) -> LcdlpR {
        LcdlpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segments on. This bit supports flashing LCD applications by turning off all segment lines, while leaving the LCD timing generator and R33 enabled. 0b = All LCD segments are off. 1b = All LCD segments are enabled and on or off according to their corresponding memory location."]
    #[inline(always)]
    pub fn lcdson(&self) -> LcdsonR {
        LcdsonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - LCD mux rate. These bits select the LCD mode. Change only while LCDON = 0. 000b = Static 001b = 2-mux 010b = 3-mux 011b = 4-mux 100b = 5-mux 101b = 6-mux 110b = 7-mux 111b = 8-mux"]
    #[inline(always)]
    pub fn lcdmxx(&self) -> LcdmxxR {
        LcdmxxR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 11:15 - LCD frequency divider. Together with LCDMXx, the LCD frequency fLCD is calculated as fLCD = fSOURCE / ((LCDDIVx + 1) * Value\\[LCDMXx\\]). Change only while LCDON = 0. 00000b = Divide by 1 00001b = Divide by 2 . 11110b = Divide by 31 11111b = Divide by 32"]
    #[inline(always)]
    pub fn lcddivx(&self) -> LcddivxR {
        LcddivxR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lcdsyncext(&self) -> LcdsyncextR {
        LcdsyncextR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD on. This bit turns the LCD module on or off. 0b = LCD module off 1b = LCD module on"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LcdonW<'_, LcdLcdctl0Spec> {
        LcdonW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD low-power waveform. This bit is only applicable for 1/3 bias mode, that is, for LCDBIASSEL = 0. 0b = Standard LCD waveforms on segment and common lines selected. 1b = Low-power LCD waveforms on segment and common lines selected."]
    #[inline(always)]
    pub fn lcdlp(&mut self) -> LcdlpW<'_, LcdLcdctl0Spec> {
        LcdlpW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD segments on. This bit supports flashing LCD applications by turning off all segment lines, while leaving the LCD timing generator and R33 enabled. 0b = All LCD segments are off. 1b = All LCD segments are enabled and on or off according to their corresponding memory location."]
    #[inline(always)]
    pub fn lcdson(&mut self) -> LcdsonW<'_, LcdLcdctl0Spec> {
        LcdsonW::new(self, 2)
    }
    #[doc = "Bits 3:5 - LCD mux rate. These bits select the LCD mode. Change only while LCDON = 0. 000b = Static 001b = 2-mux 010b = 3-mux 011b = 4-mux 100b = 5-mux 101b = 6-mux 110b = 7-mux 111b = 8-mux"]
    #[inline(always)]
    pub fn lcdmxx(&mut self) -> LcdmxxW<'_, LcdLcdctl0Spec> {
        LcdmxxW::new(self, 3)
    }
    #[doc = "Bits 11:15 - LCD frequency divider. Together with LCDMXx, the LCD frequency fLCD is calculated as fLCD = fSOURCE / ((LCDDIVx + 1) * Value\\[LCDMXx\\]). Change only while LCDON = 0. 00000b = Divide by 1 00001b = Divide by 2 . 11110b = Divide by 31 11111b = Divide by 32"]
    #[inline(always)]
    pub fn lcddivx(&mut self) -> LcddivxW<'_, LcdLcdctl0Spec> {
        LcddivxW::new(self, 11)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lcdsyncext(&mut self) -> LcdsyncextW<'_, LcdLcdctl0Spec> {
        LcdsyncextW::new(self, 24)
    }
}
#[doc = "LCD control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdctl0Spec;
impl crate::RegisterSpec for LcdLcdctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdctl0::R`](R) reader structure"]
impl crate::Readable for LcdLcdctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdctl0::W`](W) writer structure"]
impl crate::Writable for LcdLcdctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDCTL0 to value 0"]
impl crate::Resettable for LcdLcdctl0Spec {}
