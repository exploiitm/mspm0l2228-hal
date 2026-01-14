#[doc = "Register `UART2_AMASK` reader"]
pub type R = crate::R<Uart2AmaskSpec>;
#[doc = "Register `UART2_AMASK` writer"]
pub type W = crate::W<Uart2AmaskSpec>;
#[doc = "Field `MSK` reader - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
pub type MskR = crate::FieldReader;
#[doc = "Field `MSK` writer - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
pub type MskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
    #[inline(always)]
    pub fn msk(&self) -> MskR {
        MskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
    #[inline(always)]
    pub fn msk(&mut self) -> MskW<'_, Uart2AmaskSpec> {
        MskW::new(self, 0)
    }
}
#[doc = "Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_amask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_amask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2AmaskSpec;
impl crate::RegisterSpec for Uart2AmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_amask::R`](R) reader structure"]
impl crate::Readable for Uart2AmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`uart2_amask::W`](W) writer structure"]
impl crate::Writable for Uart2AmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART2_AMASK to value 0xff"]
impl crate::Resettable for Uart2AmaskSpec {
    const RESET_VALUE: u32 = 0xff;
}
