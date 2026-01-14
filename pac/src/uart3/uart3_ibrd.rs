#[doc = "Register `UART3_IBRD` reader"]
pub type R = crate::R<Uart3IbrdSpec>;
#[doc = "Register `UART3_IBRD` writer"]
pub type W = crate::W<Uart3IbrdSpec>;
#[doc = "Field `DIVINT` reader - Integer Baud-Rate Divisor"]
pub type DivintR = crate::FieldReader<u16>;
#[doc = "Field `DIVINT` writer - Integer Baud-Rate Divisor"]
pub type DivintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Integer Baud-Rate Divisor"]
    #[inline(always)]
    pub fn divint(&self) -> DivintR {
        DivintR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Integer Baud-Rate Divisor"]
    #[inline(always)]
    pub fn divint(&mut self) -> DivintW<'_, Uart3IbrdSpec> {
        DivintW::new(self, 0)
    }
}
#[doc = "UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_ibrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_ibrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart3IbrdSpec;
impl crate::RegisterSpec for Uart3IbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart3_ibrd::R`](R) reader structure"]
impl crate::Readable for Uart3IbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`uart3_ibrd::W`](W) writer structure"]
impl crate::Writable for Uart3IbrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART3_IBRD to value 0"]
impl crate::Resettable for Uart3IbrdSpec {}
