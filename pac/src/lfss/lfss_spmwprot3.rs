#[doc = "Register `LFSS_SPMWPROT3` reader"]
pub type R = crate::R<LfssSpmwprot3Spec>;
#[doc = "Register `LFSS_SPMWPROT3` writer"]
pub type W = crate::W<LfssSpmwprot3Spec>;
#[doc = "write protect SPMEM12 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp12_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp12_0> for bool {
    #[inline(always)]
    fn from(variant: Wp12_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_12_0` reader - write protect SPMEM12 - DATA0"]
pub type Wp12_0R = crate::BitReader<Wp12_0>;
impl Wp12_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp12_0 {
        match self.bits {
            false => Wp12_0::Readwrite,
            true => Wp12_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp12_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp12_0::Readonly
    }
}
#[doc = "Field `WP_12_0` writer - write protect SPMEM12 - DATA0"]
pub type Wp12_0W<'a, REG> = crate::BitWriter<'a, REG, Wp12_0>;
impl<'a, REG> Wp12_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_0::Readonly)
    }
}
#[doc = "write protect SPMEM12 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp12_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp12_1> for bool {
    #[inline(always)]
    fn from(variant: Wp12_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_12_1` reader - write protect SPMEM12 - DATA1"]
pub type Wp12_1R = crate::BitReader<Wp12_1>;
impl Wp12_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp12_1 {
        match self.bits {
            false => Wp12_1::Readwrite,
            true => Wp12_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp12_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp12_1::Readonly
    }
}
#[doc = "Field `WP_12_1` writer - write protect SPMEM12 - DATA1"]
pub type Wp12_1W<'a, REG> = crate::BitWriter<'a, REG, Wp12_1>;
impl<'a, REG> Wp12_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_1::Readonly)
    }
}
#[doc = "write protect SPMEM12 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp12_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp12_2> for bool {
    #[inline(always)]
    fn from(variant: Wp12_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_12_2` reader - write protect SPMEM12 - DATA2"]
pub type Wp12_2R = crate::BitReader<Wp12_2>;
impl Wp12_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp12_2 {
        match self.bits {
            false => Wp12_2::Readwrite,
            true => Wp12_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp12_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp12_2::Readonly
    }
}
#[doc = "Field `WP_12_2` writer - write protect SPMEM12 - DATA2"]
pub type Wp12_2W<'a, REG> = crate::BitWriter<'a, REG, Wp12_2>;
impl<'a, REG> Wp12_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_2::Readonly)
    }
}
#[doc = "write protect SPMEM12 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp12_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp12_3> for bool {
    #[inline(always)]
    fn from(variant: Wp12_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_12_3` reader - write protect SPMEM12 - DATA3"]
pub type Wp12_3R = crate::BitReader<Wp12_3>;
impl Wp12_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp12_3 {
        match self.bits {
            false => Wp12_3::Readwrite,
            true => Wp12_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp12_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp12_3::Readonly
    }
}
#[doc = "Field `WP_12_3` writer - write protect SPMEM12 - DATA3"]
pub type Wp12_3W<'a, REG> = crate::BitWriter<'a, REG, Wp12_3>;
impl<'a, REG> Wp12_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp12_3::Readonly)
    }
}
#[doc = "write protect SPMEM13 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp13_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp13_0> for bool {
    #[inline(always)]
    fn from(variant: Wp13_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_13_0` reader - write protect SPMEM13 - DATA0"]
pub type Wp13_0R = crate::BitReader<Wp13_0>;
impl Wp13_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp13_0 {
        match self.bits {
            false => Wp13_0::Readwrite,
            true => Wp13_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp13_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp13_0::Readonly
    }
}
#[doc = "Field `WP_13_0` writer - write protect SPMEM13 - DATA0"]
pub type Wp13_0W<'a, REG> = crate::BitWriter<'a, REG, Wp13_0>;
impl<'a, REG> Wp13_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_0::Readonly)
    }
}
#[doc = "write protect SPMEM13 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp13_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp13_1> for bool {
    #[inline(always)]
    fn from(variant: Wp13_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_13_1` reader - write protect SPMEM13 - DATA1"]
pub type Wp13_1R = crate::BitReader<Wp13_1>;
impl Wp13_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp13_1 {
        match self.bits {
            false => Wp13_1::Readwrite,
            true => Wp13_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp13_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp13_1::Readonly
    }
}
#[doc = "Field `WP_13_1` writer - write protect SPMEM13 - DATA1"]
pub type Wp13_1W<'a, REG> = crate::BitWriter<'a, REG, Wp13_1>;
impl<'a, REG> Wp13_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_1::Readonly)
    }
}
#[doc = "write protect SPMEM13 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp13_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp13_2> for bool {
    #[inline(always)]
    fn from(variant: Wp13_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_13_2` reader - write protect SPMEM13 - DATA2"]
pub type Wp13_2R = crate::BitReader<Wp13_2>;
impl Wp13_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp13_2 {
        match self.bits {
            false => Wp13_2::Readwrite,
            true => Wp13_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp13_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp13_2::Readonly
    }
}
#[doc = "Field `WP_13_2` writer - write protect SPMEM13 - DATA2"]
pub type Wp13_2W<'a, REG> = crate::BitWriter<'a, REG, Wp13_2>;
impl<'a, REG> Wp13_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_2::Readonly)
    }
}
#[doc = "write protect SPMEM13 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp13_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp13_3> for bool {
    #[inline(always)]
    fn from(variant: Wp13_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_13_3` reader - write protect SPMEM13 - DATA3"]
pub type Wp13_3R = crate::BitReader<Wp13_3>;
impl Wp13_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp13_3 {
        match self.bits {
            false => Wp13_3::Readwrite,
            true => Wp13_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp13_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp13_3::Readonly
    }
}
#[doc = "Field `WP_13_3` writer - write protect SPMEM13 - DATA3"]
pub type Wp13_3W<'a, REG> = crate::BitWriter<'a, REG, Wp13_3>;
impl<'a, REG> Wp13_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp13_3::Readonly)
    }
}
#[doc = "write protect SPMEM14 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp14_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp14_0> for bool {
    #[inline(always)]
    fn from(variant: Wp14_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_14_0` reader - write protect SPMEM14 - DATA0"]
pub type Wp14_0R = crate::BitReader<Wp14_0>;
impl Wp14_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp14_0 {
        match self.bits {
            false => Wp14_0::Readwrite,
            true => Wp14_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp14_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp14_0::Readonly
    }
}
#[doc = "Field `WP_14_0` writer - write protect SPMEM14 - DATA0"]
pub type Wp14_0W<'a, REG> = crate::BitWriter<'a, REG, Wp14_0>;
impl<'a, REG> Wp14_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_0::Readonly)
    }
}
#[doc = "write protect SPMEM14 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp14_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp14_1> for bool {
    #[inline(always)]
    fn from(variant: Wp14_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_14_1` reader - write protect SPMEM14 - DATA1"]
pub type Wp14_1R = crate::BitReader<Wp14_1>;
impl Wp14_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp14_1 {
        match self.bits {
            false => Wp14_1::Readwrite,
            true => Wp14_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp14_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp14_1::Readonly
    }
}
#[doc = "Field `WP_14_1` writer - write protect SPMEM14 - DATA1"]
pub type Wp14_1W<'a, REG> = crate::BitWriter<'a, REG, Wp14_1>;
impl<'a, REG> Wp14_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_1::Readonly)
    }
}
#[doc = "write protect SPMEM14- DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp14_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp14_2> for bool {
    #[inline(always)]
    fn from(variant: Wp14_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_14_2` reader - write protect SPMEM14- DATA2"]
pub type Wp14_2R = crate::BitReader<Wp14_2>;
impl Wp14_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp14_2 {
        match self.bits {
            false => Wp14_2::Readwrite,
            true => Wp14_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp14_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp14_2::Readonly
    }
}
#[doc = "Field `WP_14_2` writer - write protect SPMEM14- DATA2"]
pub type Wp14_2W<'a, REG> = crate::BitWriter<'a, REG, Wp14_2>;
impl<'a, REG> Wp14_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_2::Readonly)
    }
}
#[doc = "write protect SPMEM14 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp14_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp14_3> for bool {
    #[inline(always)]
    fn from(variant: Wp14_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_14_3` reader - write protect SPMEM14 - DATA3"]
pub type Wp14_3R = crate::BitReader<Wp14_3>;
impl Wp14_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp14_3 {
        match self.bits {
            false => Wp14_3::Readwrite,
            true => Wp14_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp14_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp14_3::Readonly
    }
}
#[doc = "Field `WP_14_3` writer - write protect SPMEM14 - DATA3"]
pub type Wp14_3W<'a, REG> = crate::BitWriter<'a, REG, Wp14_3>;
impl<'a, REG> Wp14_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp14_3::Readonly)
    }
}
#[doc = "write protect SPMEM15 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp15_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp15_0> for bool {
    #[inline(always)]
    fn from(variant: Wp15_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_15_0` reader - write protect SPMEM15 - DATA0"]
pub type Wp15_0R = crate::BitReader<Wp15_0>;
impl Wp15_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp15_0 {
        match self.bits {
            false => Wp15_0::Readwrite,
            true => Wp15_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp15_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp15_0::Readonly
    }
}
#[doc = "Field `WP_15_0` writer - write protect SPMEM15 - DATA0"]
pub type Wp15_0W<'a, REG> = crate::BitWriter<'a, REG, Wp15_0>;
impl<'a, REG> Wp15_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_0::Readonly)
    }
}
#[doc = "write protect SPMEM15 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp15_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp15_1> for bool {
    #[inline(always)]
    fn from(variant: Wp15_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_15_1` reader - write protect SPMEM15 - DATA1"]
pub type Wp15_1R = crate::BitReader<Wp15_1>;
impl Wp15_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp15_1 {
        match self.bits {
            false => Wp15_1::Readwrite,
            true => Wp15_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp15_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp15_1::Readonly
    }
}
#[doc = "Field `WP_15_1` writer - write protect SPMEM15 - DATA1"]
pub type Wp15_1W<'a, REG> = crate::BitWriter<'a, REG, Wp15_1>;
impl<'a, REG> Wp15_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_1::Readonly)
    }
}
#[doc = "write protect SPMEM15 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp15_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp15_2> for bool {
    #[inline(always)]
    fn from(variant: Wp15_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_15_2` reader - write protect SPMEM15 - DATA2"]
pub type Wp15_2R = crate::BitReader<Wp15_2>;
impl Wp15_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp15_2 {
        match self.bits {
            false => Wp15_2::Readwrite,
            true => Wp15_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp15_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp15_2::Readonly
    }
}
#[doc = "Field `WP_15_2` writer - write protect SPMEM15 - DATA2"]
pub type Wp15_2W<'a, REG> = crate::BitWriter<'a, REG, Wp15_2>;
impl<'a, REG> Wp15_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_2::Readonly)
    }
}
#[doc = "write protect SPMEM15 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp15_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp15_3> for bool {
    #[inline(always)]
    fn from(variant: Wp15_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_15_3` reader - write protect SPMEM15 - DATA3"]
pub type Wp15_3R = crate::BitReader<Wp15_3>;
impl Wp15_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp15_3 {
        match self.bits {
            false => Wp15_3::Readwrite,
            true => Wp15_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp15_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp15_3::Readonly
    }
}
#[doc = "Field `WP_15_3` writer - write protect SPMEM15 - DATA3"]
pub type Wp15_3W<'a, REG> = crate::BitWriter<'a, REG, Wp15_3>;
impl<'a, REG> Wp15_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp15_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM12 - DATA0"]
    #[inline(always)]
    pub fn wp_12_0(&self) -> Wp12_0R {
        Wp12_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM12 - DATA1"]
    #[inline(always)]
    pub fn wp_12_1(&self) -> Wp12_1R {
        Wp12_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM12 - DATA2"]
    #[inline(always)]
    pub fn wp_12_2(&self) -> Wp12_2R {
        Wp12_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM12 - DATA3"]
    #[inline(always)]
    pub fn wp_12_3(&self) -> Wp12_3R {
        Wp12_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM13 - DATA0"]
    #[inline(always)]
    pub fn wp_13_0(&self) -> Wp13_0R {
        Wp13_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM13 - DATA1"]
    #[inline(always)]
    pub fn wp_13_1(&self) -> Wp13_1R {
        Wp13_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM13 - DATA2"]
    #[inline(always)]
    pub fn wp_13_2(&self) -> Wp13_2R {
        Wp13_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM13 - DATA3"]
    #[inline(always)]
    pub fn wp_13_3(&self) -> Wp13_3R {
        Wp13_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM14 - DATA0"]
    #[inline(always)]
    pub fn wp_14_0(&self) -> Wp14_0R {
        Wp14_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM14 - DATA1"]
    #[inline(always)]
    pub fn wp_14_1(&self) -> Wp14_1R {
        Wp14_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM14- DATA2"]
    #[inline(always)]
    pub fn wp_14_2(&self) -> Wp14_2R {
        Wp14_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM14 - DATA3"]
    #[inline(always)]
    pub fn wp_14_3(&self) -> Wp14_3R {
        Wp14_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM15 - DATA0"]
    #[inline(always)]
    pub fn wp_15_0(&self) -> Wp15_0R {
        Wp15_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM15 - DATA1"]
    #[inline(always)]
    pub fn wp_15_1(&self) -> Wp15_1R {
        Wp15_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM15 - DATA2"]
    #[inline(always)]
    pub fn wp_15_2(&self) -> Wp15_2R {
        Wp15_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM15 - DATA3"]
    #[inline(always)]
    pub fn wp_15_3(&self) -> Wp15_3R {
        Wp15_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM12 - DATA0"]
    #[inline(always)]
    pub fn wp_12_0(&mut self) -> Wp12_0W<'_, LfssSpmwprot3Spec> {
        Wp12_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM12 - DATA1"]
    #[inline(always)]
    pub fn wp_12_1(&mut self) -> Wp12_1W<'_, LfssSpmwprot3Spec> {
        Wp12_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM12 - DATA2"]
    #[inline(always)]
    pub fn wp_12_2(&mut self) -> Wp12_2W<'_, LfssSpmwprot3Spec> {
        Wp12_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM12 - DATA3"]
    #[inline(always)]
    pub fn wp_12_3(&mut self) -> Wp12_3W<'_, LfssSpmwprot3Spec> {
        Wp12_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM13 - DATA0"]
    #[inline(always)]
    pub fn wp_13_0(&mut self) -> Wp13_0W<'_, LfssSpmwprot3Spec> {
        Wp13_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM13 - DATA1"]
    #[inline(always)]
    pub fn wp_13_1(&mut self) -> Wp13_1W<'_, LfssSpmwprot3Spec> {
        Wp13_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM13 - DATA2"]
    #[inline(always)]
    pub fn wp_13_2(&mut self) -> Wp13_2W<'_, LfssSpmwprot3Spec> {
        Wp13_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM13 - DATA3"]
    #[inline(always)]
    pub fn wp_13_3(&mut self) -> Wp13_3W<'_, LfssSpmwprot3Spec> {
        Wp13_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM14 - DATA0"]
    #[inline(always)]
    pub fn wp_14_0(&mut self) -> Wp14_0W<'_, LfssSpmwprot3Spec> {
        Wp14_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM14 - DATA1"]
    #[inline(always)]
    pub fn wp_14_1(&mut self) -> Wp14_1W<'_, LfssSpmwprot3Spec> {
        Wp14_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM14- DATA2"]
    #[inline(always)]
    pub fn wp_14_2(&mut self) -> Wp14_2W<'_, LfssSpmwprot3Spec> {
        Wp14_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM14 - DATA3"]
    #[inline(always)]
    pub fn wp_14_3(&mut self) -> Wp14_3W<'_, LfssSpmwprot3Spec> {
        Wp14_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM15 - DATA0"]
    #[inline(always)]
    pub fn wp_15_0(&mut self) -> Wp15_0W<'_, LfssSpmwprot3Spec> {
        Wp15_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM15 - DATA1"]
    #[inline(always)]
    pub fn wp_15_1(&mut self) -> Wp15_1W<'_, LfssSpmwprot3Spec> {
        Wp15_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM15 - DATA2"]
    #[inline(always)]
    pub fn wp_15_2(&mut self) -> Wp15_2W<'_, LfssSpmwprot3Spec> {
        Wp15_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM15 - DATA3"]
    #[inline(always)]
    pub fn wp_15_3(&mut self) -> Wp15_3W<'_, LfssSpmwprot3Spec> {
        Wp15_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot3Spec;
impl crate::RegisterSpec for LfssSpmwprot3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot3::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot3Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot3::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT3 to value 0"]
impl crate::Resettable for LfssSpmwprot3Spec {}
