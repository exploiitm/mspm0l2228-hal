#[doc = "Register `TIMA0_CPS` reader"]
pub type R = crate::R<Tima0CpsSpec>;
#[doc = "Register `TIMA0_CPS` writer"]
pub type W = crate::W<Tima0CpsSpec>;
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
    pub fn pcnt(&mut self) -> PcntW<'_, Tima0CpsSpec> {
        PcntW::new(self, 0)
    }
}
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_cps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_cps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0CpsSpec;
impl crate::RegisterSpec for Tima0CpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_cps::R`](R) reader structure"]
impl crate::Readable for Tima0CpsSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_cps::W`](W) writer structure"]
impl crate::Writable for Tima0CpsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_CPS to value 0"]
impl crate::Resettable for Tima0CpsSpec {}
