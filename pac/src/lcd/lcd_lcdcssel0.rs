#[doc = "Register `LCD_LCDCSSEL0` reader"]
pub type R = crate::R<LcdLcdcssel0Spec>;
#[doc = "Register `LCD_LCDCSSEL0` writer"]
pub type W = crate::W<LcdLcdcssel0Spec>;
#[doc = "Selects pin L0 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss0 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss0> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS0` reader - Selects pin L0 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss0R = crate::BitReader<Lcdcss0>;
impl Lcdcss0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss0 {
        match self.bits {
            false => Lcdcss0::SelSeg,
            true => Lcdcss0::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss0::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss0::SelCom
    }
}
#[doc = "Field `LCDCSS0` writer - Selects pin L0 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss0W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss0>;
impl<'a, REG> Lcdcss0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss0::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss0::SelCom)
    }
}
#[doc = "Selects pin L1 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss1 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss1> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS1` reader - Selects pin L1 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss1R = crate::BitReader<Lcdcss1>;
impl Lcdcss1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss1 {
        match self.bits {
            false => Lcdcss1::SelSeg,
            true => Lcdcss1::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss1::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss1::SelCom
    }
}
#[doc = "Field `LCDCSS1` writer - Selects pin L1 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss1W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss1>;
impl<'a, REG> Lcdcss1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss1::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss1::SelCom)
    }
}
#[doc = "Selects pin L2 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss2 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss2> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS2` reader - Selects pin L2 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss2R = crate::BitReader<Lcdcss2>;
impl Lcdcss2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss2 {
        match self.bits {
            false => Lcdcss2::SelSeg,
            true => Lcdcss2::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss2::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss2::SelCom
    }
}
#[doc = "Field `LCDCSS2` writer - Selects pin L2 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss2W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss2>;
impl<'a, REG> Lcdcss2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss2::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss2::SelCom)
    }
}
#[doc = "Selects pin L3 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss3 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss3> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS3` reader - Selects pin L3 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss3R = crate::BitReader<Lcdcss3>;
impl Lcdcss3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss3 {
        match self.bits {
            false => Lcdcss3::SelSeg,
            true => Lcdcss3::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss3::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss3::SelCom
    }
}
#[doc = "Field `LCDCSS3` writer - Selects pin L3 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss3W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss3>;
impl<'a, REG> Lcdcss3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss3::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss3::SelCom)
    }
}
#[doc = "Selects pin L4 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss4 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss4> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS4` reader - Selects pin L4 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss4R = crate::BitReader<Lcdcss4>;
impl Lcdcss4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss4 {
        match self.bits {
            false => Lcdcss4::SelSeg,
            true => Lcdcss4::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss4::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss4::SelCom
    }
}
#[doc = "Field `LCDCSS4` writer - Selects pin L4 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss4W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss4>;
impl<'a, REG> Lcdcss4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss4::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss4::SelCom)
    }
}
#[doc = "Selects pin L5 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss5 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss5> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS5` reader - Selects pin L5 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss5R = crate::BitReader<Lcdcss5>;
impl Lcdcss5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss5 {
        match self.bits {
            false => Lcdcss5::SelSeg,
            true => Lcdcss5::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss5::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss5::SelCom
    }
}
#[doc = "Field `LCDCSS5` writer - Selects pin L5 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss5W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss5>;
impl<'a, REG> Lcdcss5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss5::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss5::SelCom)
    }
}
#[doc = "Selects pin L6 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss6 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss6> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS6` reader - Selects pin L6 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss6R = crate::BitReader<Lcdcss6>;
impl Lcdcss6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss6 {
        match self.bits {
            false => Lcdcss6::SelSeg,
            true => Lcdcss6::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss6::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss6::SelCom
    }
}
#[doc = "Field `LCDCSS6` writer - Selects pin L6 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss6W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss6>;
impl<'a, REG> Lcdcss6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss6::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss6::SelCom)
    }
}
#[doc = "Selects pin L7 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss7 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss7> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS7` reader - Selects pin L7 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss7R = crate::BitReader<Lcdcss7>;
impl Lcdcss7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss7 {
        match self.bits {
            false => Lcdcss7::SelSeg,
            true => Lcdcss7::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss7::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss7::SelCom
    }
}
#[doc = "Field `LCDCSS7` writer - Selects pin L7 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss7W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss7>;
impl<'a, REG> Lcdcss7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss7::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss7::SelCom)
    }
}
#[doc = "Selects pin L8 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss8 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss8> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS8` reader - Selects pin L8 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss8R = crate::BitReader<Lcdcss8>;
impl Lcdcss8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss8 {
        match self.bits {
            false => Lcdcss8::SelSeg,
            true => Lcdcss8::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss8::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss8::SelCom
    }
}
#[doc = "Field `LCDCSS8` writer - Selects pin L8 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss8W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss8>;
impl<'a, REG> Lcdcss8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss8::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss8::SelCom)
    }
}
#[doc = "Selects pin L9 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss9 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss9> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS9` reader - Selects pin L9 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss9R = crate::BitReader<Lcdcss9>;
impl Lcdcss9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss9 {
        match self.bits {
            false => Lcdcss9::SelSeg,
            true => Lcdcss9::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss9::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss9::SelCom
    }
}
#[doc = "Field `LCDCSS9` writer - Selects pin L9 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss9W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss9>;
impl<'a, REG> Lcdcss9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss9::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss9::SelCom)
    }
}
#[doc = "Selects pin L10 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss10 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss10> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS10` reader - Selects pin L10 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss10R = crate::BitReader<Lcdcss10>;
impl Lcdcss10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss10 {
        match self.bits {
            false => Lcdcss10::SelSeg,
            true => Lcdcss10::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss10::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss10::SelCom
    }
}
#[doc = "Field `LCDCSS10` writer - Selects pin L10 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss10W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss10>;
impl<'a, REG> Lcdcss10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss10::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss10::SelCom)
    }
}
#[doc = "Selects pin L11 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss11 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss11> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS11` reader - Selects pin L11 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss11R = crate::BitReader<Lcdcss11>;
impl Lcdcss11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss11 {
        match self.bits {
            false => Lcdcss11::SelSeg,
            true => Lcdcss11::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss11::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss11::SelCom
    }
}
#[doc = "Field `LCDCSS11` writer - Selects pin L11 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss11W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss11>;
impl<'a, REG> Lcdcss11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss11::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss11::SelCom)
    }
}
#[doc = "Selects pin L12 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss12 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss12> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS12` reader - Selects pin L12 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss12R = crate::BitReader<Lcdcss12>;
impl Lcdcss12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss12 {
        match self.bits {
            false => Lcdcss12::SelSeg,
            true => Lcdcss12::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss12::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss12::SelCom
    }
}
#[doc = "Field `LCDCSS12` writer - Selects pin L12 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss12W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss12>;
impl<'a, REG> Lcdcss12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss12::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss12::SelCom)
    }
}
#[doc = "Selects pin L13 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss13 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss13> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS13` reader - Selects pin L13 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss13R = crate::BitReader<Lcdcss13>;
impl Lcdcss13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss13 {
        match self.bits {
            false => Lcdcss13::SelSeg,
            true => Lcdcss13::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss13::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss13::SelCom
    }
}
#[doc = "Field `LCDCSS13` writer - Selects pin L13 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss13W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss13>;
impl<'a, REG> Lcdcss13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss13::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss13::SelCom)
    }
}
#[doc = "Selects pin L14 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss14 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss14> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS14` reader - Selects pin L14 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss14R = crate::BitReader<Lcdcss14>;
impl Lcdcss14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss14 {
        match self.bits {
            false => Lcdcss14::SelSeg,
            true => Lcdcss14::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss14::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss14::SelCom
    }
}
#[doc = "Field `LCDCSS14` writer - Selects pin L14 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss14W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss14>;
impl<'a, REG> Lcdcss14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss14::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss14::SelCom)
    }
}
#[doc = "Selects pin L15 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss15 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss15> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS15` reader - Selects pin L15 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss15R = crate::BitReader<Lcdcss15>;
impl Lcdcss15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss15 {
        match self.bits {
            false => Lcdcss15::SelSeg,
            true => Lcdcss15::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss15::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss15::SelCom
    }
}
#[doc = "Field `LCDCSS15` writer - Selects pin L15 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss15W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss15>;
impl<'a, REG> Lcdcss15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss15::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss15::SelCom)
    }
}
impl R {
    #[doc = "Bit 0 - Selects pin L0 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss0(&self) -> Lcdcss0R {
        Lcdcss0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L1 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss1(&self) -> Lcdcss1R {
        Lcdcss1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L2 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss2(&self) -> Lcdcss2R {
        Lcdcss2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L3 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss3(&self) -> Lcdcss3R {
        Lcdcss3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L4 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss4(&self) -> Lcdcss4R {
        Lcdcss4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L5 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss5(&self) -> Lcdcss5R {
        Lcdcss5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L6 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss6(&self) -> Lcdcss6R {
        Lcdcss6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L7 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss7(&self) -> Lcdcss7R {
        Lcdcss7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L8 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss8(&self) -> Lcdcss8R {
        Lcdcss8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L9 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss9(&self) -> Lcdcss9R {
        Lcdcss9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L10 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss10(&self) -> Lcdcss10R {
        Lcdcss10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L11 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss11(&self) -> Lcdcss11R {
        Lcdcss11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L12 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss12(&self) -> Lcdcss12R {
        Lcdcss12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L13 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss13(&self) -> Lcdcss13R {
        Lcdcss13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L14 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss14(&self) -> Lcdcss14R {
        Lcdcss14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L15 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss15(&self) -> Lcdcss15R {
        Lcdcss15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L0 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss0(&mut self) -> Lcdcss0W<'_, LcdLcdcssel0Spec> {
        Lcdcss0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L1 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss1(&mut self) -> Lcdcss1W<'_, LcdLcdcssel0Spec> {
        Lcdcss1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L2 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss2(&mut self) -> Lcdcss2W<'_, LcdLcdcssel0Spec> {
        Lcdcss2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L3 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss3(&mut self) -> Lcdcss3W<'_, LcdLcdcssel0Spec> {
        Lcdcss3W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L4 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss4(&mut self) -> Lcdcss4W<'_, LcdLcdcssel0Spec> {
        Lcdcss4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L5 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss5(&mut self) -> Lcdcss5W<'_, LcdLcdcssel0Spec> {
        Lcdcss5W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L6 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss6(&mut self) -> Lcdcss6W<'_, LcdLcdcssel0Spec> {
        Lcdcss6W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L7 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss7(&mut self) -> Lcdcss7W<'_, LcdLcdcssel0Spec> {
        Lcdcss7W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L8 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss8(&mut self) -> Lcdcss8W<'_, LcdLcdcssel0Spec> {
        Lcdcss8W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L9 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss9(&mut self) -> Lcdcss9W<'_, LcdLcdcssel0Spec> {
        Lcdcss9W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L10 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss10(&mut self) -> Lcdcss10W<'_, LcdLcdcssel0Spec> {
        Lcdcss10W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L11 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss11(&mut self) -> Lcdcss11W<'_, LcdLcdcssel0Spec> {
        Lcdcss11W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L12 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss12(&mut self) -> Lcdcss12W<'_, LcdLcdcssel0Spec> {
        Lcdcss12W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L13 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss13(&mut self) -> Lcdcss13W<'_, LcdLcdcssel0Spec> {
        Lcdcss13W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L14 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss14(&mut self) -> Lcdcss14W<'_, LcdLcdcssel0Spec> {
        Lcdcss14W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L15 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss15(&mut self) -> Lcdcss15W<'_, LcdLcdcssel0Spec> {
        Lcdcss15W::new(self, 15)
    }
}
#[doc = "LCD common segment select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdcssel0Spec;
impl crate::RegisterSpec for LcdLcdcssel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdcssel0::R`](R) reader structure"]
impl crate::Readable for LcdLcdcssel0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdcssel0::W`](W) writer structure"]
impl crate::Writable for LcdLcdcssel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDCSSEL0 to value 0"]
impl crate::Resettable for LcdLcdcssel0Spec {}
