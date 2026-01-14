#[doc = "Register `TIMG4_CTTRIG` writer"]
pub type W = crate::W<Timg4CttrigSpec>;
#[doc = "Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Cross trigger generation disabled"]
    Disabled = 0,
    #[doc = "1: Generate Cross trigger pulse"]
    Generate = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` writer - Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance."]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cross trigger generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::Disabled)
    }
    #[doc = "Generate Cross trigger pulse"]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::Generate)
    }
}
impl W {
    #[doc = "Bit 0 - Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance."]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, Timg4CttrigSpec> {
        TrigW::new(self, 0)
    }
}
#[doc = "Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg4_cttrig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg4CttrigSpec;
impl crate::RegisterSpec for Timg4CttrigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timg4_cttrig::W`](W) writer structure"]
impl crate::Writable for Timg4CttrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG4_CTTRIG to value 0"]
impl crate::Resettable for Timg4CttrigSpec {}
