#[doc = "Register `SPI0_DMA_TRIG_TX_ISET` writer"]
pub type W = crate::W<Spi0DmaTrigTxIsetSpec>;
#[doc = "Set Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` writer - Set Transmit FIFO event."]
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
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Set)
    }
}
impl W {
    #[doc = "Bit 4 - Set Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, Spi0DmaTrigTxIsetSpec> {
        TxW::new(self, 4)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_tx_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0DmaTrigTxIsetSpec;
impl crate::RegisterSpec for Spi0DmaTrigTxIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi0_dma_trig_tx_iset::W`](W) writer structure"]
impl crate::Writable for Spi0DmaTrigTxIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0_DMA_TRIG_TX_ISET to value 0"]
impl crate::Resettable for Spi0DmaTrigTxIsetSpec {}
