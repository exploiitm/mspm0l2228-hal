#[doc = "Register `LCD_LCDPCTL2` reader"]
pub type R = crate::R<LcdLcdpctl2Spec>;
#[doc = "Register `LCD_LCDPCTL2` writer"]
pub type W = crate::W<LcdLcdpctl2Spec>;
#[doc = "LCD pin 32 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds32 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds32> for bool {
    #[inline(always)]
    fn from(variant: Lcds32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS32` reader - LCD pin 32 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds32R = crate::BitReader<Lcds32>;
impl Lcds32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds32 {
        match self.bits {
            false => Lcds32::SelPort,
            true => Lcds32::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds32::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds32::SelLcd
    }
}
#[doc = "Field `LCDS32` writer - LCD pin 32 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds32W<'a, REG> = crate::BitWriter<'a, REG, Lcds32>;
impl<'a, REG> Lcds32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds32::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds32::SelLcd)
    }
}
#[doc = "LCD pin 33 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds33 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds33> for bool {
    #[inline(always)]
    fn from(variant: Lcds33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS33` reader - LCD pin 33 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds33R = crate::BitReader<Lcds33>;
impl Lcds33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds33 {
        match self.bits {
            false => Lcds33::SelPort,
            true => Lcds33::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds33::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds33::SelLcd
    }
}
#[doc = "Field `LCDS33` writer - LCD pin 33 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds33W<'a, REG> = crate::BitWriter<'a, REG, Lcds33>;
impl<'a, REG> Lcds33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds33::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds33::SelLcd)
    }
}
#[doc = "LCD pin 34 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds34 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds34> for bool {
    #[inline(always)]
    fn from(variant: Lcds34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS34` reader - LCD pin 34 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds34R = crate::BitReader<Lcds34>;
impl Lcds34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds34 {
        match self.bits {
            false => Lcds34::SelPort,
            true => Lcds34::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds34::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds34::SelLcd
    }
}
#[doc = "Field `LCDS34` writer - LCD pin 34 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds34W<'a, REG> = crate::BitWriter<'a, REG, Lcds34>;
impl<'a, REG> Lcds34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds34::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds34::SelLcd)
    }
}
#[doc = "LCD pin 35 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds35 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds35> for bool {
    #[inline(always)]
    fn from(variant: Lcds35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS35` reader - LCD pin 35 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds35R = crate::BitReader<Lcds35>;
impl Lcds35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds35 {
        match self.bits {
            false => Lcds35::SelPort,
            true => Lcds35::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds35::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds35::SelLcd
    }
}
#[doc = "Field `LCDS35` writer - LCD pin 35 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds35W<'a, REG> = crate::BitWriter<'a, REG, Lcds35>;
impl<'a, REG> Lcds35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds35::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds35::SelLcd)
    }
}
#[doc = "LCD pin 36 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds36 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds36> for bool {
    #[inline(always)]
    fn from(variant: Lcds36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS36` reader - LCD pin 36 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds36R = crate::BitReader<Lcds36>;
impl Lcds36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds36 {
        match self.bits {
            false => Lcds36::SelPort,
            true => Lcds36::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds36::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds36::SelLcd
    }
}
#[doc = "Field `LCDS36` writer - LCD pin 36 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds36W<'a, REG> = crate::BitWriter<'a, REG, Lcds36>;
impl<'a, REG> Lcds36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds36::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds36::SelLcd)
    }
}
#[doc = "LCD pin 37 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds37 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds37> for bool {
    #[inline(always)]
    fn from(variant: Lcds37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS37` reader - LCD pin 37 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds37R = crate::BitReader<Lcds37>;
impl Lcds37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds37 {
        match self.bits {
            false => Lcds37::SelPort,
            true => Lcds37::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds37::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds37::SelLcd
    }
}
#[doc = "Field `LCDS37` writer - LCD pin 37 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds37W<'a, REG> = crate::BitWriter<'a, REG, Lcds37>;
impl<'a, REG> Lcds37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds37::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds37::SelLcd)
    }
}
#[doc = "LCD pin 38 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds38 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds38> for bool {
    #[inline(always)]
    fn from(variant: Lcds38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS38` reader - LCD pin 38 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds38R = crate::BitReader<Lcds38>;
impl Lcds38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds38 {
        match self.bits {
            false => Lcds38::SelPort,
            true => Lcds38::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds38::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds38::SelLcd
    }
}
#[doc = "Field `LCDS38` writer - LCD pin 38 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds38W<'a, REG> = crate::BitWriter<'a, REG, Lcds38>;
impl<'a, REG> Lcds38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds38::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds38::SelLcd)
    }
}
#[doc = "LCD pin 39 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds39 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds39> for bool {
    #[inline(always)]
    fn from(variant: Lcds39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS39` reader - LCD pin 39 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds39R = crate::BitReader<Lcds39>;
impl Lcds39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds39 {
        match self.bits {
            false => Lcds39::SelPort,
            true => Lcds39::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds39::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds39::SelLcd
    }
}
#[doc = "Field `LCDS39` writer - LCD pin 39 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds39W<'a, REG> = crate::BitWriter<'a, REG, Lcds39>;
impl<'a, REG> Lcds39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds39::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds39::SelLcd)
    }
}
#[doc = "LCD pin 40 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds40 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds40> for bool {
    #[inline(always)]
    fn from(variant: Lcds40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS40` reader - LCD pin 40 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds40R = crate::BitReader<Lcds40>;
impl Lcds40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds40 {
        match self.bits {
            false => Lcds40::SelPort,
            true => Lcds40::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds40::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds40::SelLcd
    }
}
#[doc = "Field `LCDS40` writer - LCD pin 40 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds40W<'a, REG> = crate::BitWriter<'a, REG, Lcds40>;
impl<'a, REG> Lcds40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds40::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds40::SelLcd)
    }
}
#[doc = "LCD pin 41 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds41 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds41> for bool {
    #[inline(always)]
    fn from(variant: Lcds41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS41` reader - LCD pin 41 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds41R = crate::BitReader<Lcds41>;
impl Lcds41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds41 {
        match self.bits {
            false => Lcds41::SelPort,
            true => Lcds41::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds41::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds41::SelLcd
    }
}
#[doc = "Field `LCDS41` writer - LCD pin 41 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds41W<'a, REG> = crate::BitWriter<'a, REG, Lcds41>;
impl<'a, REG> Lcds41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds41::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds41::SelLcd)
    }
}
#[doc = "LCD pin 42 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds42 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds42> for bool {
    #[inline(always)]
    fn from(variant: Lcds42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS42` reader - LCD pin 42 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds42R = crate::BitReader<Lcds42>;
impl Lcds42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds42 {
        match self.bits {
            false => Lcds42::SelPort,
            true => Lcds42::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds42::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds42::SelLcd
    }
}
#[doc = "Field `LCDS42` writer - LCD pin 42 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds42W<'a, REG> = crate::BitWriter<'a, REG, Lcds42>;
impl<'a, REG> Lcds42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds42::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds42::SelLcd)
    }
}
#[doc = "LCD pin 43 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds43 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds43> for bool {
    #[inline(always)]
    fn from(variant: Lcds43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS43` reader - LCD pin 43 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds43R = crate::BitReader<Lcds43>;
impl Lcds43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds43 {
        match self.bits {
            false => Lcds43::SelPort,
            true => Lcds43::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds43::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds43::SelLcd
    }
}
#[doc = "Field `LCDS43` writer - LCD pin 43 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds43W<'a, REG> = crate::BitWriter<'a, REG, Lcds43>;
impl<'a, REG> Lcds43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds43::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds43::SelLcd)
    }
}
#[doc = "LCD pin 44 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds44 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds44> for bool {
    #[inline(always)]
    fn from(variant: Lcds44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS44` reader - LCD pin 44 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds44R = crate::BitReader<Lcds44>;
impl Lcds44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds44 {
        match self.bits {
            false => Lcds44::SelPort,
            true => Lcds44::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds44::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds44::SelLcd
    }
}
#[doc = "Field `LCDS44` writer - LCD pin 44 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds44W<'a, REG> = crate::BitWriter<'a, REG, Lcds44>;
impl<'a, REG> Lcds44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds44::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds44::SelLcd)
    }
}
#[doc = "LCD pin 45 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds45 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds45> for bool {
    #[inline(always)]
    fn from(variant: Lcds45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS45` reader - LCD pin 45 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds45R = crate::BitReader<Lcds45>;
impl Lcds45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds45 {
        match self.bits {
            false => Lcds45::SelPort,
            true => Lcds45::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds45::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds45::SelLcd
    }
}
#[doc = "Field `LCDS45` writer - LCD pin 45 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds45W<'a, REG> = crate::BitWriter<'a, REG, Lcds45>;
impl<'a, REG> Lcds45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds45::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds45::SelLcd)
    }
}
#[doc = "LCD pin 46 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds46 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds46> for bool {
    #[inline(always)]
    fn from(variant: Lcds46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS46` reader - LCD pin 46 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds46R = crate::BitReader<Lcds46>;
impl Lcds46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds46 {
        match self.bits {
            false => Lcds46::SelPort,
            true => Lcds46::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds46::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds46::SelLcd
    }
}
#[doc = "Field `LCDS46` writer - LCD pin 46 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds46W<'a, REG> = crate::BitWriter<'a, REG, Lcds46>;
impl<'a, REG> Lcds46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds46::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds46::SelLcd)
    }
}
#[doc = "LCD pin 47 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds47 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds47> for bool {
    #[inline(always)]
    fn from(variant: Lcds47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS47` reader - LCD pin 47 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds47R = crate::BitReader<Lcds47>;
impl Lcds47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds47 {
        match self.bits {
            false => Lcds47::SelPort,
            true => Lcds47::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds47::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds47::SelLcd
    }
}
#[doc = "Field `LCDS47` writer - LCD pin 47 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds47W<'a, REG> = crate::BitWriter<'a, REG, Lcds47>;
impl<'a, REG> Lcds47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds47::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds47::SelLcd)
    }
}
impl R {
    #[doc = "Bit 0 - LCD pin 32 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds32(&self) -> Lcds32R {
        Lcds32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD pin 33 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds33(&self) -> Lcds33R {
        Lcds33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD pin 34 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds34(&self) -> Lcds34R {
        Lcds34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD pin 35 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds35(&self) -> Lcds35R {
        Lcds35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD pin 36 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds36(&self) -> Lcds36R {
        Lcds36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD pin 37 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds37(&self) -> Lcds37R {
        Lcds37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD pin 38 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds38(&self) -> Lcds38R {
        Lcds38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD pin 39 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds39(&self) -> Lcds39R {
        Lcds39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD pin 40 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds40(&self) -> Lcds40R {
        Lcds40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD pin 41 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds41(&self) -> Lcds41R {
        Lcds41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD pin 42 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds42(&self) -> Lcds42R {
        Lcds42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD pin 43 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds43(&self) -> Lcds43R {
        Lcds43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD pin 44 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds44(&self) -> Lcds44R {
        Lcds44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD pin 45 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds45(&self) -> Lcds45R {
        Lcds45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD pin 46 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds46(&self) -> Lcds46R {
        Lcds46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD pin 47 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds47(&self) -> Lcds47R {
        Lcds47R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD pin 32 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds32(&mut self) -> Lcds32W<'_, LcdLcdpctl2Spec> {
        Lcds32W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD pin 33 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds33(&mut self) -> Lcds33W<'_, LcdLcdpctl2Spec> {
        Lcds33W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD pin 34 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds34(&mut self) -> Lcds34W<'_, LcdLcdpctl2Spec> {
        Lcds34W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD pin 35 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds35(&mut self) -> Lcds35W<'_, LcdLcdpctl2Spec> {
        Lcds35W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD pin 36 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds36(&mut self) -> Lcds36W<'_, LcdLcdpctl2Spec> {
        Lcds36W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD pin 37 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds37(&mut self) -> Lcds37W<'_, LcdLcdpctl2Spec> {
        Lcds37W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD pin 38 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds38(&mut self) -> Lcds38W<'_, LcdLcdpctl2Spec> {
        Lcds38W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD pin 39 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds39(&mut self) -> Lcds39W<'_, LcdLcdpctl2Spec> {
        Lcds39W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD pin 40 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds40(&mut self) -> Lcds40W<'_, LcdLcdpctl2Spec> {
        Lcds40W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD pin 41 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds41(&mut self) -> Lcds41W<'_, LcdLcdpctl2Spec> {
        Lcds41W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD pin 42 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds42(&mut self) -> Lcds42W<'_, LcdLcdpctl2Spec> {
        Lcds42W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD pin 43 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds43(&mut self) -> Lcds43W<'_, LcdLcdpctl2Spec> {
        Lcds43W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD pin 44 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds44(&mut self) -> Lcds44W<'_, LcdLcdpctl2Spec> {
        Lcds44W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD pin 45 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds45(&mut self) -> Lcds45W<'_, LcdLcdpctl2Spec> {
        Lcds45W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD pin 46 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds46(&mut self) -> Lcds46W<'_, LcdLcdpctl2Spec> {
        Lcds46W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD pin 47 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds47(&mut self) -> Lcds47W<'_, LcdLcdpctl2Spec> {
        Lcds47W::new(self, 15)
    }
}
#[doc = "LCD port control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdpctl2Spec;
impl crate::RegisterSpec for LcdLcdpctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdpctl2::R`](R) reader structure"]
impl crate::Readable for LcdLcdpctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdpctl2::W`](W) writer structure"]
impl crate::Writable for LcdLcdpctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDPCTL2 to value 0"]
impl crate::Resettable for LcdLcdpctl2Spec {}
