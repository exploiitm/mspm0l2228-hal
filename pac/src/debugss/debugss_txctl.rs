#[doc = "Register `DEBUGSS_TXCTL` reader"]
pub type R = crate::R<DebugssTxctlSpec>;
#[doc = "Indicates data request in DSSM.TXD, set on write via Debug AP to DSSM.TXD. A read of the DSSM.TXD register by SW will clear the TX field. The tool can check that TXD is empty by reading this field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transmit {
    #[doc = "0: TXD is empty"]
    Empty = 0,
    #[doc = "1: TXD is full"]
    Full = 1,
}
impl From<Transmit> for bool {
    #[inline(always)]
    fn from(variant: Transmit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSMIT` reader - Indicates data request in DSSM.TXD, set on write via Debug AP to DSSM.TXD. A read of the DSSM.TXD register by SW will clear the TX field. The tool can check that TXD is empty by reading this field."]
pub type TransmitR = crate::BitReader<Transmit>;
impl TransmitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Transmit {
        match self.bits {
            false => Transmit::Empty,
            true => Transmit::Full,
        }
    }
    #[doc = "TXD is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Transmit::Empty
    }
    #[doc = "TXD is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Transmit::Full
    }
}
#[doc = "Field `TRANSMIT_FLAGS` reader - Generic TX flags that can be set by external debug tool. Functionality is defined by SW."]
pub type TransmitFlagsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Indicates data request in DSSM.TXD, set on write via Debug AP to DSSM.TXD. A read of the DSSM.TXD register by SW will clear the TX field. The tool can check that TXD is empty by reading this field."]
    #[inline(always)]
    pub fn transmit(&self) -> TransmitR {
        TransmitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Generic TX flags that can be set by external debug tool. Functionality is defined by SW."]
    #[inline(always)]
    pub fn transmit_flags(&self) -> TransmitFlagsR {
        TransmitFlagsR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "Transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_txctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssTxctlSpec;
impl crate::RegisterSpec for DebugssTxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_txctl::R`](R) reader structure"]
impl crate::Readable for DebugssTxctlSpec {}
#[doc = "`reset()` method sets DEBUGSS_TXCTL to value 0"]
impl crate::Resettable for DebugssTxctlSpec {}
