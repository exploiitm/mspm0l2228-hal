#[doc = "Register `UART0_LINC0` reader"]
pub type R = crate::R<Uart0Linc0Spec>;
#[doc = "Register `UART0_LINC0` writer"]
pub type W = crate::W<Uart0Linc0Spec>;
#[doc = "Field `DATA` reader - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Uart0Linc0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_linc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_linc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0Linc0Spec;
impl crate::RegisterSpec for Uart0Linc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_linc0::R`](R) reader structure"]
impl crate::Readable for Uart0Linc0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart0_linc0::W`](W) writer structure"]
impl crate::Writable for Uart0Linc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART0_LINC0 to value 0"]
impl crate::Resettable for Uart0Linc0Spec {}
