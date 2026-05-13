#[doc = "Register `KEYSTORECTL_KEYRD` reader"]
pub type R = crate::R<KeystorectlKeyrdSpec>;
#[doc = "Register `KEYSTORECTL_KEYRD` writer"]
pub type W = crate::W<KeystorectlKeyrdSpec>;
#[doc = "Key size selection. Selection of 128 or 256 bit keys\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyszsel {
    #[doc = "0: 256 bit key"]
    K256 = 0,
    #[doc = "1: 128 bit key"]
    K128 = 1,
}
impl From<Keyszsel> for u8 {
    #[inline(always)]
    fn from(variant: Keyszsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyszsel {
    type Ux = u8;
}
impl crate::IsEnum for Keyszsel {}
#[doc = "Field `KEYSZSEL` reader - Key size selection. Selection of 128 or 256 bit keys"]
pub type KeyszselR = crate::FieldReader<Keyszsel>;
impl KeyszselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyszsel> {
        match self.bits {
            0 => Some(Keyszsel::K256),
            1 => Some(Keyszsel::K128),
            _ => None,
        }
    }
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == Keyszsel::K256
    }
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        *self == Keyszsel::K128
    }
}
#[doc = "Field `KEYSZSEL` writer - Key size selection. Selection of 128 or 256 bit keys"]
pub type KeyszselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Keyszsel>;
impl<'a, REG> KeyszselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn k256(self) -> &'a mut crate::W<REG> {
        self.variant(Keyszsel::K256)
    }
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn k128(self) -> &'a mut crate::W<REG> {
        self.variant(Keyszsel::K128)
    }
}
#[doc = "Field `KEYSLOTSEL` reader - Select the key slot to read the key from. NOTE: SW needs to ensure that it is reading from the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when reading a 256-bit key, two adjacent slots will be read."]
pub type KeyslotselR = crate::FieldReader;
#[doc = "Field `KEYSLOTSEL` writer - Select the key slot to read the key from. NOTE: SW needs to ensure that it is reading from the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when reading a 256-bit key, two adjacent slots will be read."]
pub type KeyslotselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Crypto engine selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cryptosel {
    #[doc = "0: Transfer key to AES"]
    Aes = 0,
}
impl From<Cryptosel> for u8 {
    #[inline(always)]
    fn from(variant: Cryptosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cryptosel {
    type Ux = u8;
}
impl crate::IsEnum for Cryptosel {}
#[doc = "Field `CRYPTOSEL` reader - Crypto engine selector"]
pub type CryptoselR = crate::FieldReader<Cryptosel>;
impl CryptoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cryptosel> {
        match self.bits {
            0 => Some(Cryptosel::Aes),
            _ => None,
        }
    }
    #[doc = "Transfer key to AES"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == Cryptosel::Aes
    }
}
#[doc = "Field `CRYPTOSEL` writer - Crypto engine selector"]
pub type CryptoselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cryptosel>;
impl<'a, REG> CryptoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transfer key to AES"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptosel::Aes)
    }
}
impl R {
    #[doc = "Bits 0:2 - Key size selection. Selection of 128 or 256 bit keys"]
    #[inline(always)]
    pub fn keyszsel(&self) -> KeyszselR {
        KeyszselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Select the key slot to read the key from. NOTE: SW needs to ensure that it is reading from the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when reading a 256-bit key, two adjacent slots will be read."]
    #[inline(always)]
    pub fn keyslotsel(&self) -> KeyslotselR {
        KeyslotselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Crypto engine selector"]
    #[inline(always)]
    pub fn cryptosel(&self) -> CryptoselR {
        CryptoselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Key size selection. Selection of 128 or 256 bit keys"]
    #[inline(always)]
    pub fn keyszsel(&mut self) -> KeyszselW<'_, KeystorectlKeyrdSpec> {
        KeyszselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select the key slot to read the key from. NOTE: SW needs to ensure that it is reading from the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when reading a 256-bit key, two adjacent slots will be read."]
    #[inline(always)]
    pub fn keyslotsel(&mut self) -> KeyslotselW<'_, KeystorectlKeyrdSpec> {
        KeyslotselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Crypto engine selector"]
    #[inline(always)]
    pub fn cryptosel(&mut self) -> CryptoselW<'_, KeystorectlKeyrdSpec> {
        CryptoselW::new(self, 8)
    }
}
#[doc = "Key read configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keystorectl_keyrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keystorectl_keyrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeystorectlKeyrdSpec;
impl crate::RegisterSpec for KeystorectlKeyrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keystorectl_keyrd::R`](R) reader structure"]
impl crate::Readable for KeystorectlKeyrdSpec {}
#[doc = "`write(|w| ..)` method takes [`keystorectl_keyrd::W`](W) writer structure"]
impl crate::Writable for KeystorectlKeyrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSTORECTL_KEYRD to value 0"]
impl crate::Resettable for KeystorectlKeyrdSpec {}
