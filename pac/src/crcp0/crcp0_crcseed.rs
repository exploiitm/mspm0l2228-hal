#[doc = "Register `CRCP0_CRCSEED` writer"]
pub type W = crate::W<Crcp0CrcseedSpec>;
#[doc = "Field `SEED` writer - Seed Data"]
pub type SeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Seed Data"]
    #[inline(always)]
    pub fn seed(&mut self) -> SeedW<'_, Crcp0CrcseedSpec> {
        SeedW::new(self, 0)
    }
}
#[doc = "CRC Seed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcseed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crcp0CrcseedSpec;
impl crate::RegisterSpec for Crcp0CrcseedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcp0_crcseed::W`](W) writer structure"]
impl crate::Writable for Crcp0CrcseedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCP0_CRCSEED to value 0"]
impl crate::Resettable for Crcp0CrcseedSpec {}
