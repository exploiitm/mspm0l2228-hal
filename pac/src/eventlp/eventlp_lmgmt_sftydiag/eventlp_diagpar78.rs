#[doc = "Register `EVENTLP_DIAGPAR78` writer"]
pub type W = crate::W<EventlpDiagpar78Spec>;
#[doc = "Writing a 1 will cause the safety diagnostic logic to generate a diagnostic check.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assertdiag {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Generate diagnostic check"]
    Start = 1,
}
impl From<Assertdiag> for bool {
    #[inline(always)]
    fn from(variant: Assertdiag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSERTDIAG` writer - Writing a 1 will cause the safety diagnostic logic to generate a diagnostic check."]
pub type AssertdiagW<'a, REG> = crate::BitWriter<'a, REG, Assertdiag>;
impl<'a, REG> AssertdiagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Assertdiag::NoEffect)
    }
    #[doc = "Generate diagnostic check"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Assertdiag::Start)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 will cause the safety diagnostic logic to generate a diagnostic check."]
    #[inline(always)]
    pub fn assertdiag(&mut self) -> AssertdiagW<'_, EventlpDiagpar78Spec> {
        AssertdiagW::new(self, 0)
    }
}
#[doc = "Diagnostic Parity Register 78\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_diagpar78::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpDiagpar78Spec;
impl crate::RegisterSpec for EventlpDiagpar78Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eventlp_diagpar78::W`](W) writer structure"]
impl crate::Writable for EventlpDiagpar78Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_DIAGPAR78 to value 0"]
impl crate::Resettable for EventlpDiagpar78Spec {}
