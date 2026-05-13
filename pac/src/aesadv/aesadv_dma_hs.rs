#[doc = "Register `AESADV_DMA_HS` reader"]
pub type R = crate::R<AesadvDmaHsSpec>;
#[doc = "Register `AESADV_DMA_HS` writer"]
pub type W = crate::W<AesadvDmaHsSpec>;
#[doc = "When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDataAck {
    #[doc = "0: Disable DMA based data handshake"]
    DmaDisable = 0,
    #[doc = "1: Enables DMA based handshake"]
    DmaEnable = 1,
}
impl From<DmaDataAck> for bool {
    #[inline(always)]
    fn from(variant: DmaDataAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DATA_ACK` reader - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
pub type DmaDataAckR = crate::BitReader<DmaDataAck>;
impl DmaDataAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDataAck {
        match self.bits {
            false => DmaDataAck::DmaDisable,
            true => DmaDataAck::DmaEnable,
        }
    }
    #[doc = "Disable DMA based data handshake"]
    #[inline(always)]
    pub fn is_dma_disable(&self) -> bool {
        *self == DmaDataAck::DmaDisable
    }
    #[doc = "Enables DMA based handshake"]
    #[inline(always)]
    pub fn is_dma_enable(&self) -> bool {
        *self == DmaDataAck::DmaEnable
    }
}
#[doc = "Field `DMA_DATA_ACK` writer - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
pub type DmaDataAckW<'a, REG> = crate::BitWriter<'a, REG, DmaDataAck>;
impl<'a, REG> DmaDataAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA based data handshake"]
    #[inline(always)]
    pub fn dma_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDataAck::DmaDisable)
    }
    #[doc = "Enables DMA based handshake"]
    #[inline(always)]
    pub fn dma_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDataAck::DmaEnable)
    }
}
impl R {
    #[doc = "Bit 0 - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
    #[inline(always)]
    pub fn dma_data_ack(&self) -> DmaDataAckR {
        DmaDataAckR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
    #[inline(always)]
    pub fn dma_data_ack(&mut self) -> DmaDataAckW<'_, AesadvDmaHsSpec> {
        DmaDataAckW::new(self, 0)
    }
}
#[doc = "Control register for DMA handshaking\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_dma_hs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_dma_hs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvDmaHsSpec;
impl crate::RegisterSpec for AesadvDmaHsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_dma_hs::R`](R) reader structure"]
impl crate::Readable for AesadvDmaHsSpec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_dma_hs::W`](W) writer structure"]
impl crate::Writable for AesadvDmaHsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_DMA_HS to value 0"]
impl crate::Resettable for AesadvDmaHsSpec {}
