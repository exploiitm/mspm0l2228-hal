#[doc = "Register `DEBUGSS_ISET` writer"]
pub type W = crate::W<DebugssIsetSpec>;
#[doc = "Sets TXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to TXIFG is set"]
    Set = 1,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` writer - Sets TXIFG in RIS register"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG, Txifg>;
impl<'a, REG> TxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to TXIFG is set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Set)
    }
}
#[doc = "Sets RXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to RXIFG is set"]
    Set = 1,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` writer - Sets RXIFG in RIS register"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG, Rxifg>;
impl<'a, REG> RxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to RXIFG is set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Set)
    }
}
#[doc = "Sets PWRUPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to PWRUPIFG is set"]
    Set = 1,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` writer - Sets PWRUPIFG in RIS register"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrupifg>;
impl<'a, REG> PwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to PWRUPIFG is set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Set)
    }
}
#[doc = "Sets PWRDWNIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to PWRUPIFG is set"]
    Set = 1,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` writer - Sets PWRDWNIFG in RIS register"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrdwnifg>;
impl<'a, REG> PwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to PWRUPIFG is set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Sets TXIFG in RIS register"]
    #[inline(always)]
    pub fn txifg(&mut self) -> TxifgW<'_, DebugssIsetSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets RXIFG in RIS register"]
    #[inline(always)]
    pub fn rxifg(&mut self) -> RxifgW<'_, DebugssIsetSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Sets PWRUPIFG in RIS register"]
    #[inline(always)]
    pub fn pwrupifg(&mut self) -> PwrupifgW<'_, DebugssIsetSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Sets PWRDWNIFG in RIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<'_, DebugssIsetSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssIsetSpec;
impl crate::RegisterSpec for DebugssIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`debugss_iset::W`](W) writer structure"]
impl crate::Writable for DebugssIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUGSS_ISET to value 0"]
impl crate::Resettable for DebugssIsetSpec {}
