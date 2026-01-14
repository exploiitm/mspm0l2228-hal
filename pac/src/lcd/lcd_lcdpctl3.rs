#[doc = "Register `LCD_LCDPCTL3` reader"]
pub type R = crate::R<LcdLcdpctl3Spec>;
#[doc = "Register `LCD_LCDPCTL3` writer"]
pub type W = crate::W<LcdLcdpctl3Spec>;
#[doc = "LCD pin 48 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds48 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds48> for bool {
    #[inline(always)]
    fn from(variant: Lcds48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS48` reader - LCD pin 48 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds48R = crate::BitReader<Lcds48>;
impl Lcds48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds48 {
        match self.bits {
            false => Lcds48::SelPort,
            true => Lcds48::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds48::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds48::SelLcd
    }
}
#[doc = "Field `LCDS48` writer - LCD pin 48 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds48W<'a, REG> = crate::BitWriter<'a, REG, Lcds48>;
impl<'a, REG> Lcds48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds48::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds48::SelLcd)
    }
}
#[doc = "LCD pin 49 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds49 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds49> for bool {
    #[inline(always)]
    fn from(variant: Lcds49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS49` reader - LCD pin 49 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds49R = crate::BitReader<Lcds49>;
impl Lcds49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds49 {
        match self.bits {
            false => Lcds49::SelPort,
            true => Lcds49::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds49::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds49::SelLcd
    }
}
#[doc = "Field `LCDS49` writer - LCD pin 49 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds49W<'a, REG> = crate::BitWriter<'a, REG, Lcds49>;
impl<'a, REG> Lcds49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds49::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds49::SelLcd)
    }
}
#[doc = "LCD pin 50 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds50 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds50> for bool {
    #[inline(always)]
    fn from(variant: Lcds50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS50` reader - LCD pin 50 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds50R = crate::BitReader<Lcds50>;
impl Lcds50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds50 {
        match self.bits {
            false => Lcds50::SelPort,
            true => Lcds50::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds50::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds50::SelLcd
    }
}
#[doc = "Field `LCDS50` writer - LCD pin 50 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds50W<'a, REG> = crate::BitWriter<'a, REG, Lcds50>;
impl<'a, REG> Lcds50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds50::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds50::SelLcd)
    }
}
#[doc = "LCD pin 51 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds51 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds51> for bool {
    #[inline(always)]
    fn from(variant: Lcds51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS51` reader - LCD pin 51 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds51R = crate::BitReader<Lcds51>;
impl Lcds51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds51 {
        match self.bits {
            false => Lcds51::SelPort,
            true => Lcds51::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds51::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds51::SelLcd
    }
}
#[doc = "Field `LCDS51` writer - LCD pin 51 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds51W<'a, REG> = crate::BitWriter<'a, REG, Lcds51>;
impl<'a, REG> Lcds51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds51::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds51::SelLcd)
    }
}
#[doc = "LCD pin 52 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds52 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds52> for bool {
    #[inline(always)]
    fn from(variant: Lcds52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS52` reader - LCD pin 52 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds52R = crate::BitReader<Lcds52>;
impl Lcds52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds52 {
        match self.bits {
            false => Lcds52::SelPort,
            true => Lcds52::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds52::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds52::SelLcd
    }
}
#[doc = "Field `LCDS52` writer - LCD pin 52 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds52W<'a, REG> = crate::BitWriter<'a, REG, Lcds52>;
impl<'a, REG> Lcds52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds52::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds52::SelLcd)
    }
}
#[doc = "LCD pin 53 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds53 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds53> for bool {
    #[inline(always)]
    fn from(variant: Lcds53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS53` reader - LCD pin 53 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds53R = crate::BitReader<Lcds53>;
impl Lcds53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds53 {
        match self.bits {
            false => Lcds53::SelPort,
            true => Lcds53::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds53::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds53::SelLcd
    }
}
#[doc = "Field `LCDS53` writer - LCD pin 53 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds53W<'a, REG> = crate::BitWriter<'a, REG, Lcds53>;
impl<'a, REG> Lcds53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds53::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds53::SelLcd)
    }
}
#[doc = "LCD pin 54 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds54 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds54> for bool {
    #[inline(always)]
    fn from(variant: Lcds54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS54` reader - LCD pin 54 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds54R = crate::BitReader<Lcds54>;
impl Lcds54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds54 {
        match self.bits {
            false => Lcds54::SelPort,
            true => Lcds54::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds54::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds54::SelLcd
    }
}
#[doc = "Field `LCDS54` writer - LCD pin 54 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds54W<'a, REG> = crate::BitWriter<'a, REG, Lcds54>;
impl<'a, REG> Lcds54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds54::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds54::SelLcd)
    }
}
#[doc = "LCD pin 55 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds55 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds55> for bool {
    #[inline(always)]
    fn from(variant: Lcds55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS55` reader - LCD pin 55 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds55R = crate::BitReader<Lcds55>;
impl Lcds55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds55 {
        match self.bits {
            false => Lcds55::SelPort,
            true => Lcds55::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds55::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds55::SelLcd
    }
}
#[doc = "Field `LCDS55` writer - LCD pin 55 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds55W<'a, REG> = crate::BitWriter<'a, REG, Lcds55>;
impl<'a, REG> Lcds55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds55::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds55::SelLcd)
    }
}
#[doc = "LCD pin 56 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds56 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds56> for bool {
    #[inline(always)]
    fn from(variant: Lcds56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS56` reader - LCD pin 56 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds56R = crate::BitReader<Lcds56>;
impl Lcds56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds56 {
        match self.bits {
            false => Lcds56::SelPort,
            true => Lcds56::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds56::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds56::SelLcd
    }
}
#[doc = "Field `LCDS56` writer - LCD pin 56 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds56W<'a, REG> = crate::BitWriter<'a, REG, Lcds56>;
impl<'a, REG> Lcds56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds56::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds56::SelLcd)
    }
}
#[doc = "LCD pin 57 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds57 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds57> for bool {
    #[inline(always)]
    fn from(variant: Lcds57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS57` reader - LCD pin 57 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds57R = crate::BitReader<Lcds57>;
impl Lcds57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds57 {
        match self.bits {
            false => Lcds57::SelPort,
            true => Lcds57::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds57::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds57::SelLcd
    }
}
#[doc = "Field `LCDS57` writer - LCD pin 57 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds57W<'a, REG> = crate::BitWriter<'a, REG, Lcds57>;
impl<'a, REG> Lcds57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds57::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds57::SelLcd)
    }
}
#[doc = "LCD pin 58 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds58 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds58> for bool {
    #[inline(always)]
    fn from(variant: Lcds58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS58` reader - LCD pin 58 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds58R = crate::BitReader<Lcds58>;
impl Lcds58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds58 {
        match self.bits {
            false => Lcds58::SelPort,
            true => Lcds58::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds58::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds58::SelLcd
    }
}
#[doc = "Field `LCDS58` writer - LCD pin 58 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds58W<'a, REG> = crate::BitWriter<'a, REG, Lcds58>;
impl<'a, REG> Lcds58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds58::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds58::SelLcd)
    }
}
#[doc = "LCD pin 59 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds59 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds59> for bool {
    #[inline(always)]
    fn from(variant: Lcds59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS59` reader - LCD pin 59 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds59R = crate::BitReader<Lcds59>;
impl Lcds59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds59 {
        match self.bits {
            false => Lcds59::SelPort,
            true => Lcds59::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds59::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds59::SelLcd
    }
}
#[doc = "Field `LCDS59` writer - LCD pin 59 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds59W<'a, REG> = crate::BitWriter<'a, REG, Lcds59>;
impl<'a, REG> Lcds59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds59::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds59::SelLcd)
    }
}
#[doc = "LCD pin 60 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds60 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds60> for bool {
    #[inline(always)]
    fn from(variant: Lcds60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS60` reader - LCD pin 60 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds60R = crate::BitReader<Lcds60>;
impl Lcds60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds60 {
        match self.bits {
            false => Lcds60::SelPort,
            true => Lcds60::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds60::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds60::SelLcd
    }
}
#[doc = "Field `LCDS60` writer - LCD pin 60 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds60W<'a, REG> = crate::BitWriter<'a, REG, Lcds60>;
impl<'a, REG> Lcds60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds60::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds60::SelLcd)
    }
}
#[doc = "LCD pin 61 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds61 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds61> for bool {
    #[inline(always)]
    fn from(variant: Lcds61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS61` reader - LCD pin 61 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds61R = crate::BitReader<Lcds61>;
impl Lcds61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds61 {
        match self.bits {
            false => Lcds61::SelPort,
            true => Lcds61::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds61::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds61::SelLcd
    }
}
#[doc = "Field `LCDS61` writer - LCD pin 61 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds61W<'a, REG> = crate::BitWriter<'a, REG, Lcds61>;
impl<'a, REG> Lcds61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds61::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds61::SelLcd)
    }
}
#[doc = "LCD pin 62 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds62 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds62> for bool {
    #[inline(always)]
    fn from(variant: Lcds62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS62` reader - LCD pin 62 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds62R = crate::BitReader<Lcds62>;
impl Lcds62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds62 {
        match self.bits {
            false => Lcds62::SelPort,
            true => Lcds62::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds62::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds62::SelLcd
    }
}
#[doc = "Field `LCDS62` writer - LCD pin 62 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds62W<'a, REG> = crate::BitWriter<'a, REG, Lcds62>;
impl<'a, REG> Lcds62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds62::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds62::SelLcd)
    }
}
#[doc = "LCD pin 63 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds63 {
    #[doc = "0: Multiplexed pins are port functions."]
    SelPort = 0,
    #[doc = "1: Pins are LCD functions."]
    SelLcd = 1,
}
impl From<Lcds63> for bool {
    #[inline(always)]
    fn from(variant: Lcds63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS63` reader - LCD pin 63 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds63R = crate::BitReader<Lcds63>;
impl Lcds63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds63 {
        match self.bits {
            false => Lcds63::SelPort,
            true => Lcds63::SelLcd,
        }
    }
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn is_sel_port(&self) -> bool {
        *self == Lcds63::SelPort
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn is_sel_lcd(&self) -> bool {
        *self == Lcds63::SelLcd
    }
}
#[doc = "Field `LCDS63` writer - LCD pin 63 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
pub type Lcds63W<'a, REG> = crate::BitWriter<'a, REG, Lcds63>;
impl<'a, REG> Lcds63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions."]
    #[inline(always)]
    pub fn sel_port(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds63::SelPort)
    }
    #[doc = "Pins are LCD functions."]
    #[inline(always)]
    pub fn sel_lcd(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds63::SelLcd)
    }
}
impl R {
    #[doc = "Bit 0 - LCD pin 48 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds48(&self) -> Lcds48R {
        Lcds48R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD pin 49 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds49(&self) -> Lcds49R {
        Lcds49R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD pin 50 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds50(&self) -> Lcds50R {
        Lcds50R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD pin 51 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds51(&self) -> Lcds51R {
        Lcds51R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD pin 52 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds52(&self) -> Lcds52R {
        Lcds52R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD pin 53 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds53(&self) -> Lcds53R {
        Lcds53R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD pin 54 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds54(&self) -> Lcds54R {
        Lcds54R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD pin 55 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds55(&self) -> Lcds55R {
        Lcds55R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD pin 56 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds56(&self) -> Lcds56R {
        Lcds56R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD pin 57 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds57(&self) -> Lcds57R {
        Lcds57R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD pin 58 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds58(&self) -> Lcds58R {
        Lcds58R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD pin 59 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds59(&self) -> Lcds59R {
        Lcds59R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD pin 60 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds60(&self) -> Lcds60R {
        Lcds60R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD pin 61 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds61(&self) -> Lcds61R {
        Lcds61R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD pin 62 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds62(&self) -> Lcds62R {
        Lcds62R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD pin 63 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds63(&self) -> Lcds63R {
        Lcds63R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD pin 48 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds48(&mut self) -> Lcds48W<'_, LcdLcdpctl3Spec> {
        Lcds48W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD pin 49 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds49(&mut self) -> Lcds49W<'_, LcdLcdpctl3Spec> {
        Lcds49W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD pin 50 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds50(&mut self) -> Lcds50W<'_, LcdLcdpctl3Spec> {
        Lcds50W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD pin 51 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds51(&mut self) -> Lcds51W<'_, LcdLcdpctl3Spec> {
        Lcds51W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD pin 52 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds52(&mut self) -> Lcds52W<'_, LcdLcdpctl3Spec> {
        Lcds52W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD pin 53 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds53(&mut self) -> Lcds53W<'_, LcdLcdpctl3Spec> {
        Lcds53W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD pin 54 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds54(&mut self) -> Lcds54W<'_, LcdLcdpctl3Spec> {
        Lcds54W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD pin 55 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds55(&mut self) -> Lcds55W<'_, LcdLcdpctl3Spec> {
        Lcds55W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD pin 56 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds56(&mut self) -> Lcds56W<'_, LcdLcdpctl3Spec> {
        Lcds56W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD pin 57 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds57(&mut self) -> Lcds57W<'_, LcdLcdpctl3Spec> {
        Lcds57W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD pin 58 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds58(&mut self) -> Lcds58W<'_, LcdLcdpctl3Spec> {
        Lcds58W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD pin 59 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds59(&mut self) -> Lcds59W<'_, LcdLcdpctl3Spec> {
        Lcds59W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD pin 60 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds60(&mut self) -> Lcds60W<'_, LcdLcdpctl3Spec> {
        Lcds60W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD pin 61 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds61(&mut self) -> Lcds61W<'_, LcdLcdpctl3Spec> {
        Lcds61W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD pin 62 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds62(&mut self) -> Lcds62W<'_, LcdLcdpctl3Spec> {
        Lcds62W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD pin 63 enable. This bit affects only pins with multiplexed functions. Dedicated LCD pins are always LCD function. 0b = Multiplexed pins are port functions. 1b = Pins are LCD functions."]
    #[inline(always)]
    pub fn lcds63(&mut self) -> Lcds63W<'_, LcdLcdpctl3Spec> {
        Lcds63W::new(self, 15)
    }
}
#[doc = "LCD port control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdpctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdpctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdpctl3Spec;
impl crate::RegisterSpec for LcdLcdpctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdpctl3::R`](R) reader structure"]
impl crate::Readable for LcdLcdpctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdpctl3::W`](W) writer structure"]
impl crate::Writable for LcdLcdpctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDPCTL3 to value 0"]
impl crate::Resettable for LcdLcdpctl3Spec {}
