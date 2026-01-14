#[doc = "Register `LFSS_SPMWPROT1` reader"]
pub type R = crate::R<LfssSpmwprot1Spec>;
#[doc = "Register `LFSS_SPMWPROT1` writer"]
pub type W = crate::W<LfssSpmwprot1Spec>;
#[doc = "write protect SPMEM4 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp4_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp4_0> for bool {
    #[inline(always)]
    fn from(variant: Wp4_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_4_0` reader - write protect SPMEM4 - DATA0"]
pub type Wp4_0R = crate::BitReader<Wp4_0>;
impl Wp4_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp4_0 {
        match self.bits {
            false => Wp4_0::Readwrite,
            true => Wp4_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp4_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp4_0::Readonly
    }
}
#[doc = "Field `WP_4_0` writer - write protect SPMEM4 - DATA0"]
pub type Wp4_0W<'a, REG> = crate::BitWriter<'a, REG, Wp4_0>;
impl<'a, REG> Wp4_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_0::Readonly)
    }
}
#[doc = "write protect SPMEM4 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp4_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp4_1> for bool {
    #[inline(always)]
    fn from(variant: Wp4_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_4_1` reader - write protect SPMEM4 - DATA1"]
pub type Wp4_1R = crate::BitReader<Wp4_1>;
impl Wp4_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp4_1 {
        match self.bits {
            false => Wp4_1::Readwrite,
            true => Wp4_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp4_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp4_1::Readonly
    }
}
#[doc = "Field `WP_4_1` writer - write protect SPMEM4 - DATA1"]
pub type Wp4_1W<'a, REG> = crate::BitWriter<'a, REG, Wp4_1>;
impl<'a, REG> Wp4_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_1::Readonly)
    }
}
#[doc = "write protect SPMEM4 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp4_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp4_2> for bool {
    #[inline(always)]
    fn from(variant: Wp4_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_4_2` reader - write protect SPMEM4 - DATA2"]
pub type Wp4_2R = crate::BitReader<Wp4_2>;
impl Wp4_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp4_2 {
        match self.bits {
            false => Wp4_2::Readwrite,
            true => Wp4_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp4_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp4_2::Readonly
    }
}
#[doc = "Field `WP_4_2` writer - write protect SPMEM4 - DATA2"]
pub type Wp4_2W<'a, REG> = crate::BitWriter<'a, REG, Wp4_2>;
impl<'a, REG> Wp4_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_2::Readonly)
    }
}
#[doc = "write protect SPMEM4 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp4_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp4_3> for bool {
    #[inline(always)]
    fn from(variant: Wp4_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_4_3` reader - write protect SPMEM4 - DATA3"]
pub type Wp4_3R = crate::BitReader<Wp4_3>;
impl Wp4_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp4_3 {
        match self.bits {
            false => Wp4_3::Readwrite,
            true => Wp4_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp4_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp4_3::Readonly
    }
}
#[doc = "Field `WP_4_3` writer - write protect SPMEM4 - DATA3"]
pub type Wp4_3W<'a, REG> = crate::BitWriter<'a, REG, Wp4_3>;
impl<'a, REG> Wp4_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4_3::Readonly)
    }
}
#[doc = "write protect SPMEM5 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp5_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp5_0> for bool {
    #[inline(always)]
    fn from(variant: Wp5_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_5_0` reader - write protect SPMEM5 - DATA0"]
pub type Wp5_0R = crate::BitReader<Wp5_0>;
impl Wp5_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp5_0 {
        match self.bits {
            false => Wp5_0::Readwrite,
            true => Wp5_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp5_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp5_0::Readonly
    }
}
#[doc = "Field `WP_5_0` writer - write protect SPMEM5 - DATA0"]
pub type Wp5_0W<'a, REG> = crate::BitWriter<'a, REG, Wp5_0>;
impl<'a, REG> Wp5_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_0::Readonly)
    }
}
#[doc = "write protect SPMEM5 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp5_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp5_1> for bool {
    #[inline(always)]
    fn from(variant: Wp5_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_5_1` reader - write protect SPMEM5 - DATA1"]
pub type Wp5_1R = crate::BitReader<Wp5_1>;
impl Wp5_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp5_1 {
        match self.bits {
            false => Wp5_1::Readwrite,
            true => Wp5_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp5_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp5_1::Readonly
    }
}
#[doc = "Field `WP_5_1` writer - write protect SPMEM5 - DATA1"]
pub type Wp5_1W<'a, REG> = crate::BitWriter<'a, REG, Wp5_1>;
impl<'a, REG> Wp5_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_1::Readonly)
    }
}
#[doc = "write protect SPMEM5 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp5_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp5_2> for bool {
    #[inline(always)]
    fn from(variant: Wp5_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_5_2` reader - write protect SPMEM5 - DATA2"]
pub type Wp5_2R = crate::BitReader<Wp5_2>;
impl Wp5_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp5_2 {
        match self.bits {
            false => Wp5_2::Readwrite,
            true => Wp5_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp5_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp5_2::Readonly
    }
}
#[doc = "Field `WP_5_2` writer - write protect SPMEM5 - DATA2"]
pub type Wp5_2W<'a, REG> = crate::BitWriter<'a, REG, Wp5_2>;
impl<'a, REG> Wp5_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_2::Readonly)
    }
}
#[doc = "write protect SPMEM5 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp5_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp5_3> for bool {
    #[inline(always)]
    fn from(variant: Wp5_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_5_3` reader - write protect SPMEM5 - DATA3"]
pub type Wp5_3R = crate::BitReader<Wp5_3>;
impl Wp5_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp5_3 {
        match self.bits {
            false => Wp5_3::Readwrite,
            true => Wp5_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp5_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp5_3::Readonly
    }
}
#[doc = "Field `WP_5_3` writer - write protect SPMEM5 - DATA3"]
pub type Wp5_3W<'a, REG> = crate::BitWriter<'a, REG, Wp5_3>;
impl<'a, REG> Wp5_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5_3::Readonly)
    }
}
#[doc = "write protect SPMEM6 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp6_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp6_0> for bool {
    #[inline(always)]
    fn from(variant: Wp6_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_6_0` reader - write protect SPMEM6 - DATA0"]
pub type Wp6_0R = crate::BitReader<Wp6_0>;
impl Wp6_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp6_0 {
        match self.bits {
            false => Wp6_0::Readwrite,
            true => Wp6_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp6_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp6_0::Readonly
    }
}
#[doc = "Field `WP_6_0` writer - write protect SPMEM6 - DATA0"]
pub type Wp6_0W<'a, REG> = crate::BitWriter<'a, REG, Wp6_0>;
impl<'a, REG> Wp6_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_0::Readonly)
    }
}
#[doc = "write protect SPMEM6 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp6_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp6_1> for bool {
    #[inline(always)]
    fn from(variant: Wp6_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_6_1` reader - write protect SPMEM6 - DATA1"]
pub type Wp6_1R = crate::BitReader<Wp6_1>;
impl Wp6_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp6_1 {
        match self.bits {
            false => Wp6_1::Readwrite,
            true => Wp6_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp6_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp6_1::Readonly
    }
}
#[doc = "Field `WP_6_1` writer - write protect SPMEM6 - DATA1"]
pub type Wp6_1W<'a, REG> = crate::BitWriter<'a, REG, Wp6_1>;
impl<'a, REG> Wp6_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_1::Readonly)
    }
}
#[doc = "write protect SPMEM6 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp6_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp6_2> for bool {
    #[inline(always)]
    fn from(variant: Wp6_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_6_2` reader - write protect SPMEM6 - DATA2"]
pub type Wp6_2R = crate::BitReader<Wp6_2>;
impl Wp6_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp6_2 {
        match self.bits {
            false => Wp6_2::Readwrite,
            true => Wp6_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp6_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp6_2::Readonly
    }
}
#[doc = "Field `WP_6_2` writer - write protect SPMEM6 - DATA2"]
pub type Wp6_2W<'a, REG> = crate::BitWriter<'a, REG, Wp6_2>;
impl<'a, REG> Wp6_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_2::Readonly)
    }
}
#[doc = "write protect SPMEM6 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp6_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp6_3> for bool {
    #[inline(always)]
    fn from(variant: Wp6_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_6_3` reader - write protect SPMEM6 - DATA3"]
pub type Wp6_3R = crate::BitReader<Wp6_3>;
impl Wp6_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp6_3 {
        match self.bits {
            false => Wp6_3::Readwrite,
            true => Wp6_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp6_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp6_3::Readonly
    }
}
#[doc = "Field `WP_6_3` writer - write protect SPMEM6 - DATA3"]
pub type Wp6_3W<'a, REG> = crate::BitWriter<'a, REG, Wp6_3>;
impl<'a, REG> Wp6_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp6_3::Readonly)
    }
}
#[doc = "write protect SPMEM7 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp7_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp7_0> for bool {
    #[inline(always)]
    fn from(variant: Wp7_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_7_0` reader - write protect SPMEM7 - DATA0"]
pub type Wp7_0R = crate::BitReader<Wp7_0>;
impl Wp7_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp7_0 {
        match self.bits {
            false => Wp7_0::Readwrite,
            true => Wp7_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp7_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp7_0::Readonly
    }
}
#[doc = "Field `WP_7_0` writer - write protect SPMEM7 - DATA0"]
pub type Wp7_0W<'a, REG> = crate::BitWriter<'a, REG, Wp7_0>;
impl<'a, REG> Wp7_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_0::Readonly)
    }
}
#[doc = "write protect SPMEM7 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp7_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp7_1> for bool {
    #[inline(always)]
    fn from(variant: Wp7_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_7_1` reader - write protect SPMEM7 - DATA1"]
pub type Wp7_1R = crate::BitReader<Wp7_1>;
impl Wp7_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp7_1 {
        match self.bits {
            false => Wp7_1::Readwrite,
            true => Wp7_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp7_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp7_1::Readonly
    }
}
#[doc = "Field `WP_7_1` writer - write protect SPMEM7 - DATA1"]
pub type Wp7_1W<'a, REG> = crate::BitWriter<'a, REG, Wp7_1>;
impl<'a, REG> Wp7_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_1::Readonly)
    }
}
#[doc = "write protect SPMEM7 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp7_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp7_2> for bool {
    #[inline(always)]
    fn from(variant: Wp7_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_7_2` reader - write protect SPMEM7 - DATA2"]
pub type Wp7_2R = crate::BitReader<Wp7_2>;
impl Wp7_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp7_2 {
        match self.bits {
            false => Wp7_2::Readwrite,
            true => Wp7_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp7_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp7_2::Readonly
    }
}
#[doc = "Field `WP_7_2` writer - write protect SPMEM7 - DATA2"]
pub type Wp7_2W<'a, REG> = crate::BitWriter<'a, REG, Wp7_2>;
impl<'a, REG> Wp7_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_2::Readonly)
    }
}
#[doc = "write protect SPMEM7 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp7_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp7_3> for bool {
    #[inline(always)]
    fn from(variant: Wp7_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_7_3` reader - write protect SPMEM7 - DATA3"]
pub type Wp7_3R = crate::BitReader<Wp7_3>;
impl Wp7_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp7_3 {
        match self.bits {
            false => Wp7_3::Readwrite,
            true => Wp7_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp7_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp7_3::Readonly
    }
}
#[doc = "Field `WP_7_3` writer - write protect SPMEM7 - DATA3"]
pub type Wp7_3W<'a, REG> = crate::BitWriter<'a, REG, Wp7_3>;
impl<'a, REG> Wp7_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp7_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM4 - DATA0"]
    #[inline(always)]
    pub fn wp_4_0(&self) -> Wp4_0R {
        Wp4_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM4 - DATA1"]
    #[inline(always)]
    pub fn wp_4_1(&self) -> Wp4_1R {
        Wp4_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM4 - DATA2"]
    #[inline(always)]
    pub fn wp_4_2(&self) -> Wp4_2R {
        Wp4_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM4 - DATA3"]
    #[inline(always)]
    pub fn wp_4_3(&self) -> Wp4_3R {
        Wp4_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM5 - DATA0"]
    #[inline(always)]
    pub fn wp_5_0(&self) -> Wp5_0R {
        Wp5_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM5 - DATA1"]
    #[inline(always)]
    pub fn wp_5_1(&self) -> Wp5_1R {
        Wp5_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM5 - DATA2"]
    #[inline(always)]
    pub fn wp_5_2(&self) -> Wp5_2R {
        Wp5_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM5 - DATA3"]
    #[inline(always)]
    pub fn wp_5_3(&self) -> Wp5_3R {
        Wp5_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM6 - DATA0"]
    #[inline(always)]
    pub fn wp_6_0(&self) -> Wp6_0R {
        Wp6_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM6 - DATA1"]
    #[inline(always)]
    pub fn wp_6_1(&self) -> Wp6_1R {
        Wp6_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM6 - DATA2"]
    #[inline(always)]
    pub fn wp_6_2(&self) -> Wp6_2R {
        Wp6_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM6 - DATA3"]
    #[inline(always)]
    pub fn wp_6_3(&self) -> Wp6_3R {
        Wp6_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM7 - DATA0"]
    #[inline(always)]
    pub fn wp_7_0(&self) -> Wp7_0R {
        Wp7_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM7 - DATA1"]
    #[inline(always)]
    pub fn wp_7_1(&self) -> Wp7_1R {
        Wp7_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM7 - DATA2"]
    #[inline(always)]
    pub fn wp_7_2(&self) -> Wp7_2R {
        Wp7_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM7 - DATA3"]
    #[inline(always)]
    pub fn wp_7_3(&self) -> Wp7_3R {
        Wp7_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM4 - DATA0"]
    #[inline(always)]
    pub fn wp_4_0(&mut self) -> Wp4_0W<'_, LfssSpmwprot1Spec> {
        Wp4_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM4 - DATA1"]
    #[inline(always)]
    pub fn wp_4_1(&mut self) -> Wp4_1W<'_, LfssSpmwprot1Spec> {
        Wp4_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM4 - DATA2"]
    #[inline(always)]
    pub fn wp_4_2(&mut self) -> Wp4_2W<'_, LfssSpmwprot1Spec> {
        Wp4_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM4 - DATA3"]
    #[inline(always)]
    pub fn wp_4_3(&mut self) -> Wp4_3W<'_, LfssSpmwprot1Spec> {
        Wp4_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM5 - DATA0"]
    #[inline(always)]
    pub fn wp_5_0(&mut self) -> Wp5_0W<'_, LfssSpmwprot1Spec> {
        Wp5_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM5 - DATA1"]
    #[inline(always)]
    pub fn wp_5_1(&mut self) -> Wp5_1W<'_, LfssSpmwprot1Spec> {
        Wp5_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM5 - DATA2"]
    #[inline(always)]
    pub fn wp_5_2(&mut self) -> Wp5_2W<'_, LfssSpmwprot1Spec> {
        Wp5_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM5 - DATA3"]
    #[inline(always)]
    pub fn wp_5_3(&mut self) -> Wp5_3W<'_, LfssSpmwprot1Spec> {
        Wp5_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM6 - DATA0"]
    #[inline(always)]
    pub fn wp_6_0(&mut self) -> Wp6_0W<'_, LfssSpmwprot1Spec> {
        Wp6_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM6 - DATA1"]
    #[inline(always)]
    pub fn wp_6_1(&mut self) -> Wp6_1W<'_, LfssSpmwprot1Spec> {
        Wp6_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM6 - DATA2"]
    #[inline(always)]
    pub fn wp_6_2(&mut self) -> Wp6_2W<'_, LfssSpmwprot1Spec> {
        Wp6_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM6 - DATA3"]
    #[inline(always)]
    pub fn wp_6_3(&mut self) -> Wp6_3W<'_, LfssSpmwprot1Spec> {
        Wp6_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM7 - DATA0"]
    #[inline(always)]
    pub fn wp_7_0(&mut self) -> Wp7_0W<'_, LfssSpmwprot1Spec> {
        Wp7_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM7 - DATA1"]
    #[inline(always)]
    pub fn wp_7_1(&mut self) -> Wp7_1W<'_, LfssSpmwprot1Spec> {
        Wp7_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM7 - DATA2"]
    #[inline(always)]
    pub fn wp_7_2(&mut self) -> Wp7_2W<'_, LfssSpmwprot1Spec> {
        Wp7_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM7 - DATA3"]
    #[inline(always)]
    pub fn wp_7_3(&mut self) -> Wp7_3W<'_, LfssSpmwprot1Spec> {
        Wp7_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot1Spec;
impl crate::RegisterSpec for LfssSpmwprot1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot1::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot1Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot1::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT1 to value 0"]
impl crate::Resettable for LfssSpmwprot1Spec {}
