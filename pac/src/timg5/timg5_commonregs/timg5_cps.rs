#[doc = "Register `TIMG5_CPS` reader"]
pub type R = crate::R<Timg5CpsSpec>;
#[doc = "Register `TIMG5_CPS` writer"]
pub type W = crate::W<Timg5CpsSpec>;
#[doc = "Field `PCNT` reader - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
pub type PcntR = crate::FieldReader;
#[doc = "Field `PCNT` writer - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PcntW<'_, Timg5CpsSpec> {
        PcntW::new(self, 0)
    }
}
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_cps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_cps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg5CpsSpec;
impl crate::RegisterSpec for Timg5CpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg5_cps::R`](R) reader structure"]
impl crate::Readable for Timg5CpsSpec {}
#[doc = "`write(|w| ..)` method takes [`timg5_cps::W`](W) writer structure"]
impl crate::Writable for Timg5CpsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG5_CPS to value 0"]
impl crate::Resettable for Timg5CpsSpec {}
