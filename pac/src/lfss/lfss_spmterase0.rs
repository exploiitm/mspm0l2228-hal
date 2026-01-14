#[doc = "Register `LFSS_SPMTERASE0` reader"]
pub type R = crate::R<LfssSpmterase0Spec>;
#[doc = "Register `LFSS_SPMTERASE0` writer"]
pub type W = crate::W<LfssSpmterase0Spec>;
#[doc = "tamper erase enable SPMEM0 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te0_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te0_0> for bool {
    #[inline(always)]
    fn from(variant: Te0_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_0_0` reader - tamper erase enable SPMEM0 - DATA0"]
pub type Te0_0R = crate::BitReader<Te0_0>;
impl Te0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te0_0 {
        match self.bits {
            false => Te0_0::Disable,
            true => Te0_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te0_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te0_0::Enable
    }
}
#[doc = "Field `TE_0_0` writer - tamper erase enable SPMEM0 - DATA0"]
pub type Te0_0W<'a, REG> = crate::BitWriter<'a, REG, Te0_0>;
impl<'a, REG> Te0_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM0 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te0_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te0_1> for bool {
    #[inline(always)]
    fn from(variant: Te0_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_0_1` reader - tamper erase enable SPMEM0 - DATA1"]
pub type Te0_1R = crate::BitReader<Te0_1>;
impl Te0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te0_1 {
        match self.bits {
            false => Te0_1::Disable,
            true => Te0_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te0_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te0_1::Enable
    }
}
#[doc = "Field `TE_0_1` writer - tamper erase enable SPMEM0 - DATA1"]
pub type Te0_1W<'a, REG> = crate::BitWriter<'a, REG, Te0_1>;
impl<'a, REG> Te0_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM0 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te0_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te0_2> for bool {
    #[inline(always)]
    fn from(variant: Te0_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_0_2` reader - tamper erase enable SPMEM0 - DATA2"]
pub type Te0_2R = crate::BitReader<Te0_2>;
impl Te0_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te0_2 {
        match self.bits {
            false => Te0_2::Disable,
            true => Te0_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te0_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te0_2::Enable
    }
}
#[doc = "Field `TE_0_2` writer - tamper erase enable SPMEM0 - DATA2"]
pub type Te0_2W<'a, REG> = crate::BitWriter<'a, REG, Te0_2>;
impl<'a, REG> Te0_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM0 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te0_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te0_3> for bool {
    #[inline(always)]
    fn from(variant: Te0_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_0_3` reader - tamper erase enable SPMEM0 - DATA3"]
pub type Te0_3R = crate::BitReader<Te0_3>;
impl Te0_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te0_3 {
        match self.bits {
            false => Te0_3::Disable,
            true => Te0_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te0_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te0_3::Enable
    }
}
#[doc = "Field `TE_0_3` writer - tamper erase enable SPMEM0 - DATA3"]
pub type Te0_3W<'a, REG> = crate::BitWriter<'a, REG, Te0_3>;
impl<'a, REG> Te0_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM1 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te1_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te1_0> for bool {
    #[inline(always)]
    fn from(variant: Te1_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_1_0` reader - tamper erase enable SPMEM1 - DATA0"]
pub type Te1_0R = crate::BitReader<Te1_0>;
impl Te1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te1_0 {
        match self.bits {
            false => Te1_0::Disable,
            true => Te1_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te1_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te1_0::Enable
    }
}
#[doc = "Field `TE_1_0` writer - tamper erase enable SPMEM1 - DATA0"]
pub type Te1_0W<'a, REG> = crate::BitWriter<'a, REG, Te1_0>;
impl<'a, REG> Te1_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM1 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te1_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te1_1> for bool {
    #[inline(always)]
    fn from(variant: Te1_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_1_1` reader - tamper erase enable SPMEM1 - DATA1"]
pub type Te1_1R = crate::BitReader<Te1_1>;
impl Te1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te1_1 {
        match self.bits {
            false => Te1_1::Disable,
            true => Te1_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te1_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te1_1::Enable
    }
}
#[doc = "Field `TE_1_1` writer - tamper erase enable SPMEM1 - DATA1"]
pub type Te1_1W<'a, REG> = crate::BitWriter<'a, REG, Te1_1>;
impl<'a, REG> Te1_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM1 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te1_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te1_2> for bool {
    #[inline(always)]
    fn from(variant: Te1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_1_2` reader - tamper erase enable SPMEM1 - DATA2"]
pub type Te1_2R = crate::BitReader<Te1_2>;
impl Te1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te1_2 {
        match self.bits {
            false => Te1_2::Disable,
            true => Te1_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te1_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te1_2::Enable
    }
}
#[doc = "Field `TE_1_2` writer - tamper erase enable SPMEM1 - DATA2"]
pub type Te1_2W<'a, REG> = crate::BitWriter<'a, REG, Te1_2>;
impl<'a, REG> Te1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM1 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te1_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te1_3> for bool {
    #[inline(always)]
    fn from(variant: Te1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_1_3` reader - tamper erase enable SPMEM1 - DATA3"]
pub type Te1_3R = crate::BitReader<Te1_3>;
impl Te1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te1_3 {
        match self.bits {
            false => Te1_3::Disable,
            true => Te1_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te1_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te1_3::Enable
    }
}
#[doc = "Field `TE_1_3` writer - tamper erase enable SPMEM1 - DATA3"]
pub type Te1_3W<'a, REG> = crate::BitWriter<'a, REG, Te1_3>;
impl<'a, REG> Te1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM2 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te2_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te2_0> for bool {
    #[inline(always)]
    fn from(variant: Te2_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_2_0` reader - tamper erase enable SPMEM2 - DATA0"]
pub type Te2_0R = crate::BitReader<Te2_0>;
impl Te2_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te2_0 {
        match self.bits {
            false => Te2_0::Disable,
            true => Te2_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te2_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te2_0::Enable
    }
}
#[doc = "Field `TE_2_0` writer - tamper erase enable SPMEM2 - DATA0"]
pub type Te2_0W<'a, REG> = crate::BitWriter<'a, REG, Te2_0>;
impl<'a, REG> Te2_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM2 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te2_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te2_1> for bool {
    #[inline(always)]
    fn from(variant: Te2_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_2_1` reader - tamper erase enable SPMEM2 - DATA1"]
pub type Te2_1R = crate::BitReader<Te2_1>;
impl Te2_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te2_1 {
        match self.bits {
            false => Te2_1::Disable,
            true => Te2_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te2_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te2_1::Enable
    }
}
#[doc = "Field `TE_2_1` writer - tamper erase enable SPMEM2 - DATA1"]
pub type Te2_1W<'a, REG> = crate::BitWriter<'a, REG, Te2_1>;
impl<'a, REG> Te2_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM2 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te2_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te2_2> for bool {
    #[inline(always)]
    fn from(variant: Te2_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_2_2` reader - tamper erase enable SPMEM2 - DATA2"]
pub type Te2_2R = crate::BitReader<Te2_2>;
impl Te2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te2_2 {
        match self.bits {
            false => Te2_2::Disable,
            true => Te2_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te2_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te2_2::Enable
    }
}
#[doc = "Field `TE_2_2` writer - tamper erase enable SPMEM2 - DATA2"]
pub type Te2_2W<'a, REG> = crate::BitWriter<'a, REG, Te2_2>;
impl<'a, REG> Te2_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM2 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te2_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te2_3> for bool {
    #[inline(always)]
    fn from(variant: Te2_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_2_3` reader - tamper erase enable SPMEM2 - DATA3"]
pub type Te2_3R = crate::BitReader<Te2_3>;
impl Te2_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te2_3 {
        match self.bits {
            false => Te2_3::Disable,
            true => Te2_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te2_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te2_3::Enable
    }
}
#[doc = "Field `TE_2_3` writer - tamper erase enable SPMEM2 - DATA3"]
pub type Te2_3W<'a, REG> = crate::BitWriter<'a, REG, Te2_3>;
impl<'a, REG> Te2_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te2_3::Enable)
    }
}
#[doc = "tamper erase enable SPMEM3 - DATA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te3_0 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te3_0> for bool {
    #[inline(always)]
    fn from(variant: Te3_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_3_0` reader - tamper erase enable SPMEM3 - DATA0"]
pub type Te3_0R = crate::BitReader<Te3_0>;
impl Te3_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te3_0 {
        match self.bits {
            false => Te3_0::Disable,
            true => Te3_0::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te3_0::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te3_0::Enable
    }
}
#[doc = "Field `TE_3_0` writer - tamper erase enable SPMEM3 - DATA0"]
pub type Te3_0W<'a, REG> = crate::BitWriter<'a, REG, Te3_0>;
impl<'a, REG> Te3_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_0::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_0::Enable)
    }
}
#[doc = "tamper erase enable SPMEM3 - DATA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te3_1 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te3_1> for bool {
    #[inline(always)]
    fn from(variant: Te3_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_3_1` reader - tamper erase enable SPMEM3 - DATA1"]
pub type Te3_1R = crate::BitReader<Te3_1>;
impl Te3_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te3_1 {
        match self.bits {
            false => Te3_1::Disable,
            true => Te3_1::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te3_1::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te3_1::Enable
    }
}
#[doc = "Field `TE_3_1` writer - tamper erase enable SPMEM3 - DATA1"]
pub type Te3_1W<'a, REG> = crate::BitWriter<'a, REG, Te3_1>;
impl<'a, REG> Te3_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_1::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_1::Enable)
    }
}
#[doc = "tamper erase enable SPMEM3 - DATA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te3_2 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te3_2> for bool {
    #[inline(always)]
    fn from(variant: Te3_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_3_2` reader - tamper erase enable SPMEM3 - DATA2"]
pub type Te3_2R = crate::BitReader<Te3_2>;
impl Te3_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te3_2 {
        match self.bits {
            false => Te3_2::Disable,
            true => Te3_2::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te3_2::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te3_2::Enable
    }
}
#[doc = "Field `TE_3_2` writer - tamper erase enable SPMEM3 - DATA2"]
pub type Te3_2W<'a, REG> = crate::BitWriter<'a, REG, Te3_2>;
impl<'a, REG> Te3_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_2::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_2::Enable)
    }
}
#[doc = "tamper erase enable SPMEM3 - DATA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te3_3 {
    #[doc = "0: SPMEM is unmodified during tamper event"]
    Disable = 0,
    #[doc = "1: SPMEM will be erased with tamper event"]
    Enable = 1,
}
impl From<Te3_3> for bool {
    #[inline(always)]
    fn from(variant: Te3_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_3_3` reader - tamper erase enable SPMEM3 - DATA3"]
pub type Te3_3R = crate::BitReader<Te3_3>;
impl Te3_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te3_3 {
        match self.bits {
            false => Te3_3::Disable,
            true => Te3_3::Enable,
        }
    }
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te3_3::Disable
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te3_3::Enable
    }
}
#[doc = "Field `TE_3_3` writer - tamper erase enable SPMEM3 - DATA3"]
pub type Te3_3W<'a, REG> = crate::BitWriter<'a, REG, Te3_3>;
impl<'a, REG> Te3_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPMEM is unmodified during tamper event"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_3::Disable)
    }
    #[doc = "SPMEM will be erased with tamper event"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te3_3::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper erase enable SPMEM0 - DATA0"]
    #[inline(always)]
    pub fn te_0_0(&self) -> Te0_0R {
        Te0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM0 - DATA1"]
    #[inline(always)]
    pub fn te_0_1(&self) -> Te0_1R {
        Te0_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM0 - DATA2"]
    #[inline(always)]
    pub fn te_0_2(&self) -> Te0_2R {
        Te0_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM0 - DATA3"]
    #[inline(always)]
    pub fn te_0_3(&self) -> Te0_3R {
        Te0_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM1 - DATA0"]
    #[inline(always)]
    pub fn te_1_0(&self) -> Te1_0R {
        Te1_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM1 - DATA1"]
    #[inline(always)]
    pub fn te_1_1(&self) -> Te1_1R {
        Te1_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM1 - DATA2"]
    #[inline(always)]
    pub fn te_1_2(&self) -> Te1_2R {
        Te1_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM1 - DATA3"]
    #[inline(always)]
    pub fn te_1_3(&self) -> Te1_3R {
        Te1_3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM2 - DATA0"]
    #[inline(always)]
    pub fn te_2_0(&self) -> Te2_0R {
        Te2_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM2 - DATA1"]
    #[inline(always)]
    pub fn te_2_1(&self) -> Te2_1R {
        Te2_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM2 - DATA2"]
    #[inline(always)]
    pub fn te_2_2(&self) -> Te2_2R {
        Te2_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM2 - DATA3"]
    #[inline(always)]
    pub fn te_2_3(&self) -> Te2_3R {
        Te2_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM3 - DATA0"]
    #[inline(always)]
    pub fn te_3_0(&self) -> Te3_0R {
        Te3_0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM3 - DATA1"]
    #[inline(always)]
    pub fn te_3_1(&self) -> Te3_1R {
        Te3_1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM3 - DATA2"]
    #[inline(always)]
    pub fn te_3_2(&self) -> Te3_2R {
        Te3_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM3 - DATA3"]
    #[inline(always)]
    pub fn te_3_3(&self) -> Te3_3R {
        Te3_3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper erase enable SPMEM0 - DATA0"]
    #[inline(always)]
    pub fn te_0_0(&mut self) -> Te0_0W<'_, LfssSpmterase0Spec> {
        Te0_0W::new(self, 0)
    }
    #[doc = "Bit 1 - tamper erase enable SPMEM0 - DATA1"]
    #[inline(always)]
    pub fn te_0_1(&mut self) -> Te0_1W<'_, LfssSpmterase0Spec> {
        Te0_1W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper erase enable SPMEM0 - DATA2"]
    #[inline(always)]
    pub fn te_0_2(&mut self) -> Te0_2W<'_, LfssSpmterase0Spec> {
        Te0_2W::new(self, 2)
    }
    #[doc = "Bit 3 - tamper erase enable SPMEM0 - DATA3"]
    #[inline(always)]
    pub fn te_0_3(&mut self) -> Te0_3W<'_, LfssSpmterase0Spec> {
        Te0_3W::new(self, 3)
    }
    #[doc = "Bit 4 - tamper erase enable SPMEM1 - DATA0"]
    #[inline(always)]
    pub fn te_1_0(&mut self) -> Te1_0W<'_, LfssSpmterase0Spec> {
        Te1_0W::new(self, 4)
    }
    #[doc = "Bit 5 - tamper erase enable SPMEM1 - DATA1"]
    #[inline(always)]
    pub fn te_1_1(&mut self) -> Te1_1W<'_, LfssSpmterase0Spec> {
        Te1_1W::new(self, 5)
    }
    #[doc = "Bit 6 - tamper erase enable SPMEM1 - DATA2"]
    #[inline(always)]
    pub fn te_1_2(&mut self) -> Te1_2W<'_, LfssSpmterase0Spec> {
        Te1_2W::new(self, 6)
    }
    #[doc = "Bit 7 - tamper erase enable SPMEM1 - DATA3"]
    #[inline(always)]
    pub fn te_1_3(&mut self) -> Te1_3W<'_, LfssSpmterase0Spec> {
        Te1_3W::new(self, 7)
    }
    #[doc = "Bit 8 - tamper erase enable SPMEM2 - DATA0"]
    #[inline(always)]
    pub fn te_2_0(&mut self) -> Te2_0W<'_, LfssSpmterase0Spec> {
        Te2_0W::new(self, 8)
    }
    #[doc = "Bit 9 - tamper erase enable SPMEM2 - DATA1"]
    #[inline(always)]
    pub fn te_2_1(&mut self) -> Te2_1W<'_, LfssSpmterase0Spec> {
        Te2_1W::new(self, 9)
    }
    #[doc = "Bit 10 - tamper erase enable SPMEM2 - DATA2"]
    #[inline(always)]
    pub fn te_2_2(&mut self) -> Te2_2W<'_, LfssSpmterase0Spec> {
        Te2_2W::new(self, 10)
    }
    #[doc = "Bit 11 - tamper erase enable SPMEM2 - DATA3"]
    #[inline(always)]
    pub fn te_2_3(&mut self) -> Te2_3W<'_, LfssSpmterase0Spec> {
        Te2_3W::new(self, 11)
    }
    #[doc = "Bit 12 - tamper erase enable SPMEM3 - DATA0"]
    #[inline(always)]
    pub fn te_3_0(&mut self) -> Te3_0W<'_, LfssSpmterase0Spec> {
        Te3_0W::new(self, 12)
    }
    #[doc = "Bit 13 - tamper erase enable SPMEM3 - DATA1"]
    #[inline(always)]
    pub fn te_3_1(&mut self) -> Te3_1W<'_, LfssSpmterase0Spec> {
        Te3_1W::new(self, 13)
    }
    #[doc = "Bit 14 - tamper erase enable SPMEM3 - DATA2"]
    #[inline(always)]
    pub fn te_3_2(&mut self) -> Te3_2W<'_, LfssSpmterase0Spec> {
        Te3_2W::new(self, 14)
    }
    #[doc = "Bit 15 - tamper erase enable SPMEM3 - DATA3"]
    #[inline(always)]
    pub fn te_3_3(&mut self) -> Te3_3W<'_, LfssSpmterase0Spec> {
        Te3_3W::new(self, 15)
    }
}
#[doc = "Scratch Pad Memory Tamper Erase Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmterase0Spec;
impl crate::RegisterSpec for LfssSpmterase0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmterase0::R`](R) reader structure"]
impl crate::Readable for LfssSpmterase0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmterase0::W`](W) writer structure"]
impl crate::Writable for LfssSpmterase0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMTERASE0 to value 0"]
impl crate::Resettable for LfssSpmterase0Spec {}
