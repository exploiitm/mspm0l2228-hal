#[doc = "Register `UART3_TXDATA` reader"]
pub type R = crate::R<Uart3TxdataSpec>;
#[doc = "Register `UART3_TXDATA` writer"]
pub type W = crate::W<Uart3TxdataSpec>;
#[doc = "Field `DATA` reader - Data Transmitted or Received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data Transmitted or Received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Transmitted or Received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Transmitted or Received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Uart3TxdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_txdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_txdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart3TxdataSpec;
impl crate::RegisterSpec for Uart3TxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart3_txdata::R`](R) reader structure"]
impl crate::Readable for Uart3TxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`uart3_txdata::W`](W) writer structure"]
impl crate::Writable for Uart3TxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART3_TXDATA to value 0"]
impl crate::Resettable for Uart3TxdataSpec {}
