#[doc = "Register `LCD_LCDCSSEL3` reader"]
pub type R = crate::R<LcdLcdcssel3Spec>;
#[doc = "Register `LCD_LCDCSSEL3` writer"]
pub type W = crate::W<LcdLcdcssel3Spec>;
#[doc = "Selects pin L48 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss48 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss48> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS48` reader - Selects pin L48 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss48R = crate::BitReader<Lcdcss48>;
impl Lcdcss48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss48 {
        match self.bits {
            false => Lcdcss48::SelSeg,
            true => Lcdcss48::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss48::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss48::SelCom
    }
}
#[doc = "Field `LCDCSS48` writer - Selects pin L48 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss48W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss48>;
impl<'a, REG> Lcdcss48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss48::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss48::SelCom)
    }
}
#[doc = "Selects pin L49 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss49 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss49> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS49` reader - Selects pin L49 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss49R = crate::BitReader<Lcdcss49>;
impl Lcdcss49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss49 {
        match self.bits {
            false => Lcdcss49::SelSeg,
            true => Lcdcss49::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss49::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss49::SelCom
    }
}
#[doc = "Field `LCDCSS49` writer - Selects pin L49 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss49W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss49>;
impl<'a, REG> Lcdcss49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss49::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss49::SelCom)
    }
}
#[doc = "Selects pin L50 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss50 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss50> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS50` reader - Selects pin L50 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss50R = crate::BitReader<Lcdcss50>;
impl Lcdcss50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss50 {
        match self.bits {
            false => Lcdcss50::SelSeg,
            true => Lcdcss50::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss50::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss50::SelCom
    }
}
#[doc = "Field `LCDCSS50` writer - Selects pin L50 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss50W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss50>;
impl<'a, REG> Lcdcss50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss50::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss50::SelCom)
    }
}
#[doc = "Selects pin L51 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss51 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss51> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS51` reader - Selects pin L51 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss51R = crate::BitReader<Lcdcss51>;
impl Lcdcss51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss51 {
        match self.bits {
            false => Lcdcss51::SelSeg,
            true => Lcdcss51::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss51::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss51::SelCom
    }
}
#[doc = "Field `LCDCSS51` writer - Selects pin L51 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss51W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss51>;
impl<'a, REG> Lcdcss51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss51::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss51::SelCom)
    }
}
#[doc = "Selects pin L52 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss52 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss52> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS52` reader - Selects pin L52 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss52R = crate::BitReader<Lcdcss52>;
impl Lcdcss52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss52 {
        match self.bits {
            false => Lcdcss52::SelSeg,
            true => Lcdcss52::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss52::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss52::SelCom
    }
}
#[doc = "Field `LCDCSS52` writer - Selects pin L52 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss52W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss52>;
impl<'a, REG> Lcdcss52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss52::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss52::SelCom)
    }
}
#[doc = "Selects pin L53 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss53 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss53> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS53` reader - Selects pin L53 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss53R = crate::BitReader<Lcdcss53>;
impl Lcdcss53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss53 {
        match self.bits {
            false => Lcdcss53::SelSeg,
            true => Lcdcss53::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss53::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss53::SelCom
    }
}
#[doc = "Field `LCDCSS53` writer - Selects pin L53 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss53W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss53>;
impl<'a, REG> Lcdcss53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss53::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss53::SelCom)
    }
}
#[doc = "Selects pin L54 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss54 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss54> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS54` reader - Selects pin L54 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss54R = crate::BitReader<Lcdcss54>;
impl Lcdcss54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss54 {
        match self.bits {
            false => Lcdcss54::SelSeg,
            true => Lcdcss54::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss54::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss54::SelCom
    }
}
#[doc = "Field `LCDCSS54` writer - Selects pin L54 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss54W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss54>;
impl<'a, REG> Lcdcss54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss54::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss54::SelCom)
    }
}
#[doc = "Selects pin L55 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss55 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss55> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS55` reader - Selects pin L55 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss55R = crate::BitReader<Lcdcss55>;
impl Lcdcss55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss55 {
        match self.bits {
            false => Lcdcss55::SelSeg,
            true => Lcdcss55::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss55::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss55::SelCom
    }
}
#[doc = "Field `LCDCSS55` writer - Selects pin L55 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss55W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss55>;
impl<'a, REG> Lcdcss55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss55::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss55::SelCom)
    }
}
#[doc = "Selects pin L56 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss56 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss56> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS56` reader - Selects pin L56 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss56R = crate::BitReader<Lcdcss56>;
impl Lcdcss56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss56 {
        match self.bits {
            false => Lcdcss56::SelSeg,
            true => Lcdcss56::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss56::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss56::SelCom
    }
}
#[doc = "Field `LCDCSS56` writer - Selects pin L56 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss56W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss56>;
impl<'a, REG> Lcdcss56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss56::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss56::SelCom)
    }
}
#[doc = "Selects pin L57 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss57 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss57> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS57` reader - Selects pin L57 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss57R = crate::BitReader<Lcdcss57>;
impl Lcdcss57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss57 {
        match self.bits {
            false => Lcdcss57::SelSeg,
            true => Lcdcss57::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss57::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss57::SelCom
    }
}
#[doc = "Field `LCDCSS57` writer - Selects pin L57 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss57W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss57>;
impl<'a, REG> Lcdcss57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss57::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss57::SelCom)
    }
}
#[doc = "Selects pin L58 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss58 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss58> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS58` reader - Selects pin L58 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss58R = crate::BitReader<Lcdcss58>;
impl Lcdcss58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss58 {
        match self.bits {
            false => Lcdcss58::SelSeg,
            true => Lcdcss58::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss58::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss58::SelCom
    }
}
#[doc = "Field `LCDCSS58` writer - Selects pin L58 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss58W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss58>;
impl<'a, REG> Lcdcss58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss58::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss58::SelCom)
    }
}
#[doc = "Selects pin L59 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss59 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss59> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS59` reader - Selects pin L59 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss59R = crate::BitReader<Lcdcss59>;
impl Lcdcss59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss59 {
        match self.bits {
            false => Lcdcss59::SelSeg,
            true => Lcdcss59::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss59::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss59::SelCom
    }
}
#[doc = "Field `LCDCSS59` writer - Selects pin L59 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss59W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss59>;
impl<'a, REG> Lcdcss59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss59::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss59::SelCom)
    }
}
#[doc = "Selects pin L60 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss60 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss60> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS60` reader - Selects pin L60 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss60R = crate::BitReader<Lcdcss60>;
impl Lcdcss60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss60 {
        match self.bits {
            false => Lcdcss60::SelSeg,
            true => Lcdcss60::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss60::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss60::SelCom
    }
}
#[doc = "Field `LCDCSS60` writer - Selects pin L60 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss60W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss60>;
impl<'a, REG> Lcdcss60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss60::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss60::SelCom)
    }
}
#[doc = "Selects pin L61 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss61 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss61> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS61` reader - Selects pin L61 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss61R = crate::BitReader<Lcdcss61>;
impl Lcdcss61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss61 {
        match self.bits {
            false => Lcdcss61::SelSeg,
            true => Lcdcss61::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss61::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss61::SelCom
    }
}
#[doc = "Field `LCDCSS61` writer - Selects pin L61 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss61W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss61>;
impl<'a, REG> Lcdcss61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss61::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss61::SelCom)
    }
}
#[doc = "Selects pin L62 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss62 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss62> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS62` reader - Selects pin L62 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss62R = crate::BitReader<Lcdcss62>;
impl Lcdcss62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss62 {
        match self.bits {
            false => Lcdcss62::SelSeg,
            true => Lcdcss62::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss62::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss62::SelCom
    }
}
#[doc = "Field `LCDCSS62` writer - Selects pin L62 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss62W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss62>;
impl<'a, REG> Lcdcss62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss62::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss62::SelCom)
    }
}
#[doc = "Selects pin L63 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss63 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss63> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS63` reader - Selects pin L63 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss63R = crate::BitReader<Lcdcss63>;
impl Lcdcss63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss63 {
        match self.bits {
            false => Lcdcss63::SelSeg,
            true => Lcdcss63::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss63::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss63::SelCom
    }
}
#[doc = "Field `LCDCSS63` writer - Selects pin L63 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss63W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss63>;
impl<'a, REG> Lcdcss63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss63::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss63::SelCom)
    }
}
impl R {
    #[doc = "Bit 0 - Selects pin L48 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss48(&self) -> Lcdcss48R {
        Lcdcss48R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L49 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss49(&self) -> Lcdcss49R {
        Lcdcss49R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L50 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss50(&self) -> Lcdcss50R {
        Lcdcss50R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L51 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss51(&self) -> Lcdcss51R {
        Lcdcss51R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L52 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss52(&self) -> Lcdcss52R {
        Lcdcss52R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L53 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss53(&self) -> Lcdcss53R {
        Lcdcss53R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L54 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss54(&self) -> Lcdcss54R {
        Lcdcss54R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L55 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss55(&self) -> Lcdcss55R {
        Lcdcss55R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L56 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss56(&self) -> Lcdcss56R {
        Lcdcss56R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L57 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss57(&self) -> Lcdcss57R {
        Lcdcss57R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L58 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss58(&self) -> Lcdcss58R {
        Lcdcss58R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L59 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss59(&self) -> Lcdcss59R {
        Lcdcss59R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L60 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss60(&self) -> Lcdcss60R {
        Lcdcss60R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L61 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss61(&self) -> Lcdcss61R {
        Lcdcss61R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L62 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss62(&self) -> Lcdcss62R {
        Lcdcss62R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L63 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss63(&self) -> Lcdcss63R {
        Lcdcss63R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L48 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss48(&mut self) -> Lcdcss48W<'_, LcdLcdcssel3Spec> {
        Lcdcss48W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L49 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss49(&mut self) -> Lcdcss49W<'_, LcdLcdcssel3Spec> {
        Lcdcss49W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L50 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss50(&mut self) -> Lcdcss50W<'_, LcdLcdcssel3Spec> {
        Lcdcss50W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L51 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss51(&mut self) -> Lcdcss51W<'_, LcdLcdcssel3Spec> {
        Lcdcss51W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L52 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss52(&mut self) -> Lcdcss52W<'_, LcdLcdcssel3Spec> {
        Lcdcss52W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L53 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss53(&mut self) -> Lcdcss53W<'_, LcdLcdcssel3Spec> {
        Lcdcss53W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L54 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss54(&mut self) -> Lcdcss54W<'_, LcdLcdcssel3Spec> {
        Lcdcss54W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L55 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss55(&mut self) -> Lcdcss55W<'_, LcdLcdcssel3Spec> {
        Lcdcss55W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L56 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss56(&mut self) -> Lcdcss56W<'_, LcdLcdcssel3Spec> {
        Lcdcss56W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L57 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss57(&mut self) -> Lcdcss57W<'_, LcdLcdcssel3Spec> {
        Lcdcss57W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L58 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss58(&mut self) -> Lcdcss58W<'_, LcdLcdcssel3Spec> {
        Lcdcss58W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L59 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss59(&mut self) -> Lcdcss59W<'_, LcdLcdcssel3Spec> {
        Lcdcss59W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L60 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss60(&mut self) -> Lcdcss60W<'_, LcdLcdcssel3Spec> {
        Lcdcss60W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L61 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss61(&mut self) -> Lcdcss61W<'_, LcdLcdcssel3Spec> {
        Lcdcss61W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L62 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss62(&mut self) -> Lcdcss62W<'_, LcdLcdcssel3Spec> {
        Lcdcss62W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L63 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss63(&mut self) -> Lcdcss63W<'_, LcdLcdcssel3Spec> {
        Lcdcss63W::new(self, 15)
    }
}
#[doc = "LCD common segment select register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdcssel3Spec;
impl crate::RegisterSpec for LcdLcdcssel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdcssel3::R`](R) reader structure"]
impl crate::Readable for LcdLcdcssel3Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdcssel3::W`](W) writer structure"]
impl crate::Writable for LcdLcdcssel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDCSSEL3 to value 0"]
impl crate::Resettable for LcdLcdcssel3Spec {}
