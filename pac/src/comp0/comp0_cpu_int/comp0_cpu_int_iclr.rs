#[doc = "Register `COMP0_CPU_INT_ICLR` writer"]
pub type W = crate::W<Comp0CpuIntIclrSpec>;
#[doc = "Clears COMPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to COMPIFG is cleared"]
    Clr = 1,
}
impl From<Compifg> for bool {
    #[inline(always)]
    fn from(variant: Compifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPIFG` writer - Clears COMPIFG in RIS register"]
pub type CompifgW<'a, REG> = crate::BitWriter<'a, REG, Compifg>;
impl<'a, REG> CompifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Compifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to COMPIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Compifg::Clr)
    }
}
#[doc = "Clears COMPINVIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compinvifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to COMPINVIFG is cleared"]
    Clr = 1,
}
impl From<Compinvifg> for bool {
    #[inline(always)]
    fn from(variant: Compinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPINVIFG` writer - Clears COMPINVIFG in RIS register"]
pub type CompinvifgW<'a, REG> = crate::BitWriter<'a, REG, Compinvifg>;
impl<'a, REG> CompinvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Compinvifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to COMPINVIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Compinvifg::Clr)
    }
}
#[doc = "Clears OUTRDYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outrdyifg {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: RIS bit corresponding to OUTRDYIFG is cleared"]
    Clr = 1,
}
impl From<Outrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Outrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTRDYIFG` writer - Clears OUTRDYIFG in RIS register"]
pub type OutrdyifgW<'a, REG> = crate::BitWriter<'a, REG, Outrdyifg>;
impl<'a, REG> OutrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Outrdyifg::NoEffect)
    }
    #[doc = "RIS bit corresponding to OUTRDYIFG is cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Outrdyifg::Clr)
    }
}
impl W {
    #[doc = "Bit 1 - Clears COMPIFG in RIS register"]
    #[inline(always)]
    pub fn compifg(&mut self) -> CompifgW<'_, Comp0CpuIntIclrSpec> {
        CompifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears COMPINVIFG in RIS register"]
    #[inline(always)]
    pub fn compinvifg(&mut self) -> CompinvifgW<'_, Comp0CpuIntIclrSpec> {
        CompinvifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears OUTRDYIFG in RIS register"]
    #[inline(always)]
    pub fn outrdyifg(&mut self) -> OutrdyifgW<'_, Comp0CpuIntIclrSpec> {
        OutrdyifgW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_cpu_int_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0CpuIntIclrSpec;
impl crate::RegisterSpec for Comp0CpuIntIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`comp0_cpu_int_iclr::W`](W) writer structure"]
impl crate::Writable for Comp0CpuIntIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CPU_INT_ICLR to value 0"]
impl crate::Resettable for Comp0CpuIntIclrSpec {}
