#[doc = "Register `DMA_DMASA` reader"]
pub type R = crate::R<DmaDmasaSpec>;
#[doc = "Register `DMA_DMASA` writer"]
pub type W = crate::W<DmaDmasaSpec>;
#[doc = "Field `ADDR` reader - DMA Channel Source Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - DMA Channel Source Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Channel Source Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Channel Source Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DmaDmasaSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "DMA Channel Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmasa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmasa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDmasaSpec;
impl crate::RegisterSpec for DmaDmasaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_dmasa::R`](R) reader structure"]
impl crate::Readable for DmaDmasaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_dmasa::W`](W) writer structure"]
impl crate::Writable for DmaDmasaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_DMASA to value 0"]
impl crate::Resettable for DmaDmasaSpec {}
