#[doc = "Register `TIMA0_PL` reader"]
pub type R = crate::R<Tima0PlSpec>;
#[doc = "Register `TIMA0_PL` writer"]
pub type W = crate::W<Tima0PlSpec>;
#[doc = "Field `PHASE` reader - Phase Load value"]
pub type PhaseR = crate::FieldReader<u16>;
#[doc = "Field `PHASE` writer - Phase Load value"]
pub type PhaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Phase Load value"]
    #[inline(always)]
    pub fn phase(&self) -> PhaseR {
        PhaseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Phase Load value"]
    #[inline(always)]
    pub fn phase(&mut self) -> PhaseW<'_, Tima0PlSpec> {
        PhaseW::new(self, 0)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_pl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_pl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0PlSpec;
impl crate::RegisterSpec for Tima0PlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_pl::R`](R) reader structure"]
impl crate::Readable for Tima0PlSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_pl::W`](W) writer structure"]
impl crate::Writable for Tima0PlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_PL to value 0"]
impl crate::Resettable for Tima0PlSpec {}
