#[doc = "Register `DMA_GEN_EVENT_ISET` writer"]
pub type W = crate::W<DmaGenEventIsetSpec>;
#[doc = "DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach0 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach0> for bool {
    #[inline(always)]
    fn from(variant: Dmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH0` writer - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach0W<'a, REG> = crate::BitWriter<'a, REG, Dmach0>;
impl<'a, REG> Dmach0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach0::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach0::Set)
    }
}
#[doc = "DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach1 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach1> for bool {
    #[inline(always)]
    fn from(variant: Dmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH1` writer - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach1W<'a, REG> = crate::BitWriter<'a, REG, Dmach1>;
impl<'a, REG> Dmach1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach1::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach1::Set)
    }
}
#[doc = "DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach2 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach2> for bool {
    #[inline(always)]
    fn from(variant: Dmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH2` writer - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach2W<'a, REG> = crate::BitWriter<'a, REG, Dmach2>;
impl<'a, REG> Dmach2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach2::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach2::Set)
    }
}
#[doc = "DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach3 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach3> for bool {
    #[inline(always)]
    fn from(variant: Dmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH3` writer - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach3W<'a, REG> = crate::BitWriter<'a, REG, Dmach3>;
impl<'a, REG> Dmach3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach3::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach3::Set)
    }
}
#[doc = "DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach4 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach4> for bool {
    #[inline(always)]
    fn from(variant: Dmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH4` writer - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach4W<'a, REG> = crate::BitWriter<'a, REG, Dmach4>;
impl<'a, REG> Dmach4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach4::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach4::Set)
    }
}
#[doc = "DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach5 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach5> for bool {
    #[inline(always)]
    fn from(variant: Dmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH5` writer - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach5W<'a, REG> = crate::BitWriter<'a, REG, Dmach5>;
impl<'a, REG> Dmach5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach5::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach5::Set)
    }
}
#[doc = "DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmach6 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set interrupt"]
    Set = 1,
}
impl From<Dmach6> for bool {
    #[inline(always)]
    fn from(variant: Dmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACH6` writer - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach6W<'a, REG> = crate::BitWriter<'a, REG, Dmach6>;
impl<'a, REG> Dmach6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach6::NoEffect)
    }
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmach6::Set)
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
#[doc = "Field `PREIRQCH0` writer - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type Preirqch0W<'a, REG> = crate::BitWriter<'a, REG, Preirqch0>;
impl<'a, REG> Preirqch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Preirqch0::Clr)
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Preirqch0::Set)
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
#[doc = "Field `PREIRQCH1` writer - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type Preirqch1W<'a, REG> = crate::BitWriter<'a, REG, Preirqch1>;
impl<'a, REG> Preirqch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Preirqch1::Clr)
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Preirqch1::Set)
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
#[doc = "Field `PREIRQCH2` writer - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type Preirqch2W<'a, REG> = crate::BitWriter<'a, REG, Preirqch2>;
impl<'a, REG> Preirqch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Preirqch2::Clr)
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Preirqch2::Set)
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
#[doc = "Field `ADDRERR` writer - DMA address error, SRC address not reachable."]
pub type AddrerrW<'a, REG> = crate::BitWriter<'a, REG, Addrerr>;
impl<'a, REG> AddrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Addrerr::Clr)
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Addrerr::Set)
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
#[doc = "Field `DATAERR` writer - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type DataerrW<'a, REG> = crate::BitWriter<'a, REG, Dataerr>;
impl<'a, REG> DataerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dataerr::Clr)
    }
    #[doc = "Set interrupt mask bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dataerr::Set)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach0(&mut self) -> Dmach0W<'_, DmaGenEventIsetSpec> {
        Dmach0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach1(&mut self) -> Dmach1W<'_, DmaGenEventIsetSpec> {
        Dmach1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach2(&mut self) -> Dmach2W<'_, DmaGenEventIsetSpec> {
        Dmach2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach3(&mut self) -> Dmach3W<'_, DmaGenEventIsetSpec> {
        Dmach3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach4(&mut self) -> Dmach4W<'_, DmaGenEventIsetSpec> {
        Dmach4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach5(&mut self) -> Dmach5W<'_, DmaGenEventIsetSpec> {
        Dmach5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach6(&mut self) -> Dmach6W<'_, DmaGenEventIsetSpec> {
        Dmach6W::new(self, 6)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch0(&mut self) -> Preirqch0W<'_, DmaGenEventIsetSpec> {
        Preirqch0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch1(&mut self) -> Preirqch1W<'_, DmaGenEventIsetSpec> {
        Preirqch1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch2(&mut self) -> Preirqch2W<'_, DmaGenEventIsetSpec> {
        Preirqch2W::new(self, 18)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn addrerr(&mut self) -> AddrerrW<'_, DmaGenEventIsetSpec> {
        AddrerrW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn dataerr(&mut self) -> DataerrW<'_, DmaGenEventIsetSpec> {
        DataerrW::new(self, 25)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_gen_event_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaGenEventIsetSpec;
impl crate::RegisterSpec for DmaGenEventIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_gen_event_iset::W`](W) writer structure"]
impl crate::Writable for DmaGenEventIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_GEN_EVENT_ISET to value 0"]
impl crate::Resettable for DmaGenEventIsetSpec {}
