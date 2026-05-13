#[doc = "Register `DEBUGSS_RIS` reader"]
pub type R = crate::R<DebugssRisSpec>;
#[doc = "Raw interrupt status for TXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "0: TXIFG did not occur"]
    Clr = 0,
    #[doc = "1: TXIFG occurred"]
    Set = 1,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` reader - Raw interrupt status for TXIFG"]
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
    #[doc = "TXIFG did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txifg::Clr
    }
    #[doc = "TXIFG occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txifg::Set
    }
}
#[doc = "Raw interrupt status for RXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "0: RXIFG did not occur"]
    Clr = 0,
    #[doc = "1: RXIFG occurred"]
    Set = 1,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` reader - Raw interrupt status for RXIFG"]
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
    #[doc = "RXIFG did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxifg::Clr
    }
    #[doc = "RXIFG occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxifg::Set
    }
}
#[doc = "Raw interrupt status for PWRUPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "0: PWRUPIFG did not occur"]
    Clr = 0,
    #[doc = "1: PWRUPIFG occurred"]
    Set = 1,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` reader - Raw interrupt status for PWRUPIFG"]
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
    #[doc = "PWRUPIFG did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrupifg::Clr
    }
    #[doc = "PWRUPIFG occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrupifg::Set
    }
}
#[doc = "Raw interrupt status for PWRDWNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "0: PWRUPIFG did not occur"]
    Clr = 0,
    #[doc = "1: PWRUPIFG occurred"]
    Set = 1,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` reader - Raw interrupt status for PWRDWNIFG"]
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
    #[doc = "PWRUPIFG did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrdwnifg::Clr
    }
    #[doc = "PWRUPIFG occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrdwnifg::Set
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt status for TXIFG"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt status for RXIFG"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status for PWRUPIFG"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status for PWRDWNIFG"]
    #[inline(always)]
    pub fn pwrdwnifg(&self) -> PwrdwnifgR {
        PwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssRisSpec;
impl crate::RegisterSpec for DebugssRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_ris::R`](R) reader structure"]
impl crate::Readable for DebugssRisSpec {}
#[doc = "`reset()` method sets DEBUGSS_RIS to value 0"]
impl crate::Resettable for DebugssRisSpec {}
