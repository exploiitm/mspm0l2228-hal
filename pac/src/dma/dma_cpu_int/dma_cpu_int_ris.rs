#[doc = "Register `DMA_CPU_INT_RIS` reader"]
pub type R = crate::R<DmaCpuIntRisSpec>;
#[doc = "DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach0 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach0> for bool {
    #[inline(always)]
    fn from(variant: Dmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH0` reader - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach0R = crate::BitReader<Dmach0>;
impl Dmach0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach0 {
        match self.bits {
            false => Dmach0::Clr,
            true => Dmach0::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach0::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach0::Set
    }
}
#[doc = "DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach1 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach1> for bool {
    #[inline(always)]
    fn from(variant: Dmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH1` reader - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach1R = crate::BitReader<Dmach1>;
impl Dmach1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach1 {
        match self.bits {
            false => Dmach1::Clr,
            true => Dmach1::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach1::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach1::Set
    }
}
#[doc = "DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach2 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach2> for bool {
    #[inline(always)]
    fn from(variant: Dmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH2` reader - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach2R = crate::BitReader<Dmach2>;
impl Dmach2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach2 {
        match self.bits {
            false => Dmach2::Clr,
            true => Dmach2::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach2::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach2::Set
    }
}
#[doc = "DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach3 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach3> for bool {
    #[inline(always)]
    fn from(variant: Dmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH3` reader - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach3R = crate::BitReader<Dmach3>;
impl Dmach3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach3 {
        match self.bits {
            false => Dmach3::Clr,
            true => Dmach3::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach3::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach3::Set
    }
}
#[doc = "DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach4 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach4> for bool {
    #[inline(always)]
    fn from(variant: Dmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH4` reader - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach4R = crate::BitReader<Dmach4>;
impl Dmach4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach4 {
        match self.bits {
            false => Dmach4::Clr,
            true => Dmach4::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach4::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach4::Set
    }
}
#[doc = "DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach5 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach5> for bool {
    #[inline(always)]
    fn from(variant: Dmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH5` reader - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach5R = crate::BitReader<Dmach5>;
impl Dmach5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach5 {
        match self.bits {
            false => Dmach5::Clr,
            true => Dmach5::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach5::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach5::Set
    }
}
#[doc = "DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach6 {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occurred"]
    Set = 1,
}
impl From<Dmach6> for bool {
    #[inline(always)]
    fn from(variant: Dmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH6` reader - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach6R = crate::BitReader<Dmach6>;
impl Dmach6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmach6 {
        match self.bits {
            false => Dmach6::Clr,
            true => Dmach6::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dmach6::Clr
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmach6::Set
    }
}
#[doc = "Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preirqch0 {
    #[doc = "0: Clear interrupt mask bit"]
    Clr = 0,
    #[doc = "1: Set interrupt mask bit"]
    Set = 1,
}
impl From<Preirqch0> for bool {
    #[inline(always)]
    fn from(variant: Preirqch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREIRQCH0` reader - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type Preirqch0R = crate::BitReader<Preirqch0>;
impl Preirqch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preirqch0 {
        match self.bits {
            false => Preirqch0::Clr,
            true => Preirqch0::Set,
        }
    }
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Preirqch0::Clr
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Preirqch0::Set
    }
}
#[doc = "Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preirqch1 {
    #[doc = "0: Clear interrupt mask bit"]
    Clr = 0,
    #[doc = "1: Set interrupt mask bit"]
    Set = 1,
}
impl From<Preirqch1> for bool {
    #[inline(always)]
    fn from(variant: Preirqch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREIRQCH1` reader - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type Preirqch1R = crate::BitReader<Preirqch1>;
impl Preirqch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preirqch1 {
        match self.bits {
            false => Preirqch1::Clr,
            true => Preirqch1::Set,
        }
    }
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Preirqch1::Clr
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Preirqch1::Set
    }
}
#[doc = "Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preirqch2 {
    #[doc = "0: Clear interrupt mask bit"]
    Clr = 0,
    #[doc = "1: Set interrupt mask bit"]
    Set = 1,
}
impl From<Preirqch2> for bool {
    #[inline(always)]
    fn from(variant: Preirqch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREIRQCH2` reader - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type Preirqch2R = crate::BitReader<Preirqch2>;
impl Preirqch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preirqch2 {
        match self.bits {
            false => Preirqch2::Clr,
            true => Preirqch2::Set,
        }
    }
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Preirqch2::Clr
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Preirqch2::Set
    }
}
#[doc = "DMA address error, SRC address not reachable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrerr {
    #[doc = "0: Clear interrupt mask bit"]
    Clr = 0,
    #[doc = "1: Set interrupt mask bit"]
    Set = 1,
}
impl From<Addrerr> for bool {
    #[inline(always)]
    fn from(variant: Addrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRERR` reader - DMA address error, SRC address not reachable."]
pub type AddrerrR = crate::BitReader<Addrerr>;
impl AddrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrerr {
        match self.bits {
            false => Addrerr::Clr,
            true => Addrerr::Set,
        }
    }
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Addrerr::Clr
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Addrerr::Set
    }
}
#[doc = "DMA data error, SRC data might be corrupted (PAR or ECC error).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataerr {
    #[doc = "0: Clear interrupt mask bit"]
    Clr = 0,
    #[doc = "1: Set interrupt mask bit"]
    Set = 1,
}
impl From<Dataerr> for bool {
    #[inline(always)]
    fn from(variant: Dataerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAERR` reader - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type DataerrR = crate::BitReader<Dataerr>;
impl DataerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataerr {
        match self.bits {
            false => Dataerr::Clr,
            true => Dataerr::Set,
        }
    }
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dataerr::Clr
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dataerr::Set
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach0(&self) -> Dmach0R {
        Dmach0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach1(&self) -> Dmach1R {
        Dmach1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach2(&self) -> Dmach2R {
        Dmach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach3(&self) -> Dmach3R {
        Dmach3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach4(&self) -> Dmach4R {
        Dmach4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach5(&self) -> Dmach5R {
        Dmach5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach6(&self) -> Dmach6R {
        Dmach6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch0(&self) -> Preirqch0R {
        Preirqch0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch1(&self) -> Preirqch1R {
        Preirqch1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch2(&self) -> Preirqch2R {
        Preirqch2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn addrerr(&self) -> AddrerrR {
        AddrerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn dataerr(&self) -> DataerrR {
        DataerrR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpu_int_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCpuIntRisSpec;
impl crate::RegisterSpec for DmaCpuIntRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cpu_int_ris::R`](R) reader structure"]
impl crate::Readable for DmaCpuIntRisSpec {}
#[doc = "`reset()` method sets DMA_CPU_INT_RIS to value 0"]
impl crate::Resettable for DmaCpuIntRisSpec {}
