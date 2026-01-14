#[doc = "Register `LFSS_SPMWPROT2` reader"]
pub type R = crate::R<LfssSpmwprot2Spec>;
#[doc = "Register `LFSS_SPMWPROT2` writer"]
pub type W = crate::W<LfssSpmwprot2Spec>;
#[doc = "write protect SPMEM8 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp8_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp8_0> for bool {
    #[inline(always)]
    fn from(variant: Wp8_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_8_0` reader - write protect SPMEM8 - DATA0"]
pub type Wp8_0R = crate::BitReader<Wp8_0>;
impl Wp8_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp8_0 {
        match self.bits {
            false => Wp8_0::Readwrite,
            true => Wp8_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp8_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp8_0::Readonly
    }
}
#[doc = "Field `WP_8_0` writer - write protect SPMEM8 - DATA0"]
pub type Wp8_0W<'a, REG> = crate::BitWriter<'a, REG, Wp8_0>;
impl<'a, REG> Wp8_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_0::Readonly)
    }
}
#[doc = "write protect SPMEM8 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp8_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp8_1> for bool {
    #[inline(always)]
    fn from(variant: Wp8_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_8_1` reader - write protect SPMEM8 - DATA1"]
pub type Wp8_1R = crate::BitReader<Wp8_1>;
impl Wp8_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp8_1 {
        match self.bits {
            false => Wp8_1::Readwrite,
            true => Wp8_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp8_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp8_1::Readonly
    }
}
#[doc = "Field `WP_8_1` writer - write protect SPMEM8 - DATA1"]
pub type Wp8_1W<'a, REG> = crate::BitWriter<'a, REG, Wp8_1>;
impl<'a, REG> Wp8_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_1::Readonly)
    }
}
#[doc = "write protect SPMEM8 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp8_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp8_2> for bool {
    #[inline(always)]
    fn from(variant: Wp8_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_8_2` reader - write protect SPMEM8 - DATA2"]
pub type Wp8_2R = crate::BitReader<Wp8_2>;
impl Wp8_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp8_2 {
        match self.bits {
            false => Wp8_2::Readwrite,
            true => Wp8_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp8_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp8_2::Readonly
    }
}
#[doc = "Field `WP_8_2` writer - write protect SPMEM8 - DATA2"]
pub type Wp8_2W<'a, REG> = crate::BitWriter<'a, REG, Wp8_2>;
impl<'a, REG> Wp8_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_2::Readonly)
    }
}
#[doc = "write protect SPMEM8 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp8_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp8_3> for bool {
    #[inline(always)]
    fn from(variant: Wp8_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_8_3` reader - write protect SPMEM8 - DATA3"]
pub type Wp8_3R = crate::BitReader<Wp8_3>;
impl Wp8_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp8_3 {
        match self.bits {
            false => Wp8_3::Readwrite,
            true => Wp8_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp8_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp8_3::Readonly
    }
}
#[doc = "Field `WP_8_3` writer - write protect SPMEM8 - DATA3"]
pub type Wp8_3W<'a, REG> = crate::BitWriter<'a, REG, Wp8_3>;
impl<'a, REG> Wp8_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp8_3::Readonly)
    }
}
#[doc = "write protect SPMEM9 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp9_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp9_0> for bool {
    #[inline(always)]
    fn from(variant: Wp9_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_9_0` reader - write protect SPMEM9 - DATA0"]
pub type Wp9_0R = crate::BitReader<Wp9_0>;
impl Wp9_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp9_0 {
        match self.bits {
            false => Wp9_0::Readwrite,
            true => Wp9_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp9_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp9_0::Readonly
    }
}
#[doc = "Field `WP_9_0` writer - write protect SPMEM9 - DATA0"]
pub type Wp9_0W<'a, REG> = crate::BitWriter<'a, REG, Wp9_0>;
impl<'a, REG> Wp9_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_0::Readonly)
    }
}
#[doc = "write protect SPMEM9 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp9_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp9_1> for bool {
    #[inline(always)]
    fn from(variant: Wp9_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_9_1` reader - write protect SPMEM9 - DATA1"]
pub type Wp9_1R = crate::BitReader<Wp9_1>;
impl Wp9_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp9_1 {
        match self.bits {
            false => Wp9_1::Readwrite,
            true => Wp9_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp9_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp9_1::Readonly
    }
}
#[doc = "Field `WP_9_1` writer - write protect SPMEM9 - DATA1"]
pub type Wp9_1W<'a, REG> = crate::BitWriter<'a, REG, Wp9_1>;
impl<'a, REG> Wp9_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_1::Readonly)
    }
}
#[doc = "write protect SPMEM9 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp9_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp9_2> for bool {
    #[inline(always)]
    fn from(variant: Wp9_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_9_2` reader - write protect SPMEM9 - DATA2"]
pub type Wp9_2R = crate::BitReader<Wp9_2>;
impl Wp9_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp9_2 {
        match self.bits {
            false => Wp9_2::Readwrite,
            true => Wp9_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp9_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp9_2::Readonly
    }
}
#[doc = "Field `WP_9_2` writer - write protect SPMEM9 - DATA2"]
pub type Wp9_2W<'a, REG> = crate::BitWriter<'a, REG, Wp9_2>;
impl<'a, REG> Wp9_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_2::Readonly)
    }
}
#[doc = "write protect SPMEM9 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp9_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp9_3> for bool {
    #[inline(always)]
    fn from(variant: Wp9_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_9_3` reader - write protect SPMEM9 - DATA3"]
pub type Wp9_3R = crate::BitReader<Wp9_3>;
impl Wp9_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp9_3 {
        match self.bits {
            false => Wp9_3::Readwrite,
            true => Wp9_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp9_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp9_3::Readonly
    }
}
#[doc = "Field `WP_9_3` writer - write protect SPMEM9 - DATA3"]
pub type Wp9_3W<'a, REG> = crate::BitWriter<'a, REG, Wp9_3>;
impl<'a, REG> Wp9_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp9_3::Readonly)
    }
}
#[doc = "write protect SPMEM10 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp10_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp10_0> for bool {
    #[inline(always)]
    fn from(variant: Wp10_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_10_0` reader - write protect SPMEM10 - DATA0"]
pub type Wp10_0R = crate::BitReader<Wp10_0>;
impl Wp10_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp10_0 {
        match self.bits {
            false => Wp10_0::Readwrite,
            true => Wp10_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp10_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp10_0::Readonly
    }
}
#[doc = "Field `WP_10_0` writer - write protect SPMEM10 - DATA0"]
pub type Wp10_0W<'a, REG> = crate::BitWriter<'a, REG, Wp10_0>;
impl<'a, REG> Wp10_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_0::Readonly)
    }
}
#[doc = "write protect SPMEM10 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp10_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp10_1> for bool {
    #[inline(always)]
    fn from(variant: Wp10_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_10_1` reader - write protect SPMEM10 - DATA1"]
pub type Wp10_1R = crate::BitReader<Wp10_1>;
impl Wp10_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp10_1 {
        match self.bits {
            false => Wp10_1::Readwrite,
            true => Wp10_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp10_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp10_1::Readonly
    }
}
#[doc = "Field `WP_10_1` writer - write protect SPMEM10 - DATA1"]
pub type Wp10_1W<'a, REG> = crate::BitWriter<'a, REG, Wp10_1>;
impl<'a, REG> Wp10_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_1::Readonly)
    }
}
#[doc = "write protect SPMEM610- DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp10_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp10_2> for bool {
    #[inline(always)]
    fn from(variant: Wp10_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_10_2` reader - write protect SPMEM610- DATA2"]
pub type Wp10_2R = crate::BitReader<Wp10_2>;
impl Wp10_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp10_2 {
        match self.bits {
            false => Wp10_2::Readwrite,
            true => Wp10_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp10_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp10_2::Readonly
    }
}
#[doc = "Field `WP_10_2` writer - write protect SPMEM610- DATA2"]
pub type Wp10_2W<'a, REG> = crate::BitWriter<'a, REG, Wp10_2>;
impl<'a, REG> Wp10_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_2::Readonly)
    }
}
#[doc = "write protect SPMEM10 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp10_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp10_3> for bool {
    #[inline(always)]
    fn from(variant: Wp10_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_10_3` reader - write protect SPMEM10 - DATA3"]
pub type Wp10_3R = crate::BitReader<Wp10_3>;
impl Wp10_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp10_3 {
        match self.bits {
            false => Wp10_3::Readwrite,
            true => Wp10_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp10_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp10_3::Readonly
    }
}
#[doc = "Field `WP_10_3` writer - write protect SPMEM10 - DATA3"]
pub type Wp10_3W<'a, REG> = crate::BitWriter<'a, REG, Wp10_3>;
impl<'a, REG> Wp10_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp10_3::Readonly)
    }
}
#[doc = "write protect SPMEM11 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp11_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp11_0> for bool {
    #[inline(always)]
    fn from(variant: Wp11_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_11_0` reader - write protect SPMEM11 - DATA0"]
pub type Wp11_0R = crate::BitReader<Wp11_0>;
impl Wp11_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp11_0 {
        match self.bits {
            false => Wp11_0::Readwrite,
            true => Wp11_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp11_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp11_0::Readonly
    }
}
#[doc = "Field `WP_11_0` writer - write protect SPMEM11 - DATA0"]
pub type Wp11_0W<'a, REG> = crate::BitWriter<'a, REG, Wp11_0>;
impl<'a, REG> Wp11_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_0::Readonly)
    }
}
#[doc = "write protect SPMEM11 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp11_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp11_1> for bool {
    #[inline(always)]
    fn from(variant: Wp11_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_11_1` reader - write protect SPMEM11 - DATA1"]
pub type Wp11_1R = crate::BitReader<Wp11_1>;
impl Wp11_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp11_1 {
        match self.bits {
            false => Wp11_1::Readwrite,
            true => Wp11_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp11_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp11_1::Readonly
    }
}
#[doc = "Field `WP_11_1` writer - write protect SPMEM11 - DATA1"]
pub type Wp11_1W<'a, REG> = crate::BitWriter<'a, REG, Wp11_1>;
impl<'a, REG> Wp11_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_1::Readonly)
    }
}
#[doc = "write protect SPMEM11 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp11_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp11_2> for bool {
    #[inline(always)]
    fn from(variant: Wp11_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_11_2` reader - write protect SPMEM11 - DATA2"]
pub type Wp11_2R = crate::BitReader<Wp11_2>;
impl Wp11_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp11_2 {
        match self.bits {
            false => Wp11_2::Readwrite,
            true => Wp11_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp11_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp11_2::Readonly
    }
}
#[doc = "Field `WP_11_2` writer - write protect SPMEM11 - DATA2"]
pub type Wp11_2W<'a, REG> = crate::BitWriter<'a, REG, Wp11_2>;
impl<'a, REG> Wp11_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_2::Readonly)
    }
}
#[doc = "write protect SPMEM11 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp11_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp11_3> for bool {
    #[inline(always)]
    fn from(variant: Wp11_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_11_3` reader - write protect SPMEM11 - DATA3"]
pub type Wp11_3R = crate::BitReader<Wp11_3>;
impl Wp11_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp11_3 {
        match self.bits {
            false => Wp11_3::Readwrite,
            true => Wp11_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp11_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp11_3::Readonly
    }
}
#[doc = "Field `WP_11_3` writer - write protect SPMEM11 - DATA3"]
pub type Wp11_3W<'a, REG> = crate::BitWriter<'a, REG, Wp11_3>;
impl<'a, REG> Wp11_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp11_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM8 - DATA0"]
    #[inline(always)]
    pub fn wp_8_0(&self) -> Wp8_0R {
        Wp8_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM8 - DATA1"]
    #[inline(always)]
    pub fn wp_8_1(&self) -> Wp8_1R {
        Wp8_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM8 - DATA2"]
    #[inline(always)]
    pub fn wp_8_2(&self) -> Wp8_2R {
        Wp8_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM8 - DATA3"]
    #[inline(always)]
    pub fn wp_8_3(&self) -> Wp8_3R {
        Wp8_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM9 - DATA0"]
    #[inline(always)]
    pub fn wp_9_0(&self) -> Wp9_0R {
        Wp9_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM9 - DATA1"]
    #[inline(always)]
    pub fn wp_9_1(&self) -> Wp9_1R {
        Wp9_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM9 - DATA2"]
    #[inline(always)]
    pub fn wp_9_2(&self) -> Wp9_2R {
        Wp9_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM9 - DATA3"]
    #[inline(always)]
    pub fn wp_9_3(&self) -> Wp9_3R {
        Wp9_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM10 - DATA0"]
    #[inline(always)]
    pub fn wp_10_0(&self) -> Wp10_0R {
        Wp10_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM10 - DATA1"]
    #[inline(always)]
    pub fn wp_10_1(&self) -> Wp10_1R {
        Wp10_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM610- DATA2"]
    #[inline(always)]
    pub fn wp_10_2(&self) -> Wp10_2R {
        Wp10_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM10 - DATA3"]
    #[inline(always)]
    pub fn wp_10_3(&self) -> Wp10_3R {
        Wp10_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM11 - DATA0"]
    #[inline(always)]
    pub fn wp_11_0(&self) -> Wp11_0R {
        Wp11_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM11 - DATA1"]
    #[inline(always)]
    pub fn wp_11_1(&self) -> Wp11_1R {
        Wp11_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM11 - DATA2"]
    #[inline(always)]
    pub fn wp_11_2(&self) -> Wp11_2R {
        Wp11_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM11 - DATA3"]
    #[inline(always)]
    pub fn wp_11_3(&self) -> Wp11_3R {
        Wp11_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM8 - DATA0"]
    #[inline(always)]
    pub fn wp_8_0(&mut self) -> Wp8_0W<'_, LfssSpmwprot2Spec> {
        Wp8_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM8 - DATA1"]
    #[inline(always)]
    pub fn wp_8_1(&mut self) -> Wp8_1W<'_, LfssSpmwprot2Spec> {
        Wp8_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM8 - DATA2"]
    #[inline(always)]
    pub fn wp_8_2(&mut self) -> Wp8_2W<'_, LfssSpmwprot2Spec> {
        Wp8_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM8 - DATA3"]
    #[inline(always)]
    pub fn wp_8_3(&mut self) -> Wp8_3W<'_, LfssSpmwprot2Spec> {
        Wp8_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM9 - DATA0"]
    #[inline(always)]
    pub fn wp_9_0(&mut self) -> Wp9_0W<'_, LfssSpmwprot2Spec> {
        Wp9_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM9 - DATA1"]
    #[inline(always)]
    pub fn wp_9_1(&mut self) -> Wp9_1W<'_, LfssSpmwprot2Spec> {
        Wp9_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM9 - DATA2"]
    #[inline(always)]
    pub fn wp_9_2(&mut self) -> Wp9_2W<'_, LfssSpmwprot2Spec> {
        Wp9_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM9 - DATA3"]
    #[inline(always)]
    pub fn wp_9_3(&mut self) -> Wp9_3W<'_, LfssSpmwprot2Spec> {
        Wp9_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM10 - DATA0"]
    #[inline(always)]
    pub fn wp_10_0(&mut self) -> Wp10_0W<'_, LfssSpmwprot2Spec> {
        Wp10_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM10 - DATA1"]
    #[inline(always)]
    pub fn wp_10_1(&mut self) -> Wp10_1W<'_, LfssSpmwprot2Spec> {
        Wp10_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM610- DATA2"]
    #[inline(always)]
    pub fn wp_10_2(&mut self) -> Wp10_2W<'_, LfssSpmwprot2Spec> {
        Wp10_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM10 - DATA3"]
    #[inline(always)]
    pub fn wp_10_3(&mut self) -> Wp10_3W<'_, LfssSpmwprot2Spec> {
        Wp10_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM11 - DATA0"]
    #[inline(always)]
    pub fn wp_11_0(&mut self) -> Wp11_0W<'_, LfssSpmwprot2Spec> {
        Wp11_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM11 - DATA1"]
    #[inline(always)]
    pub fn wp_11_1(&mut self) -> Wp11_1W<'_, LfssSpmwprot2Spec> {
        Wp11_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM11 - DATA2"]
    #[inline(always)]
    pub fn wp_11_2(&mut self) -> Wp11_2W<'_, LfssSpmwprot2Spec> {
        Wp11_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM11 - DATA3"]
    #[inline(always)]
    pub fn wp_11_3(&mut self) -> Wp11_3W<'_, LfssSpmwprot2Spec> {
        Wp11_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot2Spec;
impl crate::RegisterSpec for LfssSpmwprot2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot2::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot2Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot2::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT2 to value 0"]
impl crate::Resettable for LfssSpmwprot2Spec {}
