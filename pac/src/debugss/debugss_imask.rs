#[doc = "Register `DEBUGSS_IMASK` reader"]
pub type R = crate::R<DebugssImaskSpec>;
#[doc = "Register `DEBUGSS_IMASK` writer"]
pub type W = crate::W<DebugssImaskSpec>;
#[doc = "Masks TXIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` reader - Masks TXIFG in MIS register"]
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
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txifg::Set
    }
}
#[doc = "Field `TXIFG` writer - Masks TXIFG in MIS register"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG, Txifg>;
impl<'a, REG> TxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Set)
    }
}
#[doc = "Masks RXIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` reader - Masks RXIFG in MIS register"]
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
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxifg::Set
    }
}
#[doc = "Field `RXIFG` writer - Masks RXIFG in MIS register"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG, Rxifg>;
impl<'a, REG> RxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Set)
    }
}
#[doc = "Masks PWRUPIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` reader - Masks PWRUPIFG in MIS register"]
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
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrupifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrupifg::Set
    }
}
#[doc = "Field `PWRUPIFG` writer - Masks PWRUPIFG in MIS register"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrupifg>;
impl<'a, REG> PwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Set)
    }
}
#[doc = "Masks PWRDWNIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` reader - Masks PWRDWNIFG in MIS register"]
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
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrdwnifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrdwnifg::Set
    }
}
#[doc = "Field `PWRDWNIFG` writer - Masks PWRDWNIFG in MIS register"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrdwnifg>;
impl<'a, REG> PwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Masks TXIFG in MIS register"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks RXIFG in MIS register"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks PWRUPIFG in MIS register"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks PWRDWNIFG in MIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&self) -> PwrdwnifgR {
        PwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks TXIFG in MIS register"]
    #[inline(always)]
    pub fn txifg(&mut self) -> TxifgW<'_, DebugssImaskSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Masks RXIFG in MIS register"]
    #[inline(always)]
    pub fn rxifg(&mut self) -> RxifgW<'_, DebugssImaskSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Masks PWRUPIFG in MIS register"]
    #[inline(always)]
    pub fn pwrupifg(&mut self) -> PwrupifgW<'_, DebugssImaskSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Masks PWRDWNIFG in MIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<'_, DebugssImaskSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssImaskSpec;
impl crate::RegisterSpec for DebugssImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_imask::R`](R) reader structure"]
impl crate::Readable for DebugssImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`debugss_imask::W`](W) writer structure"]
impl crate::Writable for DebugssImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUGSS_IMASK to value 0"]
impl crate::Resettable for DebugssImaskSpec {}
