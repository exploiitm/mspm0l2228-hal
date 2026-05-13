#[doc = "Register `UART0_LINCNT` reader"]
pub type R = crate::R<Uart0LincntSpec>;
#[doc = "Register `UART0_LINCNT` writer"]
pub type W = crate::W<Uart0LincntSpec>;
#[doc = "Field `LINCNT` reader - 16 bit up counter clocked by the functional clock of the UART."]
pub type LincntR = crate::FieldReader<u16>;
#[doc = "Field `LINCNT` writer - 16 bit up counter clocked by the functional clock of the UART."]
pub type LincntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit up counter clocked by the functional clock of the UART."]
    #[inline(always)]
    pub fn lincnt(&self) -> LincntR {
        LincntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 bit up counter clocked by the functional clock of the UART."]
    #[inline(always)]
    pub fn lincnt(&mut self) -> LincntW<'_, Uart0LincntSpec> {
        LincntW::new(self, 0)
    }
}
#[doc = "UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_lincnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_lincnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0LincntSpec;
impl crate::RegisterSpec for Uart0LincntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_lincnt::R`](R) reader structure"]
impl crate::Readable for Uart0LincntSpec {}
#[doc = "`write(|w| ..)` method takes [`uart0_lincnt::W`](W) writer structure"]
impl crate::Writable for Uart0LincntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART0_LINCNT to value 0"]
impl crate::Resettable for Uart0LincntSpec {}
