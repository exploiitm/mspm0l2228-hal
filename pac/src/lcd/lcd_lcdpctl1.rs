#[doc = "Register `LCD_LCDPCTL1` reader"]
pub type R = crate::R<LcdLcdpctl1Spec>;
#[doc = "Register `LCD_LCDPCTL1` writer"]
pub type W = crate::W<LcdLcdpctl1Spec>;
#[doc = "LCD segment line 16 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds16 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds16> for bool {
    #[inline(always)]
    fn from(variant: Lcds16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS16` reader - LCD segment line 16 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds16R = crate::BitReader<Lcds16>;
impl Lcds16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds16 {
        match self.bits {
            false => Lcds16::SelPort,
            true => Lcds16::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds16::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds16::SelLcd
    }
}
#[doc = "Field `LCDS16` writer - LCD segment line 16 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds16W<'a, REG> = crate::BitWriter<'a, REG, Lcds16>;
impl<'a, REG> Lcds16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds16::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds16::SelLcd)
    }
}
#[doc = "LCD segment line 17 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds17 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds17> for bool {
    #[inline(always)]
    fn from(variant: Lcds17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS17` reader - LCD segment line 17 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds17R = crate::BitReader<Lcds17>;
impl Lcds17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds17 {
        match self.bits {
            false => Lcds17::SelPort,
            true => Lcds17::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds17::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds17::SelLcd
    }
}
#[doc = "Field `LCDS17` writer - LCD segment line 17 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds17W<'a, REG> = crate::BitWriter<'a, REG, Lcds17>;
impl<'a, REG> Lcds17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds17::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds17::SelLcd)
    }
}
#[doc = "LCD segment line 18 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds18 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds18> for bool {
    #[inline(always)]
    fn from(variant: Lcds18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS18` reader - LCD segment line 18 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds18R = crate::BitReader<Lcds18>;
impl Lcds18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds18 {
        match self.bits {
            false => Lcds18::SelPort,
            true => Lcds18::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds18::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds18::SelLcd
    }
}
#[doc = "Field `LCDS18` writer - LCD segment line 18 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds18W<'a, REG> = crate::BitWriter<'a, REG, Lcds18>;
impl<'a, REG> Lcds18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds18::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds18::SelLcd)
    }
}
#[doc = "LCD segment line 19 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds19 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds19> for bool {
    #[inline(always)]
    fn from(variant: Lcds19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS19` reader - LCD segment line 19 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds19R = crate::BitReader<Lcds19>;
impl Lcds19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds19 {
        match self.bits {
            false => Lcds19::SelPort,
            true => Lcds19::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds19::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds19::SelLcd
    }
}
#[doc = "Field `LCDS19` writer - LCD segment line 19 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds19W<'a, REG> = crate::BitWriter<'a, REG, Lcds19>;
impl<'a, REG> Lcds19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds19::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds19::SelLcd)
    }
}
#[doc = "LCD segment line 20 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds20 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds20> for bool {
    #[inline(always)]
    fn from(variant: Lcds20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS20` reader - LCD segment line 20 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds20R = crate::BitReader<Lcds20>;
impl Lcds20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds20 {
        match self.bits {
            false => Lcds20::SelPort,
            true => Lcds20::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds20::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds20::SelLcd
    }
}
#[doc = "Field `LCDS20` writer - LCD segment line 20 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds20W<'a, REG> = crate::BitWriter<'a, REG, Lcds20>;
impl<'a, REG> Lcds20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds20::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds20::SelLcd)
    }
}
#[doc = "LCD segment line 21 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds21 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds21> for bool {
    #[inline(always)]
    fn from(variant: Lcds21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS21` reader - LCD segment line 21 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds21R = crate::BitReader<Lcds21>;
impl Lcds21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds21 {
        match self.bits {
            false => Lcds21::SelPort,
            true => Lcds21::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds21::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds21::SelLcd
    }
}
#[doc = "Field `LCDS21` writer - LCD segment line 21 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds21W<'a, REG> = crate::BitWriter<'a, REG, Lcds21>;
impl<'a, REG> Lcds21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds21::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds21::SelLcd)
    }
}
#[doc = "LCD segment line 22 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds22 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds22> for bool {
    #[inline(always)]
    fn from(variant: Lcds22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS22` reader - LCD segment line 22 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds22R = crate::BitReader<Lcds22>;
impl Lcds22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds22 {
        match self.bits {
            false => Lcds22::SelPort,
            true => Lcds22::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds22::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds22::SelLcd
    }
}
#[doc = "Field `LCDS22` writer - LCD segment line 22 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds22W<'a, REG> = crate::BitWriter<'a, REG, Lcds22>;
impl<'a, REG> Lcds22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds22::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds22::SelLcd)
    }
}
#[doc = "LCD segment line 23 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds23 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds23> for bool {
    #[inline(always)]
    fn from(variant: Lcds23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS23` reader - LCD segment line 23 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds23R = crate::BitReader<Lcds23>;
impl Lcds23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds23 {
        match self.bits {
            false => Lcds23::SelPort,
            true => Lcds23::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds23::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds23::SelLcd
    }
}
#[doc = "Field `LCDS23` writer - LCD segment line 23 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds23W<'a, REG> = crate::BitWriter<'a, REG, Lcds23>;
impl<'a, REG> Lcds23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds23::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds23::SelLcd)
    }
}
#[doc = "LCD pin 24 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds24 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds24> for bool {
    #[inline(always)]
    fn from(variant: Lcds24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS24` reader - LCD pin 24 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds24R = crate::BitReader<Lcds24>;
impl Lcds24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds24 {
        match self.bits {
            false => Lcds24::SelPort,
            true => Lcds24::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds24::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds24::SelLcd
    }
}
#[doc = "Field `LCDS24` writer - LCD pin 24 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds24W<'a, REG> = crate::BitWriter<'a, REG, Lcds24>;
impl<'a, REG> Lcds24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds24::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds24::SelLcd)
    }
}
#[doc = "LCD pin 25 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds25 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds25> for bool {
    #[inline(always)]
    fn from(variant: Lcds25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS25` reader - LCD pin 25 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds25R = crate::BitReader<Lcds25>;
impl Lcds25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds25 {
        match self.bits {
            false => Lcds25::SelPort,
            true => Lcds25::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds25::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds25::SelLcd
    }
}
#[doc = "Field `LCDS25` writer - LCD pin 25 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds25W<'a, REG> = crate::BitWriter<'a, REG, Lcds25>;
impl<'a, REG> Lcds25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds25::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds25::SelLcd)
    }
}
#[doc = "LCD pin 26 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds26 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds26> for bool {
    #[inline(always)]
    fn from(variant: Lcds26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS26` reader - LCD pin 26 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds26R = crate::BitReader<Lcds26>;
impl Lcds26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds26 {
        match self.bits {
            false => Lcds26::SelPort,
            true => Lcds26::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds26::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds26::SelLcd
    }
}
#[doc = "Field `LCDS26` writer - LCD pin 26 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds26W<'a, REG> = crate::BitWriter<'a, REG, Lcds26>;
impl<'a, REG> Lcds26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds26::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds26::SelLcd)
    }
}
#[doc = "LCD pin 27 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds27 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds27> for bool {
    #[inline(always)]
    fn from(variant: Lcds27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS27` reader - LCD pin 27 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds27R = crate::BitReader<Lcds27>;
impl Lcds27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds27 {
        match self.bits {
            false => Lcds27::SelPort,
            true => Lcds27::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds27::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds27::SelLcd
    }
}
#[doc = "Field `LCDS27` writer - LCD pin 27 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds27W<'a, REG> = crate::BitWriter<'a, REG, Lcds27>;
impl<'a, REG> Lcds27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds27::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds27::SelLcd)
    }
}
#[doc = "LCD pin 28 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds28 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds28> for bool {
    #[inline(always)]
    fn from(variant: Lcds28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS28` reader - LCD pin 28 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds28R = crate::BitReader<Lcds28>;
impl Lcds28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds28 {
        match self.bits {
            false => Lcds28::SelPort,
            true => Lcds28::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds28::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds28::SelLcd
    }
}
#[doc = "Field `LCDS28` writer - LCD pin 28 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds28W<'a, REG> = crate::BitWriter<'a, REG, Lcds28>;
impl<'a, REG> Lcds28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds28::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds28::SelLcd)
    }
}
#[doc = "LCD pin 29 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds29 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds29> for bool {
    #[inline(always)]
    fn from(variant: Lcds29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS29` reader - LCD pin 29 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds29R = crate::BitReader<Lcds29>;
impl Lcds29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds29 {
        match self.bits {
            false => Lcds29::SelPort,
            true => Lcds29::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds29::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds29::SelLcd
    }
}
#[doc = "Field `LCDS29` writer - LCD pin 29 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds29W<'a, REG> = crate::BitWriter<'a, REG, Lcds29>;
impl<'a, REG> Lcds29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds29::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds29::SelLcd)
    }
}
#[doc = "LCD pin 30 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds30 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds30> for bool {
    #[inline(always)]
    fn from(variant: Lcds30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS30` reader - LCD pin 30 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds30R = crate::BitReader<Lcds30>;
impl Lcds30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds30 {
        match self.bits {
            false => Lcds30::SelPort,
            true => Lcds30::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds30::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds30::SelLcd
    }
}
#[doc = "Field `LCDS30` writer - LCD pin 30 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds30W<'a, REG> = crate::BitWriter<'a, REG, Lcds30>;
impl<'a, REG> Lcds30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds30::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds30::SelLcd)
    }
}
#[doc = "LCD pin 31 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds31 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds31> for bool {
    #[inline(always)]
    fn from(variant: Lcds31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS31` reader - LCD pin 31 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds31R = crate::BitReader<Lcds31>;
impl Lcds31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds31 {
        match self.bits {
            false => Lcds31::SelPort,
            true => Lcds31::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds31::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds31::SelLcd
    }
}
#[doc = "Field `LCDS31` writer - LCD pin 31 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds31W<'a, REG> = crate::BitWriter<'a, REG, Lcds31>;
impl<'a, REG> Lcds31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds31::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds31::SelLcd)
    }
}
impl R {
    #[doc = "Bit 0 - LCD segment line 16 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds16(&self) -> Lcds16R {
        Lcds16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD segment line 17 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds17(&self) -> Lcds17R {
        Lcds17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segment line 18 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds18(&self) -> Lcds18R {
        Lcds18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD segment line 19 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds19(&self) -> Lcds19R {
        Lcds19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD segment line 20 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds20(&self) -> Lcds20R {
        Lcds20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD segment line 21 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds21(&self) -> Lcds21R {
        Lcds21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD segment line 22 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds22(&self) -> Lcds22R {
        Lcds22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD segment line 23 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds23(&self) -> Lcds23R {
        Lcds23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD pin 24 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds24(&self) -> Lcds24R {
        Lcds24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD pin 25 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds25(&self) -> Lcds25R {
        Lcds25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD pin 26 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds26(&self) -> Lcds26R {
        Lcds26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD pin 27 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds27(&self) -> Lcds27R {
        Lcds27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD pin 28 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds28(&self) -> Lcds28R {
        Lcds28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD pin 29 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds29(&self) -> Lcds29R {
        Lcds29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD pin 30 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds30(&self) -> Lcds30R {
        Lcds30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD pin 31 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds31(&self) -> Lcds31R {
        Lcds31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD segment line 16 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds16(&mut self) -> Lcds16W<'_, LcdLcdpctl1Spec> {
        Lcds16W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD segment line 17 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds17(&mut self) -> Lcds17W<'_, LcdLcdpctl1Spec> {
        Lcds17W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD segment line 18 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds18(&mut self) -> Lcds18W<'_, LcdLcdpctl1Spec> {
        Lcds18W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD segment line 19 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds19(&mut self) -> Lcds19W<'_, LcdLcdpctl1Spec> {
        Lcds19W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD segment line 20 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds20(&mut self) -> Lcds20W<'_, LcdLcdpctl1Spec> {
        Lcds20W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD segment line 21 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds21(&mut self) -> Lcds21W<'_, LcdLcdpctl1Spec> {
        Lcds21W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD segment line 22 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds22(&mut self) -> Lcds22W<'_, LcdLcdpctl1Spec> {
        Lcds22W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD segment line 23 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds23(&mut self) -> Lcds23W<'_, LcdLcdpctl1Spec> {
        Lcds23W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD pin 24 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds24(&mut self) -> Lcds24W<'_, LcdLcdpctl1Spec> {
        Lcds24W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD pin 25 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds25(&mut self) -> Lcds25W<'_, LcdLcdpctl1Spec> {
        Lcds25W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD pin 26 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds26(&mut self) -> Lcds26W<'_, LcdLcdpctl1Spec> {
        Lcds26W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD pin 27 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds27(&mut self) -> Lcds27W<'_, LcdLcdpctl1Spec> {
        Lcds27W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD pin 28 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds28(&mut self) -> Lcds28W<'_, LcdLcdpctl1Spec> {
        Lcds28W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD pin 29 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds29(&mut self) -> Lcds29W<'_, LcdLcdpctl1Spec> {
        Lcds29W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD pin 30 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds30(&mut self) -> Lcds30W<'_, LcdLcdpctl1Spec> {
        Lcds30W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD pin 31 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds31(&mut self) -> Lcds31W<'_, LcdLcdpctl1Spec> {
        Lcds31W::new(self, 15)
    }
}
#[doc = "LCD port control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdpctl1Spec;
impl crate::RegisterSpec for LcdLcdpctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdpctl1::R`](R) reader structure"]
impl crate::Readable for LcdLcdpctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdpctl1::W`](W) writer structure"]
impl crate::Writable for LcdLcdpctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDPCTL1 to value 0"]
impl crate::Resettable for LcdLcdpctl1Spec {}
