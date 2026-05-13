#[doc = "Register `EVENTLP_DIAGIFRST` reader"]
pub type R = crate::R<EventlpDiagifrstSpec>;
#[doc = "Register `EVENTLP_DIAGIFRST` writer"]
pub type W = crate::W<EventlpDiagifrstSpec>;
#[doc = "Writing a 1 will synchronously clear the Diagnostic Interface. The STICKY bits for status and well as the PARFV will be cleared. If a true functional failure still exists, the interface will re-assert FUNCFAIL on the cycle following the interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assertifrst {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clear"]
    Clr = 1,
}
impl From<Assertifrst> for bool {
    #[inline(always)]
    fn from(variant: Assertifrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSERTIFRST` writer - Writing a 1 will synchronously clear the Diagnostic Interface. The STICKY bits for status and well as the PARFV will be cleared. If a true functional failure still exists, the interface will re-assert FUNCFAIL on the cycle following the interface reset"]
pub type AssertifrstW<'a, REG> = crate::BitWriter<'a, REG, Assertifrst>;
impl<'a, REG> AssertifrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Assertifrst::NoEffect)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Assertifrst::Clr)
    }
}
#[doc = "Writing a 1 will synchronously clear the PARFV MMR DPINDEX field. The STICKY bit for DIAGPASS status will be cleared as well.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diagpassclr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clear"]
    Clr = 1,
}
impl From<Diagpassclr> for bool {
    #[inline(always)]
    fn from(variant: Diagpassclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIAGPASSCLR` writer - Writing a 1 will synchronously clear the PARFV MMR DPINDEX field. The STICKY bit for DIAGPASS status will be cleared as well."]
pub type DiagpassclrW<'a, REG> = crate::BitWriter<'a, REG, Diagpassclr>;
impl<'a, REG> DiagpassclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Diagpassclr::NoEffect)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Diagpassclr::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 will synchronously clear the Diagnostic Interface. The STICKY bits for status and well as the PARFV will be cleared. If a true functional failure still exists, the interface will re-assert FUNCFAIL on the cycle following the interface reset"]
    #[inline(always)]
    pub fn assertifrst(&mut self) -> AssertifrstW<'_, EventlpDiagifrstSpec> {
        AssertifrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 will synchronously clear the PARFV MMR DPINDEX field. The STICKY bit for DIAGPASS status will be cleared as well."]
    #[inline(always)]
    pub fn diagpassclr(&mut self) -> DiagpassclrW<'_, EventlpDiagifrstSpec> {
        DiagpassclrW::new(self, 1)
    }
}
#[doc = "Diagnostic Interface Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_diagifrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_diagifrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventlpDiagifrstSpec;
impl crate::RegisterSpec for EventlpDiagifrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventlp_diagifrst::R`](R) reader structure"]
impl crate::Readable for EventlpDiagifrstSpec {}
#[doc = "`write(|w| ..)` method takes [`eventlp_diagifrst::W`](W) writer structure"]
impl crate::Writable for EventlpDiagifrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTLP_DIAGIFRST to value 0"]
impl crate::Resettable for EventlpDiagifrstSpec {}
