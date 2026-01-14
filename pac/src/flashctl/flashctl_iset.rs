#[doc = "Register `FLASHCTL_ISET` writer"]
pub type W = crate::W<FlashctlIsetSpec>;
#[doc = "0: No effect 1: Set the DONE interrupt in the RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set \\[IPSTANDARD.RIS\\] bit"]
    Set = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - 0: No effect 1: Set the DONE interrupt in the RIS register"]
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
    #[doc = "Set \\[IPSTANDARD.RIS\\] bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Set)
    }
}
impl W {
    #[doc = "Bit 0 - 0: No effect 1: Set the DONE interrupt in the RIS register"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, FlashctlIsetSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlIsetSpec;
impl crate::RegisterSpec for FlashctlIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flashctl_iset::W`](W) writer structure"]
impl crate::Writable for FlashctlIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_ISET to value 0"]
impl crate::Resettable for FlashctlIsetSpec {}
