#[doc = "Register `TIMG0_LOAD` reader"]
pub type R = crate::R<Timg0LoadSpec>;
#[doc = "Register `TIMG0_LOAD` writer"]
pub type W = crate::W<Timg0LoadSpec>;
#[doc = "Field `LD` reader - Load Value"]
pub type LdR = crate::FieldReader<u16>;
#[doc = "Field `LD` writer - Load Value"]
pub type LdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Load Value"]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Load Value"]
    #[inline(always)]
    pub fn ld(&mut self) -> LdW<'_, Timg0LoadSpec> {
        LdW::new(self, 0)
    }
}
#[doc = "Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg0_load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg0_load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg0LoadSpec;
impl crate::RegisterSpec for Timg0LoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg0_load::R`](R) reader structure"]
impl crate::Readable for Timg0LoadSpec {}
#[doc = "`write(|w| ..)` method takes [`timg0_load::W`](W) writer structure"]
impl crate::Writable for Timg0LoadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG0_LOAD to value 0"]
impl crate::Resettable for Timg0LoadSpec {}
