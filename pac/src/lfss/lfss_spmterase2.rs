#[doc = "Register `LFSS_SPMTERASE2` reader"]
pub type R = crate::R<LfssSpmterase2Spec>;
#[doc = "Register `LFSS_SPMTERASE2` writer"]
pub type W = crate::W<LfssSpmterase2Spec>;
#[doc = "tamper erase enable SPMEM8 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te8_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te8_0> for bool {
    #[inline(always)]
    fn from(variant: Te8_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_8_0` reader - tamper erase enable SPMEM8 - DATA0"]
pub type Te8_0R = crate::BitReader<Te8_0>;
impl Te8_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te8_0 {
        match self.bits {
            false => Te8_0::Disable,
            true => Te8_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te8_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te8_0::Enable
    }
}
#[doc = "Field `TE_8_0` writer - tamper erase enable SPMEM8 - DATA0"]
pub type Te8_0W<'a, REG> = crate::BitWriter<'a, REG, Te8_0>;
impl<'a, REG> Te8_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM8 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te8_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te8_1> for bool {
    #[inline(always)]
    fn from(variant: Te8_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_8_1` reader - tamper erase enable SPMEM8 - DATA1"]
pub type Te8_1R = crate::BitReader<Te8_1>;
impl Te8_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te8_1 {
        match self.bits {
            false => Te8_1::Disable,
            true => Te8_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te8_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te8_1::Enable
    }
}
#[doc = "Field `TE_8_1` writer - tamper erase enable SPMEM8 - DATA1"]
pub type Te8_1W<'a, REG> = crate::BitWriter<'a, REG, Te8_1>;
impl<'a, REG> Te8_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM8 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te8_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te8_2> for bool {
    #[inline(always)]
    fn from(variant: Te8_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_8_2` reader - tamper erase enable SPMEM8 - DATA2"]
pub type Te8_2R = crate::BitReader<Te8_2>;
impl Te8_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te8_2 {
        match self.bits {
            false => Te8_2::Disable,
            true => Te8_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te8_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te8_2::Enable
    }
}
#[doc = "Field `TE_8_2` writer - tamper erase enable SPMEM8 - DATA2"]
pub type Te8_2W<'a, REG> = crate::BitWriter<'a, REG, Te8_2>;
impl<'a, REG> Te8_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM8 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te8_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te8_3> for bool {
    #[inline(always)]
    fn from(variant: Te8_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_8_3` reader - tamper erase enable SPMEM8 - DATA3"]
pub type Te8_3R = crate::BitReader<Te8_3>;
impl Te8_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te8_3 {
        match self.bits {
            false => Te8_3::Disable,
            true => Te8_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te8_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te8_3::Enable
    }
}
#[doc = "Field `TE_8_3` writer - tamper erase enable SPMEM8 - DATA3"]
pub type Te8_3W<'a, REG> = crate::BitWriter<'a, REG, Te8_3>;
impl<'a, REG> Te8_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te8_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM9 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te9_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te9_0> for bool {
    #[inline(always)]
    fn from(variant: Te9_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_9_0` reader - tamper erase enable SPMEM9 - DATA0"]
pub type Te9_0R = crate::BitReader<Te9_0>;
impl Te9_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te9_0 {
        match self.bits {
            false => Te9_0::Disable,
            true => Te9_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te9_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te9_0::Enable
    }
}
#[doc = "Field `TE_9_0` writer - tamper erase enable SPMEM9 - DATA0"]
pub type Te9_0W<'a, REG> = crate::BitWriter<'a, REG, Te9_0>;
impl<'a, REG> Te9_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM9 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te9_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te9_1> for bool {
    #[inline(always)]
    fn from(variant: Te9_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_9_1` reader - tamper erase enable SPMEM9 - DATA1"]
pub type Te9_1R = crate::BitReader<Te9_1>;
impl Te9_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te9_1 {
        match self.bits {
            false => Te9_1::Disable,
            true => Te9_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te9_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te9_1::Enable
    }
}
#[doc = "Field `TE_9_1` writer - tamper erase enable SPMEM9 - DATA1"]
pub type Te9_1W<'a, REG> = crate::BitWriter<'a, REG, Te9_1>;
impl<'a, REG> Te9_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM9 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te9_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te9_2> for bool {
    #[inline(always)]
    fn from(variant: Te9_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_9_2` reader - tamper erase enable SPMEM9 - DATA2"]
pub type Te9_2R = crate::BitReader<Te9_2>;
impl Te9_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te9_2 {
        match self.bits {
            false => Te9_2::Disable,
            true => Te9_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te9_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te9_2::Enable
    }
}
#[doc = "Field `TE_9_2` writer - tamper erase enable SPMEM9 - DATA2"]
pub type Te9_2W<'a, REG> = crate::BitWriter<'a, REG, Te9_2>;
impl<'a, REG> Te9_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM9 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te9_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te9_3> for bool {
    #[inline(always)]
    fn from(variant: Te9_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_9_3` reader - tamper erase enable SPMEM9 - DATA3"]
pub type Te9_3R = crate::BitReader<Te9_3>;
impl Te9_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te9_3 {
        match self.bits {
            false => Te9_3::Disable,
            true => Te9_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te9_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te9_3::Enable
    }
}
#[doc = "Field `TE_9_3` writer - tamper erase enable SPMEM9 - DATA3"]
pub type Te9_3W<'a, REG> = crate::BitWriter<'a, REG, Te9_3>;
impl<'a, REG> Te9_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te9_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM10 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te10_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te10_0> for bool {
    #[inline(always)]
    fn from(variant: Te10_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_10_0` reader - tamper erase enable SPMEM10 - DATA0"]
pub type Te10_0R = crate::BitReader<Te10_0>;
impl Te10_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te10_0 {
        match self.bits {
            false => Te10_0::Disable,
            true => Te10_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te10_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te10_0::Enable
    }
}
#[doc = "Field `TE_10_0` writer - tamper erase enable SPMEM10 - DATA0"]
pub type Te10_0W<'a, REG> = crate::BitWriter<'a, REG, Te10_0>;
impl<'a, REG> Te10_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM10 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te10_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te10_1> for bool {
    #[inline(always)]
    fn from(variant: Te10_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_10_1` reader - tamper erase enable SPMEM10 - DATA1"]
pub type Te10_1R = crate::BitReader<Te10_1>;
impl Te10_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te10_1 {
        match self.bits {
            false => Te10_1::Disable,
            true => Te10_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te10_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te10_1::Enable
    }
}
#[doc = "Field `TE_10_1` writer - tamper erase enable SPMEM10 - DATA1"]
pub type Te10_1W<'a, REG> = crate::BitWriter<'a, REG, Te10_1>;
impl<'a, REG> Te10_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM10 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te10_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te10_2> for bool {
    #[inline(always)]
    fn from(variant: Te10_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_10_2` reader - tamper erase enable SPMEM10 - DATA2"]
pub type Te10_2R = crate::BitReader<Te10_2>;
impl Te10_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te10_2 {
        match self.bits {
            false => Te10_2::Disable,
            true => Te10_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te10_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te10_2::Enable
    }
}
#[doc = "Field `TE_10_2` writer - tamper erase enable SPMEM10 - DATA2"]
pub type Te10_2W<'a, REG> = crate::BitWriter<'a, REG, Te10_2>;
impl<'a, REG> Te10_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM10 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te10_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te10_3> for bool {
    #[inline(always)]
    fn from(variant: Te10_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_10_3` reader - tamper erase enable SPMEM10 - DATA3"]
pub type Te10_3R = crate::BitReader<Te10_3>;
impl Te10_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te10_3 {
        match self.bits {
            false => Te10_3::Disable,
            true => Te10_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te10_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te10_3::Enable
    }
}
#[doc = "Field `TE_10_3` writer - tamper erase enable SPMEM10 - DATA3"]
pub type Te10_3W<'a, REG> = crate::BitWriter<'a, REG, Te10_3>;
impl<'a, REG> Te10_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te10_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM11 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te11_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te11_0> for bool {
    #[inline(always)]
    fn from(variant: Te11_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_11_0` reader - tamper erase enable SPMEM11 - DATA0"]
pub type Te11_0R = crate::BitReader<Te11_0>;
impl Te11_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te11_0 {
        match self.bits {
            false => Te11_0::Disable,
            true => Te11_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te11_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te11_0::Enable
    }
}
#[doc = "Field `TE_11_0` writer - tamper erase enable SPMEM11 - DATA0"]
pub type Te11_0W<'a, REG> = crate::BitWriter<'a, REG, Te11_0>;
impl<'a, REG> Te11_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM11 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te11_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te11_1> for bool {
    #[inline(always)]
    fn from(variant: Te11_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_11_1` reader - tamper erase enable SPMEM11 - DATA1"]
pub type Te11_1R = crate::BitReader<Te11_1>;
impl Te11_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te11_1 {
        match self.bits {
            false => Te11_1::Disable,
            true => Te11_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te11_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te11_1::Enable
    }
}
#[doc = "Field `TE_11_1` writer - tamper erase enable SPMEM11 - DATA1"]
pub type Te11_1W<'a, REG> = crate::BitWriter<'a, REG, Te11_1>;
impl<'a, REG> Te11_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM11 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te11_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te11_2> for bool {
    #[inline(always)]
    fn from(variant: Te11_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_11_2` reader - tamper erase enable SPMEM11 - DATA2"]
pub type Te11_2R = crate::BitReader<Te11_2>;
impl Te11_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te11_2 {
        match self.bits {
            false => Te11_2::Disable,
            true => Te11_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te11_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te11_2::Enable
    }
}
#[doc = "Field `TE_11_2` writer - tamper erase enable SPMEM11 - DATA2"]
pub type Te11_2W<'a, REG> = crate::BitWriter<'a, REG, Te11_2>;
impl<'a, REG> Te11_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM11 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te11_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te11_3> for bool {
    #[inline(always)]
    fn from(variant: Te11_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_11_3` reader - tamper erase enable SPMEM11 - DATA3"]
pub type Te11_3R = crate::BitReader<Te11_3>;
impl Te11_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te11_3 {
        match self.bits {
            false => Te11_3::Disable,
            true => Te11_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te11_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te11_3::Enable
    }
}
#[doc = "Field `TE_11_3` writer - tamper erase enable SPMEM11 - DATA3"]
pub type Te11_3W<'a, REG> = crate::BitWriter<'a, REG, Te11_3>;
impl<'a, REG> Te11_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te11_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM8 - DATA0"]
    #[inline(always)]
    pub fn te_8_0(&self) -> Te8_0R {
        Te8_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM8 - DATA1"]
    #[inline(always)]
    pub fn te_8_1(&self) -> Te8_1R {
        Te8_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM8 - DATA2"]
    #[inline(always)]
    pub fn te_8_2(&self) -> Te8_2R {
        Te8_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM8 - DATA3"]
    #[inline(always)]
    pub fn te_8_3(&self) -> Te8_3R {
        Te8_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM9 - DATA0"]
    #[inline(always)]
    pub fn te_9_0(&self) -> Te9_0R {
        Te9_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM9 - DATA1"]
    #[inline(always)]
    pub fn te_9_1(&self) -> Te9_1R {
        Te9_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM9 - DATA2"]
    #[inline(always)]
    pub fn te_9_2(&self) -> Te9_2R {
        Te9_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM9 - DATA3"]
    #[inline(always)]
    pub fn te_9_3(&self) -> Te9_3R {
        Te9_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM10 - DATA0"]
    #[inline(always)]
    pub fn te_10_0(&self) -> Te10_0R {
        Te10_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM10 - DATA1"]
    #[inline(always)]
    pub fn te_10_1(&self) -> Te10_1R {
        Te10_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM10 - DATA2"]
    #[inline(always)]
    pub fn te_10_2(&self) -> Te10_2R {
        Te10_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM10 - DATA3"]
    #[inline(always)]
    pub fn te_10_3(&self) -> Te10_3R {
        Te10_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM11 - DATA0"]
    #[inline(always)]
    pub fn te_11_0(&self) -> Te11_0R {
        Te11_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM11 - DATA1"]
    #[inline(always)]
    pub fn te_11_1(&self) -> Te11_1R {
        Te11_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM11 - DATA2"]
    #[inline(always)]
    pub fn te_11_2(&self) -> Te11_2R {
        Te11_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM11 - DATA3"]
    #[inline(always)]
    pub fn te_11_3(&self) -> Te11_3R {
        Te11_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM8 - DATA0"]
    #[inline(always)]
    pub fn te_8_0(&mut self) -> Te8_0W<'_, LfssSpmterase2Spec> {
        Te8_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM8 - DATA1"]
    #[inline(always)]
    pub fn te_8_1(&mut self) -> Te8_1W<'_, LfssSpmterase2Spec> {
        Te8_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM8 - DATA2"]
    #[inline(always)]
    pub fn te_8_2(&mut self) -> Te8_2W<'_, LfssSpmterase2Spec> {
        Te8_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM8 - DATA3"]
    #[inline(always)]
    pub fn te_8_3(&mut self) -> Te8_3W<'_, LfssSpmterase2Spec> {
        Te8_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM9 - DATA0"]
    #[inline(always)]
    pub fn te_9_0(&mut self) -> Te9_0W<'_, LfssSpmterase2Spec> {
        Te9_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM9 - DATA1"]
    #[inline(always)]
    pub fn te_9_1(&mut self) -> Te9_1W<'_, LfssSpmterase2Spec> {
        Te9_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM9 - DATA2"]
    #[inline(always)]
    pub fn te_9_2(&mut self) -> Te9_2W<'_, LfssSpmterase2Spec> {
        Te9_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM9 - DATA3"]
    #[inline(always)]
    pub fn te_9_3(&mut self) -> Te9_3W<'_, LfssSpmterase2Spec> {
        Te9_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM10 - DATA0"]
    #[inline(always)]
    pub fn te_10_0(&mut self) -> Te10_0W<'_, LfssSpmterase2Spec> {
        Te10_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM10 - DATA1"]
    #[inline(always)]
    pub fn te_10_1(&mut self) -> Te10_1W<'_, LfssSpmterase2Spec> {
        Te10_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM10 - DATA2"]
    #[inline(always)]
    pub fn te_10_2(&mut self) -> Te10_2W<'_, LfssSpmterase2Spec> {
        Te10_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM10 - DATA3"]
    #[inline(always)]
    pub fn te_10_3(&mut self) -> Te10_3W<'_, LfssSpmterase2Spec> {
        Te10_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM11 - DATA0"]
    #[inline(always)]
    pub fn te_11_0(&mut self) -> Te11_0W<'_, LfssSpmterase2Spec> {
        Te11_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM11 - DATA1"]
    #[inline(always)]
    pub fn te_11_1(&mut self) -> Te11_1W<'_, LfssSpmterase2Spec> {
        Te11_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM11 - DATA2"]
    #[inline(always)]
    pub fn te_11_2(&mut self) -> Te11_2W<'_, LfssSpmterase2Spec> {
        Te11_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM11 - DATA3"]
    #[inline(always)]
    pub fn te_11_3(&mut self) -> Te11_3W<'_, LfssSpmterase2Spec> {
        Te11_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase2Spec;
impl crate::RegisterSpec for LfssSpmterase2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase2::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase2Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase2::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE2 to value 0"]
impl crate::Resettable for LfssSpmterase2Spec {}
