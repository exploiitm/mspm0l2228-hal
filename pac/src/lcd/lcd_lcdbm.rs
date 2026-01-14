#[doc = "Register `LCD_LCDBM[%s]` reader"]
pub type R = crate::R<LcdLcdbmSpec>;
#[doc = "Register `LCD_LCDBM[%s]` writer"]
pub type W = crate::W<LcdLcdbmSpec>;
#[doc = "If LCD L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM0 1b = Pin L\\[2*index\\] is used as COM0 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM0 1b = Pin L(index) is used as COM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit0 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit0> for bool {
    #[inline(always)]
    fn from(variant: Mbit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT0` reader - If LCD L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM0 1b = Pin L\\[2*index\\] is used as COM0 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM0 1b = Pin L(index) is used as COM0"]
pub type Mbit0R = crate::BitReader<Mbit0>;
impl Mbit0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit0 {
        match self.bits {
            false => Mbit0::Off,
            true => Mbit0::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit0::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit0::On
    }
}
#[doc = "Field `MBIT0` writer - If LCD L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM0 1b = Pin L\\[2*index\\] is used as COM0 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM0 1b = Pin L(index) is used as COM0"]
pub type Mbit0W<'a, REG> = crate::BitWriter<'a, REG, Mbit0>;
impl<'a, REG> Mbit0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit0::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit0::On)
    }
}
#[doc = "If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM1 1b = Pin L\\[2*index\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM1 1b = Pin L(index) is used as COM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit1 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit1> for bool {
    #[inline(always)]
    fn from(variant: Mbit1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT1` reader - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM1 1b = Pin L\\[2*index\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM1 1b = Pin L(index) is used as COM1"]
pub type Mbit1R = crate::BitReader<Mbit1>;
impl Mbit1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit1 {
        match self.bits {
            false => Mbit1::Off,
            true => Mbit1::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit1::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit1::On
    }
}
#[doc = "Field `MBIT1` writer - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM1 1b = Pin L\\[2*index\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM1 1b = Pin L(index) is used as COM1"]
pub type Mbit1W<'a, REG> = crate::BitWriter<'a, REG, Mbit1>;
impl<'a, REG> Mbit1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit1::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit1::On)
    }
}
#[doc = "If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM2 1b = Pin L\\[2*index\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L\\[index-1\\] not used as COM2 1b = Pin L\\[index-1\\] is used as COM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit2 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit2> for bool {
    #[inline(always)]
    fn from(variant: Mbit2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT2` reader - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM2 1b = Pin L\\[2*index\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L\\[index-1\\] not used as COM2 1b = Pin L\\[index-1\\] is used as COM2"]
pub type Mbit2R = crate::BitReader<Mbit2>;
impl Mbit2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit2 {
        match self.bits {
            false => Mbit2::Off,
            true => Mbit2::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit2::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit2::On
    }
}
#[doc = "Field `MBIT2` writer - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM2 1b = Pin L\\[2*index\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L\\[index-1\\] not used as COM2 1b = Pin L\\[index-1\\] is used as COM2"]
pub type Mbit2W<'a, REG> = crate::BitWriter<'a, REG, Mbit2>;
impl<'a, REG> Mbit2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit2::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit2::On)
    }
}
#[doc = "If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM3 1b = Pin L\\[2*index\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM3 1b = Pin L(index) is used as COM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit3 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit3> for bool {
    #[inline(always)]
    fn from(variant: Mbit3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT3` reader - If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM3 1b = Pin L\\[2*index\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM3 1b = Pin L(index) is used as COM3"]
pub type Mbit3R = crate::BitReader<Mbit3>;
impl Mbit3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit3 {
        match self.bits {
            false => Mbit3::Off,
            true => Mbit3::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit3::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit3::On
    }
}
#[doc = "Field `MBIT3` writer - If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM3 1b = Pin L\\[2*index\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM3 1b = Pin L(index) is used as COM3"]
pub type Mbit3W<'a, REG> = crate::BitWriter<'a, REG, Mbit3>;
impl<'a, REG> Mbit3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit3::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit3::On)
    }
}
#[doc = "If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*ndex+1\\] not used as COM0 1b = Pin L\\[2*ndex+1\\] is used as COM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit4 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit4> for bool {
    #[inline(always)]
    fn from(variant: Mbit4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT4` reader - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*ndex+1\\] not used as COM0 1b = Pin L\\[2*ndex+1\\] is used as COM0"]
pub type Mbit4R = crate::BitReader<Mbit4>;
impl Mbit4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit4 {
        match self.bits {
            false => Mbit4::Off,
            true => Mbit4::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit4::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit4::On
    }
}
#[doc = "Field `MBIT4` writer - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*ndex+1\\] not used as COM0 1b = Pin L\\[2*ndex+1\\] is used as COM0"]
pub type Mbit4W<'a, REG> = crate::BitWriter<'a, REG, Mbit4>;
impl<'a, REG> Mbit4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit4::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit4::On)
    }
}
#[doc = "If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM1 1b = Pin L\\[2*index+1\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 6-, 7- or 8-mux (LCDMXx >= 101b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM5 1b = Pin L(index) is used as COM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit5 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit5> for bool {
    #[inline(always)]
    fn from(variant: Mbit5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT5` reader - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM1 1b = Pin L\\[2*index+1\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 6-, 7- or 8-mux (LCDMXx >= 101b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM5 1b = Pin L(index) is used as COM5"]
pub type Mbit5R = crate::BitReader<Mbit5>;
impl Mbit5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit5 {
        match self.bits {
            false => Mbit5::Off,
            true => Mbit5::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit5::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit5::On
    }
}
#[doc = "Field `MBIT5` writer - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM1 1b = Pin L\\[2*index+1\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 6-, 7- or 8-mux (LCDMXx >= 101b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM5 1b = Pin L(index) is used as COM5"]
pub type Mbit5W<'a, REG> = crate::BitWriter<'a, REG, Mbit5>;
impl<'a, REG> Mbit5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit5::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit5::On)
    }
}
#[doc = "If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM2 1b = Pin L\\[2*index+1\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 7- or 8-mux (LCDMXx >= 110b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM6 1b = Pin L(index) is used as COM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit6 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit6> for bool {
    #[inline(always)]
    fn from(variant: Mbit6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT6` reader - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM2 1b = Pin L\\[2*index+1\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 7- or 8-mux (LCDMXx >= 110b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM6 1b = Pin L(index) is used as COM6"]
pub type Mbit6R = crate::BitReader<Mbit6>;
impl Mbit6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit6 {
        match self.bits {
            false => Mbit6::Off,
            true => Mbit6::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit6::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit6::On
    }
}
#[doc = "Field `MBIT6` writer - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM2 1b = Pin L\\[2*index+1\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 7- or 8-mux (LCDMXx >= 110b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM6 1b = Pin L(index) is used as COM6"]
pub type Mbit6W<'a, REG> = crate::BitWriter<'a, REG, Mbit6>;
impl<'a, REG> Mbit6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit6::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit6::On)
    }
}
#[doc = "If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM3 1b = Pin L\\[2*index+1\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 8-mux (LCDMXx = 111b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM7 1b = Pin L(index) is used as COM7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbit7 {
    #[doc = "0: LCD segment/common off"]
    Off = 0,
    #[doc = "1: LCD segment/common on"]
    On = 1,
}
impl From<Mbit7> for bool {
    #[inline(always)]
    fn from(variant: Mbit7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBIT7` reader - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM3 1b = Pin L\\[2*index+1\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 8-mux (LCDMXx = 111b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM7 1b = Pin L(index) is used as COM7"]
pub type Mbit7R = crate::BitReader<Mbit7>;
impl Mbit7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbit7 {
        match self.bits {
            false => Mbit7::Off,
            true => Mbit7::On,
        }
    }
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mbit7::Off
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mbit7::On
    }
}
#[doc = "Field `MBIT7` writer - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM3 1b = Pin L\\[2*index+1\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 8-mux (LCDMXx = 111b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM7 1b = Pin L(index) is used as COM7"]
pub type Mbit7W<'a, REG> = crate::BitWriter<'a, REG, Mbit7>;
impl<'a, REG> Mbit7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment/common off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit7::Off)
    }
    #[doc = "LCD segment/common on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mbit7::On)
    }
}
impl R {
    #[doc = "Bit 0 - If LCD L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM0 1b = Pin L\\[2*index\\] is used as COM0 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM0 1b = Pin L(index) is used as COM0"]
    #[inline(always)]
    pub fn mbit0(&self) -> Mbit0R {
        Mbit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM1 1b = Pin L\\[2*index\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM1 1b = Pin L(index) is used as COM1"]
    #[inline(always)]
    pub fn mbit1(&self) -> Mbit1R {
        Mbit1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM2 1b = Pin L\\[2*index\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L\\[index-1\\] not used as COM2 1b = Pin L\\[index-1\\] is used as COM2"]
    #[inline(always)]
    pub fn mbit2(&self) -> Mbit2R {
        Mbit2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM3 1b = Pin L\\[2*index\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM3 1b = Pin L(index) is used as COM3"]
    #[inline(always)]
    pub fn mbit3(&self) -> Mbit3R {
        Mbit3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*ndex+1\\] not used as COM0 1b = Pin L\\[2*ndex+1\\] is used as COM0"]
    #[inline(always)]
    pub fn mbit4(&self) -> Mbit4R {
        Mbit4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM1 1b = Pin L\\[2*index+1\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 6-, 7- or 8-mux (LCDMXx >= 101b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM5 1b = Pin L(index) is used as COM5"]
    #[inline(always)]
    pub fn mbit5(&self) -> Mbit5R {
        Mbit5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM2 1b = Pin L\\[2*index+1\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 7- or 8-mux (LCDMXx >= 110b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM6 1b = Pin L(index) is used as COM6"]
    #[inline(always)]
    pub fn mbit6(&self) -> Mbit6R {
        Mbit6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM3 1b = Pin L\\[2*index+1\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 8-mux (LCDMXx = 111b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM7 1b = Pin L(index) is used as COM7"]
    #[inline(always)]
    pub fn mbit7(&self) -> Mbit7R {
        Mbit7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If LCD L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM0 1b = Pin L\\[2*index\\] is used as COM0 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM0 1b = Pin L(index) is used as COM0"]
    #[inline(always)]
    pub fn mbit0(&mut self) -> Mbit0W<'_, LcdLcdbmSpec> {
        Mbit0W::new(self, 0)
    }
    #[doc = "Bit 1 - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM1 1b = Pin L\\[2*index\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM1 1b = Pin L(index) is used as COM1"]
    #[inline(always)]
    pub fn mbit1(&mut self) -> Mbit1W<'_, LcdLcdbmSpec> {
        Mbit1W::new(self, 1)
    }
    #[doc = "Bit 2 - If LCD pin L\\[2*index\\] is selected as segment line (LCDCSS\\[2*index\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM2 1b = Pin L\\[2*index\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L\\[index-1\\] not used as COM2 1b = Pin L\\[index-1\\] is used as COM2"]
    #[inline(always)]
    pub fn mbit2(&mut self) -> Mbit2W<'_, LcdLcdbmSpec> {
        Mbit2W::new(self, 2)
    }
    #[doc = "Bit 3 - If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index\\] is selected as common line (LCDCSS\\[2*index\\] = 1b): 0b = Pin L\\[2*index\\] not used as COM3 1b = Pin L\\[2*index\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 5-, 6-, 7- or 8-mux (LCDMXx >= 100b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM3 1b = Pin L(index) is used as COM3"]
    #[inline(always)]
    pub fn mbit3(&mut self) -> Mbit3W<'_, LcdLcdbmSpec> {
        Mbit3W::new(self, 3)
    }
    #[doc = "Bit 4 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is static, 2-, 3- or 4-mux (000b <= LCDMXx <= 011b) 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*ndex+1\\] not used as COM0 1b = Pin L\\[2*ndex+1\\] is used as COM0"]
    #[inline(always)]
    pub fn mbit4(&mut self) -> Mbit4W<'_, LcdLcdbmSpec> {
        Mbit4W::new(self, 4)
    }
    #[doc = "Bit 5 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 2-, 3- or 4-mux (001b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM1 1b = Pin L\\[2*index+1\\] is used as COM1 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 6-, 7- or 8-mux (LCDMXx >= 101b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM5 1b = Pin L(index) is used as COM5"]
    #[inline(always)]
    pub fn mbit5(&mut self) -> Mbit5W<'_, LcdLcdbmSpec> {
        Mbit5W::new(self, 5)
    }
    #[doc = "Bit 6 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 3- or 4-mux (010b <= LCDMXx <= 011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM2 1b = Pin L\\[2*index+1\\] is used as COM2 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 7- or 8-mux (LCDMXx >= 110b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM6 1b = Pin L(index) is used as COM6"]
    #[inline(always)]
    pub fn mbit6(&mut self) -> Mbit6W<'_, LcdLcdbmSpec> {
        Mbit6W::new(self, 6)
    }
    #[doc = "Bit 7 - If LCD pin L\\[2*index+1\\] is selected as segment line (LCDCSS\\[2*index+1\\] = 0b) and LCD mux rate is 4-mux (LCDMXx=011b): 0b = LCD segment off 1b = LCD segment on If LCD pin L\\[2*index+1\\] is selected as common line (LCDCSS\\[2*index+1\\] = 1b): 0b = Pin L\\[2*index+1\\] not used as COM3 1b = Pin L\\[2*index+1\\] is used as COM3 If LCD pin L(index) is selected as segment line (LCDCSS(index) = 0b) and LCD mux rate is 8-mux (LCDMXx = 111b): 0b = LCD segment off 1b = LCD segment on If LCD pin L(index) is selected as common line (LCDCSS(index) = 1b): 0b = Pin L(index) not used as COM7 1b = Pin L(index) is used as COM7"]
    #[inline(always)]
    pub fn mbit7(&mut self) -> Mbit7W<'_, LcdLcdbmSpec> {
        Mbit7W::new(self, 7)
    }
}
#[doc = "LCD blinking memory index register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdbm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdbm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdbmSpec;
impl crate::RegisterSpec for LcdLcdbmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_lcdbm::R`](R) reader structure"]
impl crate::Readable for LcdLcdbmSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdbm::W`](W) writer structure"]
impl crate::Writable for LcdLcdbmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDBM[%s] to value 0"]
impl crate::Resettable for LcdLcdbmSpec {}
