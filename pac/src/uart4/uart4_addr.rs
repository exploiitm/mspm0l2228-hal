#[doc = "Register `UART4_ADDR` reader"]
pub type R = crate::R<Uart4AddrSpec>;
#[doc = "Register `UART4_ADDR` writer"]
pub type W = crate::W<Uart4AddrSpec>;
#[doc = "Field `ADDR` reader - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Uart4AddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart4AddrSpec;
impl crate::RegisterSpec for Uart4AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart4_addr::R`](R) reader structure"]
impl crate::Readable for Uart4AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart4_addr::W`](W) writer structure"]
impl crate::Writable for Uart4AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART4_ADDR to value 0"]
impl crate::Resettable for Uart4AddrSpec {}
