#[doc = "Register `UART4_IBRD` reader"]
pub type R = crate::R<Uart4IbrdSpec>;
#[doc = "Register `UART4_IBRD` writer"]
pub type W = crate::W<Uart4IbrdSpec>;
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
    pub fn divint(&mut self) -> DivintW<'_, Uart4IbrdSpec> {
        DivintW::new(self, 0)
    }
}
#[doc = "UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_ibrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_ibrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart4IbrdSpec;
impl crate::RegisterSpec for Uart4IbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart4_ibrd::R`](R) reader structure"]
impl crate::Readable for Uart4IbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`uart4_ibrd::W`](W) writer structure"]
impl crate::Writable for Uart4IbrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART4_IBRD to value 0"]
impl crate::Resettable for Uart4IbrdSpec {}
