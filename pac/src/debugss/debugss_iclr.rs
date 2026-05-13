#[doc = "Register `DEBUGSS_ICLR` writer"]
pub type W = crate::W<DebugssIclrSpec>;
#[doc = "Clears TXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to TXIFG is cleared"]
    Clr = 1,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` writer - Clears TXIFG in RIS register"]
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
    #[doc = "RIS bit corresponding to TXIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Clr)
    }
}
#[doc = "Clears RXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to RXIFG is cleared"]
    Clr = 1,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` writer - Clears RXIFG in RIS register"]
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
    #[doc = "RIS bit corresponding to RXIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Clr)
    }
}
#[doc = "Clears PWRUPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to PWRUPIFG is cleared"]
    Clr = 1,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` writer - Clears PWRUPIFG in RIS register"]
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
    #[doc = "RIS bit corresponding to PWRUPIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Clr)
    }
}
#[doc = "Clears PWRDWNIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to PWRUPIFG is cleared"]
    Clr = 1,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` writer - Clears PWRDWNIFG in RIS register"]
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
    #[doc = "RIS bit corresponding to PWRUPIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clears TXIFG in RIS register"]
    #[inline(always)]
    pub fn txifg(&mut self) -> TxifgW<'_, DebugssIclrSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears RXIFG in RIS register"]
    #[inline(always)]
    pub fn rxifg(&mut self) -> RxifgW<'_, DebugssIclrSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears PWRUPIFG in RIS register"]
    #[inline(always)]
    pub fn pwrupifg(&mut self) -> PwrupifgW<'_, DebugssIclrSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears PWRDWNIFG in RIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<'_, DebugssIclrSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssIclrSpec;
impl crate::RegisterSpec for DebugssIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`debugss_iclr::W`](W) writer structure"]
impl crate::Writable for DebugssIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUGSS_ICLR to value 0"]
impl crate::Resettable for DebugssIclrSpec {}
