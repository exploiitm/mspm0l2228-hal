#[doc = "Register `UART1_CLKCFG` reader"]
pub type R = crate::R<Uart1ClkcfgSpec>;
#[doc = "Register `UART1_CLKCFG` writer"]
pub type W = crate::W<Uart1ClkcfgSpec>;
#[doc = "Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockasync {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Blockasync> for bool {
    #[inline(always)]
    fn from(variant: Blockasync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKASYNC` reader - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type BlockasyncR = crate::BitReader<Blockasync>;
impl BlockasyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockasync {
        match self.bits {
            false => Blockasync::Disable,
            true => Blockasync::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blockasync::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blockasync::Enable
    }
}
#[doc = "Field `BLOCKASYNC` writer - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type BlockasyncW<'a, REG> = crate::BitWriter<'a, REG, Blockasync>;
impl<'a, REG> BlockasyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Enable)
    }
}
#[doc = "Key unlock for UART1 clock config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyUnlock {
    #[doc = "0: Lock config"]
    Lock = 0,
    #[doc = "169: Unlock config"]
    Unlock = 169,
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
#[doc = "Field `KEY_UNLOCK` reader - Key unlock for UART1 clock config"]
pub type KeyUnlockR = crate::FieldReader<KeyUnlock>;
impl KeyUnlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KeyUnlock> {
        match self.bits {
            0 => Some(KeyUnlock::Lock),
            169 => Some(KeyUnlock::Unlock),
            _ => None,
        }
    }
    #[doc = "Lock config"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == KeyUnlock::Lock
    }
    #[doc = "Unlock config"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == KeyUnlock::Unlock
    }
}
#[doc = "Field `KEY_UNLOCK` writer - Key unlock for UART1 clock config"]
pub type KeyUnlockW<'a, REG> = crate::FieldWriter<'a, REG, 8, KeyUnlock>;
impl<'a, REG> KeyUnlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lock config"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(KeyUnlock::Lock)
    }
    #[doc = "Unlock config"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(KeyUnlock::Unlock)
    }
}
impl R {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn blockasync(&self) -> BlockasyncR {
        BlockasyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Key unlock for UART1 clock config"]
    #[inline(always)]
    pub fn key_unlock(&self) -> KeyUnlockR {
        KeyUnlockR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn blockasync(&mut self) -> BlockasyncW<'_, Uart1ClkcfgSpec> {
        BlockasyncW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Key unlock for UART1 clock config"]
    #[inline(always)]
    pub fn key_unlock(&mut self) -> KeyUnlockW<'_, Uart1ClkcfgSpec> {
        KeyUnlockW::new(self, 24)
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1ClkcfgSpec;
impl crate::RegisterSpec for Uart1ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_clkcfg::R`](R) reader structure"]
impl crate::Readable for Uart1ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_clkcfg::W`](W) writer structure"]
impl crate::Writable for Uart1ClkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_CLKCFG to value 0"]
impl crate::Resettable for Uart1ClkcfgSpec {}
