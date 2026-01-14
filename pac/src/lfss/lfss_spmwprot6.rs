#[doc = "Register `LFSS_SPMWPROT6` reader"]
pub type R = crate::R<LfssSpmwprot6Spec>;
#[doc = "Register `LFSS_SPMWPROT6` writer"]
pub type W = crate::W<LfssSpmwprot6Spec>;
#[doc = "write protect SPMEM24 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp24_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp24_0> for bool {
    #[inline(always)]
    fn from(variant: Wp24_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_24_0` reader - write protect SPMEM24 - DATA0"]
pub type Wp24_0R = crate::BitReader<Wp24_0>;
impl Wp24_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp24_0 {
        match self.bits {
            false => Wp24_0::Readwrite,
            true => Wp24_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp24_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp24_0::Readonly
    }
}
#[doc = "Field `WP_24_0` writer - write protect SPMEM24 - DATA0"]
pub type Wp24_0W<'a, REG> = crate::BitWriter<'a, REG, Wp24_0>;
impl<'a, REG> Wp24_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_0::Readonly)
    }
}
#[doc = "write protect SPMEM24 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp24_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp24_1> for bool {
    #[inline(always)]
    fn from(variant: Wp24_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_24_1` reader - write protect SPMEM24 - DATA1"]
pub type Wp24_1R = crate::BitReader<Wp24_1>;
impl Wp24_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp24_1 {
        match self.bits {
            false => Wp24_1::Readwrite,
            true => Wp24_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp24_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp24_1::Readonly
    }
}
#[doc = "Field `WP_24_1` writer - write protect SPMEM24 - DATA1"]
pub type Wp24_1W<'a, REG> = crate::BitWriter<'a, REG, Wp24_1>;
impl<'a, REG> Wp24_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_1::Readonly)
    }
}
#[doc = "write protect SPMEM24 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp24_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp24_2> for bool {
    #[inline(always)]
    fn from(variant: Wp24_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_24_2` reader - write protect SPMEM24 - DATA2"]
pub type Wp24_2R = crate::BitReader<Wp24_2>;
impl Wp24_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp24_2 {
        match self.bits {
            false => Wp24_2::Readwrite,
            true => Wp24_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp24_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp24_2::Readonly
    }
}
#[doc = "Field `WP_24_2` writer - write protect SPMEM24 - DATA2"]
pub type Wp24_2W<'a, REG> = crate::BitWriter<'a, REG, Wp24_2>;
impl<'a, REG> Wp24_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_2::Readonly)
    }
}
#[doc = "write protect SPMEM24 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp24_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp24_3> for bool {
    #[inline(always)]
    fn from(variant: Wp24_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_24_3` reader - write protect SPMEM24 - DATA3"]
pub type Wp24_3R = crate::BitReader<Wp24_3>;
impl Wp24_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp24_3 {
        match self.bits {
            false => Wp24_3::Readwrite,
            true => Wp24_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp24_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp24_3::Readonly
    }
}
#[doc = "Field `WP_24_3` writer - write protect SPMEM24 - DATA3"]
pub type Wp24_3W<'a, REG> = crate::BitWriter<'a, REG, Wp24_3>;
impl<'a, REG> Wp24_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp24_3::Readonly)
    }
}
#[doc = "write protect SPMEM25 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp25_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp25_0> for bool {
    #[inline(always)]
    fn from(variant: Wp25_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_25_0` reader - write protect SPMEM25 - DATA0"]
pub type Wp25_0R = crate::BitReader<Wp25_0>;
impl Wp25_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp25_0 {
        match self.bits {
            false => Wp25_0::Readwrite,
            true => Wp25_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp25_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp25_0::Readonly
    }
}
#[doc = "Field `WP_25_0` writer - write protect SPMEM25 - DATA0"]
pub type Wp25_0W<'a, REG> = crate::BitWriter<'a, REG, Wp25_0>;
impl<'a, REG> Wp25_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_0::Readonly)
    }
}
#[doc = "write protect SPMEM25 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp25_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp25_1> for bool {
    #[inline(always)]
    fn from(variant: Wp25_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_25_1` reader - write protect SPMEM25 - DATA1"]
pub type Wp25_1R = crate::BitReader<Wp25_1>;
impl Wp25_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp25_1 {
        match self.bits {
            false => Wp25_1::Readwrite,
            true => Wp25_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp25_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp25_1::Readonly
    }
}
#[doc = "Field `WP_25_1` writer - write protect SPMEM25 - DATA1"]
pub type Wp25_1W<'a, REG> = crate::BitWriter<'a, REG, Wp25_1>;
impl<'a, REG> Wp25_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_1::Readonly)
    }
}
#[doc = "write protect SPMEM25 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp25_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp25_2> for bool {
    #[inline(always)]
    fn from(variant: Wp25_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_25_2` reader - write protect SPMEM25 - DATA2"]
pub type Wp25_2R = crate::BitReader<Wp25_2>;
impl Wp25_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp25_2 {
        match self.bits {
            false => Wp25_2::Readwrite,
            true => Wp25_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp25_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp25_2::Readonly
    }
}
#[doc = "Field `WP_25_2` writer - write protect SPMEM25 - DATA2"]
pub type Wp25_2W<'a, REG> = crate::BitWriter<'a, REG, Wp25_2>;
impl<'a, REG> Wp25_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_2::Readonly)
    }
}
#[doc = "write protect SPMEM25 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp25_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp25_3> for bool {
    #[inline(always)]
    fn from(variant: Wp25_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_25_3` reader - write protect SPMEM25 - DATA3"]
pub type Wp25_3R = crate::BitReader<Wp25_3>;
impl Wp25_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp25_3 {
        match self.bits {
            false => Wp25_3::Readwrite,
            true => Wp25_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp25_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp25_3::Readonly
    }
}
#[doc = "Field `WP_25_3` writer - write protect SPMEM25 - DATA3"]
pub type Wp25_3W<'a, REG> = crate::BitWriter<'a, REG, Wp25_3>;
impl<'a, REG> Wp25_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp25_3::Readonly)
    }
}
#[doc = "write protect SPMEM26 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp26_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp26_0> for bool {
    #[inline(always)]
    fn from(variant: Wp26_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_26_0` reader - write protect SPMEM26 - DATA0"]
pub type Wp26_0R = crate::BitReader<Wp26_0>;
impl Wp26_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp26_0 {
        match self.bits {
            false => Wp26_0::Readwrite,
            true => Wp26_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp26_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp26_0::Readonly
    }
}
#[doc = "Field `WP_26_0` writer - write protect SPMEM26 - DATA0"]
pub type Wp26_0W<'a, REG> = crate::BitWriter<'a, REG, Wp26_0>;
impl<'a, REG> Wp26_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_0::Readonly)
    }
}
#[doc = "write protect SPMEM26 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp26_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp26_1> for bool {
    #[inline(always)]
    fn from(variant: Wp26_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_26_1` reader - write protect SPMEM26 - DATA1"]
pub type Wp26_1R = crate::BitReader<Wp26_1>;
impl Wp26_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp26_1 {
        match self.bits {
            false => Wp26_1::Readwrite,
            true => Wp26_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp26_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp26_1::Readonly
    }
}
#[doc = "Field `WP_26_1` writer - write protect SPMEM26 - DATA1"]
pub type Wp26_1W<'a, REG> = crate::BitWriter<'a, REG, Wp26_1>;
impl<'a, REG> Wp26_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_1::Readonly)
    }
}
#[doc = "write protect SPMEM26- DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp26_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp26_2> for bool {
    #[inline(always)]
    fn from(variant: Wp26_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_26_2` reader - write protect SPMEM26- DATA2"]
pub type Wp26_2R = crate::BitReader<Wp26_2>;
impl Wp26_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp26_2 {
        match self.bits {
            false => Wp26_2::Readwrite,
            true => Wp26_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp26_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp26_2::Readonly
    }
}
#[doc = "Field `WP_26_2` writer - write protect SPMEM26- DATA2"]
pub type Wp26_2W<'a, REG> = crate::BitWriter<'a, REG, Wp26_2>;
impl<'a, REG> Wp26_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_2::Readonly)
    }
}
#[doc = "write protect SPMEM26 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp26_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp26_3> for bool {
    #[inline(always)]
    fn from(variant: Wp26_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_26_3` reader - write protect SPMEM26 - DATA3"]
pub type Wp26_3R = crate::BitReader<Wp26_3>;
impl Wp26_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp26_3 {
        match self.bits {
            false => Wp26_3::Readwrite,
            true => Wp26_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp26_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp26_3::Readonly
    }
}
#[doc = "Field `WP_26_3` writer - write protect SPMEM26 - DATA3"]
pub type Wp26_3W<'a, REG> = crate::BitWriter<'a, REG, Wp26_3>;
impl<'a, REG> Wp26_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp26_3::Readonly)
    }
}
#[doc = "write protect SPMEM27 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp27_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp27_0> for bool {
    #[inline(always)]
    fn from(variant: Wp27_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_27_0` reader - write protect SPMEM27 - DATA0"]
pub type Wp27_0R = crate::BitReader<Wp27_0>;
impl Wp27_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp27_0 {
        match self.bits {
            false => Wp27_0::Readwrite,
            true => Wp27_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp27_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp27_0::Readonly
    }
}
#[doc = "Field `WP_27_0` writer - write protect SPMEM27 - DATA0"]
pub type Wp27_0W<'a, REG> = crate::BitWriter<'a, REG, Wp27_0>;
impl<'a, REG> Wp27_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_0::Readonly)
    }
}
#[doc = "write protect SPMEM27 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp27_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp27_1> for bool {
    #[inline(always)]
    fn from(variant: Wp27_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_27_1` reader - write protect SPMEM27 - DATA1"]
pub type Wp27_1R = crate::BitReader<Wp27_1>;
impl Wp27_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp27_1 {
        match self.bits {
            false => Wp27_1::Readwrite,
            true => Wp27_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp27_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp27_1::Readonly
    }
}
#[doc = "Field `WP_27_1` writer - write protect SPMEM27 - DATA1"]
pub type Wp27_1W<'a, REG> = crate::BitWriter<'a, REG, Wp27_1>;
impl<'a, REG> Wp27_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_1::Readonly)
    }
}
#[doc = "write protect SPMEM27 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp27_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp27_2> for bool {
    #[inline(always)]
    fn from(variant: Wp27_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_27_2` reader - write protect SPMEM27 - DATA2"]
pub type Wp27_2R = crate::BitReader<Wp27_2>;
impl Wp27_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp27_2 {
        match self.bits {
            false => Wp27_2::Readwrite,
            true => Wp27_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp27_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp27_2::Readonly
    }
}
#[doc = "Field `WP_27_2` writer - write protect SPMEM27 - DATA2"]
pub type Wp27_2W<'a, REG> = crate::BitWriter<'a, REG, Wp27_2>;
impl<'a, REG> Wp27_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_2::Readonly)
    }
}
#[doc = "write protect SPMEM27 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp27_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp27_3> for bool {
    #[inline(always)]
    fn from(variant: Wp27_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_27_3` reader - write protect SPMEM27 - DATA3"]
pub type Wp27_3R = crate::BitReader<Wp27_3>;
impl Wp27_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp27_3 {
        match self.bits {
            false => Wp27_3::Readwrite,
            true => Wp27_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp27_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp27_3::Readonly
    }
}
#[doc = "Field `WP_27_3` writer - write protect SPMEM27 - DATA3"]
pub type Wp27_3W<'a, REG> = crate::BitWriter<'a, REG, Wp27_3>;
impl<'a, REG> Wp27_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp27_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM24 - DATA0"]
    #[inline(always)]
    pub fn wp_24_0(&self) -> Wp24_0R {
        Wp24_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM24 - DATA1"]
    #[inline(always)]
    pub fn wp_24_1(&self) -> Wp24_1R {
        Wp24_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM24 - DATA2"]
    #[inline(always)]
    pub fn wp_24_2(&self) -> Wp24_2R {
        Wp24_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM24 - DATA3"]
    #[inline(always)]
    pub fn wp_24_3(&self) -> Wp24_3R {
        Wp24_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM25 - DATA0"]
    #[inline(always)]
    pub fn wp_25_0(&self) -> Wp25_0R {
        Wp25_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM25 - DATA1"]
    #[inline(always)]
    pub fn wp_25_1(&self) -> Wp25_1R {
        Wp25_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM25 - DATA2"]
    #[inline(always)]
    pub fn wp_25_2(&self) -> Wp25_2R {
        Wp25_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM25 - DATA3"]
    #[inline(always)]
    pub fn wp_25_3(&self) -> Wp25_3R {
        Wp25_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM26 - DATA0"]
    #[inline(always)]
    pub fn wp_26_0(&self) -> Wp26_0R {
        Wp26_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM26 - DATA1"]
    #[inline(always)]
    pub fn wp_26_1(&self) -> Wp26_1R {
        Wp26_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM26- DATA2"]
    #[inline(always)]
    pub fn wp_26_2(&self) -> Wp26_2R {
        Wp26_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM26 - DATA3"]
    #[inline(always)]
    pub fn wp_26_3(&self) -> Wp26_3R {
        Wp26_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM27 - DATA0"]
    #[inline(always)]
    pub fn wp_27_0(&self) -> Wp27_0R {
        Wp27_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM27 - DATA1"]
    #[inline(always)]
    pub fn wp_27_1(&self) -> Wp27_1R {
        Wp27_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM27 - DATA2"]
    #[inline(always)]
    pub fn wp_27_2(&self) -> Wp27_2R {
        Wp27_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM27 - DATA3"]
    #[inline(always)]
    pub fn wp_27_3(&self) -> Wp27_3R {
        Wp27_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM24 - DATA0"]
    #[inline(always)]
    pub fn wp_24_0(&mut self) -> Wp24_0W<'_, LfssSpmwprot6Spec> {
        Wp24_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM24 - DATA1"]
    #[inline(always)]
    pub fn wp_24_1(&mut self) -> Wp24_1W<'_, LfssSpmwprot6Spec> {
        Wp24_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM24 - DATA2"]
    #[inline(always)]
    pub fn wp_24_2(&mut self) -> Wp24_2W<'_, LfssSpmwprot6Spec> {
        Wp24_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM24 - DATA3"]
    #[inline(always)]
    pub fn wp_24_3(&mut self) -> Wp24_3W<'_, LfssSpmwprot6Spec> {
        Wp24_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM25 - DATA0"]
    #[inline(always)]
    pub fn wp_25_0(&mut self) -> Wp25_0W<'_, LfssSpmwprot6Spec> {
        Wp25_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM25 - DATA1"]
    #[inline(always)]
    pub fn wp_25_1(&mut self) -> Wp25_1W<'_, LfssSpmwprot6Spec> {
        Wp25_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM25 - DATA2"]
    #[inline(always)]
    pub fn wp_25_2(&mut self) -> Wp25_2W<'_, LfssSpmwprot6Spec> {
        Wp25_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM25 - DATA3"]
    #[inline(always)]
    pub fn wp_25_3(&mut self) -> Wp25_3W<'_, LfssSpmwprot6Spec> {
        Wp25_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM26 - DATA0"]
    #[inline(always)]
    pub fn wp_26_0(&mut self) -> Wp26_0W<'_, LfssSpmwprot6Spec> {
        Wp26_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM26 - DATA1"]
    #[inline(always)]
    pub fn wp_26_1(&mut self) -> Wp26_1W<'_, LfssSpmwprot6Spec> {
        Wp26_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM26- DATA2"]
    #[inline(always)]
    pub fn wp_26_2(&mut self) -> Wp26_2W<'_, LfssSpmwprot6Spec> {
        Wp26_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM26 - DATA3"]
    #[inline(always)]
    pub fn wp_26_3(&mut self) -> Wp26_3W<'_, LfssSpmwprot6Spec> {
        Wp26_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM27 - DATA0"]
    #[inline(always)]
    pub fn wp_27_0(&mut self) -> Wp27_0W<'_, LfssSpmwprot6Spec> {
        Wp27_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM27 - DATA1"]
    #[inline(always)]
    pub fn wp_27_1(&mut self) -> Wp27_1W<'_, LfssSpmwprot6Spec> {
        Wp27_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM27 - DATA2"]
    #[inline(always)]
    pub fn wp_27_2(&mut self) -> Wp27_2W<'_, LfssSpmwprot6Spec> {
        Wp27_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM27 - DATA3"]
    #[inline(always)]
    pub fn wp_27_3(&mut self) -> Wp27_3W<'_, LfssSpmwprot6Spec> {
        Wp27_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot6Spec;
impl crate::RegisterSpec for LfssSpmwprot6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot6::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot6Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot6::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT6 to value 0"]
impl crate::Resettable for LfssSpmwprot6Spec {}
