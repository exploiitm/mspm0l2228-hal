#[doc = "Register `LFSS_SPMWPROT4` reader"]
pub type R = crate::R<LfssSpmwprot4Spec>;
#[doc = "Register `LFSS_SPMWPROT4` writer"]
pub type W = crate::W<LfssSpmwprot4Spec>;
#[doc = "write protect SPMEM16 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp16_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp16_0> for bool {
    #[inline(always)]
    fn from(variant: Wp16_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_16_0` reader - write protect SPMEM16 - DATA0"]
pub type Wp16_0R = crate::BitReader<Wp16_0>;
impl Wp16_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp16_0 {
        match self.bits {
            false => Wp16_0::Readwrite,
            true => Wp16_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp16_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp16_0::Readonly
    }
}
#[doc = "Field `WP_16_0` writer - write protect SPMEM16 - DATA0"]
pub type Wp16_0W<'a, REG> = crate::BitWriter<'a, REG, Wp16_0>;
impl<'a, REG> Wp16_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_0::Readonly)
    }
}
#[doc = "write protect SPMEM16 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp16_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp16_1> for bool {
    #[inline(always)]
    fn from(variant: Wp16_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_16_1` reader - write protect SPMEM16 - DATA1"]
pub type Wp16_1R = crate::BitReader<Wp16_1>;
impl Wp16_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp16_1 {
        match self.bits {
            false => Wp16_1::Readwrite,
            true => Wp16_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp16_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp16_1::Readonly
    }
}
#[doc = "Field `WP_16_1` writer - write protect SPMEM16 - DATA1"]
pub type Wp16_1W<'a, REG> = crate::BitWriter<'a, REG, Wp16_1>;
impl<'a, REG> Wp16_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_1::Readonly)
    }
}
#[doc = "write protect SPMEM16 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp16_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp16_2> for bool {
    #[inline(always)]
    fn from(variant: Wp16_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_16_2` reader - write protect SPMEM16 - DATA2"]
pub type Wp16_2R = crate::BitReader<Wp16_2>;
impl Wp16_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp16_2 {
        match self.bits {
            false => Wp16_2::Readwrite,
            true => Wp16_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp16_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp16_2::Readonly
    }
}
#[doc = "Field `WP_16_2` writer - write protect SPMEM16 - DATA2"]
pub type Wp16_2W<'a, REG> = crate::BitWriter<'a, REG, Wp16_2>;
impl<'a, REG> Wp16_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_2::Readonly)
    }
}
#[doc = "write protect SPMEM16 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp16_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp16_3> for bool {
    #[inline(always)]
    fn from(variant: Wp16_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_16_3` reader - write protect SPMEM16 - DATA3"]
pub type Wp16_3R = crate::BitReader<Wp16_3>;
impl Wp16_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp16_3 {
        match self.bits {
            false => Wp16_3::Readwrite,
            true => Wp16_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp16_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp16_3::Readonly
    }
}
#[doc = "Field `WP_16_3` writer - write protect SPMEM16 - DATA3"]
pub type Wp16_3W<'a, REG> = crate::BitWriter<'a, REG, Wp16_3>;
impl<'a, REG> Wp16_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp16_3::Readonly)
    }
}
#[doc = "write protect SPMEM17 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp17_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp17_0> for bool {
    #[inline(always)]
    fn from(variant: Wp17_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_17_0` reader - write protect SPMEM17 - DATA0"]
pub type Wp17_0R = crate::BitReader<Wp17_0>;
impl Wp17_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp17_0 {
        match self.bits {
            false => Wp17_0::Readwrite,
            true => Wp17_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp17_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp17_0::Readonly
    }
}
#[doc = "Field `WP_17_0` writer - write protect SPMEM17 - DATA0"]
pub type Wp17_0W<'a, REG> = crate::BitWriter<'a, REG, Wp17_0>;
impl<'a, REG> Wp17_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_0::Readonly)
    }
}
#[doc = "write protect SPMEM17 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp17_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp17_1> for bool {
    #[inline(always)]
    fn from(variant: Wp17_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_17_1` reader - write protect SPMEM17 - DATA1"]
pub type Wp17_1R = crate::BitReader<Wp17_1>;
impl Wp17_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp17_1 {
        match self.bits {
            false => Wp17_1::Readwrite,
            true => Wp17_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp17_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp17_1::Readonly
    }
}
#[doc = "Field `WP_17_1` writer - write protect SPMEM17 - DATA1"]
pub type Wp17_1W<'a, REG> = crate::BitWriter<'a, REG, Wp17_1>;
impl<'a, REG> Wp17_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_1::Readonly)
    }
}
#[doc = "write protect SPMEM17 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp17_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp17_2> for bool {
    #[inline(always)]
    fn from(variant: Wp17_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_17_2` reader - write protect SPMEM17 - DATA2"]
pub type Wp17_2R = crate::BitReader<Wp17_2>;
impl Wp17_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp17_2 {
        match self.bits {
            false => Wp17_2::Readwrite,
            true => Wp17_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp17_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp17_2::Readonly
    }
}
#[doc = "Field `WP_17_2` writer - write protect SPMEM17 - DATA2"]
pub type Wp17_2W<'a, REG> = crate::BitWriter<'a, REG, Wp17_2>;
impl<'a, REG> Wp17_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_2::Readonly)
    }
}
#[doc = "write protect SPMEM17 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp17_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp17_3> for bool {
    #[inline(always)]
    fn from(variant: Wp17_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_17_3` reader - write protect SPMEM17 - DATA3"]
pub type Wp17_3R = crate::BitReader<Wp17_3>;
impl Wp17_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp17_3 {
        match self.bits {
            false => Wp17_3::Readwrite,
            true => Wp17_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp17_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp17_3::Readonly
    }
}
#[doc = "Field `WP_17_3` writer - write protect SPMEM17 - DATA3"]
pub type Wp17_3W<'a, REG> = crate::BitWriter<'a, REG, Wp17_3>;
impl<'a, REG> Wp17_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp17_3::Readonly)
    }
}
#[doc = "write protect SPMEM18 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp18_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp18_0> for bool {
    #[inline(always)]
    fn from(variant: Wp18_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_18_0` reader - write protect SPMEM18 - DATA0"]
pub type Wp18_0R = crate::BitReader<Wp18_0>;
impl Wp18_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp18_0 {
        match self.bits {
            false => Wp18_0::Readwrite,
            true => Wp18_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp18_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp18_0::Readonly
    }
}
#[doc = "Field `WP_18_0` writer - write protect SPMEM18 - DATA0"]
pub type Wp18_0W<'a, REG> = crate::BitWriter<'a, REG, Wp18_0>;
impl<'a, REG> Wp18_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_0::Readonly)
    }
}
#[doc = "write protect SPMEM18 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp18_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp18_1> for bool {
    #[inline(always)]
    fn from(variant: Wp18_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_18_1` reader - write protect SPMEM18 - DATA1"]
pub type Wp18_1R = crate::BitReader<Wp18_1>;
impl Wp18_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp18_1 {
        match self.bits {
            false => Wp18_1::Readwrite,
            true => Wp18_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp18_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp18_1::Readonly
    }
}
#[doc = "Field `WP_18_1` writer - write protect SPMEM18 - DATA1"]
pub type Wp18_1W<'a, REG> = crate::BitWriter<'a, REG, Wp18_1>;
impl<'a, REG> Wp18_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_1::Readonly)
    }
}
#[doc = "write protect SPMEM18- DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp18_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp18_2> for bool {
    #[inline(always)]
    fn from(variant: Wp18_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_18_2` reader - write protect SPMEM18- DATA2"]
pub type Wp18_2R = crate::BitReader<Wp18_2>;
impl Wp18_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp18_2 {
        match self.bits {
            false => Wp18_2::Readwrite,
            true => Wp18_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp18_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp18_2::Readonly
    }
}
#[doc = "Field `WP_18_2` writer - write protect SPMEM18- DATA2"]
pub type Wp18_2W<'a, REG> = crate::BitWriter<'a, REG, Wp18_2>;
impl<'a, REG> Wp18_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_2::Readonly)
    }
}
#[doc = "write protect SPMEM18 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp18_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp18_3> for bool {
    #[inline(always)]
    fn from(variant: Wp18_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_18_3` reader - write protect SPMEM18 - DATA3"]
pub type Wp18_3R = crate::BitReader<Wp18_3>;
impl Wp18_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp18_3 {
        match self.bits {
            false => Wp18_3::Readwrite,
            true => Wp18_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp18_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp18_3::Readonly
    }
}
#[doc = "Field `WP_18_3` writer - write protect SPMEM18 - DATA3"]
pub type Wp18_3W<'a, REG> = crate::BitWriter<'a, REG, Wp18_3>;
impl<'a, REG> Wp18_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp18_3::Readonly)
    }
}
#[doc = "write protect SPMEM19 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp19_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp19_0> for bool {
    #[inline(always)]
    fn from(variant: Wp19_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_19_0` reader - write protect SPMEM19 - DATA0"]
pub type Wp19_0R = crate::BitReader<Wp19_0>;
impl Wp19_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp19_0 {
        match self.bits {
            false => Wp19_0::Readwrite,
            true => Wp19_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp19_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp19_0::Readonly
    }
}
#[doc = "Field `WP_19_0` writer - write protect SPMEM19 - DATA0"]
pub type Wp19_0W<'a, REG> = crate::BitWriter<'a, REG, Wp19_0>;
impl<'a, REG> Wp19_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_0::Readonly)
    }
}
#[doc = "write protect SPMEM19 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp19_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp19_1> for bool {
    #[inline(always)]
    fn from(variant: Wp19_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_19_1` reader - write protect SPMEM19 - DATA1"]
pub type Wp19_1R = crate::BitReader<Wp19_1>;
impl Wp19_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp19_1 {
        match self.bits {
            false => Wp19_1::Readwrite,
            true => Wp19_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp19_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp19_1::Readonly
    }
}
#[doc = "Field `WP_19_1` writer - write protect SPMEM19 - DATA1"]
pub type Wp19_1W<'a, REG> = crate::BitWriter<'a, REG, Wp19_1>;
impl<'a, REG> Wp19_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_1::Readonly)
    }
}
#[doc = "write protect SPMEM19 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp19_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp19_2> for bool {
    #[inline(always)]
    fn from(variant: Wp19_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_19_2` reader - write protect SPMEM19 - DATA2"]
pub type Wp19_2R = crate::BitReader<Wp19_2>;
impl Wp19_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp19_2 {
        match self.bits {
            false => Wp19_2::Readwrite,
            true => Wp19_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp19_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp19_2::Readonly
    }
}
#[doc = "Field `WP_19_2` writer - write protect SPMEM19 - DATA2"]
pub type Wp19_2W<'a, REG> = crate::BitWriter<'a, REG, Wp19_2>;
impl<'a, REG> Wp19_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_2::Readonly)
    }
}
#[doc = "write protect SPMEM19 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp19_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp19_3> for bool {
    #[inline(always)]
    fn from(variant: Wp19_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_19_3` reader - write protect SPMEM19 - DATA3"]
pub type Wp19_3R = crate::BitReader<Wp19_3>;
impl Wp19_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp19_3 {
        match self.bits {
            false => Wp19_3::Readwrite,
            true => Wp19_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp19_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp19_3::Readonly
    }
}
#[doc = "Field `WP_19_3` writer - write protect SPMEM19 - DATA3"]
pub type Wp19_3W<'a, REG> = crate::BitWriter<'a, REG, Wp19_3>;
impl<'a, REG> Wp19_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp19_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM16 - DATA0"]
    #[inline(always)]
    pub fn wp_16_0(&self) -> Wp16_0R {
        Wp16_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM16 - DATA1"]
    #[inline(always)]
    pub fn wp_16_1(&self) -> Wp16_1R {
        Wp16_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM16 - DATA2"]
    #[inline(always)]
    pub fn wp_16_2(&self) -> Wp16_2R {
        Wp16_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM16 - DATA3"]
    #[inline(always)]
    pub fn wp_16_3(&self) -> Wp16_3R {
        Wp16_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM17 - DATA0"]
    #[inline(always)]
    pub fn wp_17_0(&self) -> Wp17_0R {
        Wp17_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM17 - DATA1"]
    #[inline(always)]
    pub fn wp_17_1(&self) -> Wp17_1R {
        Wp17_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM17 - DATA2"]
    #[inline(always)]
    pub fn wp_17_2(&self) -> Wp17_2R {
        Wp17_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM17 - DATA3"]
    #[inline(always)]
    pub fn wp_17_3(&self) -> Wp17_3R {
        Wp17_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM18 - DATA0"]
    #[inline(always)]
    pub fn wp_18_0(&self) -> Wp18_0R {
        Wp18_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM18 - DATA1"]
    #[inline(always)]
    pub fn wp_18_1(&self) -> Wp18_1R {
        Wp18_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM18- DATA2"]
    #[inline(always)]
    pub fn wp_18_2(&self) -> Wp18_2R {
        Wp18_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM18 - DATA3"]
    #[inline(always)]
    pub fn wp_18_3(&self) -> Wp18_3R {
        Wp18_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM19 - DATA0"]
    #[inline(always)]
    pub fn wp_19_0(&self) -> Wp19_0R {
        Wp19_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM19 - DATA1"]
    #[inline(always)]
    pub fn wp_19_1(&self) -> Wp19_1R {
        Wp19_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM19 - DATA2"]
    #[inline(always)]
    pub fn wp_19_2(&self) -> Wp19_2R {
        Wp19_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM19 - DATA3"]
    #[inline(always)]
    pub fn wp_19_3(&self) -> Wp19_3R {
        Wp19_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM16 - DATA0"]
    #[inline(always)]
    pub fn wp_16_0(&mut self) -> Wp16_0W<'_, LfssSpmwprot4Spec> {
        Wp16_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM16 - DATA1"]
    #[inline(always)]
    pub fn wp_16_1(&mut self) -> Wp16_1W<'_, LfssSpmwprot4Spec> {
        Wp16_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM16 - DATA2"]
    #[inline(always)]
    pub fn wp_16_2(&mut self) -> Wp16_2W<'_, LfssSpmwprot4Spec> {
        Wp16_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM16 - DATA3"]
    #[inline(always)]
    pub fn wp_16_3(&mut self) -> Wp16_3W<'_, LfssSpmwprot4Spec> {
        Wp16_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM17 - DATA0"]
    #[inline(always)]
    pub fn wp_17_0(&mut self) -> Wp17_0W<'_, LfssSpmwprot4Spec> {
        Wp17_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM17 - DATA1"]
    #[inline(always)]
    pub fn wp_17_1(&mut self) -> Wp17_1W<'_, LfssSpmwprot4Spec> {
        Wp17_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM17 - DATA2"]
    #[inline(always)]
    pub fn wp_17_2(&mut self) -> Wp17_2W<'_, LfssSpmwprot4Spec> {
        Wp17_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM17 - DATA3"]
    #[inline(always)]
    pub fn wp_17_3(&mut self) -> Wp17_3W<'_, LfssSpmwprot4Spec> {
        Wp17_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM18 - DATA0"]
    #[inline(always)]
    pub fn wp_18_0(&mut self) -> Wp18_0W<'_, LfssSpmwprot4Spec> {
        Wp18_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM18 - DATA1"]
    #[inline(always)]
    pub fn wp_18_1(&mut self) -> Wp18_1W<'_, LfssSpmwprot4Spec> {
        Wp18_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM18- DATA2"]
    #[inline(always)]
    pub fn wp_18_2(&mut self) -> Wp18_2W<'_, LfssSpmwprot4Spec> {
        Wp18_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM18 - DATA3"]
    #[inline(always)]
    pub fn wp_18_3(&mut self) -> Wp18_3W<'_, LfssSpmwprot4Spec> {
        Wp18_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM19 - DATA0"]
    #[inline(always)]
    pub fn wp_19_0(&mut self) -> Wp19_0W<'_, LfssSpmwprot4Spec> {
        Wp19_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM19 - DATA1"]
    #[inline(always)]
    pub fn wp_19_1(&mut self) -> Wp19_1W<'_, LfssSpmwprot4Spec> {
        Wp19_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM19 - DATA2"]
    #[inline(always)]
    pub fn wp_19_2(&mut self) -> Wp19_2W<'_, LfssSpmwprot4Spec> {
        Wp19_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM19 - DATA3"]
    #[inline(always)]
    pub fn wp_19_3(&mut self) -> Wp19_3W<'_, LfssSpmwprot4Spec> {
        Wp19_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot4Spec;
impl crate::RegisterSpec for LfssSpmwprot4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot4::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot4Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot4::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT4 to value 0"]
impl crate::Resettable for LfssSpmwprot4Spec {}
