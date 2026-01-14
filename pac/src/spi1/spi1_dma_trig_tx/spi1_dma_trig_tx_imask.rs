#[doc = "Register `SPI1_DMA_TRIG_TX_IMASK` reader"]
pub type R = crate::R<Spi1DmaTrigTxImaskSpec>;
#[doc = "Register `SPI1_DMA_TRIG_TX_IMASK` writer"]
pub type W = crate::W<Spi1DmaTrigTxImaskSpec>;
#[doc = "Transmit FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Transmit FIFO event mask."]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::Clr,
            true => Tx::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tx::Set
    }
}
#[doc = "Field `TX` writer - Transmit FIFO event mask."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Set)
    }
}
impl R {
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, Spi1DmaTrigTxImaskSpec> {
        TxW::new(self, 4)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_tx_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_tx_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1DmaTrigTxImaskSpec;
impl crate::RegisterSpec for Spi1DmaTrigTxImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_dma_trig_tx_imask::R`](R) reader structure"]
impl crate::Readable for Spi1DmaTrigTxImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`spi1_dma_trig_tx_imask::W`](W) writer structure"]
impl crate::Writable for Spi1DmaTrigTxImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_DMA_TRIG_TX_IMASK to value 0"]
impl crate::Resettable for Spi1DmaTrigTxImaskSpec {}
