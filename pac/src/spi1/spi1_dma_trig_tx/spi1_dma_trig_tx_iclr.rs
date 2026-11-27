#[doc = "Register `SPI1_DMA_TRIG_TX_ICLR` writer"]
pub type W = crate::W<Spi1DmaTrigTxIclrSpec>;
#[doc = "Clear Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` writer - Clear Transmit FIFO event."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::NoEffect)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Clr)
    }
}
impl W {
    #[doc = "Bit 4 - Clear Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, Spi1DmaTrigTxIclrSpec> {
        TxW::new(self, 4)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_tx_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1DmaTrigTxIclrSpec;
impl crate::RegisterSpec for Spi1DmaTrigTxIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi1_dma_trig_tx_iclr::W`](W) writer structure"]
impl crate::Writable for Spi1DmaTrigTxIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_DMA_TRIG_TX_ICLR to value 0"]
impl crate::Resettable for Spi1DmaTrigTxIclrSpec {}
