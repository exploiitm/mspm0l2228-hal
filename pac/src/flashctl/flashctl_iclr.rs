#[doc = "Register `FLASHCTL_ICLR` writer"]
pub type W = crate::W<FlashctlIclrSpec>;
#[doc = "0: No effect 1: Clear the DONE interrupt in the RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear \\[IPSTANDARD.RIS\\] bit"]
    Clr = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - 0: No effect 1: Clear the DONE interrupt in the RIS register"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Done::NoEffect)
    }
    #[doc = "Clear \\[IPSTANDARD.RIS\\] bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - 0: No effect 1: Clear the DONE interrupt in the RIS register"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, FlashctlIclrSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlIclrSpec;
impl crate::RegisterSpec for FlashctlIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flashctl_iclr::W`](W) writer structure"]
impl crate::Writable for FlashctlIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_ICLR to value 0"]
impl crate::Resettable for FlashctlIclrSpec {}
