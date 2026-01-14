#[doc = "Register `LFSS_SPMWPROT0` reader"]
pub type R = crate::R<LfssSpmwprot0Spec>;
#[doc = "Register `LFSS_SPMWPROT0` writer"]
pub type W = crate::W<LfssSpmwprot0Spec>;
#[doc = "write protect SPMEM0 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp0_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp0_0> for bool {
    #[inline(always)]
    fn from(variant: Wp0_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_0_0` reader - write protect SPMEM0 - DATA0"]
pub type Wp0_0R = crate::BitReader<Wp0_0>;
impl Wp0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp0_0 {
        match self.bits {
            false => Wp0_0::Readwrite,
            true => Wp0_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp0_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp0_0::Readonly
    }
}
#[doc = "Field `WP_0_0` writer - write protect SPMEM0 - DATA0"]
pub type Wp0_0W<'a, REG> = crate::BitWriter<'a, REG, Wp0_0>;
impl<'a, REG> Wp0_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_0::Readonly)
    }
}
#[doc = "write protect SPMEM0 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp0_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp0_1> for bool {
    #[inline(always)]
    fn from(variant: Wp0_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_0_1` reader - write protect SPMEM0 - DATA1"]
pub type Wp0_1R = crate::BitReader<Wp0_1>;
impl Wp0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp0_1 {
        match self.bits {
            false => Wp0_1::Readwrite,
            true => Wp0_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp0_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp0_1::Readonly
    }
}
#[doc = "Field `WP_0_1` writer - write protect SPMEM0 - DATA1"]
pub type Wp0_1W<'a, REG> = crate::BitWriter<'a, REG, Wp0_1>;
impl<'a, REG> Wp0_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_1::Readonly)
    }
}
#[doc = "write protect SPMEM0 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp0_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp0_2> for bool {
    #[inline(always)]
    fn from(variant: Wp0_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_0_2` reader - write protect SPMEM0 - DATA2"]
pub type Wp0_2R = crate::BitReader<Wp0_2>;
impl Wp0_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp0_2 {
        match self.bits {
            false => Wp0_2::Readwrite,
            true => Wp0_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp0_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp0_2::Readonly
    }
}
#[doc = "Field `WP_0_2` writer - write protect SPMEM0 - DATA2"]
pub type Wp0_2W<'a, REG> = crate::BitWriter<'a, REG, Wp0_2>;
impl<'a, REG> Wp0_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_2::Readonly)
    }
}
#[doc = "write protect SPMEM0 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp0_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp0_3> for bool {
    #[inline(always)]
    fn from(variant: Wp0_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_0_3` reader - write protect SPMEM0 - DATA3"]
pub type Wp0_3R = crate::BitReader<Wp0_3>;
impl Wp0_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp0_3 {
        match self.bits {
            false => Wp0_3::Readwrite,
            true => Wp0_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp0_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp0_3::Readonly
    }
}
#[doc = "Field `WP_0_3` writer - write protect SPMEM0 - DATA3"]
pub type Wp0_3W<'a, REG> = crate::BitWriter<'a, REG, Wp0_3>;
impl<'a, REG> Wp0_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp0_3::Readonly)
    }
}
#[doc = "write protect SPMEM1 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp1_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp1_0> for bool {
    #[inline(always)]
    fn from(variant: Wp1_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_1_0` reader - write protect SPMEM1 - DATA0"]
pub type Wp1_0R = crate::BitReader<Wp1_0>;
impl Wp1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp1_0 {
        match self.bits {
            false => Wp1_0::Readwrite,
            true => Wp1_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp1_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp1_0::Readonly
    }
}
#[doc = "Field `WP_1_0` writer - write protect SPMEM1 - DATA0"]
pub type Wp1_0W<'a, REG> = crate::BitWriter<'a, REG, Wp1_0>;
impl<'a, REG> Wp1_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_0::Readonly)
    }
}
#[doc = "write protect SPMEM1 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp1_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp1_1> for bool {
    #[inline(always)]
    fn from(variant: Wp1_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_1_1` reader - write protect SPMEM1 - DATA1"]
pub type Wp1_1R = crate::BitReader<Wp1_1>;
impl Wp1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp1_1 {
        match self.bits {
            false => Wp1_1::Readwrite,
            true => Wp1_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp1_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp1_1::Readonly
    }
}
#[doc = "Field `WP_1_1` writer - write protect SPMEM1 - DATA1"]
pub type Wp1_1W<'a, REG> = crate::BitWriter<'a, REG, Wp1_1>;
impl<'a, REG> Wp1_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_1::Readonly)
    }
}
#[doc = "write protect SPMEM1 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp1_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp1_2> for bool {
    #[inline(always)]
    fn from(variant: Wp1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_1_2` reader - write protect SPMEM1 - DATA2"]
pub type Wp1_2R = crate::BitReader<Wp1_2>;
impl Wp1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp1_2 {
        match self.bits {
            false => Wp1_2::Readwrite,
            true => Wp1_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp1_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp1_2::Readonly
    }
}
#[doc = "Field `WP_1_2` writer - write protect SPMEM1 - DATA2"]
pub type Wp1_2W<'a, REG> = crate::BitWriter<'a, REG, Wp1_2>;
impl<'a, REG> Wp1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_2::Readonly)
    }
}
#[doc = "write protect SPMEM1 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp1_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp1_3> for bool {
    #[inline(always)]
    fn from(variant: Wp1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_1_3` reader - write protect SPMEM1 - DATA3"]
pub type Wp1_3R = crate::BitReader<Wp1_3>;
impl Wp1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp1_3 {
        match self.bits {
            false => Wp1_3::Readwrite,
            true => Wp1_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp1_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp1_3::Readonly
    }
}
#[doc = "Field `WP_1_3` writer - write protect SPMEM1 - DATA3"]
pub type Wp1_3W<'a, REG> = crate::BitWriter<'a, REG, Wp1_3>;
impl<'a, REG> Wp1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1_3::Readonly)
    }
}
#[doc = "write protect SPMEM2 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp2_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp2_0> for bool {
    #[inline(always)]
    fn from(variant: Wp2_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_2_0` reader - write protect SPMEM2 - DATA0"]
pub type Wp2_0R = crate::BitReader<Wp2_0>;
impl Wp2_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp2_0 {
        match self.bits {
            false => Wp2_0::Readwrite,
            true => Wp2_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp2_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp2_0::Readonly
    }
}
#[doc = "Field `WP_2_0` writer - write protect SPMEM2 - DATA0"]
pub type Wp2_0W<'a, REG> = crate::BitWriter<'a, REG, Wp2_0>;
impl<'a, REG> Wp2_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_0::Readonly)
    }
}
#[doc = "write protect SPMEM2 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp2_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp2_1> for bool {
    #[inline(always)]
    fn from(variant: Wp2_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_2_1` reader - write protect SPMEM2 - DATA1"]
pub type Wp2_1R = crate::BitReader<Wp2_1>;
impl Wp2_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp2_1 {
        match self.bits {
            false => Wp2_1::Readwrite,
            true => Wp2_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp2_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp2_1::Readonly
    }
}
#[doc = "Field `WP_2_1` writer - write protect SPMEM2 - DATA1"]
pub type Wp2_1W<'a, REG> = crate::BitWriter<'a, REG, Wp2_1>;
impl<'a, REG> Wp2_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_1::Readonly)
    }
}
#[doc = "write protect SPMEM2 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp2_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp2_2> for bool {
    #[inline(always)]
    fn from(variant: Wp2_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_2_2` reader - write protect SPMEM2 - DATA2"]
pub type Wp2_2R = crate::BitReader<Wp2_2>;
impl Wp2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp2_2 {
        match self.bits {
            false => Wp2_2::Readwrite,
            true => Wp2_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp2_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp2_2::Readonly
    }
}
#[doc = "Field `WP_2_2` writer - write protect SPMEM2 - DATA2"]
pub type Wp2_2W<'a, REG> = crate::BitWriter<'a, REG, Wp2_2>;
impl<'a, REG> Wp2_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_2::Readonly)
    }
}
#[doc = "write protect SPMEM2 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp2_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp2_3> for bool {
    #[inline(always)]
    fn from(variant: Wp2_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_2_3` reader - write protect SPMEM2 - DATA3"]
pub type Wp2_3R = crate::BitReader<Wp2_3>;
impl Wp2_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp2_3 {
        match self.bits {
            false => Wp2_3::Readwrite,
            true => Wp2_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp2_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp2_3::Readonly
    }
}
#[doc = "Field `WP_2_3` writer - write protect SPMEM2 - DATA3"]
pub type Wp2_3W<'a, REG> = crate::BitWriter<'a, REG, Wp2_3>;
impl<'a, REG> Wp2_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2_3::Readonly)
    }
}
#[doc = "write protect SPMEM3 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp3_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp3_0> for bool {
    #[inline(always)]
    fn from(variant: Wp3_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_3_0` reader - write protect SPMEM3 - DATA0"]
pub type Wp3_0R = crate::BitReader<Wp3_0>;
impl Wp3_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp3_0 {
        match self.bits {
            false => Wp3_0::Readwrite,
            true => Wp3_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp3_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp3_0::Readonly
    }
}
#[doc = "Field `WP_3_0` writer - write protect SPMEM3 - DATA0"]
pub type Wp3_0W<'a, REG> = crate::BitWriter<'a, REG, Wp3_0>;
impl<'a, REG> Wp3_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_0::Readonly)
    }
}
#[doc = "write protect SPMEM3 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp3_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp3_1> for bool {
    #[inline(always)]
    fn from(variant: Wp3_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_3_1` reader - write protect SPMEM3 - DATA1"]
pub type Wp3_1R = crate::BitReader<Wp3_1>;
impl Wp3_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp3_1 {
        match self.bits {
            false => Wp3_1::Readwrite,
            true => Wp3_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp3_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp3_1::Readonly
    }
}
#[doc = "Field `WP_3_1` writer - write protect SPMEM3 - DATA1"]
pub type Wp3_1W<'a, REG> = crate::BitWriter<'a, REG, Wp3_1>;
impl<'a, REG> Wp3_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_1::Readonly)
    }
}
#[doc = "write protect SPMEM3 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp3_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp3_2> for bool {
    #[inline(always)]
    fn from(variant: Wp3_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_3_2` reader - write protect SPMEM3 - DATA2"]
pub type Wp3_2R = crate::BitReader<Wp3_2>;
impl Wp3_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp3_2 {
        match self.bits {
            false => Wp3_2::Readwrite,
            true => Wp3_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp3_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp3_2::Readonly
    }
}
#[doc = "Field `WP_3_2` writer - write protect SPMEM3 - DATA2"]
pub type Wp3_2W<'a, REG> = crate::BitWriter<'a, REG, Wp3_2>;
impl<'a, REG> Wp3_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_2::Readonly)
    }
}
#[doc = "write protect SPMEM3 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp3_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp3_3> for bool {
    #[inline(always)]
    fn from(variant: Wp3_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_3_3` reader - write protect SPMEM3 - DATA3"]
pub type Wp3_3R = crate::BitReader<Wp3_3>;
impl Wp3_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp3_3 {
        match self.bits {
            false => Wp3_3::Readwrite,
            true => Wp3_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp3_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp3_3::Readonly
    }
}
#[doc = "Field `WP_3_3` writer - write protect SPMEM3 - DATA3"]
pub type Wp3_3W<'a, REG> = crate::BitWriter<'a, REG, Wp3_3>;
impl<'a, REG> Wp3_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM0 - DATA0"]
    #[inline(always)]
    pub fn wp_0_0(&self) -> Wp0_0R {
        Wp0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM0 - DATA1"]
    #[inline(always)]
    pub fn wp_0_1(&self) -> Wp0_1R {
        Wp0_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM0 - DATA2"]
    #[inline(always)]
    pub fn wp_0_2(&self) -> Wp0_2R {
        Wp0_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM0 - DATA3"]
    #[inline(always)]
    pub fn wp_0_3(&self) -> Wp0_3R {
        Wp0_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM1 - DATA0"]
    #[inline(always)]
    pub fn wp_1_0(&self) -> Wp1_0R {
        Wp1_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM1 - DATA1"]
    #[inline(always)]
    pub fn wp_1_1(&self) -> Wp1_1R {
        Wp1_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM1 - DATA2"]
    #[inline(always)]
    pub fn wp_1_2(&self) -> Wp1_2R {
        Wp1_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM1 - DATA3"]
    #[inline(always)]
    pub fn wp_1_3(&self) -> Wp1_3R {
        Wp1_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM2 - DATA0"]
    #[inline(always)]
    pub fn wp_2_0(&self) -> Wp2_0R {
        Wp2_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM2 - DATA1"]
    #[inline(always)]
    pub fn wp_2_1(&self) -> Wp2_1R {
        Wp2_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM2 - DATA2"]
    #[inline(always)]
    pub fn wp_2_2(&self) -> Wp2_2R {
        Wp2_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM2 - DATA3"]
    #[inline(always)]
    pub fn wp_2_3(&self) -> Wp2_3R {
        Wp2_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM3 - DATA0"]
    #[inline(always)]
    pub fn wp_3_0(&self) -> Wp3_0R {
        Wp3_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM3 - DATA1"]
    #[inline(always)]
    pub fn wp_3_1(&self) -> Wp3_1R {
        Wp3_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM3 - DATA2"]
    #[inline(always)]
    pub fn wp_3_2(&self) -> Wp3_2R {
        Wp3_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM3 - DATA3"]
    #[inline(always)]
    pub fn wp_3_3(&self) -> Wp3_3R {
        Wp3_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM0 - DATA0"]
    #[inline(always)]
    pub fn wp_0_0(&mut self) -> Wp0_0W<'_, LfssSpmwprot0Spec> {
        Wp0_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM0 - DATA1"]
    #[inline(always)]
    pub fn wp_0_1(&mut self) -> Wp0_1W<'_, LfssSpmwprot0Spec> {
        Wp0_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM0 - DATA2"]
    #[inline(always)]
    pub fn wp_0_2(&mut self) -> Wp0_2W<'_, LfssSpmwprot0Spec> {
        Wp0_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM0 - DATA3"]
    #[inline(always)]
    pub fn wp_0_3(&mut self) -> Wp0_3W<'_, LfssSpmwprot0Spec> {
        Wp0_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM1 - DATA0"]
    #[inline(always)]
    pub fn wp_1_0(&mut self) -> Wp1_0W<'_, LfssSpmwprot0Spec> {
        Wp1_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM1 - DATA1"]
    #[inline(always)]
    pub fn wp_1_1(&mut self) -> Wp1_1W<'_, LfssSpmwprot0Spec> {
        Wp1_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM1 - DATA2"]
    #[inline(always)]
    pub fn wp_1_2(&mut self) -> Wp1_2W<'_, LfssSpmwprot0Spec> {
        Wp1_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM1 - DATA3"]
    #[inline(always)]
    pub fn wp_1_3(&mut self) -> Wp1_3W<'_, LfssSpmwprot0Spec> {
        Wp1_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM2 - DATA0"]
    #[inline(always)]
    pub fn wp_2_0(&mut self) -> Wp2_0W<'_, LfssSpmwprot0Spec> {
        Wp2_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM2 - DATA1"]
    #[inline(always)]
    pub fn wp_2_1(&mut self) -> Wp2_1W<'_, LfssSpmwprot0Spec> {
        Wp2_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM2 - DATA2"]
    #[inline(always)]
    pub fn wp_2_2(&mut self) -> Wp2_2W<'_, LfssSpmwprot0Spec> {
        Wp2_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM2 - DATA3"]
    #[inline(always)]
    pub fn wp_2_3(&mut self) -> Wp2_3W<'_, LfssSpmwprot0Spec> {
        Wp2_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM3 - DATA0"]
    #[inline(always)]
    pub fn wp_3_0(&mut self) -> Wp3_0W<'_, LfssSpmwprot0Spec> {
        Wp3_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM3 - DATA1"]
    #[inline(always)]
    pub fn wp_3_1(&mut self) -> Wp3_1W<'_, LfssSpmwprot0Spec> {
        Wp3_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM3 - DATA2"]
    #[inline(always)]
    pub fn wp_3_2(&mut self) -> Wp3_2W<'_, LfssSpmwprot0Spec> {
        Wp3_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM3 - DATA3"]
    #[inline(always)]
    pub fn wp_3_3(&mut self) -> Wp3_3W<'_, LfssSpmwprot0Spec> {
        Wp3_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot0Spec;
impl crate::RegisterSpec for LfssSpmwprot0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot0::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot0::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT0 to value 0"]
impl crate::Resettable for LfssSpmwprot0Spec {}
