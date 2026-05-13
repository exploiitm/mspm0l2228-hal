#[doc = "Register `DMA_DMADA` reader"]
pub type R = crate::R<DmaDmadaSpec>;
#[doc = "Register `DMA_DMADA` writer"]
pub type W = crate::W<DmaDmadaSpec>;
#[doc = "Field `ADDR` reader - DMA Channel Destination Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - DMA Channel Destination Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Channel Destination Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Channel Destination Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DmaDmadaSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "DMA Channel Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmada::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmada::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDmadaSpec;
impl crate::RegisterSpec for DmaDmadaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_dmada::R`](R) reader structure"]
impl crate::Readable for DmaDmadaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_dmada::W`](W) writer structure"]
impl crate::Writable for DmaDmadaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_DMADA to value 0"]
impl crate::Resettable for DmaDmadaSpec {}
