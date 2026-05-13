#[doc = "Register `SYSCTL_MGMT_GPTIMER16B2CCQEILP0_STAT` reader"]
pub type R = crate::R<SysctlMgmtGptimer16b2ccqeilp0StatSpec>;
#[doc = "IP has been Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetstky {
    #[doc = "0: `0`"]
    Clear = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Resetstky> for bool {
    #[inline(always)]
    fn from(variant: Resetstky) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETSTKY` reader - IP has been Reset"]
pub type ResetstkyR = crate::BitReader<Resetstky>;
impl ResetstkyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetstky {
        match self.bits {
            false => Resetstky::Clear,
            true => Resetstky::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Resetstky::Clear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Resetstky::Set
    }
}
impl R {
    #[doc = "Bit 16 - IP has been Reset"]
    #[inline(always)]
    pub fn resetstky(&self) -> ResetstkyR {
        ResetstkyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccqeilp0_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlMgmtGptimer16b2ccqeilp0StatSpec;
impl crate::RegisterSpec for SysctlMgmtGptimer16b2ccqeilp0StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_mgmt_gptimer16b2ccqeilp0_stat::R`](R) reader structure"]
impl crate::Readable for SysctlMgmtGptimer16b2ccqeilp0StatSpec {}
#[doc = "`reset()` method sets SYSCTL_MGMT_GPTIMER16B2CCQEILP0_STAT to value 0"]
impl crate::Resettable for SysctlMgmtGptimer16b2ccqeilp0StatSpec {}
