#[doc = "Register `WWDT0_WWDTCNTRST` reader"]
pub type R = crate::R<Wwdt0WwdtcntrstSpec>;
#[doc = "Register `WWDT0_WWDTCNTRST` writer"]
pub type W = crate::W<Wwdt0WwdtcntrstSpec>;
#[doc = "Field `RESTART` reader - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
pub type RestartR = crate::FieldReader<u32>;
#[doc = "Field `RESTART` writer - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
pub type RestartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, Wwdt0WwdtcntrstSpec> {
        RestartW::new(self, 0)
    }
}
#[doc = "Window Watchdog Timer Counter Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtcntrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_wwdtcntrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0WwdtcntrstSpec;
impl crate::RegisterSpec for Wwdt0WwdtcntrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_wwdtcntrst::R`](R) reader structure"]
impl crate::Readable for Wwdt0WwdtcntrstSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdt0_wwdtcntrst::W`](W) writer structure"]
impl crate::Writable for Wwdt0WwdtcntrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WWDT0_WWDTCNTRST to value 0"]
impl crate::Resettable for Wwdt0WwdtcntrstSpec {}
