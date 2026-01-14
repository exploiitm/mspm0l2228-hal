#[doc = "Register `TIMG12_LOAD` reader"]
pub type R = crate::R<Timg12LoadSpec>;
#[doc = "Register `TIMG12_LOAD` writer"]
pub type W = crate::W<Timg12LoadSpec>;
#[doc = "Field `LD` reader - Load Value"]
pub type LdR = crate::FieldReader<u32>;
#[doc = "Field `LD` writer - Load Value"]
pub type LdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Load Value"]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Load Value"]
    #[inline(always)]
    pub fn ld(&mut self) -> LdW<'_, Timg12LoadSpec> {
        LdW::new(self, 0)
    }
}
#[doc = "Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg12_load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg12_load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg12LoadSpec;
impl crate::RegisterSpec for Timg12LoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg12_load::R`](R) reader structure"]
impl crate::Readable for Timg12LoadSpec {}
#[doc = "`write(|w| ..)` method takes [`timg12_load::W`](W) writer structure"]
impl crate::Writable for Timg12LoadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG12_LOAD to value 0"]
impl crate::Resettable for Timg12LoadSpec {}
