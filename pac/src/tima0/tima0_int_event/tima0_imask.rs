#[doc = "Register `TIMA0_IMASK` reader"]
pub type R = crate::R<Tima0ImaskSpec>;
#[doc = "Register `TIMA0_IMASK` writer"]
pub type W = crate::W<Tima0ImaskSpec>;
#[doc = "Zero Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Z {
    #[doc = "0: Disable Event"]
    Clr = 0,
    #[doc = "1: Enable Event"]
    Set = 1,
}
impl From<Z> for bool {
    #[inline(always)]
    fn from(variant: Z) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Z` reader - Zero Event mask"]
pub type ZR = crate::BitReader<Z>;
impl ZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Z {
        match self.bits {
            false => Z::Clr,
            true => Z::Set,
        }
    }
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Z::Clr
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Z::Set
    }
}
#[doc = "Field `Z` writer - Zero Event mask"]
pub type ZW<'a, REG> = crate::BitWriter<'a, REG, Z>;
impl<'a, REG> ZW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Z::Clr)
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Z::Set)
    }
}
#[doc = "Load Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<L> for bool {
    #[inline(always)]
    fn from(variant: L) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` reader - Load Event mask"]
pub type LR = crate::BitReader<L>;
impl LR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L {
        match self.bits {
            false => L::Clr,
            true => L::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == L::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == L::Set
    }
}
#[doc = "Field `L` writer - Load Event mask"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG, L>;
impl<'a, REG> LW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(L::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(L::Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd0> for bool {
    #[inline(always)]
    fn from(variant: Ccd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD0` reader - Capture or Compare DN event mask CCP0"]
pub type Ccd0R = crate::BitReader<Ccd0>;
impl Ccd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd0 {
        match self.bits {
            false => Ccd0::Clr,
            true => Ccd0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd0::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd0::Set
    }
}
#[doc = "Field `CCD0` writer - Capture or Compare DN event mask CCP0"]
pub type Ccd0W<'a, REG> = crate::BitWriter<'a, REG, Ccd0>;
impl<'a, REG> Ccd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd0::Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd1> for bool {
    #[inline(always)]
    fn from(variant: Ccd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD1` reader - Capture or Compare DN event mask CCP1"]
pub type Ccd1R = crate::BitReader<Ccd1>;
impl Ccd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd1 {
        match self.bits {
            false => Ccd1::Clr,
            true => Ccd1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd1::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd1::Set
    }
}
#[doc = "Field `CCD1` writer - Capture or Compare DN event mask CCP1"]
pub type Ccd1W<'a, REG> = crate::BitWriter<'a, REG, Ccd1>;
impl<'a, REG> Ccd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd1::Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd2 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd2> for bool {
    #[inline(always)]
    fn from(variant: Ccd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD2` reader - Capture or Compare DN event mask CCP2"]
pub type Ccd2R = crate::BitReader<Ccd2>;
impl Ccd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd2 {
        match self.bits {
            false => Ccd2::Clr,
            true => Ccd2::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd2::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd2::Set
    }
}
#[doc = "Field `CCD2` writer - Capture or Compare DN event mask CCP2"]
pub type Ccd2W<'a, REG> = crate::BitWriter<'a, REG, Ccd2>;
impl<'a, REG> Ccd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd2::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd2::Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd3 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd3> for bool {
    #[inline(always)]
    fn from(variant: Ccd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD3` reader - Capture or Compare DN event mask CCP3"]
pub type Ccd3R = crate::BitReader<Ccd3>;
impl Ccd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd3 {
        match self.bits {
            false => Ccd3::Clr,
            true => Ccd3::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd3::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd3::Set
    }
}
#[doc = "Field `CCD3` writer - Capture or Compare DN event mask CCP3"]
pub type Ccd3W<'a, REG> = crate::BitWriter<'a, REG, Ccd3>;
impl<'a, REG> Ccd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd3::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd3::Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu0> for bool {
    #[inline(always)]
    fn from(variant: Ccu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU0` reader - Capture or Compare UP event mask CCP0"]
pub type Ccu0R = crate::BitReader<Ccu0>;
impl Ccu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu0 {
        match self.bits {
            false => Ccu0::Clr,
            true => Ccu0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu0::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu0::Set
    }
}
#[doc = "Field `CCU0` writer - Capture or Compare UP event mask CCP0"]
pub type Ccu0W<'a, REG> = crate::BitWriter<'a, REG, Ccu0>;
impl<'a, REG> Ccu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu0::Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu1> for bool {
    #[inline(always)]
    fn from(variant: Ccu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU1` reader - Capture or Compare UP event mask CCP1"]
pub type Ccu1R = crate::BitReader<Ccu1>;
impl Ccu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu1 {
        match self.bits {
            false => Ccu1::Clr,
            true => Ccu1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu1::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu1::Set
    }
}
#[doc = "Field `CCU1` writer - Capture or Compare UP event mask CCP1"]
pub type Ccu1W<'a, REG> = crate::BitWriter<'a, REG, Ccu1>;
impl<'a, REG> Ccu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu1::Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu2 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu2> for bool {
    #[inline(always)]
    fn from(variant: Ccu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU2` reader - Capture or Compare UP event mask CCP2"]
pub type Ccu2R = crate::BitReader<Ccu2>;
impl Ccu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu2 {
        match self.bits {
            false => Ccu2::Clr,
            true => Ccu2::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu2::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu2::Set
    }
}
#[doc = "Field `CCU2` writer - Capture or Compare UP event mask CCP2"]
pub type Ccu2W<'a, REG> = crate::BitWriter<'a, REG, Ccu2>;
impl<'a, REG> Ccu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu2::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu2::Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu3 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu3> for bool {
    #[inline(always)]
    fn from(variant: Ccu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU3` reader - Capture or Compare UP event mask CCP3"]
pub type Ccu3R = crate::BitReader<Ccu3>;
impl Ccu3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu3 {
        match self.bits {
            false => Ccu3::Clr,
            true => Ccu3::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu3::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu3::Set
    }
}
#[doc = "Field `CCU3` writer - Capture or Compare UP event mask CCP3"]
pub type Ccu3W<'a, REG> = crate::BitWriter<'a, REG, Ccu3>;
impl<'a, REG> Ccu3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu3::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu3::Set)
    }
}
#[doc = "Compare DN event mask CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd4 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd4> for bool {
    #[inline(always)]
    fn from(variant: Ccd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD4` reader - Compare DN event mask CCP4"]
pub type Ccd4R = crate::BitReader<Ccd4>;
impl Ccd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd4 {
        match self.bits {
            false => Ccd4::Clr,
            true => Ccd4::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd4::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd4::Set
    }
}
#[doc = "Field `CCD4` writer - Compare DN event mask CCP4"]
pub type Ccd4W<'a, REG> = crate::BitWriter<'a, REG, Ccd4>;
impl<'a, REG> Ccd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd4::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd4::Set)
    }
}
#[doc = "Compare DN event mask CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccd5 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccd5> for bool {
    #[inline(always)]
    fn from(variant: Ccd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCD5` reader - Compare DN event mask CCP5"]
pub type Ccd5R = crate::BitReader<Ccd5>;
impl Ccd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccd5 {
        match self.bits {
            false => Ccd5::Clr,
            true => Ccd5::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccd5::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccd5::Set
    }
}
#[doc = "Field `CCD5` writer - Compare DN event mask CCP5"]
pub type Ccd5W<'a, REG> = crate::BitWriter<'a, REG, Ccd5>;
impl<'a, REG> Ccd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd5::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccd5::Set)
    }
}
#[doc = "Compare UP event mask CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu4 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu4> for bool {
    #[inline(always)]
    fn from(variant: Ccu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU4` reader - Compare UP event mask CCP4"]
pub type Ccu4R = crate::BitReader<Ccu4>;
impl Ccu4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu4 {
        match self.bits {
            false => Ccu4::Clr,
            true => Ccu4::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu4::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu4::Set
    }
}
#[doc = "Field `CCU4` writer - Compare UP event mask CCP4"]
pub type Ccu4W<'a, REG> = crate::BitWriter<'a, REG, Ccu4>;
impl<'a, REG> Ccu4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu4::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu4::Set)
    }
}
#[doc = "Compare UP event mask CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu5 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ccu5> for bool {
    #[inline(always)]
    fn from(variant: Ccu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU5` reader - Compare UP event mask CCP5"]
pub type Ccu5R = crate::BitReader<Ccu5>;
impl Ccu5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu5 {
        match self.bits {
            false => Ccu5::Clr,
            true => Ccu5::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ccu5::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ccu5::Set
    }
}
#[doc = "Field `CCU5` writer - Compare UP event mask CCP5"]
pub type Ccu5W<'a, REG> = crate::BitWriter<'a, REG, Ccu5>;
impl<'a, REG> Ccu5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu5::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu5::Set)
    }
}
#[doc = "Fault Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F {
    #[doc = "0: Disable Event"]
    Clr = 0,
    #[doc = "1: Enable Event"]
    Set = 1,
}
impl From<F> for bool {
    #[inline(always)]
    fn from(variant: F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F` reader - Fault Event mask"]
pub type FR = crate::BitReader<F>;
impl FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F {
        match self.bits {
            false => F::Clr,
            true => F::Set,
        }
    }
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == F::Clr
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == F::Set
    }
}
#[doc = "Field `F` writer - Fault Event mask"]
pub type FW<'a, REG> = crate::BitWriter<'a, REG, F>;
impl<'a, REG> FW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(F::Clr)
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(F::Set)
    }
}
#[doc = "Trigger Overflow Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tov {
    #[doc = "0: Disable Event"]
    Clr = 0,
    #[doc = "1: Enable Event"]
    Set = 1,
}
impl From<Tov> for bool {
    #[inline(always)]
    fn from(variant: Tov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOV` reader - Trigger Overflow Event mask"]
pub type TovR = crate::BitReader<Tov>;
impl TovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tov {
        match self.bits {
            false => Tov::Clr,
            true => Tov::Set,
        }
    }
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tov::Clr
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tov::Set
    }
}
#[doc = "Field `TOV` writer - Trigger Overflow Event mask"]
pub type TovW<'a, REG> = crate::BitWriter<'a, REG, Tov>;
impl<'a, REG> TovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::Clr)
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tov::Set)
    }
}
#[doc = "Repeat Counter Zero Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repc {
    #[doc = "0: Disable Event"]
    Clr = 0,
    #[doc = "1: Enable Event"]
    Set = 1,
}
impl From<Repc> for bool {
    #[inline(always)]
    fn from(variant: Repc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPC` reader - Repeat Counter Zero Event mask"]
pub type RepcR = crate::BitReader<Repc>;
impl RepcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Repc {
        match self.bits {
            false => Repc::Clr,
            true => Repc::Set,
        }
    }
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Repc::Clr
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Repc::Set
    }
}
#[doc = "Field `REPC` writer - Repeat Counter Zero Event mask"]
pub type RepcW<'a, REG> = crate::BitWriter<'a, REG, Repc>;
impl<'a, REG> RepcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Repc::Clr)
    }
    #[doc = "Enable Event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Repc::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or Compare DN event mask CCP0"]
    #[inline(always)]
    pub fn ccd0(&self) -> Ccd0R {
        Ccd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or Compare DN event mask CCP1"]
    #[inline(always)]
    pub fn ccd1(&self) -> Ccd1R {
        Ccd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture or Compare DN event mask CCP2"]
    #[inline(always)]
    pub fn ccd2(&self) -> Ccd2R {
        Ccd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture or Compare DN event mask CCP3"]
    #[inline(always)]
    pub fn ccd3(&self) -> Ccd3R {
        Ccd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or Compare UP event mask CCP0"]
    #[inline(always)]
    pub fn ccu0(&self) -> Ccu0R {
        Ccu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or Compare UP event mask CCP1"]
    #[inline(always)]
    pub fn ccu1(&self) -> Ccu1R {
        Ccu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture or Compare UP event mask CCP2"]
    #[inline(always)]
    pub fn ccu2(&self) -> Ccu2R {
        Ccu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture or Compare UP event mask CCP3"]
    #[inline(always)]
    pub fn ccu3(&self) -> Ccu3R {
        Ccu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare DN event mask CCP4"]
    #[inline(always)]
    pub fn ccd4(&self) -> Ccd4R {
        Ccd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare DN event mask CCP5"]
    #[inline(always)]
    pub fn ccd5(&self) -> Ccd5R {
        Ccd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare UP event mask CCP4"]
    #[inline(always)]
    pub fn ccu4(&self) -> Ccu4R {
        Ccu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare UP event mask CCP5"]
    #[inline(always)]
    pub fn ccu5(&self) -> Ccu5R {
        Ccu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault Event mask"]
    #[inline(always)]
    pub fn f(&self) -> FR {
        FR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero Event mask"]
    #[inline(always)]
    pub fn repc(&self) -> RepcR {
        RepcR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, Tima0ImaskSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, Tima0ImaskSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or Compare DN event mask CCP0"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, Tima0ImaskSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or Compare DN event mask CCP1"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, Tima0ImaskSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture or Compare DN event mask CCP2"]
    #[inline(always)]
    pub fn ccd2(&mut self) -> Ccd2W<'_, Tima0ImaskSpec> {
        Ccd2W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture or Compare DN event mask CCP3"]
    #[inline(always)]
    pub fn ccd3(&mut self) -> Ccd3W<'_, Tima0ImaskSpec> {
        Ccd3W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture or Compare UP event mask CCP0"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, Tima0ImaskSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or Compare UP event mask CCP1"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, Tima0ImaskSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture or Compare UP event mask CCP2"]
    #[inline(always)]
    pub fn ccu2(&mut self) -> Ccu2W<'_, Tima0ImaskSpec> {
        Ccu2W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture or Compare UP event mask CCP3"]
    #[inline(always)]
    pub fn ccu3(&mut self) -> Ccu3W<'_, Tima0ImaskSpec> {
        Ccu3W::new(self, 11)
    }
    #[doc = "Bit 12 - Compare DN event mask CCP4"]
    #[inline(always)]
    pub fn ccd4(&mut self) -> Ccd4W<'_, Tima0ImaskSpec> {
        Ccd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare DN event mask CCP5"]
    #[inline(always)]
    pub fn ccd5(&mut self) -> Ccd5W<'_, Tima0ImaskSpec> {
        Ccd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare UP event mask CCP4"]
    #[inline(always)]
    pub fn ccu4(&mut self) -> Ccu4W<'_, Tima0ImaskSpec> {
        Ccu4W::new(self, 14)
    }
    #[doc = "Bit 15 - Compare UP event mask CCP5"]
    #[inline(always)]
    pub fn ccu5(&mut self) -> Ccu5W<'_, Tima0ImaskSpec> {
        Ccu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault Event mask"]
    #[inline(always)]
    pub fn f(&mut self) -> FW<'_, Tima0ImaskSpec> {
        FW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, Tima0ImaskSpec> {
        TovW::new(self, 25)
    }
    #[doc = "Bit 26 - Repeat Counter Zero Event mask"]
    #[inline(always)]
    pub fn repc(&mut self) -> RepcW<'_, Tima0ImaskSpec> {
        RepcW::new(self, 26)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0ImaskSpec;
impl crate::RegisterSpec for Tima0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_imask::R`](R) reader structure"]
impl crate::Readable for Tima0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_imask::W`](W) writer structure"]
impl crate::Writable for Tima0ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_IMASK to value 0"]
impl crate::Resettable for Tima0ImaskSpec {}
