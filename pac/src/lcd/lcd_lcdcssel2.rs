#[doc = "Register `LCD_LCDCSSEL2` reader"]
pub type R = crate::R<LcdLcdcssel2Spec>;
#[doc = "Register `LCD_LCDCSSEL2` writer"]
pub type W = crate::W<LcdLcdcssel2Spec>;
#[doc = "Selects pin L32 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss32 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss32> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS32` reader - Selects pin L32 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss32R = crate::BitReader<Lcdcss32>;
impl Lcdcss32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss32 {
        match self.bits {
            false => Lcdcss32::SelSeg,
            true => Lcdcss32::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss32::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss32::SelCom
    }
}
#[doc = "Field `LCDCSS32` writer - Selects pin L32 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss32W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss32>;
impl<'a, REG> Lcdcss32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss32::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss32::SelCom)
    }
}
#[doc = "Selects pin L33 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss33 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss33> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS33` reader - Selects pin L33 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss33R = crate::BitReader<Lcdcss33>;
impl Lcdcss33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss33 {
        match self.bits {
            false => Lcdcss33::SelSeg,
            true => Lcdcss33::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss33::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss33::SelCom
    }
}
#[doc = "Field `LCDCSS33` writer - Selects pin L33 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss33W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss33>;
impl<'a, REG> Lcdcss33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss33::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss33::SelCom)
    }
}
#[doc = "Selects pin L34 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss34 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss34> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS34` reader - Selects pin L34 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss34R = crate::BitReader<Lcdcss34>;
impl Lcdcss34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss34 {
        match self.bits {
            false => Lcdcss34::SelSeg,
            true => Lcdcss34::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss34::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss34::SelCom
    }
}
#[doc = "Field `LCDCSS34` writer - Selects pin L34 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss34W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss34>;
impl<'a, REG> Lcdcss34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss34::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss34::SelCom)
    }
}
#[doc = "Selects pin L35 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss35 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss35> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS35` reader - Selects pin L35 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss35R = crate::BitReader<Lcdcss35>;
impl Lcdcss35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss35 {
        match self.bits {
            false => Lcdcss35::SelSeg,
            true => Lcdcss35::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss35::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss35::SelCom
    }
}
#[doc = "Field `LCDCSS35` writer - Selects pin L35 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss35W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss35>;
impl<'a, REG> Lcdcss35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss35::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss35::SelCom)
    }
}
#[doc = "Selects pin L36 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss36 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss36> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS36` reader - Selects pin L36 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss36R = crate::BitReader<Lcdcss36>;
impl Lcdcss36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss36 {
        match self.bits {
            false => Lcdcss36::SelSeg,
            true => Lcdcss36::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss36::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss36::SelCom
    }
}
#[doc = "Field `LCDCSS36` writer - Selects pin L36 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss36W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss36>;
impl<'a, REG> Lcdcss36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss36::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss36::SelCom)
    }
}
#[doc = "Selects pin L37 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss37 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss37> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS37` reader - Selects pin L37 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss37R = crate::BitReader<Lcdcss37>;
impl Lcdcss37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss37 {
        match self.bits {
            false => Lcdcss37::SelSeg,
            true => Lcdcss37::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss37::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss37::SelCom
    }
}
#[doc = "Field `LCDCSS37` writer - Selects pin L37 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss37W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss37>;
impl<'a, REG> Lcdcss37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss37::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss37::SelCom)
    }
}
#[doc = "Selects pin L38 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss38 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss38> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS38` reader - Selects pin L38 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss38R = crate::BitReader<Lcdcss38>;
impl Lcdcss38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss38 {
        match self.bits {
            false => Lcdcss38::SelSeg,
            true => Lcdcss38::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss38::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss38::SelCom
    }
}
#[doc = "Field `LCDCSS38` writer - Selects pin L38 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss38W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss38>;
impl<'a, REG> Lcdcss38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss38::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss38::SelCom)
    }
}
#[doc = "Selects pin L39 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss39 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss39> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS39` reader - Selects pin L39 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss39R = crate::BitReader<Lcdcss39>;
impl Lcdcss39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss39 {
        match self.bits {
            false => Lcdcss39::SelSeg,
            true => Lcdcss39::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss39::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss39::SelCom
    }
}
#[doc = "Field `LCDCSS39` writer - Selects pin L39 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss39W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss39>;
impl<'a, REG> Lcdcss39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss39::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss39::SelCom)
    }
}
#[doc = "Selects pin L40 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss40 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss40> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS40` reader - Selects pin L40 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss40R = crate::BitReader<Lcdcss40>;
impl Lcdcss40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss40 {
        match self.bits {
            false => Lcdcss40::SelSeg,
            true => Lcdcss40::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss40::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss40::SelCom
    }
}
#[doc = "Field `LCDCSS40` writer - Selects pin L40 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss40W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss40>;
impl<'a, REG> Lcdcss40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss40::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss40::SelCom)
    }
}
#[doc = "Selects pin L41 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss41 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss41> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS41` reader - Selects pin L41 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss41R = crate::BitReader<Lcdcss41>;
impl Lcdcss41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss41 {
        match self.bits {
            false => Lcdcss41::SelSeg,
            true => Lcdcss41::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss41::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss41::SelCom
    }
}
#[doc = "Field `LCDCSS41` writer - Selects pin L41 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss41W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss41>;
impl<'a, REG> Lcdcss41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss41::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss41::SelCom)
    }
}
#[doc = "Selects pin L42 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss42 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss42> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS42` reader - Selects pin L42 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss42R = crate::BitReader<Lcdcss42>;
impl Lcdcss42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss42 {
        match self.bits {
            false => Lcdcss42::SelSeg,
            true => Lcdcss42::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss42::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss42::SelCom
    }
}
#[doc = "Field `LCDCSS42` writer - Selects pin L42 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss42W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss42>;
impl<'a, REG> Lcdcss42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss42::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss42::SelCom)
    }
}
#[doc = "Selects pin L43 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss43 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss43> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS43` reader - Selects pin L43 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss43R = crate::BitReader<Lcdcss43>;
impl Lcdcss43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss43 {
        match self.bits {
            false => Lcdcss43::SelSeg,
            true => Lcdcss43::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss43::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss43::SelCom
    }
}
#[doc = "Field `LCDCSS43` writer - Selects pin L43 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss43W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss43>;
impl<'a, REG> Lcdcss43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss43::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss43::SelCom)
    }
}
#[doc = "Selects pin L44 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss44 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss44> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS44` reader - Selects pin L44 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss44R = crate::BitReader<Lcdcss44>;
impl Lcdcss44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss44 {
        match self.bits {
            false => Lcdcss44::SelSeg,
            true => Lcdcss44::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss44::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss44::SelCom
    }
}
#[doc = "Field `LCDCSS44` writer - Selects pin L44 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss44W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss44>;
impl<'a, REG> Lcdcss44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss44::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss44::SelCom)
    }
}
#[doc = "Selects pin L45 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss45 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss45> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS45` reader - Selects pin L45 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss45R = crate::BitReader<Lcdcss45>;
impl Lcdcss45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss45 {
        match self.bits {
            false => Lcdcss45::SelSeg,
            true => Lcdcss45::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss45::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss45::SelCom
    }
}
#[doc = "Field `LCDCSS45` writer - Selects pin L45 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss45W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss45>;
impl<'a, REG> Lcdcss45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss45::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss45::SelCom)
    }
}
#[doc = "Selects pin L46 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss46 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss46> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS46` reader - Selects pin L46 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss46R = crate::BitReader<Lcdcss46>;
impl Lcdcss46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss46 {
        match self.bits {
            false => Lcdcss46::SelSeg,
            true => Lcdcss46::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss46::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss46::SelCom
    }
}
#[doc = "Field `LCDCSS46` writer - Selects pin L46 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss46W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss46>;
impl<'a, REG> Lcdcss46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss46::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss46::SelCom)
    }
}
#[doc = "Selects pin L47 as either common or segment line. 0b = Segment line 1b = Common line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcss47 {
    #[doc = "0: Segment line"]
    SelSeg = 0,
    #[doc = "1: Common line"]
    SelCom = 1,
}
impl From<Lcdcss47> for bool {
    #[inline(always)]
    fn from(variant: Lcdcss47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCSS47` reader - Selects pin L47 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss47R = crate::BitReader<Lcdcss47>;
impl Lcdcss47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcss47 {
        match self.bits {
            false => Lcdcss47::SelSeg,
            true => Lcdcss47::SelCom,
        }
    }
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn is_sel_seg(&self) -> bool {
        *self == Lcdcss47::SelSeg
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn is_sel_com(&self) -> bool {
        *self == Lcdcss47::SelCom
    }
}
#[doc = "Field `LCDCSS47` writer - Selects pin L47 as either common or segment line. 0b = Segment line 1b = Common line"]
pub type Lcdcss47W<'a, REG> = crate::BitWriter<'a, REG, Lcdcss47>;
impl<'a, REG> Lcdcss47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn sel_seg(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss47::SelSeg)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn sel_com(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcss47::SelCom)
    }
}
impl R {
    #[doc = "Bit 0 - Selects pin L32 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss32(&self) -> Lcdcss32R {
        Lcdcss32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L33 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss33(&self) -> Lcdcss33R {
        Lcdcss33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L34 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss34(&self) -> Lcdcss34R {
        Lcdcss34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L35 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss35(&self) -> Lcdcss35R {
        Lcdcss35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L36 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss36(&self) -> Lcdcss36R {
        Lcdcss36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L37 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss37(&self) -> Lcdcss37R {
        Lcdcss37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L38 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss38(&self) -> Lcdcss38R {
        Lcdcss38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L39 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss39(&self) -> Lcdcss39R {
        Lcdcss39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L40 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss40(&self) -> Lcdcss40R {
        Lcdcss40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L41 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss41(&self) -> Lcdcss41R {
        Lcdcss41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L42 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss42(&self) -> Lcdcss42R {
        Lcdcss42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L43 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss43(&self) -> Lcdcss43R {
        Lcdcss43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L44 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss44(&self) -> Lcdcss44R {
        Lcdcss44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L45 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss45(&self) -> Lcdcss45R {
        Lcdcss45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L46 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss46(&self) -> Lcdcss46R {
        Lcdcss46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L47 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss47(&self) -> Lcdcss47R {
        Lcdcss47R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L32 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss32(&mut self) -> Lcdcss32W<'_, LcdLcdcssel2Spec> {
        Lcdcss32W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L33 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss33(&mut self) -> Lcdcss33W<'_, LcdLcdcssel2Spec> {
        Lcdcss33W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L34 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss34(&mut self) -> Lcdcss34W<'_, LcdLcdcssel2Spec> {
        Lcdcss34W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L35 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss35(&mut self) -> Lcdcss35W<'_, LcdLcdcssel2Spec> {
        Lcdcss35W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L36 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss36(&mut self) -> Lcdcss36W<'_, LcdLcdcssel2Spec> {
        Lcdcss36W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L37 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss37(&mut self) -> Lcdcss37W<'_, LcdLcdcssel2Spec> {
        Lcdcss37W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L38 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss38(&mut self) -> Lcdcss38W<'_, LcdLcdcssel2Spec> {
        Lcdcss38W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L39 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss39(&mut self) -> Lcdcss39W<'_, LcdLcdcssel2Spec> {
        Lcdcss39W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L40 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss40(&mut self) -> Lcdcss40W<'_, LcdLcdcssel2Spec> {
        Lcdcss40W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L41 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss41(&mut self) -> Lcdcss41W<'_, LcdLcdcssel2Spec> {
        Lcdcss41W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L42 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss42(&mut self) -> Lcdcss42W<'_, LcdLcdcssel2Spec> {
        Lcdcss42W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L43 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss43(&mut self) -> Lcdcss43W<'_, LcdLcdcssel2Spec> {
        Lcdcss43W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L44 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss44(&mut self) -> Lcdcss44W<'_, LcdLcdcssel2Spec> {
        Lcdcss44W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L45 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss45(&mut self) -> Lcdcss45W<'_, LcdLcdcssel2Spec> {
        Lcdcss45W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L46 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss46(&mut self) -> Lcdcss46W<'_, LcdLcdcssel2Spec> {
        Lcdcss46W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L47 as either common or segment line. 0b = Segment line 1b = Common line"]
    #[inline(always)]
    pub fn lcdcss47(&mut self) -> Lcdcss47W<'_, LcdLcdcssel2Spec> {
        Lcdcss47W::new(self, 15)
    }
}
#[doc = "LCD common segment select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdcssel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdcssel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdcssel2Spec;
impl crate::RegisterSpec for LcdLcdcssel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdcssel2::R`](R) reader structure"]
impl crate::Readable for LcdLcdcssel2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdcssel2::W`](W) writer structure"]
impl crate::Writable for LcdLcdcssel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDCSSEL2 to value 0"]
impl crate::Resettable for LcdLcdcssel2Spec {}
