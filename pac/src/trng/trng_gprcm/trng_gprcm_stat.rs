#[doc = "Register `TRNG_GPRCM_STAT` reader"]
pub type R = crate::R<TrngGprcmStatSpec>;
#[doc = "This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetstky {
    #[doc = "0: The peripheral has not been reset since this bit was last cleared by RESETSTKYCLR in the RSTCTL register"]
    Nores = 0,
    #[doc = "1: The peripheral was reset since the last bit clear"]
    Reset = 1,
}
impl From<Resetstky> for bool {
    #[inline(always)]
    fn from(variant: Resetstky) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETSTKY` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type ResetstkyR = crate::BitReader<Resetstky>;
impl ResetstkyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetstky {
        match self.bits {
            false => Resetstky::Nores,
            true => Resetstky::Reset,
        }
    }
    #[doc = "The peripheral has not been reset since this bit was last cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn is_nores(&self) -> bool {
        *self == Resetstky::Nores
    }
    #[doc = "The peripheral was reset since the last bit clear"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Resetstky::Reset
    }
}
impl R {
    #[doc = "Bit 16 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn resetstky(&self) -> ResetstkyR {
        ResetstkyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_gprcm_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngGprcmStatSpec;
impl crate::RegisterSpec for TrngGprcmStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_gprcm_stat::R`](R) reader structure"]
impl crate::Readable for TrngGprcmStatSpec {}
#[doc = "`reset()` method sets TRNG_GPRCM_STAT to value 0"]
impl crate::Resettable for TrngGprcmStatSpec {}
