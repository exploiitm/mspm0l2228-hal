#[doc = "Register `LFSS_SPMTERASE7` reader"]
pub type R = crate::R<LfssSpmterase7Spec>;
#[doc = "Register `LFSS_SPMTERASE7` writer"]
pub type W = crate::W<LfssSpmterase7Spec>;
#[doc = "tamper erase enable SPMEM28 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te28_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te28_0> for bool {
    #[inline(always)]
    fn from(variant: Te28_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_28_0` reader - tamper erase enable SPMEM28 - DATA0"]
pub type Te28_0R = crate::BitReader<Te28_0>;
impl Te28_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te28_0 {
        match self.bits {
            false => Te28_0::Disable,
            true => Te28_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te28_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te28_0::Enable
    }
}
#[doc = "Field `TE_28_0` writer - tamper erase enable SPMEM28 - DATA0"]
pub type Te28_0W<'a, REG> = crate::BitWriter<'a, REG, Te28_0>;
impl<'a, REG> Te28_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM28 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te28_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te28_1> for bool {
    #[inline(always)]
    fn from(variant: Te28_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_28_1` reader - tamper erase enable SPMEM28 - DATA1"]
pub type Te28_1R = crate::BitReader<Te28_1>;
impl Te28_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te28_1 {
        match self.bits {
            false => Te28_1::Disable,
            true => Te28_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te28_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te28_1::Enable
    }
}
#[doc = "Field `TE_28_1` writer - tamper erase enable SPMEM28 - DATA1"]
pub type Te28_1W<'a, REG> = crate::BitWriter<'a, REG, Te28_1>;
impl<'a, REG> Te28_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM28 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te28_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te28_2> for bool {
    #[inline(always)]
    fn from(variant: Te28_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_28_2` reader - tamper erase enable SPMEM28 - DATA2"]
pub type Te28_2R = crate::BitReader<Te28_2>;
impl Te28_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te28_2 {
        match self.bits {
            false => Te28_2::Disable,
            true => Te28_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te28_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te28_2::Enable
    }
}
#[doc = "Field `TE_28_2` writer - tamper erase enable SPMEM28 - DATA2"]
pub type Te28_2W<'a, REG> = crate::BitWriter<'a, REG, Te28_2>;
impl<'a, REG> Te28_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM28 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te28_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te28_3> for bool {
    #[inline(always)]
    fn from(variant: Te28_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_28_3` reader - tamper erase enable SPMEM28 - DATA3"]
pub type Te28_3R = crate::BitReader<Te28_3>;
impl Te28_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te28_3 {
        match self.bits {
            false => Te28_3::Disable,
            true => Te28_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te28_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te28_3::Enable
    }
}
#[doc = "Field `TE_28_3` writer - tamper erase enable SPMEM28 - DATA3"]
pub type Te28_3W<'a, REG> = crate::BitWriter<'a, REG, Te28_3>;
impl<'a, REG> Te28_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te28_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM29 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te29_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te29_0> for bool {
    #[inline(always)]
    fn from(variant: Te29_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_29_0` reader - tamper erase enable SPMEM29 - DATA0"]
pub type Te29_0R = crate::BitReader<Te29_0>;
impl Te29_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te29_0 {
        match self.bits {
            false => Te29_0::Disable,
            true => Te29_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te29_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te29_0::Enable
    }
}
#[doc = "Field `TE_29_0` writer - tamper erase enable SPMEM29 - DATA0"]
pub type Te29_0W<'a, REG> = crate::BitWriter<'a, REG, Te29_0>;
impl<'a, REG> Te29_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM29 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te29_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te29_1> for bool {
    #[inline(always)]
    fn from(variant: Te29_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_29_1` reader - tamper erase enable SPMEM29 - DATA1"]
pub type Te29_1R = crate::BitReader<Te29_1>;
impl Te29_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te29_1 {
        match self.bits {
            false => Te29_1::Disable,
            true => Te29_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te29_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te29_1::Enable
    }
}
#[doc = "Field `TE_29_1` writer - tamper erase enable SPMEM29 - DATA1"]
pub type Te29_1W<'a, REG> = crate::BitWriter<'a, REG, Te29_1>;
impl<'a, REG> Te29_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM29 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te29_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te29_2> for bool {
    #[inline(always)]
    fn from(variant: Te29_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_29_2` reader - tamper erase enable SPMEM29 - DATA2"]
pub type Te29_2R = crate::BitReader<Te29_2>;
impl Te29_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te29_2 {
        match self.bits {
            false => Te29_2::Disable,
            true => Te29_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te29_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te29_2::Enable
    }
}
#[doc = "Field `TE_29_2` writer - tamper erase enable SPMEM29 - DATA2"]
pub type Te29_2W<'a, REG> = crate::BitWriter<'a, REG, Te29_2>;
impl<'a, REG> Te29_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM29 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te29_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te29_3> for bool {
    #[inline(always)]
    fn from(variant: Te29_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_29_3` reader - tamper erase enable SPMEM29 - DATA3"]
pub type Te29_3R = crate::BitReader<Te29_3>;
impl Te29_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te29_3 {
        match self.bits {
            false => Te29_3::Disable,
            true => Te29_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te29_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te29_3::Enable
    }
}
#[doc = "Field `TE_29_3` writer - tamper erase enable SPMEM29 - DATA3"]
pub type Te29_3W<'a, REG> = crate::BitWriter<'a, REG, Te29_3>;
impl<'a, REG> Te29_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te29_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM30 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te30_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te30_0> for bool {
    #[inline(always)]
    fn from(variant: Te30_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_30_0` reader - tamper erase enable SPMEM30 - DATA0"]
pub type Te30_0R = crate::BitReader<Te30_0>;
impl Te30_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te30_0 {
        match self.bits {
            false => Te30_0::Disable,
            true => Te30_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te30_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te30_0::Enable
    }
}
#[doc = "Field `TE_30_0` writer - tamper erase enable SPMEM30 - DATA0"]
pub type Te30_0W<'a, REG> = crate::BitWriter<'a, REG, Te30_0>;
impl<'a, REG> Te30_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM30 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te30_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te30_1> for bool {
    #[inline(always)]
    fn from(variant: Te30_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_30_1` reader - tamper erase enable SPMEM30 - DATA1"]
pub type Te30_1R = crate::BitReader<Te30_1>;
impl Te30_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te30_1 {
        match self.bits {
            false => Te30_1::Disable,
            true => Te30_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te30_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te30_1::Enable
    }
}
#[doc = "Field `TE_30_1` writer - tamper erase enable SPMEM30 - DATA1"]
pub type Te30_1W<'a, REG> = crate::BitWriter<'a, REG, Te30_1>;
impl<'a, REG> Te30_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM30 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te30_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te30_2> for bool {
    #[inline(always)]
    fn from(variant: Te30_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_30_2` reader - tamper erase enable SPMEM30 - DATA2"]
pub type Te30_2R = crate::BitReader<Te30_2>;
impl Te30_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te30_2 {
        match self.bits {
            false => Te30_2::Disable,
            true => Te30_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te30_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te30_2::Enable
    }
}
#[doc = "Field `TE_30_2` writer - tamper erase enable SPMEM30 - DATA2"]
pub type Te30_2W<'a, REG> = crate::BitWriter<'a, REG, Te30_2>;
impl<'a, REG> Te30_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM30 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te30_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te30_3> for bool {
    #[inline(always)]
    fn from(variant: Te30_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_30_3` reader - tamper erase enable SPMEM30 - DATA3"]
pub type Te30_3R = crate::BitReader<Te30_3>;
impl Te30_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te30_3 {
        match self.bits {
            false => Te30_3::Disable,
            true => Te30_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te30_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te30_3::Enable
    }
}
#[doc = "Field `TE_30_3` writer - tamper erase enable SPMEM30 - DATA3"]
pub type Te30_3W<'a, REG> = crate::BitWriter<'a, REG, Te30_3>;
impl<'a, REG> Te30_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te30_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM31 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te31_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te31_0> for bool {
    #[inline(always)]
    fn from(variant: Te31_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_31_0` reader - tamper erase enable SPMEM31 - DATA0"]
pub type Te31_0R = crate::BitReader<Te31_0>;
impl Te31_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te31_0 {
        match self.bits {
            false => Te31_0::Disable,
            true => Te31_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te31_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te31_0::Enable
    }
}
#[doc = "Field `TE_31_0` writer - tamper erase enable SPMEM31 - DATA0"]
pub type Te31_0W<'a, REG> = crate::BitWriter<'a, REG, Te31_0>;
impl<'a, REG> Te31_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM31 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te31_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te31_1> for bool {
    #[inline(always)]
    fn from(variant: Te31_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_31_1` reader - tamper erase enable SPMEM31 - DATA1"]
pub type Te31_1R = crate::BitReader<Te31_1>;
impl Te31_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te31_1 {
        match self.bits {
            false => Te31_1::Disable,
            true => Te31_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te31_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te31_1::Enable
    }
}
#[doc = "Field `TE_31_1` writer - tamper erase enable SPMEM31 - DATA1"]
pub type Te31_1W<'a, REG> = crate::BitWriter<'a, REG, Te31_1>;
impl<'a, REG> Te31_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM31 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te31_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te31_2> for bool {
    #[inline(always)]
    fn from(variant: Te31_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_31_2` reader - tamper erase enable SPMEM31 - DATA2"]
pub type Te31_2R = crate::BitReader<Te31_2>;
impl Te31_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te31_2 {
        match self.bits {
            false => Te31_2::Disable,
            true => Te31_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te31_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te31_2::Enable
    }
}
#[doc = "Field `TE_31_2` writer - tamper erase enable SPMEM31 - DATA2"]
pub type Te31_2W<'a, REG> = crate::BitWriter<'a, REG, Te31_2>;
impl<'a, REG> Te31_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM31 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te31_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te31_3> for bool {
    #[inline(always)]
    fn from(variant: Te31_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_31_3` reader - tamper erase enable SPMEM31 - DATA3"]
pub type Te31_3R = crate::BitReader<Te31_3>;
impl Te31_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te31_3 {
        match self.bits {
            false => Te31_3::Disable,
            true => Te31_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te31_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te31_3::Enable
    }
}
#[doc = "Field `TE_31_3` writer - tamper erase enable SPMEM31 - DATA3"]
pub type Te31_3W<'a, REG> = crate::BitWriter<'a, REG, Te31_3>;
impl<'a, REG> Te31_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te31_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM28 - DATA0"]
    #[inline(always)]
    pub fn te_28_0(&self) -> Te28_0R {
        Te28_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM28 - DATA1"]
    #[inline(always)]
    pub fn te_28_1(&self) -> Te28_1R {
        Te28_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM28 - DATA2"]
    #[inline(always)]
    pub fn te_28_2(&self) -> Te28_2R {
        Te28_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM28 - DATA3"]
    #[inline(always)]
    pub fn te_28_3(&self) -> Te28_3R {
        Te28_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM29 - DATA0"]
    #[inline(always)]
    pub fn te_29_0(&self) -> Te29_0R {
        Te29_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM29 - DATA1"]
    #[inline(always)]
    pub fn te_29_1(&self) -> Te29_1R {
        Te29_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM29 - DATA2"]
    #[inline(always)]
    pub fn te_29_2(&self) -> Te29_2R {
        Te29_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM29 - DATA3"]
    #[inline(always)]
    pub fn te_29_3(&self) -> Te29_3R {
        Te29_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM30 - DATA0"]
    #[inline(always)]
    pub fn te_30_0(&self) -> Te30_0R {
        Te30_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM30 - DATA1"]
    #[inline(always)]
    pub fn te_30_1(&self) -> Te30_1R {
        Te30_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM30 - DATA2"]
    #[inline(always)]
    pub fn te_30_2(&self) -> Te30_2R {
        Te30_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM30 - DATA3"]
    #[inline(always)]
    pub fn te_30_3(&self) -> Te30_3R {
        Te30_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM31 - DATA0"]
    #[inline(always)]
    pub fn te_31_0(&self) -> Te31_0R {
        Te31_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM31 - DATA1"]
    #[inline(always)]
    pub fn te_31_1(&self) -> Te31_1R {
        Te31_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM31 - DATA2"]
    #[inline(always)]
    pub fn te_31_2(&self) -> Te31_2R {
        Te31_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM31 - DATA3"]
    #[inline(always)]
    pub fn te_31_3(&self) -> Te31_3R {
        Te31_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM28 - DATA0"]
    #[inline(always)]
    pub fn te_28_0(&mut self) -> Te28_0W<'_, LfssSpmterase7Spec> {
        Te28_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM28 - DATA1"]
    #[inline(always)]
    pub fn te_28_1(&mut self) -> Te28_1W<'_, LfssSpmterase7Spec> {
        Te28_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM28 - DATA2"]
    #[inline(always)]
    pub fn te_28_2(&mut self) -> Te28_2W<'_, LfssSpmterase7Spec> {
        Te28_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM28 - DATA3"]
    #[inline(always)]
    pub fn te_28_3(&mut self) -> Te28_3W<'_, LfssSpmterase7Spec> {
        Te28_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM29 - DATA0"]
    #[inline(always)]
    pub fn te_29_0(&mut self) -> Te29_0W<'_, LfssSpmterase7Spec> {
        Te29_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM29 - DATA1"]
    #[inline(always)]
    pub fn te_29_1(&mut self) -> Te29_1W<'_, LfssSpmterase7Spec> {
        Te29_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM29 - DATA2"]
    #[inline(always)]
    pub fn te_29_2(&mut self) -> Te29_2W<'_, LfssSpmterase7Spec> {
        Te29_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM29 - DATA3"]
    #[inline(always)]
    pub fn te_29_3(&mut self) -> Te29_3W<'_, LfssSpmterase7Spec> {
        Te29_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM30 - DATA0"]
    #[inline(always)]
    pub fn te_30_0(&mut self) -> Te30_0W<'_, LfssSpmterase7Spec> {
        Te30_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM30 - DATA1"]
    #[inline(always)]
    pub fn te_30_1(&mut self) -> Te30_1W<'_, LfssSpmterase7Spec> {
        Te30_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM30 - DATA2"]
    #[inline(always)]
    pub fn te_30_2(&mut self) -> Te30_2W<'_, LfssSpmterase7Spec> {
        Te30_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM30 - DATA3"]
    #[inline(always)]
    pub fn te_30_3(&mut self) -> Te30_3W<'_, LfssSpmterase7Spec> {
        Te30_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM31 - DATA0"]
    #[inline(always)]
    pub fn te_31_0(&mut self) -> Te31_0W<'_, LfssSpmterase7Spec> {
        Te31_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM31 - DATA1"]
    #[inline(always)]
    pub fn te_31_1(&mut self) -> Te31_1W<'_, LfssSpmterase7Spec> {
        Te31_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM31 - DATA2"]
    #[inline(always)]
    pub fn te_31_2(&mut self) -> Te31_2W<'_, LfssSpmterase7Spec> {
        Te31_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM31 - DATA3"]
    #[inline(always)]
    pub fn te_31_3(&mut self) -> Te31_3W<'_, LfssSpmterase7Spec> {
        Te31_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase7Spec;
impl crate::RegisterSpec for LfssSpmterase7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase7::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase7Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase7::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE7 to value 0"]
impl crate::Resettable for LfssSpmterase7Spec {}
