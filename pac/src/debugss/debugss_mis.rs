#[doc = "Register `DEBUGSS_MIS` reader"]
pub type R = crate::R<DebugssMisSpec>;
#[doc = "Masked interrupt status for TXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "0: TXIFG did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: TXIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` reader - Masked interrupt status for TXIFG"]
pub type TxifgR = crate::BitReader<Txifg>;
impl TxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txifg {
        match self.bits {
            false => Txifg::Clr,
            true => Txifg::Set,
        }
    }
    #[doc = "TXIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txifg::Clr
    }
    #[doc = "TXIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txifg::Set
    }
}
#[doc = "Masked interrupt status for RXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "0: RXIFG did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: RXIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` reader - Masked interrupt status for RXIFG"]
pub type RxifgR = crate::BitReader<Rxifg>;
impl RxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxifg {
        match self.bits {
            false => Rxifg::Clr,
            true => Rxifg::Set,
        }
    }
    #[doc = "RXIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxifg::Clr
    }
    #[doc = "RXIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxifg::Set
    }
}
#[doc = "Masked interrupt status for PWRUPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "0: PWRUPIFG did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: PWRUPIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` reader - Masked interrupt status for PWRUPIFG"]
pub type PwrupifgR = crate::BitReader<Pwrupifg>;
impl PwrupifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrupifg {
        match self.bits {
            false => Pwrupifg::Clr,
            true => Pwrupifg::Set,
        }
    }
    #[doc = "PWRUPIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrupifg::Clr
    }
    #[doc = "PWRUPIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrupifg::Set
    }
}
#[doc = "Masked interrupt status for PWRDWNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "0: PWRUPIFG did not request an interrupt service routine"]
    Clr = 0,
    #[doc = "1: PWRUPIFG requests an interrupt service routine"]
    Set = 1,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` reader - Masked interrupt status for PWRDWNIFG"]
pub type PwrdwnifgR = crate::BitReader<Pwrdwnifg>;
impl PwrdwnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrdwnifg {
        match self.bits {
            false => Pwrdwnifg::Clr,
            true => Pwrdwnifg::Set,
        }
    }
    #[doc = "PWRUPIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrdwnifg::Clr
    }
    #[doc = "PWRUPIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrdwnifg::Set
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt status for TXIFG"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt status for RXIFG"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt status for PWRUPIFG"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt status for PWRDWNIFG"]
    #[inline(always)]
    pub fn pwrdwnifg(&self) -> PwrdwnifgR {
        PwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssMisSpec;
impl crate::RegisterSpec for DebugssMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_mis::R`](R) reader structure"]
impl crate::Readable for DebugssMisSpec {}
#[doc = "`reset()` method sets DEBUGSS_MIS to value 0"]
impl crate::Resettable for DebugssMisSpec {}
