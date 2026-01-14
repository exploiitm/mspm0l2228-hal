#[doc = "Register `GPIOA_PWREN` reader"]
pub type R = crate::R<GpioaPwrenSpec>;
#[doc = "Register `GPIOA_PWREN` writer"]
pub type W = crate::W<GpioaPwrenSpec>;
#[doc = "Enable the power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable the power"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Enable the power"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "Key unlock for GPIOA power enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyUnlock {
    #[doc = "0: Lock pwren"]
    Lock = 0,
    #[doc = "38: Unlock pwren"]
    Unlock = 38,
}
impl From<KeyUnlock> for u8 {
    #[inline(always)]
    fn from(variant: KeyUnlock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KeyUnlock {
    type Ux = u8;
}
impl crate::IsEnum for KeyUnlock {}
#[doc = "Field `KEY_UNLOCK` reader - Key unlock for GPIOA power enable"]
pub type KeyUnlockR = crate::FieldReader<KeyUnlock>;
impl KeyUnlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KeyUnlock> {
        match self.bits {
            0 => Some(KeyUnlock::Lock),
            38 => Some(KeyUnlock::Unlock),
            _ => None,
        }
    }
    #[doc = "Lock pwren"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == KeyUnlock::Lock
    }
    #[doc = "Unlock pwren"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == KeyUnlock::Unlock
    }
}
#[doc = "Field `KEY_UNLOCK` writer - Key unlock for GPIOA power enable"]
pub type KeyUnlockW<'a, REG> = crate::FieldWriter<'a, REG, 8, KeyUnlock>;
impl<'a, REG> KeyUnlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lock pwren"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(KeyUnlock::Lock)
    }
    #[doc = "Unlock pwren"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(KeyUnlock::Unlock)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the power"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:31 - Key unlock for GPIOA power enable"]
    #[inline(always)]
    pub fn key_unlock(&self) -> KeyUnlockR {
        KeyUnlockR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the power"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, GpioaPwrenSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Key unlock for GPIOA power enable"]
    #[inline(always)]
    pub fn key_unlock(&mut self) -> KeyUnlockW<'_, GpioaPwrenSpec> {
        KeyUnlockW::new(self, 24)
    }
}
#[doc = "Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa_pwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_pwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaPwrenSpec;
impl crate::RegisterSpec for GpioaPwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa_pwren::R`](R) reader structure"]
impl crate::Readable for GpioaPwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa_pwren::W`](W) writer structure"]
impl crate::Writable for GpioaPwrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA_PWREN to value 0"]
impl crate::Resettable for GpioaPwrenSpec {}
