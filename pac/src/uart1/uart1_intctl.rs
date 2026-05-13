#[doc = "Register `UART1_INTCTL` reader"]
pub type R = crate::R<Uart1IntctlSpec>;
#[doc = "Register `UART1_INTCTL` writer"]
pub type W = crate::W<Uart1IntctlSpec>;
#[doc = "Writing a 1 to this field re-evaluates the interrupt sources.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inteval {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Eval = 1,
}
impl From<Inteval> for bool {
    #[inline(always)]
    fn from(variant: Inteval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEVAL` writer - Writing a 1 to this field re-evaluates the interrupt sources."]
pub type IntevalW<'a, REG> = crate::BitWriter<'a, REG, Inteval>;
impl<'a, REG> IntevalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inteval::Disable)
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn eval(self) -> &'a mut crate::W<REG> {
        self.variant(Inteval::Eval)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this field re-evaluates the interrupt sources."]
    #[inline(always)]
    pub fn inteval(&mut self) -> IntevalW<'_, Uart1IntctlSpec> {
        IntevalW::new(self, 0)
    }
}
#[doc = "Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_intctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_intctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1IntctlSpec;
impl crate::RegisterSpec for Uart1IntctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_intctl::R`](R) reader structure"]
impl crate::Readable for Uart1IntctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_intctl::W`](W) writer structure"]
impl crate::Writable for Uart1IntctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_INTCTL to value 0"]
impl crate::Resettable for Uart1IntctlSpec {}
