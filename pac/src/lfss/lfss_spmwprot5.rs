#[doc = "Register `LFSS_SPMWPROT5` reader"]
pub type R = crate::R<LfssSpmwprot5Spec>;
#[doc = "Register `LFSS_SPMWPROT5` writer"]
pub type W = crate::W<LfssSpmwprot5Spec>;
#[doc = "write protect SPMEM20 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp20_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp20_0> for bool {
    #[inline(always)]
    fn from(variant: Wp20_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_20_0` reader - write protect SPMEM20 - DATA0"]
pub type Wp20_0R = crate::BitReader<Wp20_0>;
impl Wp20_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp20_0 {
        match self.bits {
            false => Wp20_0::Readwrite,
            true => Wp20_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp20_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp20_0::Readonly
    }
}
#[doc = "Field `WP_20_0` writer - write protect SPMEM20 - DATA0"]
pub type Wp20_0W<'a, REG> = crate::BitWriter<'a, REG, Wp20_0>;
impl<'a, REG> Wp20_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_0::Readonly)
    }
}
#[doc = "write protect SPMEM20 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp20_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp20_1> for bool {
    #[inline(always)]
    fn from(variant: Wp20_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_20_1` reader - write protect SPMEM20 - DATA1"]
pub type Wp20_1R = crate::BitReader<Wp20_1>;
impl Wp20_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp20_1 {
        match self.bits {
            false => Wp20_1::Readwrite,
            true => Wp20_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp20_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp20_1::Readonly
    }
}
#[doc = "Field `WP_20_1` writer - write protect SPMEM20 - DATA1"]
pub type Wp20_1W<'a, REG> = crate::BitWriter<'a, REG, Wp20_1>;
impl<'a, REG> Wp20_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_1::Readonly)
    }
}
#[doc = "write protect SPMEM20 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp20_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp20_2> for bool {
    #[inline(always)]
    fn from(variant: Wp20_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_20_2` reader - write protect SPMEM20 - DATA2"]
pub type Wp20_2R = crate::BitReader<Wp20_2>;
impl Wp20_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp20_2 {
        match self.bits {
            false => Wp20_2::Readwrite,
            true => Wp20_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp20_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp20_2::Readonly
    }
}
#[doc = "Field `WP_20_2` writer - write protect SPMEM20 - DATA2"]
pub type Wp20_2W<'a, REG> = crate::BitWriter<'a, REG, Wp20_2>;
impl<'a, REG> Wp20_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_2::Readonly)
    }
}
#[doc = "write protect SPMEM20 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp20_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp20_3> for bool {
    #[inline(always)]
    fn from(variant: Wp20_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_20_3` reader - write protect SPMEM20 - DATA3"]
pub type Wp20_3R = crate::BitReader<Wp20_3>;
impl Wp20_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp20_3 {
        match self.bits {
            false => Wp20_3::Readwrite,
            true => Wp20_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp20_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp20_3::Readonly
    }
}
#[doc = "Field `WP_20_3` writer - write protect SPMEM20 - DATA3"]
pub type Wp20_3W<'a, REG> = crate::BitWriter<'a, REG, Wp20_3>;
impl<'a, REG> Wp20_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp20_3::Readonly)
    }
}
#[doc = "write protect SPMEM21 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp21_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp21_0> for bool {
    #[inline(always)]
    fn from(variant: Wp21_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_21_0` reader - write protect SPMEM21 - DATA0"]
pub type Wp21_0R = crate::BitReader<Wp21_0>;
impl Wp21_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp21_0 {
        match self.bits {
            false => Wp21_0::Readwrite,
            true => Wp21_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp21_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp21_0::Readonly
    }
}
#[doc = "Field `WP_21_0` writer - write protect SPMEM21 - DATA0"]
pub type Wp21_0W<'a, REG> = crate::BitWriter<'a, REG, Wp21_0>;
impl<'a, REG> Wp21_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_0::Readonly)
    }
}
#[doc = "write protect SPMEM21 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp21_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp21_1> for bool {
    #[inline(always)]
    fn from(variant: Wp21_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_21_1` reader - write protect SPMEM21 - DATA1"]
pub type Wp21_1R = crate::BitReader<Wp21_1>;
impl Wp21_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp21_1 {
        match self.bits {
            false => Wp21_1::Readwrite,
            true => Wp21_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp21_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp21_1::Readonly
    }
}
#[doc = "Field `WP_21_1` writer - write protect SPMEM21 - DATA1"]
pub type Wp21_1W<'a, REG> = crate::BitWriter<'a, REG, Wp21_1>;
impl<'a, REG> Wp21_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_1::Readonly)
    }
}
#[doc = "write protect SPMEM21 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp21_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp21_2> for bool {
    #[inline(always)]
    fn from(variant: Wp21_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_21_2` reader - write protect SPMEM21 - DATA2"]
pub type Wp21_2R = crate::BitReader<Wp21_2>;
impl Wp21_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp21_2 {
        match self.bits {
            false => Wp21_2::Readwrite,
            true => Wp21_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp21_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp21_2::Readonly
    }
}
#[doc = "Field `WP_21_2` writer - write protect SPMEM21 - DATA2"]
pub type Wp21_2W<'a, REG> = crate::BitWriter<'a, REG, Wp21_2>;
impl<'a, REG> Wp21_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_2::Readonly)
    }
}
#[doc = "write protect SPMEM21 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp21_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp21_3> for bool {
    #[inline(always)]
    fn from(variant: Wp21_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_21_3` reader - write protect SPMEM21 - DATA3"]
pub type Wp21_3R = crate::BitReader<Wp21_3>;
impl Wp21_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp21_3 {
        match self.bits {
            false => Wp21_3::Readwrite,
            true => Wp21_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp21_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp21_3::Readonly
    }
}
#[doc = "Field `WP_21_3` writer - write protect SPMEM21 - DATA3"]
pub type Wp21_3W<'a, REG> = crate::BitWriter<'a, REG, Wp21_3>;
impl<'a, REG> Wp21_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp21_3::Readonly)
    }
}
#[doc = "write protect SPMEM22 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp22_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp22_0> for bool {
    #[inline(always)]
    fn from(variant: Wp22_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_22_0` reader - write protect SPMEM22 - DATA0"]
pub type Wp22_0R = crate::BitReader<Wp22_0>;
impl Wp22_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp22_0 {
        match self.bits {
            false => Wp22_0::Readwrite,
            true => Wp22_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp22_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp22_0::Readonly
    }
}
#[doc = "Field `WP_22_0` writer - write protect SPMEM22 - DATA0"]
pub type Wp22_0W<'a, REG> = crate::BitWriter<'a, REG, Wp22_0>;
impl<'a, REG> Wp22_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_0::Readonly)
    }
}
#[doc = "write protect SPMEM22 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp22_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp22_1> for bool {
    #[inline(always)]
    fn from(variant: Wp22_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_22_1` reader - write protect SPMEM22 - DATA1"]
pub type Wp22_1R = crate::BitReader<Wp22_1>;
impl Wp22_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp22_1 {
        match self.bits {
            false => Wp22_1::Readwrite,
            true => Wp22_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp22_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp22_1::Readonly
    }
}
#[doc = "Field `WP_22_1` writer - write protect SPMEM22 - DATA1"]
pub type Wp22_1W<'a, REG> = crate::BitWriter<'a, REG, Wp22_1>;
impl<'a, REG> Wp22_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_1::Readonly)
    }
}
#[doc = "write protect SPMEM22- DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp22_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp22_2> for bool {
    #[inline(always)]
    fn from(variant: Wp22_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_22_2` reader - write protect SPMEM22- DATA2"]
pub type Wp22_2R = crate::BitReader<Wp22_2>;
impl Wp22_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp22_2 {
        match self.bits {
            false => Wp22_2::Readwrite,
            true => Wp22_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp22_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp22_2::Readonly
    }
}
#[doc = "Field `WP_22_2` writer - write protect SPMEM22- DATA2"]
pub type Wp22_2W<'a, REG> = crate::BitWriter<'a, REG, Wp22_2>;
impl<'a, REG> Wp22_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_2::Readonly)
    }
}
#[doc = "write protect SPMEM22 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp22_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp22_3> for bool {
    #[inline(always)]
    fn from(variant: Wp22_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_22_3` reader - write protect SPMEM22 - DATA3"]
pub type Wp22_3R = crate::BitReader<Wp22_3>;
impl Wp22_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp22_3 {
        match self.bits {
            false => Wp22_3::Readwrite,
            true => Wp22_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp22_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp22_3::Readonly
    }
}
#[doc = "Field `WP_22_3` writer - write protect SPMEM22 - DATA3"]
pub type Wp22_3W<'a, REG> = crate::BitWriter<'a, REG, Wp22_3>;
impl<'a, REG> Wp22_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp22_3::Readonly)
    }
}
#[doc = "write protect SPMEM23 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp23_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp23_0> for bool {
    #[inline(always)]
    fn from(variant: Wp23_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_23_0` reader - write protect SPMEM23 - DATA0"]
pub type Wp23_0R = crate::BitReader<Wp23_0>;
impl Wp23_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp23_0 {
        match self.bits {
            false => Wp23_0::Readwrite,
            true => Wp23_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp23_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp23_0::Readonly
    }
}
#[doc = "Field `WP_23_0` writer - write protect SPMEM23 - DATA0"]
pub type Wp23_0W<'a, REG> = crate::BitWriter<'a, REG, Wp23_0>;
impl<'a, REG> Wp23_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_0::Readonly)
    }
}
#[doc = "write protect SPMEM23 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp23_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp23_1> for bool {
    #[inline(always)]
    fn from(variant: Wp23_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_23_1` reader - write protect SPMEM23 - DATA1"]
pub type Wp23_1R = crate::BitReader<Wp23_1>;
impl Wp23_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp23_1 {
        match self.bits {
            false => Wp23_1::Readwrite,
            true => Wp23_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp23_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp23_1::Readonly
    }
}
#[doc = "Field `WP_23_1` writer - write protect SPMEM23 - DATA1"]
pub type Wp23_1W<'a, REG> = crate::BitWriter<'a, REG, Wp23_1>;
impl<'a, REG> Wp23_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_1::Readonly)
    }
}
#[doc = "write protect SPMEM23 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp23_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp23_2> for bool {
    #[inline(always)]
    fn from(variant: Wp23_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_23_2` reader - write protect SPMEM23 - DATA2"]
pub type Wp23_2R = crate::BitReader<Wp23_2>;
impl Wp23_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp23_2 {
        match self.bits {
            false => Wp23_2::Readwrite,
            true => Wp23_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp23_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp23_2::Readonly
    }
}
#[doc = "Field `WP_23_2` writer - write protect SPMEM23 - DATA2"]
pub type Wp23_2W<'a, REG> = crate::BitWriter<'a, REG, Wp23_2>;
impl<'a, REG> Wp23_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_2::Readonly)
    }
}
#[doc = "write protect SPMEM23 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp23_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp23_3> for bool {
    #[inline(always)]
    fn from(variant: Wp23_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_23_3` reader - write protect SPMEM23 - DATA3"]
pub type Wp23_3R = crate::BitReader<Wp23_3>;
impl Wp23_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp23_3 {
        match self.bits {
            false => Wp23_3::Readwrite,
            true => Wp23_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp23_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp23_3::Readonly
    }
}
#[doc = "Field `WP_23_3` writer - write protect SPMEM23 - DATA3"]
pub type Wp23_3W<'a, REG> = crate::BitWriter<'a, REG, Wp23_3>;
impl<'a, REG> Wp23_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp23_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM20 - DATA0"]
    #[inline(always)]
    pub fn wp_20_0(&self) -> Wp20_0R {
        Wp20_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM20 - DATA1"]
    #[inline(always)]
    pub fn wp_20_1(&self) -> Wp20_1R {
        Wp20_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM20 - DATA2"]
    #[inline(always)]
    pub fn wp_20_2(&self) -> Wp20_2R {
        Wp20_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM20 - DATA3"]
    #[inline(always)]
    pub fn wp_20_3(&self) -> Wp20_3R {
        Wp20_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM21 - DATA0"]
    #[inline(always)]
    pub fn wp_21_0(&self) -> Wp21_0R {
        Wp21_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM21 - DATA1"]
    #[inline(always)]
    pub fn wp_21_1(&self) -> Wp21_1R {
        Wp21_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM21 - DATA2"]
    #[inline(always)]
    pub fn wp_21_2(&self) -> Wp21_2R {
        Wp21_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM21 - DATA3"]
    #[inline(always)]
    pub fn wp_21_3(&self) -> Wp21_3R {
        Wp21_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM22 - DATA0"]
    #[inline(always)]
    pub fn wp_22_0(&self) -> Wp22_0R {
        Wp22_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM22 - DATA1"]
    #[inline(always)]
    pub fn wp_22_1(&self) -> Wp22_1R {
        Wp22_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM22- DATA2"]
    #[inline(always)]
    pub fn wp_22_2(&self) -> Wp22_2R {
        Wp22_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM22 - DATA3"]
    #[inline(always)]
    pub fn wp_22_3(&self) -> Wp22_3R {
        Wp22_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM23 - DATA0"]
    #[inline(always)]
    pub fn wp_23_0(&self) -> Wp23_0R {
        Wp23_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM23 - DATA1"]
    #[inline(always)]
    pub fn wp_23_1(&self) -> Wp23_1R {
        Wp23_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM23 - DATA2"]
    #[inline(always)]
    pub fn wp_23_2(&self) -> Wp23_2R {
        Wp23_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM23 - DATA3"]
    #[inline(always)]
    pub fn wp_23_3(&self) -> Wp23_3R {
        Wp23_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM20 - DATA0"]
    #[inline(always)]
    pub fn wp_20_0(&mut self) -> Wp20_0W<'_, LfssSpmwprot5Spec> {
        Wp20_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM20 - DATA1"]
    #[inline(always)]
    pub fn wp_20_1(&mut self) -> Wp20_1W<'_, LfssSpmwprot5Spec> {
        Wp20_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM20 - DATA2"]
    #[inline(always)]
    pub fn wp_20_2(&mut self) -> Wp20_2W<'_, LfssSpmwprot5Spec> {
        Wp20_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM20 - DATA3"]
    #[inline(always)]
    pub fn wp_20_3(&mut self) -> Wp20_3W<'_, LfssSpmwprot5Spec> {
        Wp20_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM21 - DATA0"]
    #[inline(always)]
    pub fn wp_21_0(&mut self) -> Wp21_0W<'_, LfssSpmwprot5Spec> {
        Wp21_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM21 - DATA1"]
    #[inline(always)]
    pub fn wp_21_1(&mut self) -> Wp21_1W<'_, LfssSpmwprot5Spec> {
        Wp21_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM21 - DATA2"]
    #[inline(always)]
    pub fn wp_21_2(&mut self) -> Wp21_2W<'_, LfssSpmwprot5Spec> {
        Wp21_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM21 - DATA3"]
    #[inline(always)]
    pub fn wp_21_3(&mut self) -> Wp21_3W<'_, LfssSpmwprot5Spec> {
        Wp21_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM22 - DATA0"]
    #[inline(always)]
    pub fn wp_22_0(&mut self) -> Wp22_0W<'_, LfssSpmwprot5Spec> {
        Wp22_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM22 - DATA1"]
    #[inline(always)]
    pub fn wp_22_1(&mut self) -> Wp22_1W<'_, LfssSpmwprot5Spec> {
        Wp22_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM22- DATA2"]
    #[inline(always)]
    pub fn wp_22_2(&mut self) -> Wp22_2W<'_, LfssSpmwprot5Spec> {
        Wp22_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM22 - DATA3"]
    #[inline(always)]
    pub fn wp_22_3(&mut self) -> Wp22_3W<'_, LfssSpmwprot5Spec> {
        Wp22_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM23 - DATA0"]
    #[inline(always)]
    pub fn wp_23_0(&mut self) -> Wp23_0W<'_, LfssSpmwprot5Spec> {
        Wp23_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM23 - DATA1"]
    #[inline(always)]
    pub fn wp_23_1(&mut self) -> Wp23_1W<'_, LfssSpmwprot5Spec> {
        Wp23_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM23 - DATA2"]
    #[inline(always)]
    pub fn wp_23_2(&mut self) -> Wp23_2W<'_, LfssSpmwprot5Spec> {
        Wp23_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM23 - DATA3"]
    #[inline(always)]
    pub fn wp_23_3(&mut self) -> Wp23_3W<'_, LfssSpmwprot5Spec> {
        Wp23_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot5Spec;
impl crate::RegisterSpec for LfssSpmwprot5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot5::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot5Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot5::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT5 to value 0"]
impl crate::Resettable for LfssSpmwprot5Spec {}
