#[doc = "Register `LFSS_SPMTERASE1` reader"]
pub type R = crate::R<LfssSpmterase1Spec>;
#[doc = "Register `LFSS_SPMTERASE1` writer"]
pub type W = crate::W<LfssSpmterase1Spec>;
#[doc = "tamper erase enable SPMEM4 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te4_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te4_0> for bool {
    #[inline(always)]
    fn from(variant: Te4_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_4_0` reader - tamper erase enable SPMEM4 - DATA0"]
pub type Te4_0R = crate::BitReader<Te4_0>;
impl Te4_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te4_0 {
        match self.bits {
            false => Te4_0::Disable,
            true => Te4_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te4_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te4_0::Enable
    }
}
#[doc = "Field `TE_4_0` writer - tamper erase enable SPMEM4 - DATA0"]
pub type Te4_0W<'a, REG> = crate::BitWriter<'a, REG, Te4_0>;
impl<'a, REG> Te4_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM4 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te4_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te4_1> for bool {
    #[inline(always)]
    fn from(variant: Te4_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_4_1` reader - tamper erase enable SPMEM4 - DATA1"]
pub type Te4_1R = crate::BitReader<Te4_1>;
impl Te4_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te4_1 {
        match self.bits {
            false => Te4_1::Disable,
            true => Te4_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te4_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te4_1::Enable
    }
}
#[doc = "Field `TE_4_1` writer - tamper erase enable SPMEM4 - DATA1"]
pub type Te4_1W<'a, REG> = crate::BitWriter<'a, REG, Te4_1>;
impl<'a, REG> Te4_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM4 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te4_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te4_2> for bool {
    #[inline(always)]
    fn from(variant: Te4_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_4_2` reader - tamper erase enable SPMEM4 - DATA2"]
pub type Te4_2R = crate::BitReader<Te4_2>;
impl Te4_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te4_2 {
        match self.bits {
            false => Te4_2::Disable,
            true => Te4_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te4_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te4_2::Enable
    }
}
#[doc = "Field `TE_4_2` writer - tamper erase enable SPMEM4 - DATA2"]
pub type Te4_2W<'a, REG> = crate::BitWriter<'a, REG, Te4_2>;
impl<'a, REG> Te4_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM4 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te4_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te4_3> for bool {
    #[inline(always)]
    fn from(variant: Te4_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_4_3` reader - tamper erase enable SPMEM4 - DATA3"]
pub type Te4_3R = crate::BitReader<Te4_3>;
impl Te4_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te4_3 {
        match self.bits {
            false => Te4_3::Disable,
            true => Te4_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te4_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te4_3::Enable
    }
}
#[doc = "Field `TE_4_3` writer - tamper erase enable SPMEM4 - DATA3"]
pub type Te4_3W<'a, REG> = crate::BitWriter<'a, REG, Te4_3>;
impl<'a, REG> Te4_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te4_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM5 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te5_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te5_0> for bool {
    #[inline(always)]
    fn from(variant: Te5_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_5_0` reader - tamper erase enable SPMEM5 - DATA0"]
pub type Te5_0R = crate::BitReader<Te5_0>;
impl Te5_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te5_0 {
        match self.bits {
            false => Te5_0::Disable,
            true => Te5_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te5_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te5_0::Enable
    }
}
#[doc = "Field `TE_5_0` writer - tamper erase enable SPMEM5 - DATA0"]
pub type Te5_0W<'a, REG> = crate::BitWriter<'a, REG, Te5_0>;
impl<'a, REG> Te5_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM5 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te5_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te5_1> for bool {
    #[inline(always)]
    fn from(variant: Te5_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_5_1` reader - tamper erase enable SPMEM5 - DATA1"]
pub type Te5_1R = crate::BitReader<Te5_1>;
impl Te5_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te5_1 {
        match self.bits {
            false => Te5_1::Disable,
            true => Te5_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te5_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te5_1::Enable
    }
}
#[doc = "Field `TE_5_1` writer - tamper erase enable SPMEM5 - DATA1"]
pub type Te5_1W<'a, REG> = crate::BitWriter<'a, REG, Te5_1>;
impl<'a, REG> Te5_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM5 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te5_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te5_2> for bool {
    #[inline(always)]
    fn from(variant: Te5_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_5_2` reader - tamper erase enable SPMEM5 - DATA2"]
pub type Te5_2R = crate::BitReader<Te5_2>;
impl Te5_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te5_2 {
        match self.bits {
            false => Te5_2::Disable,
            true => Te5_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te5_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te5_2::Enable
    }
}
#[doc = "Field `TE_5_2` writer - tamper erase enable SPMEM5 - DATA2"]
pub type Te5_2W<'a, REG> = crate::BitWriter<'a, REG, Te5_2>;
impl<'a, REG> Te5_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM5 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te5_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te5_3> for bool {
    #[inline(always)]
    fn from(variant: Te5_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_5_3` reader - tamper erase enable SPMEM5 - DATA3"]
pub type Te5_3R = crate::BitReader<Te5_3>;
impl Te5_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te5_3 {
        match self.bits {
            false => Te5_3::Disable,
            true => Te5_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te5_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te5_3::Enable
    }
}
#[doc = "Field `TE_5_3` writer - tamper erase enable SPMEM5 - DATA3"]
pub type Te5_3W<'a, REG> = crate::BitWriter<'a, REG, Te5_3>;
impl<'a, REG> Te5_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te5_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM6 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te6_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te6_0> for bool {
    #[inline(always)]
    fn from(variant: Te6_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_6_0` reader - tamper erase enable SPMEM6 - DATA0"]
pub type Te6_0R = crate::BitReader<Te6_0>;
impl Te6_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te6_0 {
        match self.bits {
            false => Te6_0::Disable,
            true => Te6_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te6_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te6_0::Enable
    }
}
#[doc = "Field `TE_6_0` writer - tamper erase enable SPMEM6 - DATA0"]
pub type Te6_0W<'a, REG> = crate::BitWriter<'a, REG, Te6_0>;
impl<'a, REG> Te6_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM6 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te6_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te6_1> for bool {
    #[inline(always)]
    fn from(variant: Te6_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_6_1` reader - tamper erase enable SPMEM6 - DATA1"]
pub type Te6_1R = crate::BitReader<Te6_1>;
impl Te6_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te6_1 {
        match self.bits {
            false => Te6_1::Disable,
            true => Te6_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te6_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te6_1::Enable
    }
}
#[doc = "Field `TE_6_1` writer - tamper erase enable SPMEM6 - DATA1"]
pub type Te6_1W<'a, REG> = crate::BitWriter<'a, REG, Te6_1>;
impl<'a, REG> Te6_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM6 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te6_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te6_2> for bool {
    #[inline(always)]
    fn from(variant: Te6_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_6_2` reader - tamper erase enable SPMEM6 - DATA2"]
pub type Te6_2R = crate::BitReader<Te6_2>;
impl Te6_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te6_2 {
        match self.bits {
            false => Te6_2::Disable,
            true => Te6_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te6_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te6_2::Enable
    }
}
#[doc = "Field `TE_6_2` writer - tamper erase enable SPMEM6 - DATA2"]
pub type Te6_2W<'a, REG> = crate::BitWriter<'a, REG, Te6_2>;
impl<'a, REG> Te6_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM6 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te6_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te6_3> for bool {
    #[inline(always)]
    fn from(variant: Te6_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_6_3` reader - tamper erase enable SPMEM6 - DATA3"]
pub type Te6_3R = crate::BitReader<Te6_3>;
impl Te6_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te6_3 {
        match self.bits {
            false => Te6_3::Disable,
            true => Te6_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te6_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te6_3::Enable
    }
}
#[doc = "Field `TE_6_3` writer - tamper erase enable SPMEM6 - DATA3"]
pub type Te6_3W<'a, REG> = crate::BitWriter<'a, REG, Te6_3>;
impl<'a, REG> Te6_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te6_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM7 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te7_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te7_0> for bool {
    #[inline(always)]
    fn from(variant: Te7_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_7_0` reader - tamper erase enable SPMEM7 - DATA0"]
pub type Te7_0R = crate::BitReader<Te7_0>;
impl Te7_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te7_0 {
        match self.bits {
            false => Te7_0::Disable,
            true => Te7_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te7_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te7_0::Enable
    }
}
#[doc = "Field `TE_7_0` writer - tamper erase enable SPMEM7 - DATA0"]
pub type Te7_0W<'a, REG> = crate::BitWriter<'a, REG, Te7_0>;
impl<'a, REG> Te7_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM7 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te7_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te7_1> for bool {
    #[inline(always)]
    fn from(variant: Te7_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_7_1` reader - tamper erase enable SPMEM7 - DATA1"]
pub type Te7_1R = crate::BitReader<Te7_1>;
impl Te7_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te7_1 {
        match self.bits {
            false => Te7_1::Disable,
            true => Te7_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te7_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te7_1::Enable
    }
}
#[doc = "Field `TE_7_1` writer - tamper erase enable SPMEM7 - DATA1"]
pub type Te7_1W<'a, REG> = crate::BitWriter<'a, REG, Te7_1>;
impl<'a, REG> Te7_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM7 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te7_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te7_2> for bool {
    #[inline(always)]
    fn from(variant: Te7_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_7_2` reader - tamper erase enable SPMEM7 - DATA2"]
pub type Te7_2R = crate::BitReader<Te7_2>;
impl Te7_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te7_2 {
        match self.bits {
            false => Te7_2::Disable,
            true => Te7_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te7_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te7_2::Enable
    }
}
#[doc = "Field `TE_7_2` writer - tamper erase enable SPMEM7 - DATA2"]
pub type Te7_2W<'a, REG> = crate::BitWriter<'a, REG, Te7_2>;
impl<'a, REG> Te7_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM7 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te7_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te7_3> for bool {
    #[inline(always)]
    fn from(variant: Te7_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_7_3` reader - tamper erase enable SPMEM7 - DATA3"]
pub type Te7_3R = crate::BitReader<Te7_3>;
impl Te7_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te7_3 {
        match self.bits {
            false => Te7_3::Disable,
            true => Te7_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te7_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te7_3::Enable
    }
}
#[doc = "Field `TE_7_3` writer - tamper erase enable SPMEM7 - DATA3"]
pub type Te7_3W<'a, REG> = crate::BitWriter<'a, REG, Te7_3>;
impl<'a, REG> Te7_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te7_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM4 - DATA0"]
    #[inline(always)]
    pub fn te_4_0(&self) -> Te4_0R {
        Te4_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM4 - DATA1"]
    #[inline(always)]
    pub fn te_4_1(&self) -> Te4_1R {
        Te4_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM4 - DATA2"]
    #[inline(always)]
    pub fn te_4_2(&self) -> Te4_2R {
        Te4_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM4 - DATA3"]
    #[inline(always)]
    pub fn te_4_3(&self) -> Te4_3R {
        Te4_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM5 - DATA0"]
    #[inline(always)]
    pub fn te_5_0(&self) -> Te5_0R {
        Te5_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM5 - DATA1"]
    #[inline(always)]
    pub fn te_5_1(&self) -> Te5_1R {
        Te5_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM5 - DATA2"]
    #[inline(always)]
    pub fn te_5_2(&self) -> Te5_2R {
        Te5_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM5 - DATA3"]
    #[inline(always)]
    pub fn te_5_3(&self) -> Te5_3R {
        Te5_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM6 - DATA0"]
    #[inline(always)]
    pub fn te_6_0(&self) -> Te6_0R {
        Te6_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM6 - DATA1"]
    #[inline(always)]
    pub fn te_6_1(&self) -> Te6_1R {
        Te6_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM6 - DATA2"]
    #[inline(always)]
    pub fn te_6_2(&self) -> Te6_2R {
        Te6_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM6 - DATA3"]
    #[inline(always)]
    pub fn te_6_3(&self) -> Te6_3R {
        Te6_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM7 - DATA0"]
    #[inline(always)]
    pub fn te_7_0(&self) -> Te7_0R {
        Te7_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM7 - DATA1"]
    #[inline(always)]
    pub fn te_7_1(&self) -> Te7_1R {
        Te7_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM7 - DATA2"]
    #[inline(always)]
    pub fn te_7_2(&self) -> Te7_2R {
        Te7_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM7 - DATA3"]
    #[inline(always)]
    pub fn te_7_3(&self) -> Te7_3R {
        Te7_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM4 - DATA0"]
    #[inline(always)]
    pub fn te_4_0(&mut self) -> Te4_0W<'_, LfssSpmterase1Spec> {
        Te4_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM4 - DATA1"]
    #[inline(always)]
    pub fn te_4_1(&mut self) -> Te4_1W<'_, LfssSpmterase1Spec> {
        Te4_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM4 - DATA2"]
    #[inline(always)]
    pub fn te_4_2(&mut self) -> Te4_2W<'_, LfssSpmterase1Spec> {
        Te4_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM4 - DATA3"]
    #[inline(always)]
    pub fn te_4_3(&mut self) -> Te4_3W<'_, LfssSpmterase1Spec> {
        Te4_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM5 - DATA0"]
    #[inline(always)]
    pub fn te_5_0(&mut self) -> Te5_0W<'_, LfssSpmterase1Spec> {
        Te5_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM5 - DATA1"]
    #[inline(always)]
    pub fn te_5_1(&mut self) -> Te5_1W<'_, LfssSpmterase1Spec> {
        Te5_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM5 - DATA2"]
    #[inline(always)]
    pub fn te_5_2(&mut self) -> Te5_2W<'_, LfssSpmterase1Spec> {
        Te5_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM5 - DATA3"]
    #[inline(always)]
    pub fn te_5_3(&mut self) -> Te5_3W<'_, LfssSpmterase1Spec> {
        Te5_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM6 - DATA0"]
    #[inline(always)]
    pub fn te_6_0(&mut self) -> Te6_0W<'_, LfssSpmterase1Spec> {
        Te6_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM6 - DATA1"]
    #[inline(always)]
    pub fn te_6_1(&mut self) -> Te6_1W<'_, LfssSpmterase1Spec> {
        Te6_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM6 - DATA2"]
    #[inline(always)]
    pub fn te_6_2(&mut self) -> Te6_2W<'_, LfssSpmterase1Spec> {
        Te6_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM6 - DATA3"]
    #[inline(always)]
    pub fn te_6_3(&mut self) -> Te6_3W<'_, LfssSpmterase1Spec> {
        Te6_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM7 - DATA0"]
    #[inline(always)]
    pub fn te_7_0(&mut self) -> Te7_0W<'_, LfssSpmterase1Spec> {
        Te7_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM7 - DATA1"]
    #[inline(always)]
    pub fn te_7_1(&mut self) -> Te7_1W<'_, LfssSpmterase1Spec> {
        Te7_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM7 - DATA2"]
    #[inline(always)]
    pub fn te_7_2(&mut self) -> Te7_2W<'_, LfssSpmterase1Spec> {
        Te7_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM7 - DATA3"]
    #[inline(always)]
    pub fn te_7_3(&mut self) -> Te7_3W<'_, LfssSpmterase1Spec> {
        Te7_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase1Spec;
impl crate::RegisterSpec for LfssSpmterase1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase1::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase1Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase1::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE1 to value 0"]
impl crate::Resettable for LfssSpmterase1Spec {}
