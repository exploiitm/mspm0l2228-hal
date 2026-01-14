#[doc = "Register `LFSS_SPMTERASE3` reader"]
pub type R = crate::R<LfssSpmterase3Spec>;
#[doc = "Register `LFSS_SPMTERASE3` writer"]
pub type W = crate::W<LfssSpmterase3Spec>;
#[doc = "tamper erase enable SPMEM12 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te12_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te12_0> for bool {
    #[inline(always)]
    fn from(variant: Te12_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_12_0` reader - tamper erase enable SPMEM12 - DATA0"]
pub type Te12_0R = crate::BitReader<Te12_0>;
impl Te12_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te12_0 {
        match self.bits {
            false => Te12_0::Disable,
            true => Te12_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te12_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te12_0::Enable
    }
}
#[doc = "Field `TE_12_0` writer - tamper erase enable SPMEM12 - DATA0"]
pub type Te12_0W<'a, REG> = crate::BitWriter<'a, REG, Te12_0>;
impl<'a, REG> Te12_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM12 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te12_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te12_1> for bool {
    #[inline(always)]
    fn from(variant: Te12_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_12_1` reader - tamper erase enable SPMEM12 - DATA1"]
pub type Te12_1R = crate::BitReader<Te12_1>;
impl Te12_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te12_1 {
        match self.bits {
            false => Te12_1::Disable,
            true => Te12_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te12_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te12_1::Enable
    }
}
#[doc = "Field `TE_12_1` writer - tamper erase enable SPMEM12 - DATA1"]
pub type Te12_1W<'a, REG> = crate::BitWriter<'a, REG, Te12_1>;
impl<'a, REG> Te12_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM12 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te12_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te12_2> for bool {
    #[inline(always)]
    fn from(variant: Te12_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_12_2` reader - tamper erase enable SPMEM12 - DATA2"]
pub type Te12_2R = crate::BitReader<Te12_2>;
impl Te12_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te12_2 {
        match self.bits {
            false => Te12_2::Disable,
            true => Te12_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te12_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te12_2::Enable
    }
}
#[doc = "Field `TE_12_2` writer - tamper erase enable SPMEM12 - DATA2"]
pub type Te12_2W<'a, REG> = crate::BitWriter<'a, REG, Te12_2>;
impl<'a, REG> Te12_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM12 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te12_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te12_3> for bool {
    #[inline(always)]
    fn from(variant: Te12_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_12_3` reader - tamper erase enable SPMEM12 - DATA3"]
pub type Te12_3R = crate::BitReader<Te12_3>;
impl Te12_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te12_3 {
        match self.bits {
            false => Te12_3::Disable,
            true => Te12_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te12_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te12_3::Enable
    }
}
#[doc = "Field `TE_12_3` writer - tamper erase enable SPMEM12 - DATA3"]
pub type Te12_3W<'a, REG> = crate::BitWriter<'a, REG, Te12_3>;
impl<'a, REG> Te12_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te12_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM13 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te13_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te13_0> for bool {
    #[inline(always)]
    fn from(variant: Te13_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_13_0` reader - tamper erase enable SPMEM13 - DATA0"]
pub type Te13_0R = crate::BitReader<Te13_0>;
impl Te13_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te13_0 {
        match self.bits {
            false => Te13_0::Disable,
            true => Te13_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te13_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te13_0::Enable
    }
}
#[doc = "Field `TE_13_0` writer - tamper erase enable SPMEM13 - DATA0"]
pub type Te13_0W<'a, REG> = crate::BitWriter<'a, REG, Te13_0>;
impl<'a, REG> Te13_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM13 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te13_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te13_1> for bool {
    #[inline(always)]
    fn from(variant: Te13_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_13_1` reader - tamper erase enable SPMEM13 - DATA1"]
pub type Te13_1R = crate::BitReader<Te13_1>;
impl Te13_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te13_1 {
        match self.bits {
            false => Te13_1::Disable,
            true => Te13_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te13_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te13_1::Enable
    }
}
#[doc = "Field `TE_13_1` writer - tamper erase enable SPMEM13 - DATA1"]
pub type Te13_1W<'a, REG> = crate::BitWriter<'a, REG, Te13_1>;
impl<'a, REG> Te13_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM13 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te13_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te13_2> for bool {
    #[inline(always)]
    fn from(variant: Te13_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_13_2` reader - tamper erase enable SPMEM13 - DATA2"]
pub type Te13_2R = crate::BitReader<Te13_2>;
impl Te13_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te13_2 {
        match self.bits {
            false => Te13_2::Disable,
            true => Te13_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te13_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te13_2::Enable
    }
}
#[doc = "Field `TE_13_2` writer - tamper erase enable SPMEM13 - DATA2"]
pub type Te13_2W<'a, REG> = crate::BitWriter<'a, REG, Te13_2>;
impl<'a, REG> Te13_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM13 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te13_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te13_3> for bool {
    #[inline(always)]
    fn from(variant: Te13_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_13_3` reader - tamper erase enable SPMEM13 - DATA3"]
pub type Te13_3R = crate::BitReader<Te13_3>;
impl Te13_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te13_3 {
        match self.bits {
            false => Te13_3::Disable,
            true => Te13_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te13_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te13_3::Enable
    }
}
#[doc = "Field `TE_13_3` writer - tamper erase enable SPMEM13 - DATA3"]
pub type Te13_3W<'a, REG> = crate::BitWriter<'a, REG, Te13_3>;
impl<'a, REG> Te13_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te13_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM14 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te14_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te14_0> for bool {
    #[inline(always)]
    fn from(variant: Te14_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_14_0` reader - tamper erase enable SPMEM14 - DATA0"]
pub type Te14_0R = crate::BitReader<Te14_0>;
impl Te14_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te14_0 {
        match self.bits {
            false => Te14_0::Disable,
            true => Te14_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te14_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te14_0::Enable
    }
}
#[doc = "Field `TE_14_0` writer - tamper erase enable SPMEM14 - DATA0"]
pub type Te14_0W<'a, REG> = crate::BitWriter<'a, REG, Te14_0>;
impl<'a, REG> Te14_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM14 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te14_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te14_1> for bool {
    #[inline(always)]
    fn from(variant: Te14_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_14_1` reader - tamper erase enable SPMEM14 - DATA1"]
pub type Te14_1R = crate::BitReader<Te14_1>;
impl Te14_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te14_1 {
        match self.bits {
            false => Te14_1::Disable,
            true => Te14_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te14_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te14_1::Enable
    }
}
#[doc = "Field `TE_14_1` writer - tamper erase enable SPMEM14 - DATA1"]
pub type Te14_1W<'a, REG> = crate::BitWriter<'a, REG, Te14_1>;
impl<'a, REG> Te14_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM14 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te14_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te14_2> for bool {
    #[inline(always)]
    fn from(variant: Te14_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_14_2` reader - tamper erase enable SPMEM14 - DATA2"]
pub type Te14_2R = crate::BitReader<Te14_2>;
impl Te14_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te14_2 {
        match self.bits {
            false => Te14_2::Disable,
            true => Te14_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te14_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te14_2::Enable
    }
}
#[doc = "Field `TE_14_2` writer - tamper erase enable SPMEM14 - DATA2"]
pub type Te14_2W<'a, REG> = crate::BitWriter<'a, REG, Te14_2>;
impl<'a, REG> Te14_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM14 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te14_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te14_3> for bool {
    #[inline(always)]
    fn from(variant: Te14_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_14_3` reader - tamper erase enable SPMEM14 - DATA3"]
pub type Te14_3R = crate::BitReader<Te14_3>;
impl Te14_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te14_3 {
        match self.bits {
            false => Te14_3::Disable,
            true => Te14_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te14_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te14_3::Enable
    }
}
#[doc = "Field `TE_14_3` writer - tamper erase enable SPMEM14 - DATA3"]
pub type Te14_3W<'a, REG> = crate::BitWriter<'a, REG, Te14_3>;
impl<'a, REG> Te14_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te14_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM15 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te15_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te15_0> for bool {
    #[inline(always)]
    fn from(variant: Te15_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_15_0` reader - tamper erase enable SPMEM15 - DATA0"]
pub type Te15_0R = crate::BitReader<Te15_0>;
impl Te15_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te15_0 {
        match self.bits {
            false => Te15_0::Disable,
            true => Te15_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te15_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te15_0::Enable
    }
}
#[doc = "Field `TE_15_0` writer - tamper erase enable SPMEM15 - DATA0"]
pub type Te15_0W<'a, REG> = crate::BitWriter<'a, REG, Te15_0>;
impl<'a, REG> Te15_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM15 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te15_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te15_1> for bool {
    #[inline(always)]
    fn from(variant: Te15_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_15_1` reader - tamper erase enable SPMEM15 - DATA1"]
pub type Te15_1R = crate::BitReader<Te15_1>;
impl Te15_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te15_1 {
        match self.bits {
            false => Te15_1::Disable,
            true => Te15_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te15_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te15_1::Enable
    }
}
#[doc = "Field `TE_15_1` writer - tamper erase enable SPMEM15 - DATA1"]
pub type Te15_1W<'a, REG> = crate::BitWriter<'a, REG, Te15_1>;
impl<'a, REG> Te15_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM15 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te15_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te15_2> for bool {
    #[inline(always)]
    fn from(variant: Te15_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_15_2` reader - tamper erase enable SPMEM15 - DATA2"]
pub type Te15_2R = crate::BitReader<Te15_2>;
impl Te15_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te15_2 {
        match self.bits {
            false => Te15_2::Disable,
            true => Te15_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te15_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te15_2::Enable
    }
}
#[doc = "Field `TE_15_2` writer - tamper erase enable SPMEM15 - DATA2"]
pub type Te15_2W<'a, REG> = crate::BitWriter<'a, REG, Te15_2>;
impl<'a, REG> Te15_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM15 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te15_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te15_3> for bool {
    #[inline(always)]
    fn from(variant: Te15_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_15_3` reader - tamper erase enable SPMEM15 - DATA3"]
pub type Te15_3R = crate::BitReader<Te15_3>;
impl Te15_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te15_3 {
        match self.bits {
            false => Te15_3::Disable,
            true => Te15_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te15_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te15_3::Enable
    }
}
#[doc = "Field `TE_15_3` writer - tamper erase enable SPMEM15 - DATA3"]
pub type Te15_3W<'a, REG> = crate::BitWriter<'a, REG, Te15_3>;
impl<'a, REG> Te15_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te15_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM12 - DATA0"]
    #[inline(always)]
    pub fn te_12_0(&self) -> Te12_0R {
        Te12_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM12 - DATA1"]
    #[inline(always)]
    pub fn te_12_1(&self) -> Te12_1R {
        Te12_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM12 - DATA2"]
    #[inline(always)]
    pub fn te_12_2(&self) -> Te12_2R {
        Te12_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM12 - DATA3"]
    #[inline(always)]
    pub fn te_12_3(&self) -> Te12_3R {
        Te12_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM13 - DATA0"]
    #[inline(always)]
    pub fn te_13_0(&self) -> Te13_0R {
        Te13_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM13 - DATA1"]
    #[inline(always)]
    pub fn te_13_1(&self) -> Te13_1R {
        Te13_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM13 - DATA2"]
    #[inline(always)]
    pub fn te_13_2(&self) -> Te13_2R {
        Te13_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM13 - DATA3"]
    #[inline(always)]
    pub fn te_13_3(&self) -> Te13_3R {
        Te13_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM14 - DATA0"]
    #[inline(always)]
    pub fn te_14_0(&self) -> Te14_0R {
        Te14_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM14 - DATA1"]
    #[inline(always)]
    pub fn te_14_1(&self) -> Te14_1R {
        Te14_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM14 - DATA2"]
    #[inline(always)]
    pub fn te_14_2(&self) -> Te14_2R {
        Te14_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM14 - DATA3"]
    #[inline(always)]
    pub fn te_14_3(&self) -> Te14_3R {
        Te14_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM15 - DATA0"]
    #[inline(always)]
    pub fn te_15_0(&self) -> Te15_0R {
        Te15_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM15 - DATA1"]
    #[inline(always)]
    pub fn te_15_1(&self) -> Te15_1R {
        Te15_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM15 - DATA2"]
    #[inline(always)]
    pub fn te_15_2(&self) -> Te15_2R {
        Te15_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM15 - DATA3"]
    #[inline(always)]
    pub fn te_15_3(&self) -> Te15_3R {
        Te15_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM12 - DATA0"]
    #[inline(always)]
    pub fn te_12_0(&mut self) -> Te12_0W<'_, LfssSpmterase3Spec> {
        Te12_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM12 - DATA1"]
    #[inline(always)]
    pub fn te_12_1(&mut self) -> Te12_1W<'_, LfssSpmterase3Spec> {
        Te12_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM12 - DATA2"]
    #[inline(always)]
    pub fn te_12_2(&mut self) -> Te12_2W<'_, LfssSpmterase3Spec> {
        Te12_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM12 - DATA3"]
    #[inline(always)]
    pub fn te_12_3(&mut self) -> Te12_3W<'_, LfssSpmterase3Spec> {
        Te12_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM13 - DATA0"]
    #[inline(always)]
    pub fn te_13_0(&mut self) -> Te13_0W<'_, LfssSpmterase3Spec> {
        Te13_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM13 - DATA1"]
    #[inline(always)]
    pub fn te_13_1(&mut self) -> Te13_1W<'_, LfssSpmterase3Spec> {
        Te13_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM13 - DATA2"]
    #[inline(always)]
    pub fn te_13_2(&mut self) -> Te13_2W<'_, LfssSpmterase3Spec> {
        Te13_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM13 - DATA3"]
    #[inline(always)]
    pub fn te_13_3(&mut self) -> Te13_3W<'_, LfssSpmterase3Spec> {
        Te13_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM14 - DATA0"]
    #[inline(always)]
    pub fn te_14_0(&mut self) -> Te14_0W<'_, LfssSpmterase3Spec> {
        Te14_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM14 - DATA1"]
    #[inline(always)]
    pub fn te_14_1(&mut self) -> Te14_1W<'_, LfssSpmterase3Spec> {
        Te14_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM14 - DATA2"]
    #[inline(always)]
    pub fn te_14_2(&mut self) -> Te14_2W<'_, LfssSpmterase3Spec> {
        Te14_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM14 - DATA3"]
    #[inline(always)]
    pub fn te_14_3(&mut self) -> Te14_3W<'_, LfssSpmterase3Spec> {
        Te14_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM15 - DATA0"]
    #[inline(always)]
    pub fn te_15_0(&mut self) -> Te15_0W<'_, LfssSpmterase3Spec> {
        Te15_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM15 - DATA1"]
    #[inline(always)]
    pub fn te_15_1(&mut self) -> Te15_1W<'_, LfssSpmterase3Spec> {
        Te15_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM15 - DATA2"]
    #[inline(always)]
    pub fn te_15_2(&mut self) -> Te15_2W<'_, LfssSpmterase3Spec> {
        Te15_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM15 - DATA3"]
    #[inline(always)]
    pub fn te_15_3(&mut self) -> Te15_3W<'_, LfssSpmterase3Spec> {
        Te15_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase3Spec;
impl crate::RegisterSpec for LfssSpmterase3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase3::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase3Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase3::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE3 to value 0"]
impl crate::Resettable for LfssSpmterase3Spec {}
