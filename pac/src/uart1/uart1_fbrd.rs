#[doc = "Register `UART1_FBRD` reader"]
pub type R = crate::R<Uart1FbrdSpec>;
#[doc = "Register `UART1_FBRD` writer"]
pub type W = crate::W<Uart1FbrdSpec>;
#[doc = "Field `DIVFRAC` reader - Fractional Baud-Rate Divisor"]
pub type DivfracR = crate::FieldReader;
#[doc = "Field `DIVFRAC` writer - Fractional Baud-Rate Divisor"]
pub type DivfracW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn divfrac(&self) -> DivfracR {
        DivfracR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn divfrac(&mut self) -> DivfracW<'_, Uart1FbrdSpec> {
        DivfracW::new(self, 0)
    }
}
#[doc = "UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_fbrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_fbrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1FbrdSpec;
impl crate::RegisterSpec for Uart1FbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_fbrd::R`](R) reader structure"]
impl crate::Readable for Uart1FbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_fbrd::W`](W) writer structure"]
impl crate::Writable for Uart1FbrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_FBRD to value 0"]
impl crate::Resettable for Uart1FbrdSpec {}
