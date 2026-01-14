#[doc = "Register `LCD_LCDCSSEL1` reader"]
pub type R = crate::R<LcdLcdcssel1Spec>;
#[doc = "Register `LCD_LCDCSSEL1` writer"]
pub type W = crate::W<LcdLcdcssel1Spec>;
#[doc = "Selects pin L16 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss16 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss16> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS16` reader - Selects pin L16 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss16R = crate::BitReader<Lcdcss16>;
impl Lcdcss16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss16 {
        match self.bits {
            false => Lcdcss16::SelSeg,
            true => Lcdcss16::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss16::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss16::SelCom
    }
}
#[doc = "Field `LCDCSS16` writer - Selects pin L16 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss16W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss16>;
impl<'a, REG> Lcdcss16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss16::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss16::SelCom)
    }
}
#[doc = "Selects pin L17 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss17 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss17> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS17` reader - Selects pin L17 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss17R = crate::BitReader<Lcdcss17>;
impl Lcdcss17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss17 {
        match self.bits {
            false => Lcdcss17::SelSeg,
            true => Lcdcss17::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss17::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss17::SelCom
    }
}
#[doc = "Field `LCDCSS17` writer - Selects pin L17 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss17W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss17>;
impl<'a, REG> Lcdcss17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss17::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss17::SelCom)
    }
}
#[doc = "Selects pin L18 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss18 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss18> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS18` reader - Selects pin L18 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss18R = crate::BitReader<Lcdcss18>;
impl Lcdcss18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss18 {
        match self.bits {
            false => Lcdcss18::SelSeg,
            true => Lcdcss18::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss18::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss18::SelCom
    }
}
#[doc = "Field `LCDCSS18` writer - Selects pin L18 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss18W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss18>;
impl<'a, REG> Lcdcss18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss18::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss18::SelCom)
    }
}
#[doc = "Selects pin L19 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss19 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss19> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS19` reader - Selects pin L19 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss19R = crate::BitReader<Lcdcss19>;
impl Lcdcss19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss19 {
        match self.bits {
            false => Lcdcss19::SelSeg,
            true => Lcdcss19::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss19::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss19::SelCom
    }
}
#[doc = "Field `LCDCSS19` writer - Selects pin L19 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss19W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss19>;
impl<'a, REG> Lcdcss19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss19::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss19::SelCom)
    }
}
#[doc = "Selects pin L20 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss20 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss20> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS20` reader - Selects pin L20 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss20R = crate::BitReader<Lcdcss20>;
impl Lcdcss20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss20 {
        match self.bits {
            false => Lcdcss20::SelSeg,
            true => Lcdcss20::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss20::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss20::SelCom
    }
}
#[doc = "Field `LCDCSS20` writer - Selects pin L20 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss20W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss20>;
impl<'a, REG> Lcdcss20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss20::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss20::SelCom)
    }
}
#[doc = "Selects pin L21 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss21 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss21> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS21` reader - Selects pin L21 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss21R = crate::BitReader<Lcdcss21>;
impl Lcdcss21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss21 {
        match self.bits {
            false => Lcdcss21::SelSeg,
            true => Lcdcss21::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss21::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss21::SelCom
    }
}
#[doc = "Field `LCDCSS21` writer - Selects pin L21 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss21W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss21>;
impl<'a, REG> Lcdcss21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss21::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss21::SelCom)
    }
}
#[doc = "Selects pin L22 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss22 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss22> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS22` reader - Selects pin L22 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss22R = crate::BitReader<Lcdcss22>;
impl Lcdcss22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss22 {
        match self.bits {
            false => Lcdcss22::SelSeg,
            true => Lcdcss22::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss22::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss22::SelCom
    }
}
#[doc = "Field `LCDCSS22` writer - Selects pin L22 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss22W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss22>;
impl<'a, REG> Lcdcss22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss22::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss22::SelCom)
    }
}
#[doc = "Selects pin L23 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss23 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss23> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS23` reader - Selects pin L23 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss23R = crate::BitReader<Lcdcss23>;
impl Lcdcss23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss23 {
        match self.bits {
            false => Lcdcss23::SelSeg,
            true => Lcdcss23::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss23::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss23::SelCom
    }
}
#[doc = "Field `LCDCSS23` writer - Selects pin L23 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss23W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss23>;
impl<'a, REG> Lcdcss23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss23::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss23::SelCom)
    }
}
#[doc = "Selects pin L24 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss24 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss24> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS24` reader - Selects pin L24 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss24R = crate::BitReader<Lcdcss24>;
impl Lcdcss24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss24 {
        match self.bits {
            false => Lcdcss24::SelSeg,
            true => Lcdcss24::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss24::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss24::SelCom
    }
}
#[doc = "Field `LCDCSS24` writer - Selects pin L24 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss24W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss24>;
impl<'a, REG> Lcdcss24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss24::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss24::SelCom)
    }
}
#[doc = "Selects pin L25 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss25 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss25> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS25` reader - Selects pin L25 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss25R = crate::BitReader<Lcdcss25>;
impl Lcdcss25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss25 {
        match self.bits {
            false => Lcdcss25::SelSeg,
            true => Lcdcss25::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss25::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss25::SelCom
    }
}
#[doc = "Field `LCDCSS25` writer - Selects pin L25 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss25W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss25>;
impl<'a, REG> Lcdcss25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss25::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss25::SelCom)
    }
}
#[doc = "Selects pin L26 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss26 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss26> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS26` reader - Selects pin L26 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss26R = crate::BitReader<Lcdcss26>;
impl Lcdcss26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss26 {
        match self.bits {
            false => Lcdcss26::SelSeg,
            true => Lcdcss26::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss26::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss26::SelCom
    }
}
#[doc = "Field `LCDCSS26` writer - Selects pin L26 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss26W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss26>;
impl<'a, REG> Lcdcss26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss26::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss26::SelCom)
    }
}
#[doc = "Selects pin L27 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss27 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss27> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS27` reader - Selects pin L27 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss27R = crate::BitReader<Lcdcss27>;
impl Lcdcss27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss27 {
        match self.bits {
            false => Lcdcss27::SelSeg,
            true => Lcdcss27::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss27::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss27::SelCom
    }
}
#[doc = "Field `LCDCSS27` writer - Selects pin L27 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss27W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss27>;
impl<'a, REG> Lcdcss27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss27::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss27::SelCom)
    }
}
#[doc = "Selects pin L28 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss28 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss28> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS28` reader - Selects pin L28 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss28R = crate::BitReader<Lcdcss28>;
impl Lcdcss28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss28 {
        match self.bits {
            false => Lcdcss28::SelSeg,
            true => Lcdcss28::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss28::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss28::SelCom
    }
}
#[doc = "Field `LCDCSS28` writer - Selects pin L28 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss28W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss28>;
impl<'a, REG> Lcdcss28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss28::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss28::SelCom)
    }
}
#[doc = "Selects pin L29 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss29 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss29> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS29` reader - Selects pin L29 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss29R = crate::BitReader<Lcdcss29>;
impl Lcdcss29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss29 {
        match self.bits {
            false => Lcdcss29::SelSeg,
            true => Lcdcss29::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss29::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss29::SelCom
    }
}
#[doc = "Field `LCDCSS29` writer - Selects pin L29 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss29W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss29>;
impl<'a, REG> Lcdcss29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss29::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss29::SelCom)
    }
}
#[doc = "Selects pin L30 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss30 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss30> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS30` reader - Selects pin L30 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss30R = crate::BitReader<Lcdcss30>;
impl Lcdcss30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss30 {
        match self.bits {
            false => Lcdcss30::SelSeg,
            true => Lcdcss30::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss30::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss30::SelCom
    }
}
#[doc = "Field `LCDCSS30` writer - Selects pin L30 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss30W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss30>;
impl<'a, REG> Lcdcss30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss30::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss30::SelCom)
    }
}
#[doc = "Selects pin L31 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss31 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss31> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS31` reader - Selects pin L31 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss31R = crate::BitReader<Lcdcss31>;
impl Lcdcss31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss31 {
        match self.bits {
            false => Lcdcss31::SelSeg,
            true => Lcdcss31::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss31::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss31::SelCom
    }
}
#[doc = "Field `LCDCSS31` writer - Selects pin L31 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss31W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss31>;
impl<'a, REG> Lcdcss31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss31::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss31::SelCom)
    }
}
impl R {
    #[doc = "Bit 0 - Selects pin L16 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss16(&self) -> Lcdcss16R {
        Lcdcss16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L17 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss17(&self) -> Lcdcss17R {
        Lcdcss17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L18 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss18(&self) -> Lcdcss18R {
        Lcdcss18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L19 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss19(&self) -> Lcdcss19R {
        Lcdcss19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L20 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss20(&self) -> Lcdcss20R {
        Lcdcss20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L21 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss21(&self) -> Lcdcss21R {
        Lcdcss21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L22 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss22(&self) -> Lcdcss22R {
        Lcdcss22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L23 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss23(&self) -> Lcdcss23R {
        Lcdcss23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L24 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss24(&self) -> Lcdcss24R {
        Lcdcss24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L25 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss25(&self) -> Lcdcss25R {
        Lcdcss25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L26 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss26(&self) -> Lcdcss26R {
        Lcdcss26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L27 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss27(&self) -> Lcdcss27R {
        Lcdcss27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L28 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss28(&self) -> Lcdcss28R {
        Lcdcss28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L29 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss29(&self) -> Lcdcss29R {
        Lcdcss29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L30 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss30(&self) -> Lcdcss30R {
        Lcdcss30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L31 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss31(&self) -> Lcdcss31R {
        Lcdcss31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L16 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss16(&mut self) -> Lcdcss16W<'_, LcdLcdcssel1Spec> {
        Lcdcss16W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L17 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss17(&mut self) -> Lcdcss17W<'_, LcdLcdcssel1Spec> {
        Lcdcss17W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L18 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss18(&mut self) -> Lcdcss18W<'_, LcdLcdcssel1Spec> {
        Lcdcss18W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L19 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss19(&mut self) -> Lcdcss19W<'_, LcdLcdcssel1Spec> {
        Lcdcss19W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L20 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss20(&mut self) -> Lcdcss20W<'_, LcdLcdcssel1Spec> {
        Lcdcss20W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L21 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss21(&mut self) -> Lcdcss21W<'_, LcdLcdcssel1Spec> {
        Lcdcss21W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L22 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss22(&mut self) -> Lcdcss22W<'_, LcdLcdcssel1Spec> {
        Lcdcss22W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L23 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss23(&mut self) -> Lcdcss23W<'_, LcdLcdcssel1Spec> {
        Lcdcss23W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L24 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss24(&mut self) -> Lcdcss24W<'_, LcdLcdcssel1Spec> {
        Lcdcss24W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L25 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss25(&mut self) -> Lcdcss25W<'_, LcdLcdcssel1Spec> {
        Lcdcss25W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L26 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss26(&mut self) -> Lcdcss26W<'_, LcdLcdcssel1Spec> {
        Lcdcss26W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L27 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss27(&mut self) -> Lcdcss27W<'_, LcdLcdcssel1Spec> {
        Lcdcss27W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L28 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss28(&mut self) -> Lcdcss28W<'_, LcdLcdcssel1Spec> {
        Lcdcss28W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L29 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss29(&mut self) -> Lcdcss29W<'_, LcdLcdcssel1Spec> {
        Lcdcss29W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L30 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss30(&mut self) -> Lcdcss30W<'_, LcdLcdcssel1Spec> {
        Lcdcss30W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L31 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss31(&mut self) -> Lcdcss31W<'_, LcdLcdcssel1Spec> {
        Lcdcss31W::new(self, 15)
    }
}
#[doc = "LCD common segment select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdcssel1Spec;
impl crate::RegisterSpec for LcdLcdcssel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdcssel1::R`](R) reader structure"]
impl crate::Readable for LcdLcdcssel1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdcssel1::W`](W) writer structure"]
impl crate::Writable for LcdLcdcssel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDCSSEL1 to value 0"]
impl crate::Resettable for LcdLcdcssel1Spec {}
