#[doc = "Register `LFSS_SPMTERASE5` reader"]
pub type R = crate::R<LfssSpmterase5Spec>;
#[doc = "Register `LFSS_SPMTERASE5` writer"]
pub type W = crate::W<LfssSpmterase5Spec>;
#[doc = "tamper erase enable SPMEM20 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te20_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te20_0> for bool {
    #[inline(always)]
    fn from(variant: Te20_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_20_0` reader - tamper erase enable SPMEM20 - DATA0"]
pub type Te20_0R = crate::BitReader<Te20_0>;
impl Te20_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te20_0 {
        match self.bits {
            false => Te20_0::Disable,
            true => Te20_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te20_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te20_0::Enable
    }
}
#[doc = "Field `TE_20_0` writer - tamper erase enable SPMEM20 - DATA0"]
pub type Te20_0W<'a, REG> = crate::BitWriter<'a, REG, Te20_0>;
impl<'a, REG> Te20_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM20 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te20_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te20_1> for bool {
    #[inline(always)]
    fn from(variant: Te20_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_20_1` reader - tamper erase enable SPMEM20 - DATA1"]
pub type Te20_1R = crate::BitReader<Te20_1>;
impl Te20_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te20_1 {
        match self.bits {
            false => Te20_1::Disable,
            true => Te20_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te20_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te20_1::Enable
    }
}
#[doc = "Field `TE_20_1` writer - tamper erase enable SPMEM20 - DATA1"]
pub type Te20_1W<'a, REG> = crate::BitWriter<'a, REG, Te20_1>;
impl<'a, REG> Te20_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM20 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te20_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te20_2> for bool {
    #[inline(always)]
    fn from(variant: Te20_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_20_2` reader - tamper erase enable SPMEM20 - DATA2"]
pub type Te20_2R = crate::BitReader<Te20_2>;
impl Te20_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te20_2 {
        match self.bits {
            false => Te20_2::Disable,
            true => Te20_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te20_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te20_2::Enable
    }
}
#[doc = "Field `TE_20_2` writer - tamper erase enable SPMEM20 - DATA2"]
pub type Te20_2W<'a, REG> = crate::BitWriter<'a, REG, Te20_2>;
impl<'a, REG> Te20_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM20 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te20_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te20_3> for bool {
    #[inline(always)]
    fn from(variant: Te20_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_20_3` reader - tamper erase enable SPMEM20 - DATA3"]
pub type Te20_3R = crate::BitReader<Te20_3>;
impl Te20_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te20_3 {
        match self.bits {
            false => Te20_3::Disable,
            true => Te20_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te20_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te20_3::Enable
    }
}
#[doc = "Field `TE_20_3` writer - tamper erase enable SPMEM20 - DATA3"]
pub type Te20_3W<'a, REG> = crate::BitWriter<'a, REG, Te20_3>;
impl<'a, REG> Te20_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te20_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM21 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te21_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te21_0> for bool {
    #[inline(always)]
    fn from(variant: Te21_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_21_0` reader - tamper erase enable SPMEM21 - DATA0"]
pub type Te21_0R = crate::BitReader<Te21_0>;
impl Te21_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te21_0 {
        match self.bits {
            false => Te21_0::Disable,
            true => Te21_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te21_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te21_0::Enable
    }
}
#[doc = "Field `TE_21_0` writer - tamper erase enable SPMEM21 - DATA0"]
pub type Te21_0W<'a, REG> = crate::BitWriter<'a, REG, Te21_0>;
impl<'a, REG> Te21_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM21 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te21_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te21_1> for bool {
    #[inline(always)]
    fn from(variant: Te21_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_21_1` reader - tamper erase enable SPMEM21 - DATA1"]
pub type Te21_1R = crate::BitReader<Te21_1>;
impl Te21_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te21_1 {
        match self.bits {
            false => Te21_1::Disable,
            true => Te21_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te21_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te21_1::Enable
    }
}
#[doc = "Field `TE_21_1` writer - tamper erase enable SPMEM21 - DATA1"]
pub type Te21_1W<'a, REG> = crate::BitWriter<'a, REG, Te21_1>;
impl<'a, REG> Te21_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM21 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te21_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te21_2> for bool {
    #[inline(always)]
    fn from(variant: Te21_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_21_2` reader - tamper erase enable SPMEM21 - DATA2"]
pub type Te21_2R = crate::BitReader<Te21_2>;
impl Te21_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te21_2 {
        match self.bits {
            false => Te21_2::Disable,
            true => Te21_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te21_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te21_2::Enable
    }
}
#[doc = "Field `TE_21_2` writer - tamper erase enable SPMEM21 - DATA2"]
pub type Te21_2W<'a, REG> = crate::BitWriter<'a, REG, Te21_2>;
impl<'a, REG> Te21_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM21 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te21_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te21_3> for bool {
    #[inline(always)]
    fn from(variant: Te21_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_21_3` reader - tamper erase enable SPMEM21 - DATA3"]
pub type Te21_3R = crate::BitReader<Te21_3>;
impl Te21_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te21_3 {
        match self.bits {
            false => Te21_3::Disable,
            true => Te21_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te21_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te21_3::Enable
    }
}
#[doc = "Field `TE_21_3` writer - tamper erase enable SPMEM21 - DATA3"]
pub type Te21_3W<'a, REG> = crate::BitWriter<'a, REG, Te21_3>;
impl<'a, REG> Te21_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te21_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM22 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te22_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te22_0> for bool {
    #[inline(always)]
    fn from(variant: Te22_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_22_0` reader - tamper erase enable SPMEM22 - DATA0"]
pub type Te22_0R = crate::BitReader<Te22_0>;
impl Te22_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te22_0 {
        match self.bits {
            false => Te22_0::Disable,
            true => Te22_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te22_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te22_0::Enable
    }
}
#[doc = "Field `TE_22_0` writer - tamper erase enable SPMEM22 - DATA0"]
pub type Te22_0W<'a, REG> = crate::BitWriter<'a, REG, Te22_0>;
impl<'a, REG> Te22_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM22 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te22_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te22_1> for bool {
    #[inline(always)]
    fn from(variant: Te22_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_22_1` reader - tamper erase enable SPMEM22 - DATA1"]
pub type Te22_1R = crate::BitReader<Te22_1>;
impl Te22_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te22_1 {
        match self.bits {
            false => Te22_1::Disable,
            true => Te22_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te22_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te22_1::Enable
    }
}
#[doc = "Field `TE_22_1` writer - tamper erase enable SPMEM22 - DATA1"]
pub type Te22_1W<'a, REG> = crate::BitWriter<'a, REG, Te22_1>;
impl<'a, REG> Te22_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM22 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te22_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te22_2> for bool {
    #[inline(always)]
    fn from(variant: Te22_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_22_2` reader - tamper erase enable SPMEM22 - DATA2"]
pub type Te22_2R = crate::BitReader<Te22_2>;
impl Te22_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te22_2 {
        match self.bits {
            false => Te22_2::Disable,
            true => Te22_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te22_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te22_2::Enable
    }
}
#[doc = "Field `TE_22_2` writer - tamper erase enable SPMEM22 - DATA2"]
pub type Te22_2W<'a, REG> = crate::BitWriter<'a, REG, Te22_2>;
impl<'a, REG> Te22_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM22 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te22_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te22_3> for bool {
    #[inline(always)]
    fn from(variant: Te22_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_22_3` reader - tamper erase enable SPMEM22 - DATA3"]
pub type Te22_3R = crate::BitReader<Te22_3>;
impl Te22_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te22_3 {
        match self.bits {
            false => Te22_3::Disable,
            true => Te22_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te22_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te22_3::Enable
    }
}
#[doc = "Field `TE_22_3` writer - tamper erase enable SPMEM22 - DATA3"]
pub type Te22_3W<'a, REG> = crate::BitWriter<'a, REG, Te22_3>;
impl<'a, REG> Te22_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te22_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM23 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te23_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te23_0> for bool {
    #[inline(always)]
    fn from(variant: Te23_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_23_0` reader - tamper erase enable SPMEM23 - DATA0"]
pub type Te23_0R = crate::BitReader<Te23_0>;
impl Te23_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te23_0 {
        match self.bits {
            false => Te23_0::Disable,
            true => Te23_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te23_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te23_0::Enable
    }
}
#[doc = "Field `TE_23_0` writer - tamper erase enable SPMEM23 - DATA0"]
pub type Te23_0W<'a, REG> = crate::BitWriter<'a, REG, Te23_0>;
impl<'a, REG> Te23_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM23 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te23_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te23_1> for bool {
    #[inline(always)]
    fn from(variant: Te23_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_23_1` reader - tamper erase enable SPMEM23 - DATA1"]
pub type Te23_1R = crate::BitReader<Te23_1>;
impl Te23_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te23_1 {
        match self.bits {
            false => Te23_1::Disable,
            true => Te23_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te23_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te23_1::Enable
    }
}
#[doc = "Field `TE_23_1` writer - tamper erase enable SPMEM23 - DATA1"]
pub type Te23_1W<'a, REG> = crate::BitWriter<'a, REG, Te23_1>;
impl<'a, REG> Te23_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM23 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te23_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te23_2> for bool {
    #[inline(always)]
    fn from(variant: Te23_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_23_2` reader - tamper erase enable SPMEM23 - DATA2"]
pub type Te23_2R = crate::BitReader<Te23_2>;
impl Te23_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te23_2 {
        match self.bits {
            false => Te23_2::Disable,
            true => Te23_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te23_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te23_2::Enable
    }
}
#[doc = "Field `TE_23_2` writer - tamper erase enable SPMEM23 - DATA2"]
pub type Te23_2W<'a, REG> = crate::BitWriter<'a, REG, Te23_2>;
impl<'a, REG> Te23_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM23 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te23_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te23_3> for bool {
    #[inline(always)]
    fn from(variant: Te23_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_23_3` reader - tamper erase enable SPMEM23 - DATA3"]
pub type Te23_3R = crate::BitReader<Te23_3>;
impl Te23_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te23_3 {
        match self.bits {
            false => Te23_3::Disable,
            true => Te23_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te23_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te23_3::Enable
    }
}
#[doc = "Field `TE_23_3` writer - tamper erase enable SPMEM23 - DATA3"]
pub type Te23_3W<'a, REG> = crate::BitWriter<'a, REG, Te23_3>;
impl<'a, REG> Te23_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te23_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM20 - DATA0"]
    #[inline(always)]
    pub fn te_20_0(&self) -> Te20_0R {
        Te20_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM20 - DATA1"]
    #[inline(always)]
    pub fn te_20_1(&self) -> Te20_1R {
        Te20_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM20 - DATA2"]
    #[inline(always)]
    pub fn te_20_2(&self) -> Te20_2R {
        Te20_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM20 - DATA3"]
    #[inline(always)]
    pub fn te_20_3(&self) -> Te20_3R {
        Te20_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM21 - DATA0"]
    #[inline(always)]
    pub fn te_21_0(&self) -> Te21_0R {
        Te21_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM21 - DATA1"]
    #[inline(always)]
    pub fn te_21_1(&self) -> Te21_1R {
        Te21_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM21 - DATA2"]
    #[inline(always)]
    pub fn te_21_2(&self) -> Te21_2R {
        Te21_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM21 - DATA3"]
    #[inline(always)]
    pub fn te_21_3(&self) -> Te21_3R {
        Te21_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM22 - DATA0"]
    #[inline(always)]
    pub fn te_22_0(&self) -> Te22_0R {
        Te22_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM22 - DATA1"]
    #[inline(always)]
    pub fn te_22_1(&self) -> Te22_1R {
        Te22_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM22 - DATA2"]
    #[inline(always)]
    pub fn te_22_2(&self) -> Te22_2R {
        Te22_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM22 - DATA3"]
    #[inline(always)]
    pub fn te_22_3(&self) -> Te22_3R {
        Te22_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM23 - DATA0"]
    #[inline(always)]
    pub fn te_23_0(&self) -> Te23_0R {
        Te23_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM23 - DATA1"]
    #[inline(always)]
    pub fn te_23_1(&self) -> Te23_1R {
        Te23_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM23 - DATA2"]
    #[inline(always)]
    pub fn te_23_2(&self) -> Te23_2R {
        Te23_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM23 - DATA3"]
    #[inline(always)]
    pub fn te_23_3(&self) -> Te23_3R {
        Te23_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM20 - DATA0"]
    #[inline(always)]
    pub fn te_20_0(&mut self) -> Te20_0W<'_, LfssSpmterase5Spec> {
        Te20_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM20 - DATA1"]
    #[inline(always)]
    pub fn te_20_1(&mut self) -> Te20_1W<'_, LfssSpmterase5Spec> {
        Te20_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM20 - DATA2"]
    #[inline(always)]
    pub fn te_20_2(&mut self) -> Te20_2W<'_, LfssSpmterase5Spec> {
        Te20_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM20 - DATA3"]
    #[inline(always)]
    pub fn te_20_3(&mut self) -> Te20_3W<'_, LfssSpmterase5Spec> {
        Te20_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM21 - DATA0"]
    #[inline(always)]
    pub fn te_21_0(&mut self) -> Te21_0W<'_, LfssSpmterase5Spec> {
        Te21_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM21 - DATA1"]
    #[inline(always)]
    pub fn te_21_1(&mut self) -> Te21_1W<'_, LfssSpmterase5Spec> {
        Te21_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM21 - DATA2"]
    #[inline(always)]
    pub fn te_21_2(&mut self) -> Te21_2W<'_, LfssSpmterase5Spec> {
        Te21_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM21 - DATA3"]
    #[inline(always)]
    pub fn te_21_3(&mut self) -> Te21_3W<'_, LfssSpmterase5Spec> {
        Te21_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM22 - DATA0"]
    #[inline(always)]
    pub fn te_22_0(&mut self) -> Te22_0W<'_, LfssSpmterase5Spec> {
        Te22_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM22 - DATA1"]
    #[inline(always)]
    pub fn te_22_1(&mut self) -> Te22_1W<'_, LfssSpmterase5Spec> {
        Te22_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM22 - DATA2"]
    #[inline(always)]
    pub fn te_22_2(&mut self) -> Te22_2W<'_, LfssSpmterase5Spec> {
        Te22_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM22 - DATA3"]
    #[inline(always)]
    pub fn te_22_3(&mut self) -> Te22_3W<'_, LfssSpmterase5Spec> {
        Te22_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM23 - DATA0"]
    #[inline(always)]
    pub fn te_23_0(&mut self) -> Te23_0W<'_, LfssSpmterase5Spec> {
        Te23_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM23 - DATA1"]
    #[inline(always)]
    pub fn te_23_1(&mut self) -> Te23_1W<'_, LfssSpmterase5Spec> {
        Te23_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM23 - DATA2"]
    #[inline(always)]
    pub fn te_23_2(&mut self) -> Te23_2W<'_, LfssSpmterase5Spec> {
        Te23_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM23 - DATA3"]
    #[inline(always)]
    pub fn te_23_3(&mut self) -> Te23_3W<'_, LfssSpmterase5Spec> {
        Te23_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase5Spec;
impl crate::RegisterSpec for LfssSpmterase5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase5::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase5Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase5::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE5 to value 0"]
impl crate::Resettable for LfssSpmterase5Spec {}
