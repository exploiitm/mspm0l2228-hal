#[doc = "Register `LFSS_TSCTL` reader"]
pub type R = crate::R<LfssTsctlSpec>;
#[doc = "Register `LFSS_TSCTL` writer"]
pub type W = crate::W<LfssTsctlSpec>;
#[doc = "Time Stamp by Tamper I/O 0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen0 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen0> for bool {
    #[inline(always)]
    fn from(variant: Tstioen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN0` reader - Time Stamp by Tamper I/O 0 enable"]
pub type Tstioen0R = crate::BitReader<Tstioen0>;
impl Tstioen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen0 {
        match self.bits {
            false => Tstioen0::Disable,
            true => Tstioen0::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen0::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen0::Enable
    }
}
#[doc = "Field `TSTIOEN0` writer - Time Stamp by Tamper I/O 0 enable"]
pub type Tstioen0W<'a, REG> = crate::BitWriter<'a, REG, Tstioen0>;
impl<'a, REG> Tstioen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen0::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen0::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen1 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen1> for bool {
    #[inline(always)]
    fn from(variant: Tstioen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN1` reader - Time Stamp by Tamper I/O 1 enable"]
pub type Tstioen1R = crate::BitReader<Tstioen1>;
impl Tstioen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen1 {
        match self.bits {
            false => Tstioen1::Disable,
            true => Tstioen1::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen1::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen1::Enable
    }
}
#[doc = "Field `TSTIOEN1` writer - Time Stamp by Tamper I/O 1 enable"]
pub type Tstioen1W<'a, REG> = crate::BitWriter<'a, REG, Tstioen1>;
impl<'a, REG> Tstioen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen1::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen1::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen2 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen2> for bool {
    #[inline(always)]
    fn from(variant: Tstioen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN2` reader - Time Stamp by Tamper I/O 2 enable"]
pub type Tstioen2R = crate::BitReader<Tstioen2>;
impl Tstioen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen2 {
        match self.bits {
            false => Tstioen2::Disable,
            true => Tstioen2::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen2::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen2::Enable
    }
}
#[doc = "Field `TSTIOEN2` writer - Time Stamp by Tamper I/O 2 enable"]
pub type Tstioen2W<'a, REG> = crate::BitWriter<'a, REG, Tstioen2>;
impl<'a, REG> Tstioen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen2::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen2::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen3 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen3> for bool {
    #[inline(always)]
    fn from(variant: Tstioen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN3` reader - Time Stamp by Tamper I/O 3 enable"]
pub type Tstioen3R = crate::BitReader<Tstioen3>;
impl Tstioen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen3 {
        match self.bits {
            false => Tstioen3::Disable,
            true => Tstioen3::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen3::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen3::Enable
    }
}
#[doc = "Field `TSTIOEN3` writer - Time Stamp by Tamper I/O 3 enable"]
pub type Tstioen3W<'a, REG> = crate::BitWriter<'a, REG, Tstioen3>;
impl<'a, REG> Tstioen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen3::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen3::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen4 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen4> for bool {
    #[inline(always)]
    fn from(variant: Tstioen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN4` reader - Time Stamp by Tamper I/O 4 enable"]
pub type Tstioen4R = crate::BitReader<Tstioen4>;
impl Tstioen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen4 {
        match self.bits {
            false => Tstioen4::Disable,
            true => Tstioen4::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen4::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen4::Enable
    }
}
#[doc = "Field `TSTIOEN4` writer - Time Stamp by Tamper I/O 4 enable"]
pub type Tstioen4W<'a, REG> = crate::BitWriter<'a, REG, Tstioen4>;
impl<'a, REG> Tstioen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen4::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen4::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen5 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen5> for bool {
    #[inline(always)]
    fn from(variant: Tstioen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN5` reader - Time Stamp by Tamper I/O 5 enable"]
pub type Tstioen5R = crate::BitReader<Tstioen5>;
impl Tstioen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen5 {
        match self.bits {
            false => Tstioen5::Disable,
            true => Tstioen5::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen5::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen5::Enable
    }
}
#[doc = "Field `TSTIOEN5` writer - Time Stamp by Tamper I/O 5 enable"]
pub type Tstioen5W<'a, REG> = crate::BitWriter<'a, REG, Tstioen5>;
impl<'a, REG> Tstioen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen5::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen5::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen6 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen6> for bool {
    #[inline(always)]
    fn from(variant: Tstioen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN6` reader - Time Stamp by Tamper I/O 6 enable"]
pub type Tstioen6R = crate::BitReader<Tstioen6>;
impl Tstioen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen6 {
        match self.bits {
            false => Tstioen6::Disable,
            true => Tstioen6::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen6::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen6::Enable
    }
}
#[doc = "Field `TSTIOEN6` writer - Time Stamp by Tamper I/O 6 enable"]
pub type Tstioen6W<'a, REG> = crate::BitWriter<'a, REG, Tstioen6>;
impl<'a, REG> Tstioen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen6::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen6::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 7 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen7 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen7> for bool {
    #[inline(always)]
    fn from(variant: Tstioen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN7` reader - Time Stamp by Tamper I/O 7 enable"]
pub type Tstioen7R = crate::BitReader<Tstioen7>;
impl Tstioen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen7 {
        match self.bits {
            false => Tstioen7::Disable,
            true => Tstioen7::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen7::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen7::Enable
    }
}
#[doc = "Field `TSTIOEN7` writer - Time Stamp by Tamper I/O 7 enable"]
pub type Tstioen7W<'a, REG> = crate::BitWriter<'a, REG, Tstioen7>;
impl<'a, REG> Tstioen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen7::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen7::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 8 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen8 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen8> for bool {
    #[inline(always)]
    fn from(variant: Tstioen8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN8` reader - Time Stamp by Tamper I/O 8 enable"]
pub type Tstioen8R = crate::BitReader<Tstioen8>;
impl Tstioen8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen8 {
        match self.bits {
            false => Tstioen8::Disable,
            true => Tstioen8::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen8::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen8::Enable
    }
}
#[doc = "Field `TSTIOEN8` writer - Time Stamp by Tamper I/O 8 enable"]
pub type Tstioen8W<'a, REG> = crate::BitWriter<'a, REG, Tstioen8>;
impl<'a, REG> Tstioen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen8::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen8::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 9 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen9 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen9> for bool {
    #[inline(always)]
    fn from(variant: Tstioen9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN9` reader - Time Stamp by Tamper I/O 9 enable"]
pub type Tstioen9R = crate::BitReader<Tstioen9>;
impl Tstioen9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen9 {
        match self.bits {
            false => Tstioen9::Disable,
            true => Tstioen9::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen9::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen9::Enable
    }
}
#[doc = "Field `TSTIOEN9` writer - Time Stamp by Tamper I/O 9 enable"]
pub type Tstioen9W<'a, REG> = crate::BitWriter<'a, REG, Tstioen9>;
impl<'a, REG> Tstioen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen9::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen9::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 10 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen10 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen10> for bool {
    #[inline(always)]
    fn from(variant: Tstioen10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN10` reader - Time Stamp by Tamper I/O 10 enable"]
pub type Tstioen10R = crate::BitReader<Tstioen10>;
impl Tstioen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen10 {
        match self.bits {
            false => Tstioen10::Disable,
            true => Tstioen10::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen10::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen10::Enable
    }
}
#[doc = "Field `TSTIOEN10` writer - Time Stamp by Tamper I/O 10 enable"]
pub type Tstioen10W<'a, REG> = crate::BitWriter<'a, REG, Tstioen10>;
impl<'a, REG> Tstioen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen10::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen10::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 11 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen11 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen11> for bool {
    #[inline(always)]
    fn from(variant: Tstioen11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN11` reader - Time Stamp by Tamper I/O 11 enable"]
pub type Tstioen11R = crate::BitReader<Tstioen11>;
impl Tstioen11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen11 {
        match self.bits {
            false => Tstioen11::Disable,
            true => Tstioen11::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen11::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen11::Enable
    }
}
#[doc = "Field `TSTIOEN11` writer - Time Stamp by Tamper I/O 11 enable"]
pub type Tstioen11W<'a, REG> = crate::BitWriter<'a, REG, Tstioen11>;
impl<'a, REG> Tstioen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen11::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen11::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 12 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen12 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen12> for bool {
    #[inline(always)]
    fn from(variant: Tstioen12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN12` reader - Time Stamp by Tamper I/O 12 enable"]
pub type Tstioen12R = crate::BitReader<Tstioen12>;
impl Tstioen12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen12 {
        match self.bits {
            false => Tstioen12::Disable,
            true => Tstioen12::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen12::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen12::Enable
    }
}
#[doc = "Field `TSTIOEN12` writer - Time Stamp by Tamper I/O 12 enable"]
pub type Tstioen12W<'a, REG> = crate::BitWriter<'a, REG, Tstioen12>;
impl<'a, REG> Tstioen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen12::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen12::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 13 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen13 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen13> for bool {
    #[inline(always)]
    fn from(variant: Tstioen13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN13` reader - Time Stamp by Tamper I/O 13 enable"]
pub type Tstioen13R = crate::BitReader<Tstioen13>;
impl Tstioen13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen13 {
        match self.bits {
            false => Tstioen13::Disable,
            true => Tstioen13::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen13::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen13::Enable
    }
}
#[doc = "Field `TSTIOEN13` writer - Time Stamp by Tamper I/O 13 enable"]
pub type Tstioen13W<'a, REG> = crate::BitWriter<'a, REG, Tstioen13>;
impl<'a, REG> Tstioen13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen13::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen13::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 14 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen14 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen14> for bool {
    #[inline(always)]
    fn from(variant: Tstioen14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN14` reader - Time Stamp by Tamper I/O 14 enable"]
pub type Tstioen14R = crate::BitReader<Tstioen14>;
impl Tstioen14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen14 {
        match self.bits {
            false => Tstioen14::Disable,
            true => Tstioen14::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen14::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen14::Enable
    }
}
#[doc = "Field `TSTIOEN14` writer - Time Stamp by Tamper I/O 14 enable"]
pub type Tstioen14W<'a, REG> = crate::BitWriter<'a, REG, Tstioen14>;
impl<'a, REG> Tstioen14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen14::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen14::Enable)
    }
}
#[doc = "Time Stamp by Tamper I/O 15 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstioen15 {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tstioen15> for bool {
    #[inline(always)]
    fn from(variant: Tstioen15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIOEN15` reader - Time Stamp by Tamper I/O 15 enable"]
pub type Tstioen15R = crate::BitReader<Tstioen15>;
impl Tstioen15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstioen15 {
        match self.bits {
            false => Tstioen15::Disable,
            true => Tstioen15::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstioen15::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tstioen15::Enable
    }
}
#[doc = "Field `TSTIOEN15` writer - Time Stamp by Tamper I/O 15 enable"]
pub type Tstioen15W<'a, REG> = crate::BitWriter<'a, REG, Tstioen15>;
impl<'a, REG> Tstioen15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen15::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstioen15::Enable)
    }
}
#[doc = "Time Stamp by VDD Loss detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsvdden {
    #[doc = "0: function disabled"]
    Disable = 0,
    #[doc = "1: function enabled"]
    Enable = 1,
}
impl From<Tsvdden> for bool {
    #[inline(always)]
    fn from(variant: Tsvdden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVDDEN` reader - Time Stamp by VDD Loss detection enable"]
pub type TsvddenR = crate::BitReader<Tsvdden>;
impl TsvddenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsvdden {
        match self.bits {
            false => Tsvdden::Disable,
            true => Tsvdden::Enable,
        }
    }
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tsvdden::Disable
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tsvdden::Enable
    }
}
#[doc = "Field `TSVDDEN` writer - Time Stamp by VDD Loss detection enable"]
pub type TsvddenW<'a, REG> = crate::BitWriter<'a, REG, Tsvdden>;
impl<'a, REG> TsvddenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tsvdden::Disable)
    }
    #[doc = "function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tsvdden::Enable)
    }
}
#[doc = "Defines the capture method of the RTC timestamp when a time stamp event occurens.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tscapture {
    #[doc = "0: Time stamp holds RTC capture at first occurrence of time stamp event."]
    First = 0,
    #[doc = "1: Time stamp holds RTC capture at last occurrence of time stamp event."]
    Last = 1,
}
impl From<Tscapture> for bool {
    #[inline(always)]
    fn from(variant: Tscapture) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCAPTURE` reader - Defines the capture method of the RTC timestamp when a time stamp event occurens."]
pub type TscaptureR = crate::BitReader<Tscapture>;
impl TscaptureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tscapture {
        match self.bits {
            false => Tscapture::First,
            true => Tscapture::Last,
        }
    }
    #[doc = "Time stamp holds RTC capture at first occurrence of time stamp event."]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == Tscapture::First
    }
    #[doc = "Time stamp holds RTC capture at last occurrence of time stamp event."]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == Tscapture::Last
    }
}
#[doc = "Field `TSCAPTURE` writer - Defines the capture method of the RTC timestamp when a time stamp event occurens."]
pub type TscaptureW<'a, REG> = crate::BitWriter<'a, REG, Tscapture>;
impl<'a, REG> TscaptureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time stamp holds RTC capture at first occurrence of time stamp event."]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(Tscapture::First)
    }
    #[doc = "Time stamp holds RTC capture at last occurrence of time stamp event."]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(Tscapture::Last)
    }
}
impl R {
    #[doc = "Bit 0 - Time Stamp by Tamper I/O 0 enable"]
    #[inline(always)]
    pub fn tstioen0(&self) -> Tstioen0R {
        Tstioen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Stamp by Tamper I/O 1 enable"]
    #[inline(always)]
    pub fn tstioen1(&self) -> Tstioen1R {
        Tstioen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Stamp by Tamper I/O 2 enable"]
    #[inline(always)]
    pub fn tstioen2(&self) -> Tstioen2R {
        Tstioen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Stamp by Tamper I/O 3 enable"]
    #[inline(always)]
    pub fn tstioen3(&self) -> Tstioen3R {
        Tstioen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Stamp by Tamper I/O 4 enable"]
    #[inline(always)]
    pub fn tstioen4(&self) -> Tstioen4R {
        Tstioen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time Stamp by Tamper I/O 5 enable"]
    #[inline(always)]
    pub fn tstioen5(&self) -> Tstioen5R {
        Tstioen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Time Stamp by Tamper I/O 6 enable"]
    #[inline(always)]
    pub fn tstioen6(&self) -> Tstioen6R {
        Tstioen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time Stamp by Tamper I/O 7 enable"]
    #[inline(always)]
    pub fn tstioen7(&self) -> Tstioen7R {
        Tstioen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time Stamp by Tamper I/O 8 enable"]
    #[inline(always)]
    pub fn tstioen8(&self) -> Tstioen8R {
        Tstioen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Time Stamp by Tamper I/O 9 enable"]
    #[inline(always)]
    pub fn tstioen9(&self) -> Tstioen9R {
        Tstioen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Time Stamp by Tamper I/O 10 enable"]
    #[inline(always)]
    pub fn tstioen10(&self) -> Tstioen10R {
        Tstioen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time Stamp by Tamper I/O 11 enable"]
    #[inline(always)]
    pub fn tstioen11(&self) -> Tstioen11R {
        Tstioen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time Stamp by Tamper I/O 12 enable"]
    #[inline(always)]
    pub fn tstioen12(&self) -> Tstioen12R {
        Tstioen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Time Stamp by Tamper I/O 13 enable"]
    #[inline(always)]
    pub fn tstioen13(&self) -> Tstioen13R {
        Tstioen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Time Stamp by Tamper I/O 14 enable"]
    #[inline(always)]
    pub fn tstioen14(&self) -> Tstioen14R {
        Tstioen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Time Stamp by Tamper I/O 15 enable"]
    #[inline(always)]
    pub fn tstioen15(&self) -> Tstioen15R {
        Tstioen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Time Stamp by VDD Loss detection enable"]
    #[inline(always)]
    pub fn tsvdden(&self) -> TsvddenR {
        TsvddenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Defines the capture method of the RTC timestamp when a time stamp event occurens."]
    #[inline(always)]
    pub fn tscapture(&self) -> TscaptureR {
        TscaptureR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Stamp by Tamper I/O 0 enable"]
    #[inline(always)]
    pub fn tstioen0(&mut self) -> Tstioen0W<'_, LfssTsctlSpec> {
        Tstioen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Time Stamp by Tamper I/O 1 enable"]
    #[inline(always)]
    pub fn tstioen1(&mut self) -> Tstioen1W<'_, LfssTsctlSpec> {
        Tstioen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Time Stamp by Tamper I/O 2 enable"]
    #[inline(always)]
    pub fn tstioen2(&mut self) -> Tstioen2W<'_, LfssTsctlSpec> {
        Tstioen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Time Stamp by Tamper I/O 3 enable"]
    #[inline(always)]
    pub fn tstioen3(&mut self) -> Tstioen3W<'_, LfssTsctlSpec> {
        Tstioen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Time Stamp by Tamper I/O 4 enable"]
    #[inline(always)]
    pub fn tstioen4(&mut self) -> Tstioen4W<'_, LfssTsctlSpec> {
        Tstioen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Time Stamp by Tamper I/O 5 enable"]
    #[inline(always)]
    pub fn tstioen5(&mut self) -> Tstioen5W<'_, LfssTsctlSpec> {
        Tstioen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Time Stamp by Tamper I/O 6 enable"]
    #[inline(always)]
    pub fn tstioen6(&mut self) -> Tstioen6W<'_, LfssTsctlSpec> {
        Tstioen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Time Stamp by Tamper I/O 7 enable"]
    #[inline(always)]
    pub fn tstioen7(&mut self) -> Tstioen7W<'_, LfssTsctlSpec> {
        Tstioen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Time Stamp by Tamper I/O 8 enable"]
    #[inline(always)]
    pub fn tstioen8(&mut self) -> Tstioen8W<'_, LfssTsctlSpec> {
        Tstioen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Time Stamp by Tamper I/O 9 enable"]
    #[inline(always)]
    pub fn tstioen9(&mut self) -> Tstioen9W<'_, LfssTsctlSpec> {
        Tstioen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Time Stamp by Tamper I/O 10 enable"]
    #[inline(always)]
    pub fn tstioen10(&mut self) -> Tstioen10W<'_, LfssTsctlSpec> {
        Tstioen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Time Stamp by Tamper I/O 11 enable"]
    #[inline(always)]
    pub fn tstioen11(&mut self) -> Tstioen11W<'_, LfssTsctlSpec> {
        Tstioen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Time Stamp by Tamper I/O 12 enable"]
    #[inline(always)]
    pub fn tstioen12(&mut self) -> Tstioen12W<'_, LfssTsctlSpec> {
        Tstioen12W::new(self, 12)
    }
    #[doc = "Bit 13 - Time Stamp by Tamper I/O 13 enable"]
    #[inline(always)]
    pub fn tstioen13(&mut self) -> Tstioen13W<'_, LfssTsctlSpec> {
        Tstioen13W::new(self, 13)
    }
    #[doc = "Bit 14 - Time Stamp by Tamper I/O 14 enable"]
    #[inline(always)]
    pub fn tstioen14(&mut self) -> Tstioen14W<'_, LfssTsctlSpec> {
        Tstioen14W::new(self, 14)
    }
    #[doc = "Bit 15 - Time Stamp by Tamper I/O 15 enable"]
    #[inline(always)]
    pub fn tstioen15(&mut self) -> Tstioen15W<'_, LfssTsctlSpec> {
        Tstioen15W::new(self, 15)
    }
    #[doc = "Bit 16 - Time Stamp by VDD Loss detection enable"]
    #[inline(always)]
    pub fn tsvdden(&mut self) -> TsvddenW<'_, LfssTsctlSpec> {
        TsvddenW::new(self, 16)
    }
    #[doc = "Bit 20 - Defines the capture method of the RTC timestamp when a time stamp event occurens."]
    #[inline(always)]
    pub fn tscapture(&mut self) -> TscaptureW<'_, LfssTsctlSpec> {
        TscaptureW::new(self, 20)
    }
}
#[doc = "Time Stamp Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTsctlSpec;
impl crate::RegisterSpec for LfssTsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tsctl::R`](R) reader structure"]
impl crate::Readable for LfssTsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tsctl::W`](W) writer structure"]
impl crate::Writable for LfssTsctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TSCTL to value 0"]
impl crate::Resettable for LfssTsctlSpec {}
