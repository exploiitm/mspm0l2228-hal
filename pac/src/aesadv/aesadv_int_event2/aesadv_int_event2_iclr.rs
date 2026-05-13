#[doc = "Register `AESADV_INT_EVENT2_ICLR` writer"]
pub type W = crate::W<AesadvIntEvent2IclrSpec>;
#[doc = "TRIG1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig1 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Trig1> for bool {
    #[inline(always)]
    fn from(variant: Trig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG1` writer - TRIG1 event"]
pub type Trig1W<'a, REG> = crate::BitWriter<'a, REG, Trig1>;
impl<'a, REG> Trig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trig1::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trig1::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG1 event"]
    #[inline(always)]
    pub fn trig1(&mut self) -> Trig1W<'_, AesadvIntEvent2IclrSpec> {
        Trig1W::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent2IclrSpec;
impl crate::RegisterSpec for AesadvIntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for AesadvIntEvent2IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for AesadvIntEvent2IclrSpec {}
