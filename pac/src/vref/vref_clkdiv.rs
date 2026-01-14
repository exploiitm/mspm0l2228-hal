#[doc = "Register `VREF_CLKDIV` reader"]
pub type R = crate::R<VrefClkdivSpec>;
#[doc = "Register `VREF_CLKDIV` writer"]
pub type W = crate::W<VrefClkdivSpec>;
#[doc = "Field `RATIO` reader - Selects divide ratio of module clock to be used in sample and hold logic"]
pub type RatioR = crate::FieldReader;
#[doc = "Field `RATIO` writer - Selects divide ratio of module clock to be used in sample and hold logic"]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock to be used in sample and hold logic"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock to be used in sample and hold logic"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RatioW<'_, VrefClkdivSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefClkdivSpec;
impl crate::RegisterSpec for VrefClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_clkdiv::R`](R) reader structure"]
impl crate::Readable for VrefClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`vref_clkdiv::W`](W) writer structure"]
impl crate::Writable for VrefClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREF_CLKDIV to value 0"]
impl crate::Resettable for VrefClkdivSpec {}
