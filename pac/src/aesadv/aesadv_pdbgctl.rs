#[doc = "Register `AESADV_PDBGCTL` reader"]
pub type R = crate::R<AesadvPdbgctlSpec>;
#[doc = "Free run control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Free {
    #[doc = "1: The peripheral ignores the state of the Core Halted input"]
    Run = 1,
}
impl From<Free> for bool {
    #[inline(always)]
    fn from(variant: Free) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREE` reader - Free run control"]
pub type FreeR = crate::BitReader<Free>;
impl FreeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Free> {
        match self.bits {
            true => Some(Free::Run),
            _ => None,
        }
    }
    #[doc = "The peripheral ignores the state of the Core Halted input"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Free::Run
    }
}
impl R {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
}
#[doc = "Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_pdbgctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvPdbgctlSpec;
impl crate::RegisterSpec for AesadvPdbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_pdbgctl::R`](R) reader structure"]
impl crate::Readable for AesadvPdbgctlSpec {}
#[doc = "`reset()` method sets AESADV_PDBGCTL to value 0"]
impl crate::Resettable for AesadvPdbgctlSpec {}
