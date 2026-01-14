#[doc = "Register `LFSS_SPMTERASE6` reader"]
pub type R = crate::R<LfssSpmterase6Spec>;
#[doc = "Register `LFSS_SPMTERASE6` writer"]
pub type W = crate::W<LfssSpmterase6Spec>;
#[doc = "tamper erase enable SPMEM24 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te24_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te24_0> for bool {
    #[inline(always)]
    fn from(variant: Te24_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_24_0` reader - tamper erase enable SPMEM24 - DATA0"]
pub type Te24_0R = crate::BitReader<Te24_0>;
impl Te24_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te24_0 {
        match self.bits {
            false => Te24_0::Disable,
            true => Te24_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te24_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te24_0::Enable
    }
}
#[doc = "Field `TE_24_0` writer - tamper erase enable SPMEM24 - DATA0"]
pub type Te24_0W<'a, REG> = crate::BitWriter<'a, REG, Te24_0>;
impl<'a, REG> Te24_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM24 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te24_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te24_1> for bool {
    #[inline(always)]
    fn from(variant: Te24_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_24_1` reader - tamper erase enable SPMEM24 - DATA1"]
pub type Te24_1R = crate::BitReader<Te24_1>;
impl Te24_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te24_1 {
        match self.bits {
            false => Te24_1::Disable,
            true => Te24_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te24_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te24_1::Enable
    }
}
#[doc = "Field `TE_24_1` writer - tamper erase enable SPMEM24 - DATA1"]
pub type Te24_1W<'a, REG> = crate::BitWriter<'a, REG, Te24_1>;
impl<'a, REG> Te24_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM24 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te24_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te24_2> for bool {
    #[inline(always)]
    fn from(variant: Te24_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_24_2` reader - tamper erase enable SPMEM24 - DATA2"]
pub type Te24_2R = crate::BitReader<Te24_2>;
impl Te24_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te24_2 {
        match self.bits {
            false => Te24_2::Disable,
            true => Te24_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te24_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te24_2::Enable
    }
}
#[doc = "Field `TE_24_2` writer - tamper erase enable SPMEM24 - DATA2"]
pub type Te24_2W<'a, REG> = crate::BitWriter<'a, REG, Te24_2>;
impl<'a, REG> Te24_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM24 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te24_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te24_3> for bool {
    #[inline(always)]
    fn from(variant: Te24_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_24_3` reader - tamper erase enable SPMEM24 - DATA3"]
pub type Te24_3R = crate::BitReader<Te24_3>;
impl Te24_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te24_3 {
        match self.bits {
            false => Te24_3::Disable,
            true => Te24_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te24_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te24_3::Enable
    }
}
#[doc = "Field `TE_24_3` writer - tamper erase enable SPMEM24 - DATA3"]
pub type Te24_3W<'a, REG> = crate::BitWriter<'a, REG, Te24_3>;
impl<'a, REG> Te24_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te24_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM25 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te25_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te25_0> for bool {
    #[inline(always)]
    fn from(variant: Te25_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_25_0` reader - tamper erase enable SPMEM25 - DATA0"]
pub type Te25_0R = crate::BitReader<Te25_0>;
impl Te25_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te25_0 {
        match self.bits {
            false => Te25_0::Disable,
            true => Te25_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te25_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te25_0::Enable
    }
}
#[doc = "Field `TE_25_0` writer - tamper erase enable SPMEM25 - DATA0"]
pub type Te25_0W<'a, REG> = crate::BitWriter<'a, REG, Te25_0>;
impl<'a, REG> Te25_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM25 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te25_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te25_1> for bool {
    #[inline(always)]
    fn from(variant: Te25_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_25_1` reader - tamper erase enable SPMEM25 - DATA1"]
pub type Te25_1R = crate::BitReader<Te25_1>;
impl Te25_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te25_1 {
        match self.bits {
            false => Te25_1::Disable,
            true => Te25_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te25_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te25_1::Enable
    }
}
#[doc = "Field `TE_25_1` writer - tamper erase enable SPMEM25 - DATA1"]
pub type Te25_1W<'a, REG> = crate::BitWriter<'a, REG, Te25_1>;
impl<'a, REG> Te25_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM25 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te25_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te25_2> for bool {
    #[inline(always)]
    fn from(variant: Te25_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_25_2` reader - tamper erase enable SPMEM25 - DATA2"]
pub type Te25_2R = crate::BitReader<Te25_2>;
impl Te25_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te25_2 {
        match self.bits {
            false => Te25_2::Disable,
            true => Te25_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te25_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te25_2::Enable
    }
}
#[doc = "Field `TE_25_2` writer - tamper erase enable SPMEM25 - DATA2"]
pub type Te25_2W<'a, REG> = crate::BitWriter<'a, REG, Te25_2>;
impl<'a, REG> Te25_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM25 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te25_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te25_3> for bool {
    #[inline(always)]
    fn from(variant: Te25_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_25_3` reader - tamper erase enable SPMEM25 - DATA3"]
pub type Te25_3R = crate::BitReader<Te25_3>;
impl Te25_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te25_3 {
        match self.bits {
            false => Te25_3::Disable,
            true => Te25_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te25_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te25_3::Enable
    }
}
#[doc = "Field `TE_25_3` writer - tamper erase enable SPMEM25 - DATA3"]
pub type Te25_3W<'a, REG> = crate::BitWriter<'a, REG, Te25_3>;
impl<'a, REG> Te25_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te25_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM26 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te26_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te26_0> for bool {
    #[inline(always)]
    fn from(variant: Te26_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_26_0` reader - tamper erase enable SPMEM26 - DATA0"]
pub type Te26_0R = crate::BitReader<Te26_0>;
impl Te26_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te26_0 {
        match self.bits {
            false => Te26_0::Disable,
            true => Te26_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te26_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te26_0::Enable
    }
}
#[doc = "Field `TE_26_0` writer - tamper erase enable SPMEM26 - DATA0"]
pub type Te26_0W<'a, REG> = crate::BitWriter<'a, REG, Te26_0>;
impl<'a, REG> Te26_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM26 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te26_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te26_1> for bool {
    #[inline(always)]
    fn from(variant: Te26_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_26_1` reader - tamper erase enable SPMEM26 - DATA1"]
pub type Te26_1R = crate::BitReader<Te26_1>;
impl Te26_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te26_1 {
        match self.bits {
            false => Te26_1::Disable,
            true => Te26_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te26_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te26_1::Enable
    }
}
#[doc = "Field `TE_26_1` writer - tamper erase enable SPMEM26 - DATA1"]
pub type Te26_1W<'a, REG> = crate::BitWriter<'a, REG, Te26_1>;
impl<'a, REG> Te26_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM26 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te26_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te26_2> for bool {
    #[inline(always)]
    fn from(variant: Te26_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_26_2` reader - tamper erase enable SPMEM26 - DATA2"]
pub type Te26_2R = crate::BitReader<Te26_2>;
impl Te26_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te26_2 {
        match self.bits {
            false => Te26_2::Disable,
            true => Te26_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te26_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te26_2::Enable
    }
}
#[doc = "Field `TE_26_2` writer - tamper erase enable SPMEM26 - DATA2"]
pub type Te26_2W<'a, REG> = crate::BitWriter<'a, REG, Te26_2>;
impl<'a, REG> Te26_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM26 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te26_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te26_3> for bool {
    #[inline(always)]
    fn from(variant: Te26_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_26_3` reader - tamper erase enable SPMEM26 - DATA3"]
pub type Te26_3R = crate::BitReader<Te26_3>;
impl Te26_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te26_3 {
        match self.bits {
            false => Te26_3::Disable,
            true => Te26_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te26_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te26_3::Enable
    }
}
#[doc = "Field `TE_26_3` writer - tamper erase enable SPMEM26 - DATA3"]
pub type Te26_3W<'a, REG> = crate::BitWriter<'a, REG, Te26_3>;
impl<'a, REG> Te26_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te26_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM27 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te27_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te27_0> for bool {
    #[inline(always)]
    fn from(variant: Te27_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_27_0` reader - tamper erase enable SPMEM27 - DATA0"]
pub type Te27_0R = crate::BitReader<Te27_0>;
impl Te27_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te27_0 {
        match self.bits {
            false => Te27_0::Disable,
            true => Te27_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te27_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te27_0::Enable
    }
}
#[doc = "Field `TE_27_0` writer - tamper erase enable SPMEM27 - DATA0"]
pub type Te27_0W<'a, REG> = crate::BitWriter<'a, REG, Te27_0>;
impl<'a, REG> Te27_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM27 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te27_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te27_1> for bool {
    #[inline(always)]
    fn from(variant: Te27_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_27_1` reader - tamper erase enable SPMEM27 - DATA1"]
pub type Te27_1R = crate::BitReader<Te27_1>;
impl Te27_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te27_1 {
        match self.bits {
            false => Te27_1::Disable,
            true => Te27_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te27_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te27_1::Enable
    }
}
#[doc = "Field `TE_27_1` writer - tamper erase enable SPMEM27 - DATA1"]
pub type Te27_1W<'a, REG> = crate::BitWriter<'a, REG, Te27_1>;
impl<'a, REG> Te27_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM27 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te27_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te27_2> for bool {
    #[inline(always)]
    fn from(variant: Te27_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_27_2` reader - tamper erase enable SPMEM27 - DATA2"]
pub type Te27_2R = crate::BitReader<Te27_2>;
impl Te27_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te27_2 {
        match self.bits {
            false => Te27_2::Disable,
            true => Te27_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te27_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te27_2::Enable
    }
}
#[doc = "Field `TE_27_2` writer - tamper erase enable SPMEM27 - DATA2"]
pub type Te27_2W<'a, REG> = crate::BitWriter<'a, REG, Te27_2>;
impl<'a, REG> Te27_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM27 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te27_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te27_3> for bool {
    #[inline(always)]
    fn from(variant: Te27_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_27_3` reader - tamper erase enable SPMEM27 - DATA3"]
pub type Te27_3R = crate::BitReader<Te27_3>;
impl Te27_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te27_3 {
        match self.bits {
            false => Te27_3::Disable,
            true => Te27_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te27_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te27_3::Enable
    }
}
#[doc = "Field `TE_27_3` writer - tamper erase enable SPMEM27 - DATA3"]
pub type Te27_3W<'a, REG> = crate::BitWriter<'a, REG, Te27_3>;
impl<'a, REG> Te27_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te27_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM24 - DATA0"]
    #[inline(always)]
    pub fn te_24_0(&self) -> Te24_0R {
        Te24_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM24 - DATA1"]
    #[inline(always)]
    pub fn te_24_1(&self) -> Te24_1R {
        Te24_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM24 - DATA2"]
    #[inline(always)]
    pub fn te_24_2(&self) -> Te24_2R {
        Te24_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM24 - DATA3"]
    #[inline(always)]
    pub fn te_24_3(&self) -> Te24_3R {
        Te24_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM25 - DATA0"]
    #[inline(always)]
    pub fn te_25_0(&self) -> Te25_0R {
        Te25_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM25 - DATA1"]
    #[inline(always)]
    pub fn te_25_1(&self) -> Te25_1R {
        Te25_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM25 - DATA2"]
    #[inline(always)]
    pub fn te_25_2(&self) -> Te25_2R {
        Te25_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM25 - DATA3"]
    #[inline(always)]
    pub fn te_25_3(&self) -> Te25_3R {
        Te25_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM26 - DATA0"]
    #[inline(always)]
    pub fn te_26_0(&self) -> Te26_0R {
        Te26_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM26 - DATA1"]
    #[inline(always)]
    pub fn te_26_1(&self) -> Te26_1R {
        Te26_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM26 - DATA2"]
    #[inline(always)]
    pub fn te_26_2(&self) -> Te26_2R {
        Te26_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM26 - DATA3"]
    #[inline(always)]
    pub fn te_26_3(&self) -> Te26_3R {
        Te26_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM27 - DATA0"]
    #[inline(always)]
    pub fn te_27_0(&self) -> Te27_0R {
        Te27_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM27 - DATA1"]
    #[inline(always)]
    pub fn te_27_1(&self) -> Te27_1R {
        Te27_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM27 - DATA2"]
    #[inline(always)]
    pub fn te_27_2(&self) -> Te27_2R {
        Te27_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM27 - DATA3"]
    #[inline(always)]
    pub fn te_27_3(&self) -> Te27_3R {
        Te27_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM24 - DATA0"]
    #[inline(always)]
    pub fn te_24_0(&mut self) -> Te24_0W<'_, LfssSpmterase6Spec> {
        Te24_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM24 - DATA1"]
    #[inline(always)]
    pub fn te_24_1(&mut self) -> Te24_1W<'_, LfssSpmterase6Spec> {
        Te24_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM24 - DATA2"]
    #[inline(always)]
    pub fn te_24_2(&mut self) -> Te24_2W<'_, LfssSpmterase6Spec> {
        Te24_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM24 - DATA3"]
    #[inline(always)]
    pub fn te_24_3(&mut self) -> Te24_3W<'_, LfssSpmterase6Spec> {
        Te24_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM25 - DATA0"]
    #[inline(always)]
    pub fn te_25_0(&mut self) -> Te25_0W<'_, LfssSpmterase6Spec> {
        Te25_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM25 - DATA1"]
    #[inline(always)]
    pub fn te_25_1(&mut self) -> Te25_1W<'_, LfssSpmterase6Spec> {
        Te25_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM25 - DATA2"]
    #[inline(always)]
    pub fn te_25_2(&mut self) -> Te25_2W<'_, LfssSpmterase6Spec> {
        Te25_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM25 - DATA3"]
    #[inline(always)]
    pub fn te_25_3(&mut self) -> Te25_3W<'_, LfssSpmterase6Spec> {
        Te25_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM26 - DATA0"]
    #[inline(always)]
    pub fn te_26_0(&mut self) -> Te26_0W<'_, LfssSpmterase6Spec> {
        Te26_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM26 - DATA1"]
    #[inline(always)]
    pub fn te_26_1(&mut self) -> Te26_1W<'_, LfssSpmterase6Spec> {
        Te26_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM26 - DATA2"]
    #[inline(always)]
    pub fn te_26_2(&mut self) -> Te26_2W<'_, LfssSpmterase6Spec> {
        Te26_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM26 - DATA3"]
    #[inline(always)]
    pub fn te_26_3(&mut self) -> Te26_3W<'_, LfssSpmterase6Spec> {
        Te26_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM27 - DATA0"]
    #[inline(always)]
    pub fn te_27_0(&mut self) -> Te27_0W<'_, LfssSpmterase6Spec> {
        Te27_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM27 - DATA1"]
    #[inline(always)]
    pub fn te_27_1(&mut self) -> Te27_1W<'_, LfssSpmterase6Spec> {
        Te27_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM27 - DATA2"]
    #[inline(always)]
    pub fn te_27_2(&mut self) -> Te27_2W<'_, LfssSpmterase6Spec> {
        Te27_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM27 - DATA3"]
    #[inline(always)]
    pub fn te_27_3(&mut self) -> Te27_3W<'_, LfssSpmterase6Spec> {
        Te27_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase6Spec;
impl crate::RegisterSpec for LfssSpmterase6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase6::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase6Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase6::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE6 to value 0"]
impl crate::Resettable for LfssSpmterase6Spec {}
