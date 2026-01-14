#[doc = "Register `GPIOC_RSTCTL` writer"]
pub type W = crate::W<GpiocRstctlSpec>;
#[doc = "Assert reset to the peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetassert {
    #[doc = "0: Writing 0 has no effect"]
    Nop = 0,
    #[doc = "1: Assert reset"]
    Assert = 1,
}
impl From<Resetassert> for bool {
    #[inline(always)]
    fn from(variant: Resetassert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETASSERT` writer - Assert reset to the peripheral"]
pub type ResetassertW<'a, REG> = crate::BitWriter<'a, REG, Resetassert>;
impl<'a, REG> ResetassertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Resetassert::Nop)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Resetassert::Assert)
    }
}
#[doc = "Clear the RESETSTKY bit in the STAT register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetstkyclr {
    #[doc = "0: Writing 0 has no effect"]
    Nop = 0,
    #[doc = "1: Clear reset sticky bit"]
    Clr = 1,
}
impl From<Resetstkyclr> for bool {
    #[inline(always)]
    fn from(variant: Resetstkyclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETSTKYCLR` writer - Clear the RESETSTKY bit in the STAT register"]
pub type ResetstkyclrW<'a, REG> = crate::BitWriter<'a, REG, Resetstkyclr>;
impl<'a, REG> ResetstkyclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstkyclr::Nop)
    }
    #[doc = "Clear reset sticky bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Resetstkyclr::Clr)
    }
}
#[doc = "Key unlock for GPIOC reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyUnlockw {
    #[doc = "0: Lock reset"]
    Lock = 0,
    #[doc = "177: Unlock reset"]
    Unlock = 177,
}
impl From<KeyUnlockw> for u8 {
    #[inline(always)]
    fn from(variant: KeyUnlockw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KeyUnlockw {
    type Ux = u8;
}
impl crate::IsEnum for KeyUnlockw {}
#[doc = "Field `KEY_UNLOCK` writer - Key unlock for GPIOC reset control"]
pub type KeyUnlockW<'a, REG> = crate::FieldWriter<'a, REG, 8, KeyUnlockw>;
impl<'a, REG> KeyUnlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lock reset"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(KeyUnlockw::Lock)
    }
    #[doc = "Unlock reset"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(KeyUnlockw::Unlock)
    }
}
impl W {
    #[doc = "Bit 0 - Assert reset to the peripheral"]
    #[inline(always)]
    pub fn resetassert(&mut self) -> ResetassertW<'_, GpiocRstctlSpec> {
        ResetassertW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the RESETSTKY bit in the STAT register"]
    #[inline(always)]
    pub fn resetstkyclr(&mut self) -> ResetstkyclrW<'_, GpiocRstctlSpec> {
        ResetstkyclrW::new(self, 1)
    }
    #[doc = "Bits 24:31 - Key unlock for GPIOC reset control"]
    #[inline(always)]
    pub fn key_unlock(&mut self) -> KeyUnlockW<'_, GpiocRstctlSpec> {
        KeyUnlockW::new(self, 24)
    }
}
#[doc = "Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_rstctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocRstctlSpec;
impl crate::RegisterSpec for GpiocRstctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioc_rstctl::W`](W) writer structure"]
impl crate::Writable for GpiocRstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_RSTCTL to value 0"]
impl crate::Resettable for GpiocRstctlSpec {}
