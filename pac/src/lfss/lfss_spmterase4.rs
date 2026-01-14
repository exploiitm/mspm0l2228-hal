#[doc = "Register `LFSS_SPMTERASE4` reader"]
pub type R = crate::R<LfssSpmterase4Spec>;
#[doc = "Register `LFSS_SPMTERASE4` writer"]
pub type W = crate::W<LfssSpmterase4Spec>;
#[doc = "tamper erase enable SPMEM16 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te16_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te16_0> for bool {
    #[inline(always)]
    fn from(variant: Te16_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_16_0` reader - tamper erase enable SPMEM16 - DATA0"]
pub type Te16_0R = crate::BitReader<Te16_0>;
impl Te16_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te16_0 {
        match self.bits {
            false => Te16_0::Disable,
            true => Te16_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te16_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te16_0::Enable
    }
}
#[doc = "Field `TE_16_0` writer - tamper erase enable SPMEM16 - DATA0"]
pub type Te16_0W<'a, REG> = crate::BitWriter<'a, REG, Te16_0>;
impl<'a, REG> Te16_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM16 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te16_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te16_1> for bool {
    #[inline(always)]
    fn from(variant: Te16_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_16_1` reader - tamper erase enable SPMEM16 - DATA1"]
pub type Te16_1R = crate::BitReader<Te16_1>;
impl Te16_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te16_1 {
        match self.bits {
            false => Te16_1::Disable,
            true => Te16_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te16_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te16_1::Enable
    }
}
#[doc = "Field `TE_16_1` writer - tamper erase enable SPMEM16 - DATA1"]
pub type Te16_1W<'a, REG> = crate::BitWriter<'a, REG, Te16_1>;
impl<'a, REG> Te16_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM16 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te16_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te16_2> for bool {
    #[inline(always)]
    fn from(variant: Te16_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_16_2` reader - tamper erase enable SPMEM16 - DATA2"]
pub type Te16_2R = crate::BitReader<Te16_2>;
impl Te16_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te16_2 {
        match self.bits {
            false => Te16_2::Disable,
            true => Te16_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te16_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te16_2::Enable
    }
}
#[doc = "Field `TE_16_2` writer - tamper erase enable SPMEM16 - DATA2"]
pub type Te16_2W<'a, REG> = crate::BitWriter<'a, REG, Te16_2>;
impl<'a, REG> Te16_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM16 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te16_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te16_3> for bool {
    #[inline(always)]
    fn from(variant: Te16_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_16_3` reader - tamper erase enable SPMEM16 - DATA3"]
pub type Te16_3R = crate::BitReader<Te16_3>;
impl Te16_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te16_3 {
        match self.bits {
            false => Te16_3::Disable,
            true => Te16_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te16_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te16_3::Enable
    }
}
#[doc = "Field `TE_16_3` writer - tamper erase enable SPMEM16 - DATA3"]
pub type Te16_3W<'a, REG> = crate::BitWriter<'a, REG, Te16_3>;
impl<'a, REG> Te16_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te16_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM17 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te17_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te17_0> for bool {
    #[inline(always)]
    fn from(variant: Te17_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_17_0` reader - tamper erase enable SPMEM17 - DATA0"]
pub type Te17_0R = crate::BitReader<Te17_0>;
impl Te17_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te17_0 {
        match self.bits {
            false => Te17_0::Disable,
            true => Te17_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te17_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te17_0::Enable
    }
}
#[doc = "Field `TE_17_0` writer - tamper erase enable SPMEM17 - DATA0"]
pub type Te17_0W<'a, REG> = crate::BitWriter<'a, REG, Te17_0>;
impl<'a, REG> Te17_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM17 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te17_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te17_1> for bool {
    #[inline(always)]
    fn from(variant: Te17_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_17_1` reader - tamper erase enable SPMEM17 - DATA1"]
pub type Te17_1R = crate::BitReader<Te17_1>;
impl Te17_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te17_1 {
        match self.bits {
            false => Te17_1::Disable,
            true => Te17_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te17_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te17_1::Enable
    }
}
#[doc = "Field `TE_17_1` writer - tamper erase enable SPMEM17 - DATA1"]
pub type Te17_1W<'a, REG> = crate::BitWriter<'a, REG, Te17_1>;
impl<'a, REG> Te17_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM17 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te17_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te17_2> for bool {
    #[inline(always)]
    fn from(variant: Te17_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_17_2` reader - tamper erase enable SPMEM17 - DATA2"]
pub type Te17_2R = crate::BitReader<Te17_2>;
impl Te17_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te17_2 {
        match self.bits {
            false => Te17_2::Disable,
            true => Te17_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te17_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te17_2::Enable
    }
}
#[doc = "Field `TE_17_2` writer - tamper erase enable SPMEM17 - DATA2"]
pub type Te17_2W<'a, REG> = crate::BitWriter<'a, REG, Te17_2>;
impl<'a, REG> Te17_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM17 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te17_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te17_3> for bool {
    #[inline(always)]
    fn from(variant: Te17_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_17_3` reader - tamper erase enable SPMEM17 - DATA3"]
pub type Te17_3R = crate::BitReader<Te17_3>;
impl Te17_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te17_3 {
        match self.bits {
            false => Te17_3::Disable,
            true => Te17_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te17_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te17_3::Enable
    }
}
#[doc = "Field `TE_17_3` writer - tamper erase enable SPMEM17 - DATA3"]
pub type Te17_3W<'a, REG> = crate::BitWriter<'a, REG, Te17_3>;
impl<'a, REG> Te17_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te17_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM18 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te18_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te18_0> for bool {
    #[inline(always)]
    fn from(variant: Te18_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_18_0` reader - tamper erase enable SPMEM18 - DATA0"]
pub type Te18_0R = crate::BitReader<Te18_0>;
impl Te18_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te18_0 {
        match self.bits {
            false => Te18_0::Disable,
            true => Te18_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te18_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te18_0::Enable
    }
}
#[doc = "Field `TE_18_0` writer - tamper erase enable SPMEM18 - DATA0"]
pub type Te18_0W<'a, REG> = crate::BitWriter<'a, REG, Te18_0>;
impl<'a, REG> Te18_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM18 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te18_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te18_1> for bool {
    #[inline(always)]
    fn from(variant: Te18_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_18_1` reader - tamper erase enable SPMEM18 - DATA1"]
pub type Te18_1R = crate::BitReader<Te18_1>;
impl Te18_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te18_1 {
        match self.bits {
            false => Te18_1::Disable,
            true => Te18_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te18_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te18_1::Enable
    }
}
#[doc = "Field `TE_18_1` writer - tamper erase enable SPMEM18 - DATA1"]
pub type Te18_1W<'a, REG> = crate::BitWriter<'a, REG, Te18_1>;
impl<'a, REG> Te18_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM18 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te18_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te18_2> for bool {
    #[inline(always)]
    fn from(variant: Te18_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_18_2` reader - tamper erase enable SPMEM18 - DATA2"]
pub type Te18_2R = crate::BitReader<Te18_2>;
impl Te18_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te18_2 {
        match self.bits {
            false => Te18_2::Disable,
            true => Te18_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te18_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te18_2::Enable
    }
}
#[doc = "Field `TE_18_2` writer - tamper erase enable SPMEM18 - DATA2"]
pub type Te18_2W<'a, REG> = crate::BitWriter<'a, REG, Te18_2>;
impl<'a, REG> Te18_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM18 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te18_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te18_3> for bool {
    #[inline(always)]
    fn from(variant: Te18_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_18_3` reader - tamper erase enable SPMEM18 - DATA3"]
pub type Te18_3R = crate::BitReader<Te18_3>;
impl Te18_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te18_3 {
        match self.bits {
            false => Te18_3::Disable,
            true => Te18_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te18_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te18_3::Enable
    }
}
#[doc = "Field `TE_18_3` writer - tamper erase enable SPMEM18 - DATA3"]
pub type Te18_3W<'a, REG> = crate::BitWriter<'a, REG, Te18_3>;
impl<'a, REG> Te18_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te18_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM19 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te19_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te19_0> for bool {
    #[inline(always)]
    fn from(variant: Te19_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_19_0` reader - tamper erase enable SPMEM19 - DATA0"]
pub type Te19_0R = crate::BitReader<Te19_0>;
impl Te19_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te19_0 {
        match self.bits {
            false => Te19_0::Disable,
            true => Te19_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te19_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te19_0::Enable
    }
}
#[doc = "Field `TE_19_0` writer - tamper erase enable SPMEM19 - DATA0"]
pub type Te19_0W<'a, REG> = crate::BitWriter<'a, REG, Te19_0>;
impl<'a, REG> Te19_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM19 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te19_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te19_1> for bool {
    #[inline(always)]
    fn from(variant: Te19_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_19_1` reader - tamper erase enable SPMEM19 - DATA1"]
pub type Te19_1R = crate::BitReader<Te19_1>;
impl Te19_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te19_1 {
        match self.bits {
            false => Te19_1::Disable,
            true => Te19_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te19_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te19_1::Enable
    }
}
#[doc = "Field `TE_19_1` writer - tamper erase enable SPMEM19 - DATA1"]
pub type Te19_1W<'a, REG> = crate::BitWriter<'a, REG, Te19_1>;
impl<'a, REG> Te19_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM19 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te19_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te19_2> for bool {
    #[inline(always)]
    fn from(variant: Te19_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_19_2` reader - tamper erase enable SPMEM19 - DATA2"]
pub type Te19_2R = crate::BitReader<Te19_2>;
impl Te19_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te19_2 {
        match self.bits {
            false => Te19_2::Disable,
            true => Te19_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te19_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te19_2::Enable
    }
}
#[doc = "Field `TE_19_2` writer - tamper erase enable SPMEM19 - DATA2"]
pub type Te19_2W<'a, REG> = crate::BitWriter<'a, REG, Te19_2>;
impl<'a, REG> Te19_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM19 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te19_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te19_3> for bool {
    #[inline(always)]
    fn from(variant: Te19_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_19_3` reader - tamper erase enable SPMEM19 - DATA3"]
pub type Te19_3R = crate::BitReader<Te19_3>;
impl Te19_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te19_3 {
        match self.bits {
            false => Te19_3::Disable,
            true => Te19_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te19_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te19_3::Enable
    }
}
#[doc = "Field `TE_19_3` writer - tamper erase enable SPMEM19 - DATA3"]
pub type Te19_3W<'a, REG> = crate::BitWriter<'a, REG, Te19_3>;
impl<'a, REG> Te19_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te19_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM16 - DATA0"]
    #[inline(always)]
    pub fn te_16_0(&self) -> Te16_0R {
        Te16_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM16 - DATA1"]
    #[inline(always)]
    pub fn te_16_1(&self) -> Te16_1R {
        Te16_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM16 - DATA2"]
    #[inline(always)]
    pub fn te_16_2(&self) -> Te16_2R {
        Te16_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM16 - DATA3"]
    #[inline(always)]
    pub fn te_16_3(&self) -> Te16_3R {
        Te16_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM17 - DATA0"]
    #[inline(always)]
    pub fn te_17_0(&self) -> Te17_0R {
        Te17_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM17 - DATA1"]
    #[inline(always)]
    pub fn te_17_1(&self) -> Te17_1R {
        Te17_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM17 - DATA2"]
    #[inline(always)]
    pub fn te_17_2(&self) -> Te17_2R {
        Te17_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM17 - DATA3"]
    #[inline(always)]
    pub fn te_17_3(&self) -> Te17_3R {
        Te17_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM18 - DATA0"]
    #[inline(always)]
    pub fn te_18_0(&self) -> Te18_0R {
        Te18_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM18 - DATA1"]
    #[inline(always)]
    pub fn te_18_1(&self) -> Te18_1R {
        Te18_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM18 - DATA2"]
    #[inline(always)]
    pub fn te_18_2(&self) -> Te18_2R {
        Te18_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM18 - DATA3"]
    #[inline(always)]
    pub fn te_18_3(&self) -> Te18_3R {
        Te18_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM19 - DATA0"]
    #[inline(always)]
    pub fn te_19_0(&self) -> Te19_0R {
        Te19_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM19 - DATA1"]
    #[inline(always)]
    pub fn te_19_1(&self) -> Te19_1R {
        Te19_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM19 - DATA2"]
    #[inline(always)]
    pub fn te_19_2(&self) -> Te19_2R {
        Te19_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM19 - DATA3"]
    #[inline(always)]
    pub fn te_19_3(&self) -> Te19_3R {
        Te19_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM16 - DATA0"]
    #[inline(always)]
    pub fn te_16_0(&mut self) -> Te16_0W<'_, LfssSpmterase4Spec> {
        Te16_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM16 - DATA1"]
    #[inline(always)]
    pub fn te_16_1(&mut self) -> Te16_1W<'_, LfssSpmterase4Spec> {
        Te16_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM16 - DATA2"]
    #[inline(always)]
    pub fn te_16_2(&mut self) -> Te16_2W<'_, LfssSpmterase4Spec> {
        Te16_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM16 - DATA3"]
    #[inline(always)]
    pub fn te_16_3(&mut self) -> Te16_3W<'_, LfssSpmterase4Spec> {
        Te16_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM17 - DATA0"]
    #[inline(always)]
    pub fn te_17_0(&mut self) -> Te17_0W<'_, LfssSpmterase4Spec> {
        Te17_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM17 - DATA1"]
    #[inline(always)]
    pub fn te_17_1(&mut self) -> Te17_1W<'_, LfssSpmterase4Spec> {
        Te17_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM17 - DATA2"]
    #[inline(always)]
    pub fn te_17_2(&mut self) -> Te17_2W<'_, LfssSpmterase4Spec> {
        Te17_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM17 - DATA3"]
    #[inline(always)]
    pub fn te_17_3(&mut self) -> Te17_3W<'_, LfssSpmterase4Spec> {
        Te17_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM18 - DATA0"]
    #[inline(always)]
    pub fn te_18_0(&mut self) -> Te18_0W<'_, LfssSpmterase4Spec> {
        Te18_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM18 - DATA1"]
    #[inline(always)]
    pub fn te_18_1(&mut self) -> Te18_1W<'_, LfssSpmterase4Spec> {
        Te18_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM18 - DATA2"]
    #[inline(always)]
    pub fn te_18_2(&mut self) -> Te18_2W<'_, LfssSpmterase4Spec> {
        Te18_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM18 - DATA3"]
    #[inline(always)]
    pub fn te_18_3(&mut self) -> Te18_3W<'_, LfssSpmterase4Spec> {
        Te18_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM19 - DATA0"]
    #[inline(always)]
    pub fn te_19_0(&mut self) -> Te19_0W<'_, LfssSpmterase4Spec> {
        Te19_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM19 - DATA1"]
    #[inline(always)]
    pub fn te_19_1(&mut self) -> Te19_1W<'_, LfssSpmterase4Spec> {
        Te19_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM19 - DATA2"]
    #[inline(always)]
    pub fn te_19_2(&mut self) -> Te19_2W<'_, LfssSpmterase4Spec> {
        Te19_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM19 - DATA3"]
    #[inline(always)]
    pub fn te_19_3(&mut self) -> Te19_3W<'_, LfssSpmterase4Spec> {
        Te19_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase4Spec;
impl crate::RegisterSpec for LfssSpmterase4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase4::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase4Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase4::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE4 to value 0"]
impl crate::Resettable for LfssSpmterase4Spec {}
