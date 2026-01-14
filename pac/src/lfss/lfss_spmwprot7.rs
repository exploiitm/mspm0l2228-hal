#[doc = "Register `LFSS_SPMWPROT7` reader"]
pub type R = crate::R<LfssSpmwprot7Spec>;
#[doc = "Register `LFSS_SPMWPROT7` writer"]
pub type W = crate::W<LfssSpmwprot7Spec>;
#[doc = "write protect SPMEM28 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp28_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp28_0> for bool {
    #[inline(always)]
    fn from(variant: Wp28_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_28_0` reader - write protect SPMEM28 - DATA0"]
pub type Wp28_0R = crate::BitReader<Wp28_0>;
impl Wp28_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp28_0 {
        match self.bits {
            false => Wp28_0::Readwrite,
            true => Wp28_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp28_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp28_0::Readonly
    }
}
#[doc = "Field `WP_28_0` writer - write protect SPMEM28 - DATA0"]
pub type Wp28_0W<'a, REG> = crate::BitWriter<'a, REG, Wp28_0>;
impl<'a, REG> Wp28_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_0::Readonly)
    }
}
#[doc = "write protect SPMEM28 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp28_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp28_1> for bool {
    #[inline(always)]
    fn from(variant: Wp28_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_28_1` reader - write protect SPMEM28 - DATA1"]
pub type Wp28_1R = crate::BitReader<Wp28_1>;
impl Wp28_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp28_1 {
        match self.bits {
            false => Wp28_1::Readwrite,
            true => Wp28_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp28_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp28_1::Readonly
    }
}
#[doc = "Field `WP_28_1` writer - write protect SPMEM28 - DATA1"]
pub type Wp28_1W<'a, REG> = crate::BitWriter<'a, REG, Wp28_1>;
impl<'a, REG> Wp28_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_1::Readonly)
    }
}
#[doc = "write protect SPMEM28 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp28_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp28_2> for bool {
    #[inline(always)]
    fn from(variant: Wp28_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_28_2` reader - write protect SPMEM28 - DATA2"]
pub type Wp28_2R = crate::BitReader<Wp28_2>;
impl Wp28_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp28_2 {
        match self.bits {
            false => Wp28_2::Readwrite,
            true => Wp28_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp28_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp28_2::Readonly
    }
}
#[doc = "Field `WP_28_2` writer - write protect SPMEM28 - DATA2"]
pub type Wp28_2W<'a, REG> = crate::BitWriter<'a, REG, Wp28_2>;
impl<'a, REG> Wp28_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_2::Readonly)
    }
}
#[doc = "write protect SPMEM28 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp28_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp28_3> for bool {
    #[inline(always)]
    fn from(variant: Wp28_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_28_3` reader - write protect SPMEM28 - DATA3"]
pub type Wp28_3R = crate::BitReader<Wp28_3>;
impl Wp28_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp28_3 {
        match self.bits {
            false => Wp28_3::Readwrite,
            true => Wp28_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp28_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp28_3::Readonly
    }
}
#[doc = "Field `WP_28_3` writer - write protect SPMEM28 - DATA3"]
pub type Wp28_3W<'a, REG> = crate::BitWriter<'a, REG, Wp28_3>;
impl<'a, REG> Wp28_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp28_3::Readonly)
    }
}
#[doc = "write protect SPMEM29 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp29_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp29_0> for bool {
    #[inline(always)]
    fn from(variant: Wp29_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_29_0` reader - write protect SPMEM29 - DATA0"]
pub type Wp29_0R = crate::BitReader<Wp29_0>;
impl Wp29_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp29_0 {
        match self.bits {
            false => Wp29_0::Readwrite,
            true => Wp29_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp29_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp29_0::Readonly
    }
}
#[doc = "Field `WP_29_0` writer - write protect SPMEM29 - DATA0"]
pub type Wp29_0W<'a, REG> = crate::BitWriter<'a, REG, Wp29_0>;
impl<'a, REG> Wp29_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_0::Readonly)
    }
}
#[doc = "write protect SPMEM29 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp29_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp29_1> for bool {
    #[inline(always)]
    fn from(variant: Wp29_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_29_1` reader - write protect SPMEM29 - DATA1"]
pub type Wp29_1R = crate::BitReader<Wp29_1>;
impl Wp29_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp29_1 {
        match self.bits {
            false => Wp29_1::Readwrite,
            true => Wp29_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp29_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp29_1::Readonly
    }
}
#[doc = "Field `WP_29_1` writer - write protect SPMEM29 - DATA1"]
pub type Wp29_1W<'a, REG> = crate::BitWriter<'a, REG, Wp29_1>;
impl<'a, REG> Wp29_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_1::Readonly)
    }
}
#[doc = "write protect SPMEM29 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp29_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp29_2> for bool {
    #[inline(always)]
    fn from(variant: Wp29_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_29_2` reader - write protect SPMEM29 - DATA2"]
pub type Wp29_2R = crate::BitReader<Wp29_2>;
impl Wp29_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp29_2 {
        match self.bits {
            false => Wp29_2::Readwrite,
            true => Wp29_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp29_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp29_2::Readonly
    }
}
#[doc = "Field `WP_29_2` writer - write protect SPMEM29 - DATA2"]
pub type Wp29_2W<'a, REG> = crate::BitWriter<'a, REG, Wp29_2>;
impl<'a, REG> Wp29_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_2::Readonly)
    }
}
#[doc = "write protect SPMEM29 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp29_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp29_3> for bool {
    #[inline(always)]
    fn from(variant: Wp29_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_29_3` reader - write protect SPMEM29 - DATA3"]
pub type Wp29_3R = crate::BitReader<Wp29_3>;
impl Wp29_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp29_3 {
        match self.bits {
            false => Wp29_3::Readwrite,
            true => Wp29_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp29_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp29_3::Readonly
    }
}
#[doc = "Field `WP_29_3` writer - write protect SPMEM29 - DATA3"]
pub type Wp29_3W<'a, REG> = crate::BitWriter<'a, REG, Wp29_3>;
impl<'a, REG> Wp29_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp29_3::Readonly)
    }
}
#[doc = "write protect SPMEM30 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp30_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp30_0> for bool {
    #[inline(always)]
    fn from(variant: Wp30_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_30_0` reader - write protect SPMEM30 - DATA0"]
pub type Wp30_0R = crate::BitReader<Wp30_0>;
impl Wp30_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp30_0 {
        match self.bits {
            false => Wp30_0::Readwrite,
            true => Wp30_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp30_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp30_0::Readonly
    }
}
#[doc = "Field `WP_30_0` writer - write protect SPMEM30 - DATA0"]
pub type Wp30_0W<'a, REG> = crate::BitWriter<'a, REG, Wp30_0>;
impl<'a, REG> Wp30_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_0::Readonly)
    }
}
#[doc = "write protect SPMEM30 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp30_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp30_1> for bool {
    #[inline(always)]
    fn from(variant: Wp30_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_30_1` reader - write protect SPMEM30 - DATA1"]
pub type Wp30_1R = crate::BitReader<Wp30_1>;
impl Wp30_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp30_1 {
        match self.bits {
            false => Wp30_1::Readwrite,
            true => Wp30_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp30_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp30_1::Readonly
    }
}
#[doc = "Field `WP_30_1` writer - write protect SPMEM30 - DATA1"]
pub type Wp30_1W<'a, REG> = crate::BitWriter<'a, REG, Wp30_1>;
impl<'a, REG> Wp30_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_1::Readonly)
    }
}
#[doc = "write protect SPMEM30- DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp30_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp30_2> for bool {
    #[inline(always)]
    fn from(variant: Wp30_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_30_2` reader - write protect SPMEM30- DATA2"]
pub type Wp30_2R = crate::BitReader<Wp30_2>;
impl Wp30_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp30_2 {
        match self.bits {
            false => Wp30_2::Readwrite,
            true => Wp30_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp30_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp30_2::Readonly
    }
}
#[doc = "Field `WP_30_2` writer - write protect SPMEM30- DATA2"]
pub type Wp30_2W<'a, REG> = crate::BitWriter<'a, REG, Wp30_2>;
impl<'a, REG> Wp30_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_2::Readonly)
    }
}
#[doc = "write protect SPMEM30 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp30_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp30_3> for bool {
    #[inline(always)]
    fn from(variant: Wp30_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_30_3` reader - write protect SPMEM30 - DATA3"]
pub type Wp30_3R = crate::BitReader<Wp30_3>;
impl Wp30_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp30_3 {
        match self.bits {
            false => Wp30_3::Readwrite,
            true => Wp30_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp30_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp30_3::Readonly
    }
}
#[doc = "Field `WP_30_3` writer - write protect SPMEM30 - DATA3"]
pub type Wp30_3W<'a, REG> = crate::BitWriter<'a, REG, Wp30_3>;
impl<'a, REG> Wp30_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp30_3::Readonly)
    }
}
#[doc = "write protect SPMEM31 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp31_0 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp31_0> for bool {
    #[inline(always)]
    fn from(variant: Wp31_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_31_0` reader - write protect SPMEM31 - DATA0"]
pub type Wp31_0R = crate::BitReader<Wp31_0>;
impl Wp31_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp31_0 {
        match self.bits {
            false => Wp31_0::Readwrite,
            true => Wp31_0::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp31_0::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp31_0::Readonly
    }
}
#[doc = "Field `WP_31_0` writer - write protect SPMEM31 - DATA0"]
pub type Wp31_0W<'a, REG> = crate::BitWriter<'a, REG, Wp31_0>;
impl<'a, REG> Wp31_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_0::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_0::Readonly)
    }
}
#[doc = "write protect SPMEM31 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp31_1 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp31_1> for bool {
    #[inline(always)]
    fn from(variant: Wp31_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_31_1` reader - write protect SPMEM31 - DATA1"]
pub type Wp31_1R = crate::BitReader<Wp31_1>;
impl Wp31_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp31_1 {
        match self.bits {
            false => Wp31_1::Readwrite,
            true => Wp31_1::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp31_1::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp31_1::Readonly
    }
}
#[doc = "Field `WP_31_1` writer - write protect SPMEM31 - DATA1"]
pub type Wp31_1W<'a, REG> = crate::BitWriter<'a, REG, Wp31_1>;
impl<'a, REG> Wp31_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_1::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_1::Readonly)
    }
}
#[doc = "write protect SPMEM31 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp31_2 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp31_2> for bool {
    #[inline(always)]
    fn from(variant: Wp31_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_31_2` reader - write protect SPMEM31 - DATA2"]
pub type Wp31_2R = crate::BitReader<Wp31_2>;
impl Wp31_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp31_2 {
        match self.bits {
            false => Wp31_2::Readwrite,
            true => Wp31_2::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp31_2::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp31_2::Readonly
    }
}
#[doc = "Field `WP_31_2` writer - write protect SPMEM31 - DATA2"]
pub type Wp31_2W<'a, REG> = crate::BitWriter<'a, REG, Wp31_2>;
impl<'a, REG> Wp31_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_2::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_2::Readonly)
    }
}
#[doc = "write protect SPMEM31 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp31_3 {
    #[doc = "0: SPMEM is read and write access"]
    Readwrite = 0,
    #[doc = "1: SPMEM is read only access"]
    Readonly = 1,
}
impl From<Wp31_3> for bool {
    #[inline(always)]
    fn from(variant: Wp31_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_31_3` reader - write protect SPMEM31 - DATA3"]
pub type Wp31_3R = crate::BitReader<Wp31_3>;
impl Wp31_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp31_3 {
        match self.bits {
            false => Wp31_3::Readwrite,
            true => Wp31_3::Readonly,
        }
    }
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Wp31_3::Readwrite
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Wp31_3::Readonly
    }
}
#[doc = "Field `WP_31_3` writer - write protect SPMEM31 - DATA3"]
pub type Wp31_3W<'a, REG> = crate::BitWriter<'a, REG, Wp31_3>;
impl<'a, REG> Wp31_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is read and write access"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_3::Readwrite)
    }
    #[doc = "SPMEM is read only access"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Wp31_3::Readonly)
    }
}
impl R {
    #[doc = "Bit 0 - write protect SPMEM28 - DATA0"]
    #[inline(always)]
    pub fn wp_28_0(&self) -> Wp28_0R {
        Wp28_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write protect SPMEM28 - DATA1"]
    #[inline(always)]
    pub fn wp_28_1(&self) -> Wp28_1R {
        Wp28_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write protect SPMEM28 - DATA2"]
    #[inline(always)]
    pub fn wp_28_2(&self) -> Wp28_2R {
        Wp28_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write protect SPMEM28 - DATA3"]
    #[inline(always)]
    pub fn wp_28_3(&self) -> Wp28_3R {
        Wp28_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write protect SPMEM29 - DATA0"]
    #[inline(always)]
    pub fn wp_29_0(&self) -> Wp29_0R {
        Wp29_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write protect SPMEM29 - DATA1"]
    #[inline(always)]
    pub fn wp_29_1(&self) -> Wp29_1R {
        Wp29_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write protect SPMEM29 - DATA2"]
    #[inline(always)]
    pub fn wp_29_2(&self) -> Wp29_2R {
        Wp29_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write protect SPMEM29 - DATA3"]
    #[inline(always)]
    pub fn wp_29_3(&self) -> Wp29_3R {
        Wp29_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write protect SPMEM30 - DATA0"]
    #[inline(always)]
    pub fn wp_30_0(&self) -> Wp30_0R {
        Wp30_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write protect SPMEM30 - DATA1"]
    #[inline(always)]
    pub fn wp_30_1(&self) -> Wp30_1R {
        Wp30_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write protect SPMEM30- DATA2"]
    #[inline(always)]
    pub fn wp_30_2(&self) -> Wp30_2R {
        Wp30_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write protect SPMEM30 - DATA3"]
    #[inline(always)]
    pub fn wp_30_3(&self) -> Wp30_3R {
        Wp30_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write protect SPMEM31 - DATA0"]
    #[inline(always)]
    pub fn wp_31_0(&self) -> Wp31_0R {
        Wp31_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write protect SPMEM31 - DATA1"]
    #[inline(always)]
    pub fn wp_31_1(&self) -> Wp31_1R {
        Wp31_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write protect SPMEM31 - DATA2"]
    #[inline(always)]
    pub fn wp_31_2(&self) -> Wp31_2R {
        Wp31_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write protect SPMEM31 - DATA3"]
    #[inline(always)]
    pub fn wp_31_3(&self) -> Wp31_3R {
        Wp31_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write protect SPMEM28 - DATA0"]
    #[inline(always)]
    pub fn wp_28_0(&mut self) -> Wp28_0W<'_, LfssSpmwprot7Spec> {
        Wp28_0W::new(self, 0)
    }
    #[doc = "Bit 1 - write protect SPMEM28 - DATA1"]
    #[inline(always)]
    pub fn wp_28_1(&mut self) -> Wp28_1W<'_, LfssSpmwprot7Spec> {
        Wp28_1W::new(self, 1)
    }
    #[doc = "Bit 2 - write protect SPMEM28 - DATA2"]
    #[inline(always)]
    pub fn wp_28_2(&mut self) -> Wp28_2W<'_, LfssSpmwprot7Spec> {
        Wp28_2W::new(self, 2)
    }
    #[doc = "Bit 3 - write protect SPMEM28 - DATA3"]
    #[inline(always)]
    pub fn wp_28_3(&mut self) -> Wp28_3W<'_, LfssSpmwprot7Spec> {
        Wp28_3W::new(self, 3)
    }
    #[doc = "Bit 4 - write protect SPMEM29 - DATA0"]
    #[inline(always)]
    pub fn wp_29_0(&mut self) -> Wp29_0W<'_, LfssSpmwprot7Spec> {
        Wp29_0W::new(self, 4)
    }
    #[doc = "Bit 5 - write protect SPMEM29 - DATA1"]
    #[inline(always)]
    pub fn wp_29_1(&mut self) -> Wp29_1W<'_, LfssSpmwprot7Spec> {
        Wp29_1W::new(self, 5)
    }
    #[doc = "Bit 6 - write protect SPMEM29 - DATA2"]
    #[inline(always)]
    pub fn wp_29_2(&mut self) -> Wp29_2W<'_, LfssSpmwprot7Spec> {
        Wp29_2W::new(self, 6)
    }
    #[doc = "Bit 7 - write protect SPMEM29 - DATA3"]
    #[inline(always)]
    pub fn wp_29_3(&mut self) -> Wp29_3W<'_, LfssSpmwprot7Spec> {
        Wp29_3W::new(self, 7)
    }
    #[doc = "Bit 8 - write protect SPMEM30 - DATA0"]
    #[inline(always)]
    pub fn wp_30_0(&mut self) -> Wp30_0W<'_, LfssSpmwprot7Spec> {
        Wp30_0W::new(self, 8)
    }
    #[doc = "Bit 9 - write protect SPMEM30 - DATA1"]
    #[inline(always)]
    pub fn wp_30_1(&mut self) -> Wp30_1W<'_, LfssSpmwprot7Spec> {
        Wp30_1W::new(self, 9)
    }
    #[doc = "Bit 10 - write protect SPMEM30- DATA2"]
    #[inline(always)]
    pub fn wp_30_2(&mut self) -> Wp30_2W<'_, LfssSpmwprot7Spec> {
        Wp30_2W::new(self, 10)
    }
    #[doc = "Bit 11 - write protect SPMEM30 - DATA3"]
    #[inline(always)]
    pub fn wp_30_3(&mut self) -> Wp30_3W<'_, LfssSpmwprot7Spec> {
        Wp30_3W::new(self, 11)
    }
    #[doc = "Bit 12 - write protect SPMEM31 - DATA0"]
    #[inline(always)]
    pub fn wp_31_0(&mut self) -> Wp31_0W<'_, LfssSpmwprot7Spec> {
        Wp31_0W::new(self, 12)
    }
    #[doc = "Bit 13 - write protect SPMEM31 - DATA1"]
    #[inline(always)]
    pub fn wp_31_1(&mut self) -> Wp31_1W<'_, LfssSpmwprot7Spec> {
        Wp31_1W::new(self, 13)
    }
    #[doc = "Bit 14 - write protect SPMEM31 - DATA2"]
    #[inline(always)]
    pub fn wp_31_2(&mut self) -> Wp31_2W<'_, LfssSpmwprot7Spec> {
        Wp31_2W::new(self, 14)
    }
    #[doc = "Bit 15 - write protect SPMEM31 - DATA3"]
    #[inline(always)]
    pub fn wp_31_3(&mut self) -> Wp31_3W<'_, LfssSpmwprot7Spec> {
        Wp31_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Write Protect Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmwprot7Spec;
impl crate::RegisterSpec for LfssSpmwprot7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmwprot7::R`](R) reader structure"]
impl crate::Readable for LfssSpmwprot7Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmwprot7::W`](W) writer structure"]
impl crate::Writable for LfssSpmwprot7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMWPROT7 to value 0"]
impl crate::Resettable for LfssSpmwprot7Spec {}
